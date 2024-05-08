use std::collections::{btree_map, BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use thiserror::Error;
use lsp_types as lsp;
use crate::model::symbols::*;
use crate::model::symbol_variant::SymbolVariant;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};

//TODO move symbols stuff into dedicated package, also diagnostics
//TODO some sort of type that will allow searching through symtabs of the entire dependency tree
/// Contains information about all scanned symbols. Symbols are identified by their path.
/// On a given unique path only one symbol can be present.
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    symbols: BTreeMap<SymbolPathBuf, SymbolVariant>,
    /// SymbolPath roots of symbols associated with given local paths in a source tree
    source_path_assocs: HashMap<PathBuf, Vec<SymbolPathBuf>>
}

#[derive(Debug, Clone, Error)]
#[error("symbol path already occupied")]
pub struct PathOccupiedError {
    pub occupied_path: SymbolPathBuf,
    pub occupied_location: Option<SymbolLocation>
}

#[derive(Debug, Clone, Error)]
#[error("symbol could not be merged into another a symbol table")]
pub struct MergeConflictError {
    pub occupied_path: SymbolPathBuf,
    pub occupied_location: Option<SymbolLocation>,
    pub incoming_location: SymbolLocation
}


impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }


    pub(crate) fn insert<S>(&mut self, sym: S)
    where S: Symbol + Into<SymbolVariant> {
        self.symbols.insert(sym.path().to_owned(), sym.into());
    }

    pub(crate) fn insert_primary<S>(&mut self, sym: S)
    where S: PrimarySymbol + LocatableSymbol + Into<SymbolVariant> {
        self.source_path_assocs.entry(sym.local_source_path().to_owned())
            .or_default()
            .push(sym.path().to_owned());

        self.symbols.insert(sym.path().to_owned(), sym.into());
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    pub fn contains(&self, path: &SymbolPath) -> Result<(), PathOccupiedError> {
        if let Some(occupying) = self.symbols.get(path) {
            let occupying_sym = occupying.as_dyn();
            Err(PathOccupiedError {
                occupied_path: occupying_sym.path().to_sympath_buf(),
                occupied_location: self.locate(path)
            })
        } else {
            Ok(())
        }
    }

    pub fn get(&self, path: &SymbolPath) -> Option<&SymbolVariant> {
        self.symbols.get(path)
    }

    pub(crate) fn get_mut(&mut self, path: &SymbolPath) -> Option<&mut SymbolVariant> {
        self.symbols.get_mut(path)
    }

    pub fn locate(&self, path: &SymbolPath) -> Option<SymbolLocation> {
        let local_source_path = path.root()
            .and_then(|root| self.symbols.get(root))
            .and_then(|v| v.local_source_path());

        let label_range = self.symbols.get(path)
            .and_then(|v| v.label_range());

        if let (Some(local_source_path), Some(label_range)) = (local_source_path, label_range) {
            Some(SymbolLocation { 
                local_source_path: local_source_path.to_owned(), 
                label_range
            })
        } else {
            None
        }
    }
 
    pub fn remove_for_source(&mut self, local_source_path: &Path) {
        let for_removal: Vec<_> = 
            self.get_for_source(local_source_path)
            .map(|sym| sym.as_dyn().path().to_owned())
            .collect();

        for sympath in for_removal {
            self.symbols.remove(&sympath);
        }

        self.source_path_assocs
            .get_mut(local_source_path)
            .map(|assocs| assocs.clear());
    }


    /// Iterate over direct children of a symbol in a symbol hierarchy.
    /// Symbols are returned ordered by their symbol path.
    pub fn get_children<'a>(&'a self, path: &SymbolPath) -> SymbolChildren<'a> {
        SymbolChildren::new(self, path)
    }

    /// Iterate over symbols attributed to a given local source path.
    /// Symbols are returned ordered by their symbol path.
    pub fn get_for_source<'a>(&'a self, local_source_path: &Path) -> FileSymbols<'a> {
        FileSymbols::new(self, local_source_path)
    }

    /// Returns an iterator going through all base classes of a given class symbol.
    /// The first symbol is the one pointed to by the starting path (if it points to an existing class symbol).
    pub fn class_hierarchy<'a>(&'a self, sympath: &SymbolPath) -> ClassHierarchy<'a> {
        ClassHierarchy::new(self, sympath)
    }

    /// Iterator going through all base states of a given state symbol.
    /// The first symbol is the one pointed to by the starting path (if it points to an existing state symbol).
    pub fn state_hierarchy<'a>(&'a self, sympath: &SymbolPath) -> StateHierarchy<'a> {
        StateHierarchy::new(self, sympath)
    }



    pub(crate) fn merge(&mut self, mut other: Self) -> HashMap<PathBuf, Vec<MergeConflictError>> {
        let mut errors = HashMap::new();
        if other.is_empty() {
            return errors;
        }

        let mut root_children_sympaths = Vec::new();
        for (file_path, sympath_roots) in other.source_path_assocs {
            let mut file_errors = Vec::new();

            self.source_path_assocs.entry(file_path.clone())
                .or_default();

            for root in &sympath_roots {
                let root_variant = other.symbols.remove(root).unwrap();
                if let Some(occupying_variant) = self.symbols.get(root) {
                    let occupying_sym = occupying_variant.as_dyn();
                    file_errors.push(MergeConflictError {
                        occupied_path: occupying_sym.path().to_sympath_buf(),
                        occupied_location: self.locate(root),
                        incoming_location: SymbolLocation { 
                            local_source_path: file_path.to_owned(), 
                            label_range: root_variant.label_range().unwrap_or_default()
                        }
                    });

                    continue;
                }

                self.symbols.insert(root.to_owned(), root_variant);
                self.source_path_assocs.get_mut(&file_path).unwrap().push(root.to_owned());


                root_children_sympaths.extend(
                    other.symbols.range(root.clone()..)
                    .take_while(|(p, _)| p.starts_with(&root))
                    .map(|(p, _)| p)
                    .cloned());

                let mut sympath_to_skip = SymbolPathBuf::empty();
                for incoming_sympath in root_children_sympaths.iter() {
                    let incoming_variant = other.symbols.remove(incoming_sympath).unwrap();

                    // if some symbol is a duplicate we can skip its children
                    // elements from BTreeMap come in key-ascending order, so we can expect 
                    // possible children symbols to be right after the parent
                    if !sympath_to_skip.is_empty() && incoming_sympath.starts_with(&sympath_to_skip) {
                        continue;
                    }

                    if let Some(occupying_variant) = self.symbols.get(incoming_sympath) {
                        // array symbols do not get declared in a normal sense
                        // they get dynamically created when coming accross an array var declaration
                        // so testing for duplicate for an array in perticular doesn't make sense
                        if occupying_variant.is_array() {
                            continue;
                        }

                        let occupying_sym = occupying_variant.as_dyn();
                        file_errors.push(MergeConflictError {
                            occupied_path: occupying_sym.path().to_sympath_buf(),
                            occupied_location: self.locate(&incoming_sympath),
                            incoming_location: SymbolLocation { 
                                local_source_path: file_path.to_owned(), 
                                label_range: incoming_variant.label_range().unwrap_or_default()
                            }
                        });

                        incoming_sympath.clone_into(&mut sympath_to_skip);
                    } else {
                        self.symbols.insert(incoming_sympath.to_owned(), incoming_variant);
                        sympath_to_skip.clear();
                    }
                }

                root_children_sympaths.clear();
            }

            errors.insert(file_path.clone(), file_errors);
        }

        errors
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolLocation {
    pub local_source_path: PathBuf,
    pub label_range: lsp::Range
}



/// Iterate over direct children of a symbol in a symbol hierarchy
#[derive(Clone)]
pub struct SymbolChildren<'st> {
    iter: btree_map::Range<'st, SymbolPathBuf, SymbolVariant>,
    parent_sympath: SymbolPathBuf,
    children_comp_count: usize
}

