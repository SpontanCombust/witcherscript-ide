use lsp_types as lsp;
use witcherscript::attribs::FunctionParameterSpecifier;
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct FunctionParameterSymbol {
    path: MemberDataSymbolPath,
    range: lsp::Range,
    label_range: lsp::Range,
    pub specifiers: SpecifierBitmask<FunctionParameterSpecifier>,
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

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl FunctionParameterSymbol {
    pub fn new(path: MemberDataSymbolPath, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            range,
            label_range,
            specifiers: SpecifierBitmask::new(),
            type_path: TypeSymbolPath::empty(),
            ordinal: 0
        }
    }
}