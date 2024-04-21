use std::collections::HashMap;
use lsp_types as lsp;
use abs_path::AbsPath;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    path: BasicTypeSymbolPath,
    decl_file_path: AbsPath,
    range: lsp::Range,
    pub variants: HashMap<SymbolPathBuf, EnumVariantSymbol>
}

impl Symbol for EnumSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Enum
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl PrimarySymbol for EnumSymbol {
    fn decl_file_path(&self) -> &AbsPath {
        &self.decl_file_path
    }
}

impl LocatableSymbol for EnumSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl EnumSymbol {
    pub fn new(path: BasicTypeSymbolPath, decl_file_path: AbsPath, range: lsp::Range) -> Self {
        Self {
            path,
            decl_file_path,
            range,
            variants: HashMap::new()
        }
    }
}



#[derive(Debug, Clone)]
pub struct EnumVariantSymbol {
    path: DataSymbolPath,
    range: lsp::Range
}

impl Symbol for EnumVariantSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::EnumVariant
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for EnumVariantSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl EnumVariantSymbol {
    pub fn new(path: DataSymbolPath, range: lsp::Range) -> Self {
        Self {
            path,
            range
        }
    }
}