impl<'st> SymbolChildren<'st> {
    fn new(symtab: &'st SymbolTable, sympath: &SymbolPath) -> Self {
        Self {
            iter: symtab.symbols.range(sympath.to_owned()..),
            parent_sympath: sympath.to_owned(),
            children_comp_count: sympath.components().count() + 1
        }
    }
}

impl<'st> Iterator for SymbolChildren<'st> {
    type Item = &'st SymbolVariant;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
            .filter(|(sympath, _)| sympath.starts_with(&self.parent_sympath) && sympath.components().count() == self.children_comp_count)
            .map(|(_, variant)| variant)
    }
}


/// Iterate over symbols associated with a script file at a given path
pub struct FileSymbols<'st> {
    iter: Box<dyn Iterator<Item = &'st SymbolVariant> + 'st>
}

impl<'st> FileSymbols<'st> {
    fn new(symtab: &'st SymbolTable, local_source_path: &Path) -> Self {
        let roots = symtab.source_path_assocs
            .get(local_source_path)
            .map(|v| v.as_slice())
            .unwrap_or_default();

        let iter = roots.iter()
            .map(|root| symtab.symbols.range(root.to_owned()..)
                            .take_while(|(p, _)| p.starts_with(root))
                            .map(|(_, v)| v))
            .flatten();

        Self {
            iter: Box::new(iter)
        }
    }
} 

impl<'st> Iterator for FileSymbols<'st> {
    type Item = &'st SymbolVariant;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}


/// Iterator going through all base classes of a given class symbol 
/// starting from the class pointed to by the `start_path` parameter (if it points to an existing class symbol).
#[derive(Clone)]
pub struct ClassHierarchy<'st> {
    symtab: &'st SymbolTable,
    current_path: SymbolPathBuf
}

impl<'st> ClassHierarchy<'st> {
    fn new(symtab: &'st SymbolTable, start_path: &SymbolPath) -> Self {
        Self {
            symtab,
            current_path: start_path.to_owned()
        }
    }
}

impl<'st> Iterator for ClassHierarchy<'st> {
    type Item = &'st ClassSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_path.is_empty() {
            None
        } else if let Some(class) = self.symtab.get(&self.current_path).and_then(|v| v.try_as_class_ref()) {
            self.current_path = class.base_path.as_ref().map(|p| p.clone().into()).unwrap_or_default();
            Some(class)
        } else {
            None
        }
    }
}


/// Iterator going through all base states of a given state symbol
/// starting from the state pointed to by the `start_path` parameter (if it points to an existing state symbol).
#[derive(Clone)]
pub struct StateHierarchy<'st> {
    symtab: &'st SymbolTable,
    current_path: SymbolPathBuf
}

impl<'st> StateHierarchy<'st> {
    fn new(symtab: &'st SymbolTable, start_path: &SymbolPath) -> Self {
        Self {
            symtab,
            current_path: start_path.to_owned()
        }
    }
}

impl<'st> Iterator for StateHierarchy<'st> {
    type Item = &'st StateSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_path.is_empty() {
            None
        } else if let Some(state) = self.symtab.get(&self.current_path).and_then(|v| v.try_as_state_ref()) {
            self.current_path = state.base_state_path.as_ref().map(|p| p.clone().into()).unwrap_or_default();
            Some(state)
        } else {
            None
        }
    }
}