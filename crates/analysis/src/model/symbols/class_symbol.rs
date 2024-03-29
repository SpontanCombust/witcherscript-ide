use std::collections::HashSet;
use witcherscript::attribs::{ClassSpecifier, AutobindSpecifier};
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct ClassSymbol {
    path: BasicTypeSymbolPath,
    pub specifiers: HashSet<ClassSpecifier>,
    pub base_path: Option<TypeSymbolPath>
}

impl Symbol for ClassSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Class
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl ClassSymbol {
    pub fn new(path: BasicTypeSymbolPath) -> Self {
        Self {
            path,
            specifiers: HashSet::new(),
            base_path: None
        }
    }
}



#[derive(Debug, Clone)]
pub struct AutobindSymbol {
    path: DataSymbolPath,
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

impl AutobindSymbol {
    pub fn new(path: DataSymbolPath) -> Self {
        Self {
            path,
            specifiers: HashSet::new(),
            type_path: TypeSymbolPath::empty()
        }
    }
}