use std::collections::HashSet;
use uuid::Uuid;
use witcherscript::attribs::MemberVarSpecifier;
use super::{Symbol, SymbolType, ERROR_SYMBOL_ID, SymbolData, NATIVE_SYMBOL_SCRIPT_ID};


#[derive(Debug, Clone)]
pub struct MemberVarSymbolData {
    pub specifiers: HashSet<MemberVarSpecifier>,
    pub type_id: Uuid,
}

impl Default for MemberVarSymbolData {
    fn default() -> Self {
        Self { 
            specifiers: HashSet::new(),
            type_id: ERROR_SYMBOL_ID,
        }
    }
}

impl SymbolData for MemberVarSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::MemberVar;
}

pub type MemberVarSymbol = Symbol<MemberVarSymbolData>;



#[derive(Debug, Clone)]
pub struct LocalVarSymbolData {
    pub type_id: Uuid,
}

impl Default for LocalVarSymbolData {
    fn default() -> Self {
        Self { 
            type_id: ERROR_SYMBOL_ID 
        }
    }
}

impl SymbolData for LocalVarSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::LocalVar;
}

pub type LocalVarSymbol = Symbol<LocalVarSymbolData>;



#[derive(Debug, Clone)]
pub struct GlobalVarSymbolData {
    type_id: Uuid
}

impl GlobalVarSymbolData {
    pub fn type_id(&self) -> Uuid {
        self.type_id
    }
}

impl SymbolData for GlobalVarSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::GlobalVar;
}

pub type GlobalVarSymbol = Symbol<GlobalVarSymbolData>;

impl GlobalVarSymbol {
    pub fn new_with_type(name: &str, type_id: Uuid) -> Self {
        Self::new(name, NATIVE_SYMBOL_SCRIPT_ID, GlobalVarSymbolData { type_id })
    }
}