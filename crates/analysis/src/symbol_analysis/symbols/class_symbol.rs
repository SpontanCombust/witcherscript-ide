use std::path::{Path, PathBuf};
use lsp_types as lsp;
use witcherscript::attribs::{ClassSpecifier, AutobindSpecifier};
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct ClassSymbol {
    path: BasicTypeSymbolPath,
    local_source_path: PathBuf,
    range: lsp::Range,
    label_range: lsp::Range,
    pub specifiers: SymbolSpecifiers<ClassSpecifier>,
    pub base_path: Option<BasicTypeSymbolPath>
}

impl Symbol for ClassSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Class
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl PrimarySymbol for ClassSymbol {
    fn local_source_path(&self) -> &Path {
        &self.local_source_path
    }
}

impl LocatableSymbol for ClassSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl ClassSymbol {
    pub fn new(path: BasicTypeSymbolPath, local_source_path: PathBuf, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            local_source_path,
            range,
            label_range,
            specifiers: SymbolSpecifiers::new(),
            base_path: None
        }
    }
}



#[derive(Debug, Clone)]
pub struct AutobindSymbol {
    path: MemberDataSymbolPath,
    range: lsp::Range,
    label_range: lsp::Range,
    pub specifiers: SymbolSpecifiers<AutobindSpecifier>,
    pub type_path: TypeSymbolPath,
}

impl Symbol for AutobindSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Autobind
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for AutobindSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }

    fn label_range(&self) -> lsp::Range {
        self.label_range
    }
}

impl AutobindSymbol {
    pub fn new(path: MemberDataSymbolPath, range: lsp::Range, label_range: lsp::Range) -> Self {
        Self {
            path,
            range,
            label_range,
            specifiers: SymbolSpecifiers::new(),
            type_path: TypeSymbolPath::empty()
        }
    }
}