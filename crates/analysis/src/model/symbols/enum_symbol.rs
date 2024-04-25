use std::{collections::HashMap, path::{Path, PathBuf}};
use lsp_types as lsp;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    path: BasicTypeSymbolPath,
    local_source_path: PathBuf,
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
    fn local_source_path(&self) -> &Path {
        &self.local_source_path
    }
}

impl LocatableSymbol for EnumSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl EnumSymbol {
    pub fn new(path: BasicTypeSymbolPath, local_source_path: PathBuf, range: lsp::Range) -> Self {
        Self {
            path,
            local_source_path,
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