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



/// Enum variants/members are not technically primary symbols, but because they are visible in the global scope
/// and can be used anywhere as unqualified identifiers, we treat them as such. Enums in WS seem to behave just like in C. 
/// In that they are an exception to the symbol path system - you can't query for members of an enum by checking the path.
#[derive(Debug, Clone)]
pub struct EnumVariantSymbol {
    path: GlobalDataSymbolPath,
    local_source_path: PathBuf,
    range: lsp::Range,
    label_range: lsp::Range,
    pub parent_enum_path: BasicTypeSymbolPath,
    pub value: i32
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

impl PrimarySymbol for EnumVariantSymbol {
    fn local_source_path(&self) -> &Path {
        &self.local_source_path
    }
}

impl EnumVariantSymbol {
    pub fn new(path: GlobalDataSymbolPath, local_source_path: PathBuf, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            local_source_path,
            range,
            label_range,
            parent_enum_path: BasicTypeSymbolPath::unknown(),
            value: 0
        }
    }
}