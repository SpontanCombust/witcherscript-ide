use std::path::{Path, PathBuf};
use lsp_types as lsp;
use witcherscript::attribs::StructSpecifier;
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct StructSymbol {
    path: BasicTypeSymbolPath,
    local_source_path: PathBuf,
    range: lsp::Range,
    label_range: lsp::Range,
    pub specifiers: SymbolSpecifiers<StructSpecifier>
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
    fn local_source_path(&self) -> &Path {
        &self.local_source_path
    }
}

impl LocatableSymbol for StructSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl StructSymbol {
    pub fn new(path: BasicTypeSymbolPath, local_source_path: PathBuf, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            range,
            label_range,
            local_source_path,
            specifiers: SymbolSpecifiers::new()
        }
    }
}


/// Struct constructor
#[derive(Debug, Clone)]
pub struct ConstructorSymbol {
    path: GlobalCallableSymbolPath,
    local_source_path: PathBuf,
    range: lsp::Range,
    label_range: lsp::Range,
    pub parent_type_path: BasicTypeSymbolPath
}

impl Symbol for ConstructorSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Constructor
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl PrimarySymbol for ConstructorSymbol {
    fn local_source_path(&self) -> &Path {
        &self.local_source_path
    }
}

impl LocatableSymbol for ConstructorSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl ConstructorSymbol {
    pub fn new(path: GlobalCallableSymbolPath, local_source_path: PathBuf, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            range,
            label_range,
            local_source_path,
            parent_type_path: BasicTypeSymbolPath::unknown()
        }
    }
}
