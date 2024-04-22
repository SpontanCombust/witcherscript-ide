use std::collections::{HashMap, BTreeMap};
use thiserror::Error;
use lsp_types as lsp;
use abs_path::AbsPath;
use crate::model::symbols::*;
use crate::model::symbol_variant::SymbolVariant;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};


/// Contains information about all scanned symbols. Symbols are identified by their path.
/// On a given unique path only one symbol can be present.
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    symbols: BTreeMap<SymbolPathBuf, SymbolVariant>,
    /// SymbolPath roots of symbols associated with given files
    file_assocs: HashMap<AbsPath, Vec<SymbolPathBuf>>
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
        self.file_assocs.entry(sym.decl_file_path().to_owned())
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
                if let (Some(file_path), Some(range)) = (v.decl_file_path(), v.range()) {
                    Some(SymbolLocation { 
                        file_path: file_path.to_owned(), 
                        range
                    })
                } else {
                    None
                }
            })
    }
 
    pub fn remove_for_file(&mut self, file_path: &AbsPath) {
        if let Some(sympaths) = self.file_assocs.get_mut(file_path) {
            for root in sympaths.iter() {
                self.symbols.retain(|sp, _| !sp.root().map(|r| r == root).unwrap_or(false));
            }
            sympaths.clear();
        }
    }

    pub fn get_children<'a, 'b>(&'a self, path: &'b SymbolPath) -> impl Iterator<Item = &'a SymbolVariant> where 'b: 'a {
        let comp_count = path.components().count() + 1;

        self.symbols.range(path.to_sympath_buf()..)
            .take_while(|(p, _)| p.starts_with(path))
            .filter(move |(p, _)| p.components().count() == comp_count)
            .map(|(_, v)| v)
    }

    pub(crate) fn merge(&mut self, mut other: Self) -> HashMap<AbsPath, Vec<MergeConflictError>> {
        let mut errors = HashMap::new();
        if other.is_empty() {
            return errors;
        }

        let mut file_sympaths = Vec::new();
        for (file_path, sympath_roots) in other.file_assocs {
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
                if incoming_sympath.starts_with(&sympath_to_skip) {
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
                            file_path: file_path.clone(), 
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
            self.file_assocs.insert(file_path, sympath_roots);

            file_sympaths.clear();
        }

        errors
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolLocation {
    pub file_path: AbsPath,
    pub range: lsp::Range
}