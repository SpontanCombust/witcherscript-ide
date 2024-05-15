use std::path::{Path, PathBuf};
use lsp_types as lsp;
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    path: BasicTypeSymbolPath,
    local_source_path: PathBuf,
    range: lsp::Range,
    label_range: lsp::Range
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

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl EnumSymbol {
    pub fn new(path: BasicTypeSymbolPath, local_source_path: PathBuf, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            local_source_path,
            range,
            label_range
        }
    }
}



#[derive(Debug, Clone)]
pub struct EnumVariantSymbol {
    path: DataSymbolPath,
    range: lsp::Range,
    label_range: lsp::Range
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

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl EnumVariantSymbol {
    pub fn new(path: DataSymbolPath, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            range,
            label_range
        }
    }
}