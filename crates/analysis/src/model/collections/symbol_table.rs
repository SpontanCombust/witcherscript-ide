use std::collections::{BTreeMap, HashMap};
use abs_path::AbsPath;
use thiserror::Error;
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
    pub occupyed_type: SymbolType,
}


impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }


    pub fn insert<S>(&mut self, sym: S)
    where S: Symbol + Into<SymbolVariant> {
        self.symbols.insert(sym.path().to_owned(), sym.into());
    }

    pub fn insert_primary<S>(&mut self, sym: S)
    where S: PrimarySymbol + Into<SymbolVariant> {
        if let Some(assocs) = self.file_assocs.get_mut(sym.decl_file_path()) {
            assocs.push(sym.path().to_owned());
        } else {
            self.file_assocs.insert(sym.decl_file_path().to_owned(), vec![sym.path().to_owned()]);
        }

        self.symbols.insert(sym.path().to_owned(), sym.into());
    }

    pub fn contains(&self, path: &SymbolPath) -> Result<(), PathOccupiedError> {
        if let Some(occupying) = self.symbols.get(path) {
            let occupying_sym = occupying.as_dyn();
            Err(PathOccupiedError {
                occupyed_type: occupying_sym.typ(),
                occupied_path: occupying_sym.path().to_sympath_buf()
            })
        } else {
            Ok(())
        }
    }

    pub fn get(&self, path: &SymbolPath) -> Option<&SymbolVariant> {
        self.symbols.get(path)
    }

    pub fn get_mut(&mut self, path: &SymbolPath) -> Option<&mut SymbolVariant> {
        self.symbols.get_mut(path)
    }

    pub fn remove(&mut self, path: &SymbolPath) -> Option<SymbolVariant> {
        self.symbols.remove(path)
    }

    pub fn get_children<'a, 'b>(&'a self, path: &'b SymbolPath) -> impl Iterator<Item = &'a SymbolVariant> where 'b: 'a {
        let comp_count = path.components().count() + 1;

        self.symbols.range(path.to_sympath_buf()..)
            .take_while(|(p, _)| p.starts_with(path))
            .filter(move |(p, _)| p.components().count() == comp_count)
            .map(|(_, v)| v)
    }
}
