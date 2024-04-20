use std::collections::HashSet;
use witcherscript::attribs::FunctionParameterSpecifier;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct FunctionParameterSymbol {
    path: DataSymbolPath,
    pub specifiers: HashSet<FunctionParameterSpecifier>,
    pub type_path: TypeSymbolPath,
    pub ordinal: usize
}

impl Symbol for FunctionParameterSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Parameter
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl FunctionParameterSymbol {
    pub fn new(path: DataSymbolPath) -> Self {
        Self {
            path,
            specifiers: HashSet::new(),
            type_path: TypeSymbolPath::empty(),
            ordinal: 0
        }
    }
}