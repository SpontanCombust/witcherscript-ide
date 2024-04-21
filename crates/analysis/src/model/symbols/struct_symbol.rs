use std::collections::HashSet;
use lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript::attribs::StructSpecifier;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct StructSymbol {
    path: BasicTypeSymbolPath,
    decl_file_path: AbsPath,
    range: lsp::Range,
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

impl PrimarySymbol for StructSymbol {
    fn decl_file_path(&self) -> &AbsPath {
        &self.decl_file_path
    }
}

impl LocatableSymbol for StructSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl StructSymbol {
    pub fn new(path: BasicTypeSymbolPath, decl_file_path: AbsPath, range: lsp::Range) -> Self {
        Self {
            path,
            range,
            decl_file_path,
            specifiers: HashSet::new()
        }
    }
}