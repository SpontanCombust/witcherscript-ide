use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;
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
    source_path_assocs: HashMap<Arc<Path>, Vec<SymbolPathBuf>>,
    /// Keeps track of where array type symbols have been referenced
    array_type_refs: HashMap<SymbolPathBuf, HashSet<PathBuf>>
}

#[derive(Debug, Clone, Error)]
#[error("symbol path already occupied")]
pub struct PathOccupiedError {
    pub occupied_path: SymbolPathBuf,
    pub occupied_typ: SymbolType,
    pub occupied_location: Option<SymbolLocation>,
}

#[derive(Debug, Clone, Error)]
#[error("symbol could not be merged into another a symbol table")]
pub struct MergeConflictError {
    pub occupied_path: SymbolPathBuf,
    pub occupied_typ: SymbolType,
    pub occupied_location: Option<SymbolLocation>,
    pub incoming_location: SymbolLocation,
    pub incoming_typ: SymbolType
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


    pub(crate) fn insert_symbol<S>(&mut self, sym: S)
    where S: Symbol + Into<SymbolVariant> {
        self.symbols.insert(sym.path().to_owned(), sym.into());
    }

    pub(crate) fn insert_primary_symbol<S>(&mut self, sym: S)
    where S: PrimarySymbol + LocatableSymbol + Into<SymbolVariant> {
        self.source_path_assocs.entry(sym.location().local_source_path.clone())
            .or_default()
            .push(sym.path().to_owned());

        self.symbols.insert(sym.path().to_owned(), sym.into());
    }

    pub(crate) fn insert_array_type_symbol(&mut self, sym: ArrayTypeSymbol, ref_local_source_path: &Path) {
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
    /// If you only want to know if the path exists in the symbol table without extra info, use [`Self::contains_symbol`] instead.
    pub fn test_contains_symbol(&self, path: &SymbolPath) -> Result<(), PathOccupiedError> {
        if let Some(occupying) = self.symbols.get(path) {
            Err(PathOccupiedError {
                occupied_path: occupying.path().to_sympath_buf(),
                occupied_location: occupying.location().cloned(),
                occupied_typ: occupying.typ()
            })
        } else {
            Ok(())
        }
    }

    /// Returns whether the given symbol path exists in the symbol table.
    /// If you want to know more about the occupying symbol, use [`Self::test_contains_symbol`] instead.
    #[inline]
    pub fn contains_symbol(&self, path: &SymbolPath) -> bool {
        self.symbols.contains_key(path)
    }

    #[inline]
    pub fn get_symbol<'a>(&'a self, path: &SymbolPath) -> Option<&'a SymbolVariant> {
        self.symbols.get(path)
    }

    #[inline]
    pub fn locate_symbol<'a>(&'a self, path: &SymbolPath) -> Option<&'a SymbolLocation> {
        self.get_symbol(path).and_then(|symvar| symvar.location())
    }

    pub fn get_symbol_with_location<'a>(&'a self, path: &SymbolPath) -> Option<(&'a SymbolVariant, &'a SymbolLocation)> {
        let symvar = self.get_symbol(path)?;
        let loc = symvar.location()?;
        Some((symvar, loc))
    }
 
    pub fn remove_symbols_for_source(&mut self, local_source_path: &Path) {
        let for_removal: Vec<_> = 
            self.get_symbols_for_source(local_source_path)
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

    pub fn dispose_unreferenced_array_symbols(&mut self) {
        let mut for_removal = Vec::new();

        for (array_sympath, refs) in self.array_type_refs.iter() {
            if refs.is_empty() {
                for_removal.push(array_sympath.to_owned());
                for_removal.extend(self.get_symbol_descendants(&array_sympath).map(|v| v.path().to_owned()));
            }
        }

        for sympath in for_removal {
            self.symbols.remove(&sympath);
        }
    }


    pub(crate) fn iter(&self) -> impl Iterator<Item = (&SymbolPath, &SymbolVariant)> {
        self.symbols.iter().map(|(p, v)| (p.as_sympath(), v))
    }


    /// Iterate over direct children of a symbol in a symbol hierarchy.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_symbol_children<'a>(&'a self, path: &SymbolPath) -> SymbolChildren<'a> {
        SymbolChildren::new(self, path)
    }

    /// Iterate over direct children of a symbol in a symbol hierarchy with automatic type conversion.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_symbol_children_filtered<'a, F>(&'a self, sym: &F) -> FilteredSymbolChildren<'a, F>
    where F: ChildrenSymbolsFilter<'a> {
        FilteredSymbolChildren::new(self, sym)
    }

    /// Iterate over all descendants of a symbol in a symbol hierarchy.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_symbol_descendants<'a>(&'a self, path: &SymbolPath) -> SymbolDescendants<'a> {
        SymbolDescendants::new(self, path)
    }


    #[inline]
    pub fn get_primary_symbols_for_source<'a>(&'a self, local_source_path: &Path) -> FilePrimarySymbols<'a> {
        FilePrimarySymbols::new(self, local_source_path)
    }

    /// Iterate over symbols attributed to a given local source path.
    /// Symbols are returned ordered by their symbol path.
    #[inline]
    pub fn get_symbols_for_source<'a>(&'a self, local_source_path: &Path) -> FileSymbols<'a> {
        FileSymbols::new(self, local_source_path)
    }


    pub(crate) fn merge(&mut self, other: Self) -> Vec<MergeConflictError> {
        let mut errors = Vec::new();
        if other.is_empty() {
            return errors;
        }

        let mut sympath_to_skip = SymbolPathBuf::empty();
        for (incoming_sympath, incoming_variant) in other.symbols {
            // if some symbol is a duplicate we can skip its children
            // elements from BTreeMap come in key-ascending order, so we can expect 
            // possible children symbols to be right after the parent
            if !sympath_to_skip.is_empty() && incoming_sympath.starts_with(&sympath_to_skip) {
                continue;
            }

            if let Some(occupying_variant) = self.symbols.get(&incoming_sympath) {
                incoming_sympath.clone_into(&mut sympath_to_skip);

                if occupying_variant.is_array() || occupying_variant.path().has_missing() {
                    continue;
                }

                if let Some(incoming_location) = incoming_variant.location().cloned() {
                    errors.push(MergeConflictError {
                        occupied_path: occupying_variant.path().to_owned(),
                        occupied_typ: occupying_variant.typ(),
                        occupied_location: occupying_variant.location().cloned(),
                        incoming_typ: incoming_variant.typ(),
                        incoming_location,
                    });
                }
            } else {
                self.symbols.insert(incoming_sympath.to_owned(), incoming_variant);
                sympath_to_skip.clear();
            }
        }

        for (local_source_path, assocs) in other.source_path_assocs {
            self.source_path_assocs.entry(local_source_path)
                .or_default()
                .extend(
                    assocs.into_iter()
                    .filter(|sympath| self.symbols.contains_key(sympath))
                )
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
