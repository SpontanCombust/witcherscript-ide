use std::collections::HashSet;
use abs_path::AbsPath;
use witcherscript::attribs::{ClassSpecifier, AutobindSpecifier};
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct ClassSymbol {
    path: BasicTypeSymbolPath,
    decl_file_path: AbsPath,
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

impl ClassSymbol {
    pub fn new(path: BasicTypeSymbolPath, decl_file_path: AbsPath) -> Self {
        Self {
            path,
            decl_file_path,
            specifiers: HashSet::new(),
            base_path: None
        }
    }

    
    pub fn decl_file_path(&self) -> &AbsPath {
        &self.decl_file_path
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