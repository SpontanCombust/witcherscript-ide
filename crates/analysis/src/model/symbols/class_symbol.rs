use std::{collections::HashSet, path::{Path, PathBuf}};
use lsp_types as lsp;
use witcherscript::attribs::{ClassSpecifier, AutobindSpecifier};
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct ClassSymbol {
    path: BasicTypeSymbolPath,
    local_source_path: PathBuf,
    range: lsp::Range,
    pub specifiers: HashSet<ClassSpecifier>,
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
}

impl ClassSymbol {
    pub fn new(path: BasicTypeSymbolPath, local_source_path: PathBuf, range: lsp::Range) -> Self {
        Self {
            path,
            local_source_path,
            range,
            specifiers: HashSet::new(),
            base_path: None
        }
    }
}



#[derive(Debug, Clone)]
pub struct AutobindSymbol {
    path: DataSymbolPath,
    range: lsp::Range,
    pub specifiers: HashSet<AutobindSpecifier>,
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
}

impl AutobindSymbol {
    pub fn new(path: DataSymbolPath, range: lsp::Range) -> Self {
        Self {
            path,
            range,
            specifiers: HashSet::new(),
            type_path: TypeSymbolPath::empty()
        }
    }
}