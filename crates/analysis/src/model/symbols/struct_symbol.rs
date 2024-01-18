use std::collections::HashSet;
use witcherscript::attribs::StructSpecifier;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct StructSymbol {
    path: BasicTypeSymbolPath,
    pub specifiers: HashSet<StructSpecifier>
}

impl Symbol for StructSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Struct
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl StructSymbol {
    pub fn new(path: BasicTypeSymbolPath) -> Self {
        Self {
            path,
            specifiers: HashSet::new()
        }
    }
}