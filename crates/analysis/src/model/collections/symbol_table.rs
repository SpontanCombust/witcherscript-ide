use std::collections::{btree_map, BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use thiserror::Error;
use lsp_types as lsp;
use crate::model::symbols::*;
use crate::model::symbol_variant::SymbolVariant;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};

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
    pub occupied_type: SymbolType,
    pub occupied_location: Option<SymbolLocation>
}

#[derive(Debug, Clone, Error)]
#[error("symbol could not be merged into another a symbol table")]
pub struct MergeConflictError {
    pub occupied_path: SymbolPathBuf,
    pub occupied_type: SymbolType,
    pub occupied_location: Option<SymbolLocation>,
    pub incoming_type: SymbolType,
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
                occupied_type: occupying_sym.typ(),
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
        path.root()
            .and_then(|root| self.symbols.get(root))
            .and_then(|v| {
                if let (Some(file_path), Some(range)) = (v.local_source_path(), v.range()) {
                    Some(SymbolLocation { 
                        local_source_path: file_path.to_owned(), 
                        range
                    })
                } else {
                    None
                }
            })
    }
 
    pub fn remove_for_source(&mut self, local_source_path: &Path) {
        if let Some(sympaths) = self.source_path_assocs.get_mut(local_source_path) {
            for root in sympaths.iter() {
                self.symbols.retain(|sp, _| !sp.root().map(|r| r == root).unwrap_or(false));
            }
            sympaths.clear();
        }
    }


    /// Iterate over direct children of a symbol in a symbol hierarchy
    pub fn get_children<'a>(&'a self, path: &SymbolPath) -> SymbolChildren<'a> {
        SymbolChildren::new(self, path)
    }

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

        let mut file_sympaths = Vec::new();
        for (file_path, sympath_roots) in other.source_path_assocs {
            let mut file_errors = Vec::new();

            for root in &sympath_roots {
                let range = other.symbols.range(root.clone()..)
                    .take_while(|(p, _)| p.starts_with(&root))
                    .map(|(p, _)| p)
                    .cloned();

                file_sympaths.extend(range);
            }

            let mut file_sympaths_iter = file_sympaths.iter();
            let mut sympath_to_skip = SymbolPathBuf::empty();
            while let Some(incoming_sympath) = file_sympaths_iter.next() {
                let incoming_variant = other.symbols.remove(incoming_sympath).unwrap();

                // if a primary symbol is a duplicate we can skip its children
                // elements from BTreeMap come in key-ascending order, so we can expect 
                // possible children symbols to be right after the parent
                if !sympath_to_skip.is_empty() && incoming_sympath.starts_with(&sympath_to_skip) {
                    continue;
                }

                if let Some(occupying_variant) = self.symbols.get(incoming_sympath) {
                    // array symbols get dynamically injected as their use is encountered
                    // so it gets a special treatment here
                    if occupying_variant.is_array() {
                        continue;
                    }

                    let occupying_sym = occupying_variant.as_dyn();
                    let incoming_sym = incoming_variant.as_dyn();
                    file_errors.push(MergeConflictError {
                        occupied_type: occupying_sym.typ(),
                        occupied_path: occupying_sym.path().to_sympath_buf(),
                        occupied_location: self.locate(&incoming_sympath),
                        incoming_type: incoming_sym.typ(),
                        incoming_location: SymbolLocation { 
                            local_source_path: file_path.clone(), 
                            range: incoming_variant.range().unwrap_or_default()
                        }
                    });

                    sympath_to_skip.clone_from(incoming_sympath);
                } else {
                    self.symbols.insert(incoming_sympath.to_owned(), incoming_variant);
                    sympath_to_skip.clear();
                }
            }

            errors.insert(file_path.clone(), file_errors);
            self.source_path_assocs.insert(file_path, sympath_roots);

            file_sympaths.clear();
        }

        errors
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolLocation {
    pub local_source_path: PathBuf,
    pub range: lsp::Range
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