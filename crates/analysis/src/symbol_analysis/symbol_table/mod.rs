use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;
use lsp_types as lsp;
use abs_path::AbsPath;
use crate::symbol_analysis::symbols::*;
use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};


pub mod iter;
use iter::*;

pub mod marcher;


/// Contains information about all scanned symbols. Symbols are identified by their path.
/// On a given unique path only one symbol can be present.
#[derive(Debug, Clone)]
pub struct SymbolTable {
    script_root: Arc<AbsPath>,
    symbols: BTreeMap<SymbolPathBuf, SymbolVariant>,
    /// SymbolPath roots of symbols associated with given local paths in a source tree
    source_path_assocs: HashMap<PathBuf, Vec<SymbolPathBuf>>,
    /// Keeps track of where array type symbols have been referenced
    array_type_refs: HashMap<SymbolPathBuf, HashSet<PathBuf>>
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
    pub fn new(scripts_root: Arc<AbsPath>) -> Self {
        Self {
            script_root: scripts_root,
            symbols: BTreeMap::new(),
            source_path_assocs: HashMap::new(),
            array_type_refs: HashMap::new()
        }
    }

    pub fn script_root(&self) -> &AbsPath {
        &self.script_root
    }

    pub fn script_root_arc(&self) -> Arc<AbsPath> {
        self.script_root.clone()
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

    pub(crate) fn insert_primitive(&mut self, sym: PrimitiveTypeSymbol) {
        if let Some(alias) = &sym.alias {
            self.symbols.insert(alias.to_owned(), sym.clone().into());
        }
        self.symbols.insert(sym.path().to_owned(), sym.into());
    }

    pub(crate) fn insert_array(&mut self, sym: ArrayTypeSymbol, ref_local_source_path: &Path) {
        self.array_type_refs
            .entry(sym.path().to_owned())
            .or_default()
            .insert(ref_local_source_path.to_owned());

        self.symbols.insert(sym.path().to_owned(), sym.into());
    }


    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// If the path is occupied returns Err(PathOccupiedError).
    /// Otherwise if the path was not found returns Ok.
    /// If you only want to know if the path exists in the symbol table without extra info, use [`Self::contains`] instead.
    pub fn test_contains(&self, path: &SymbolPath) -> Result<(), PathOccupiedError> {
        if let Some(occupying) = self.symbols.get(path) {
            Err(PathOccupiedError {
                occupied_path: occupying.path().to_sympath_buf(),
                occupied_location: self.locate(path)
            })
        } else {
            Ok(())
        }
    }

    /// Returns whether the given symbol path exists in the symbol table.
    /// If you want to know more about the occupying symbol, use [`Self::test_contains`] instead.
    #[inline]
    pub fn contains(&self, path: &SymbolPath) -> bool {
        self.symbols.contains_key(path)
    }

    #[inline]
    pub fn get<'a>(&'a self, path: &SymbolPath) -> Option<&'a SymbolVariant> {
        self.symbols.get(path)
    }

    #[inline]
    pub(crate) fn get_mut<'a>(&'a mut self, path: &SymbolPath) -> Option<&'a mut SymbolVariant> {
        self.symbols.get_mut(path)
    }

    #[inline]
    pub fn locate(&self, path: &SymbolPath) -> Option<SymbolLocation> {
        let (_, loc) = self.get_with_location(path)?;
        Some(loc)
    }

    pub fn get_with_location<'a>(&'a self, path: &SymbolPath) -> Option<(&'a SymbolVariant, SymbolLocation)> {
        let local_source_path = path.root()
            .and_then(|root| self.symbols.get(root))
            .and_then(|v| v.local_source_path())?;

        let symvar = self.symbols.get(path)?;
        let range = symvar.range()?;
        let label_range = symvar.label_range()?;

        Some((symvar, SymbolLocation { 
            abs_source_path: self.script_root.join(local_source_path).unwrap(),
            local_source_path: local_source_path.to_owned(), 
            range,
            label_range
        }))
    }
 
    pub fn remove_for_source(&mut self, local_source_path: &Path) {
        let for_removal: Vec<_> = 
            self.get_for_source(local_source_path)
            .map(|sym| sym.path().to_owned())
            .collect();

        for sympath in for_removal {
            self.symbols.remove(&sympath);
        }

        self.source_path_assocs
            .get_mut(local_source_path)
            .map(|assocs| assocs.clear());

        for (_, refs) in self.array_type_refs.iter_mut() {
            refs.remove(local_source_path);
        }
    }


    /// Iterate over direct children of a symbol in a symbol hierarchy.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_children<'a>(&'a self, path: &SymbolPath) -> SymbolChildren<'a> {
        SymbolChildren::new(self, path)
    }

    /// Iterate over direct children of a class symbol in a symbol hierarchy.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_class_children<'a>(&'a self, class_path: &SymbolPath) -> ClassSymbolChildren<'a> {
        ClassSymbolChildren::new(self, class_path)
    }

    /// Iterate over direct children of a state symbol in a symbol hierarchy.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_state_children<'a>(&'a self, state_path: &SymbolPath) -> StateSymbolChildren<'a> {
        StateSymbolChildren::new(self, state_path)
    }

    /// Iterate over direct children of a struct symbol in a symbol hierarchy.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_struct_children<'a>(&'a self, struct_path: &SymbolPath) -> StructSymbolChildren<'a> {
        StructSymbolChildren::new(self, struct_path)
    }

    /// Iterate over direct children of any callable symbol in a symbol hierarchy.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_callable_children<'a>(&'a self, callable_path: &SymbolPath) -> CallableSymbolChildren<'a> {
        CallableSymbolChildren::new(self, callable_path)
    }


    /// Iterate over symbols attributed to a given local source path.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_for_source<'a>(&'a self, local_source_path: &Path) -> FileSymbols<'a> {
        FileSymbols::new(self, local_source_path)
    }


    pub(crate) fn merge(&mut self, mut other: Self) -> Vec<MergeConflictError> {
        let mut errors = Vec::new();
        if other.is_empty() {
            return errors;
        }

        let mut root_children_sympaths = Vec::new();
        for (file_path, sympath_roots) in other.source_path_assocs {
            self.source_path_assocs.entry(file_path.clone())
                .or_default();

            for root in sympath_roots {
                let root_variant = other.symbols.remove(&root).unwrap();
                if let Some(occupying_variant) = self.symbols.get(&root) {
                    if !occupying_variant.path().has_missing() {
                        errors.push(MergeConflictError {
                            occupied_path: occupying_variant.path().to_sympath_buf(),
                            occupied_location: self.locate(&root),
                            incoming_location: SymbolLocation { 
                                abs_source_path: self.script_root.join(&file_path).unwrap(),
                                local_source_path: file_path.to_owned(),
                                range: root_variant.range().unwrap_or_default(),
                                label_range: root_variant.label_range().unwrap_or_default()
                            }
                        });
                    }

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
                        incoming_sympath.clone_into(&mut sympath_to_skip);

                        if !occupying_variant.path().has_missing() {
                            errors.push(MergeConflictError {
                                occupied_path: occupying_variant.path().to_sympath_buf(),
                                occupied_location: self.locate(&incoming_sympath),
                                incoming_location: SymbolLocation { 
                                    abs_source_path: self.script_root.join(&file_path).unwrap(),
                                    local_source_path: file_path.to_owned(), 
                                    range: incoming_variant.range().unwrap_or_default(),
                                    label_range: incoming_variant.label_range().unwrap_or_default()
                                }
                            });
                        }
                    } else {
                        self.symbols.insert(incoming_sympath.to_owned(), incoming_variant);
                        sympath_to_skip.clear();
                    }
                }

                root_children_sympaths.clear();
            }
        }

        // The rest is symbols that cannot be pin-pointed in a file
        // for example, array type symbols
        //
        // Array symbols do not get declared in a normal sense.
        // They get dynamically created when coming accross an array var declaration,
        // so testing for duplicate for an array in perticular doesn't make sense.
        // Instead of that, silently skip those symbols if they're already present.
        let mut sympath_to_skip = SymbolPathBuf::empty();
        for (sympath, symvar) in other.symbols {
            if self.symbols.contains_key(&sympath) {
                sympath.clone_into(&mut sympath_to_skip);
            } else {
                self.symbols.insert(sympath, symvar);
                sympath_to_skip.clear();
            }
        }

        for (array_sympath, array_refs) in other.array_type_refs {
            self.array_type_refs
                .entry(array_sympath)
                .or_default()
                .extend(array_refs);
        }

        errors
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolLocation {
    pub abs_source_path: AbsPath,
    pub local_source_path: PathBuf,
    pub range: lsp::Range,
    pub label_range: lsp::Range
}
