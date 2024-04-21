use std::collections::HashSet;
use lsp_types as lsp;
use abs_path::AbsPath;
use witcherscript::attribs::{ClassSpecifier, AutobindSpecifier};
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct ClassSymbol {
    path: BasicTypeSymbolPath,
    decl_file_path: AbsPath,
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
    fn decl_file_path(&self) -> &AbsPath {
        &self.decl_file_path
    }
}

impl LocatableSymbol for ClassSymbol {
    fn range(&self) -> lsp::Range {
        self.range
    }
}

impl ClassSymbol {
    pub fn new(path: BasicTypeSymbolPath, decl_file_path: AbsPath, range: lsp::Range) -> Self {
        Self {
            path,
            decl_file_path,
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