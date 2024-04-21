use std::collections::HashSet;
use lsp_types as lsp;
use witcherscript::attribs::FunctionParameterSpecifier;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct FunctionParameterSymbol {
    path: DataSymbolPath,
    range: lsp::Range,
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

impl LocatableSymbol for FunctionParameterSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl FunctionParameterSymbol {
    pub fn new(path: DataSymbolPath, range: lsp::Range) -> Self {
        Self {
            path,
            range,
            specifiers: HashSet::new(),
            type_path: TypeSymbolPath::empty(),
            ordinal: 0
        }
    }
}