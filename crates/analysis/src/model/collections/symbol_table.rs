use std::collections::BTreeMap;
use thiserror::Error;
use crate::model::symbols::*;
use crate::model::symbol_variant::SymbolVariant;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};


/// Contains information about all scanned symbols. Symbols are identified by their path.
/// On a given unique path only one symbol can be present.
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    map: BTreeMap<SymbolPathBuf, SymbolVariant>
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


    pub fn insert<S>(&mut self, sym: S) -> Result<(), PathOccupiedError> 
    where S: Symbol + Into<SymbolVariant> {
        if let Some(occupying) = self.map.get(sym.path()) {
            let occupying_sym = occupying.as_dyn();
            Err(PathOccupiedError {
                occupyed_type: occupying_sym.typ(),
                occupied_path: occupying_sym.path().to_sympath_buf()
            })
        } else {
            self.map.insert(sym.path().to_sympath_buf(), sym.into());
            Ok(())
        }
    }

    pub fn get(&self, path: &SymbolPath) -> Option<&SymbolVariant> {
        self.map.get(path)
    }

    pub fn get_mut(&mut self, path: &SymbolPath) -> Option<&mut SymbolVariant> {
        self.map.get_mut(path)
    }

    pub fn remove(&mut self, path: &SymbolPath) -> Option<SymbolVariant> {
        self.map.remove(path)
    }

    pub fn get_children<'a, 'b>(&'a self, path: &'b SymbolPath) -> impl Iterator<Item = &'a SymbolVariant> where 'b: 'a {
        let comp_count = path.components().count() + 1;

        self.map.range(path.to_sympath_buf()..)
            .take_while(|(p, _)| p.starts_with(path))
            .filter(move |(p, _)| p.components().count() == comp_count)
            .map(|(_, v)| v)
    }
}
