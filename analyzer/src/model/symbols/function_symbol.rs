use std::collections::HashSet;
use witcherscript::attribs::*;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct GlobalFunctionSymbol {
    path: GlobalCallableSymbolPath,
    pub specifiers: HashSet<GlobalFunctionSpecifier>,
    pub flavour: Option<GlobalFunctionFlavour>,
    pub return_type_path: SymbolPath
}

impl Symbol for GlobalFunctionSymbol {
    const SYMBOL_TYPE: SymbolType = SymbolType::GlobalFunction;

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl GlobalFunctionSymbol {
    pub fn new(path: GlobalCallableSymbolPath) -> Self {
        Self {
            path,
            specifiers: HashSet::new(),
            flavour: None,
            return_type_path: SymbolPath::empty()
        }
    }
}



#[derive(Debug, Clone)]
pub struct MemberFunctionSymbol {
    path: MemberCallableSymbolPath,
    pub specifiers: HashSet<MemberFunctionSpecifier>,
    pub flavour: Option<MemberFunctionFlavour>,
    pub return_type_path: SymbolPath
}

impl Symbol for MemberFunctionSymbol {
    const SYMBOL_TYPE: SymbolType = SymbolType::MemberFunction;

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl MemberFunctionSymbol {
    pub fn new(path: MemberCallableSymbolPath) -> Self {
        Self {
            path,
            specifiers: HashSet::new(),
            flavour: None,
            return_type_path: SymbolPath::empty()
        }
    }
}



#[derive(Debug, Clone)]
pub struct EventSymbol {
    path: MemberCallableSymbolPath
}

impl Symbol for EventSymbol {
    const SYMBOL_TYPE: SymbolType = SymbolType::Event;

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl EventSymbol {
    pub fn new(path: MemberCallableSymbolPath) -> Self {
        Self {
            path
        }
    }
}
