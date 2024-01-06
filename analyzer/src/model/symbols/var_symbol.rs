use std::collections::HashSet;
use witcherscript::attribs::MemberVarSpecifier;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct MemberVarSymbol {
    path: DataSymbolPath,
    pub specifiers: HashSet<MemberVarSpecifier>,
    pub type_path: TypeSymbolPath,
}

impl Symbol for MemberVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl MemberVarSymbol {
    pub fn new(path: DataSymbolPath) -> Self {
        Self {
            path,
            specifiers: HashSet::new(),
            type_path: TypeSymbolPath::empty()
        }
    }
}



#[derive(Debug, Clone)]
pub struct LocalVarSymbol {
    path: DataSymbolPath,
    pub type_path: SymbolPath,
}

impl Symbol for LocalVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::LocalVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocalVarSymbol {
    pub fn new(path: DataSymbolPath) -> Self {
        Self {
            path,
            type_path: SymbolPath::empty()
        }
    }
}



#[derive(Debug, Clone)]
pub struct GlobalVarSymbol {
    path: SymbolPath,
    type_path: SymbolPath
}

impl Symbol for GlobalVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::GlobalVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl GlobalVarSymbol {
    // there is a fixed amount of predefined globals, so a standard 'new' is not required
    pub fn new(name: &str, type_path: SymbolPath) -> Self {
        Self {
            path: SymbolPath::new(name, SymbolCategory::Data),
            type_path
        }
    }

    pub fn type_path(&self) -> &SymbolPath {
        &self.type_path
    }
}
