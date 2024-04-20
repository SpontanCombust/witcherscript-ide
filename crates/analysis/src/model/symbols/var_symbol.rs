use std::collections::HashSet;
use witcherscript::attribs::MemberVarSpecifier;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


#[derive(Debug, Clone)]
pub struct MemberVarSymbol {
    path: DataSymbolPath,
    pub specifiers: HashSet<MemberVarSpecifier>,
    pub type_path: TypeSymbolPath,
    pub ordinal: usize // used in the context of struct constructors
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
            type_path: TypeSymbolPath::empty(),
            ordinal: 0
        }
    }
}



#[derive(Debug, Clone)]
pub struct LocalVarSymbol {
    path: DataSymbolPath,
    pub type_path: TypeSymbolPath,
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
            type_path: TypeSymbolPath::empty()
        }
    }
}



#[derive(Debug, Clone)]
pub struct GlobalVarSymbol {
    path: SymbolPathBuf,
    type_path: BasicTypeSymbolPath
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
    pub fn new(name: &str, type_path: BasicTypeSymbolPath) -> Self {
        Self {
            path: SymbolPathBuf::new(name, SymbolCategory::Data),
            type_path
        }
    }

    pub fn type_path(&self) -> &SymbolPathBuf {
        &self.type_path
    }
}



#[derive(Debug, Clone)]
pub struct SpecialVarSymbol {
    path: SpecialVarSymbolPath,
    type_path: BasicTypeSymbolPath
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialVarSymbolKind {
    This,
    Super,
    Parent,
    VirtualParent
}

impl Symbol for SpecialVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::GlobalVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl SpecialVarSymbol {
    pub fn new(path: SpecialVarSymbolPath, type_path: BasicTypeSymbolPath) -> Self {
        Self {
            path,
            type_path
        }
    }

    pub fn type_path(&self) -> &SymbolPathBuf {
        &self.type_path
    }

    pub fn kind(&self) -> SpecialVarSymbolKind {
        self.path.kind
    }
}
