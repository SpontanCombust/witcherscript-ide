use std::collections::HashSet;
use uuid::Uuid;
use witcherscript::attribs::*;
use super::{FunctionParameterSymbol, Symbol, SymbolType, ERROR_SYMBOL_ID, SymbolData};


#[derive(Debug, Clone)]
pub struct GlobalFunctionSymbolData {
    pub specifiers: HashSet<GlobalFunctionSpecifier>,
    pub flavour: Option<GlobalFunctionFlavour>,
    pub param_ids: Vec<Uuid>,
    pub return_type_id: Uuid
}

impl Default for GlobalFunctionSymbolData {
    fn default() -> Self {
        Self { 
            specifiers: HashSet::new(), 
            flavour: None, 
            param_ids: Vec::new(), 
            return_type_id: ERROR_SYMBOL_ID
        }
    }
}

impl SymbolData for GlobalFunctionSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::GlobalFunction;
}

pub type GlobalFunctionSymbol = Symbol<GlobalFunctionSymbolData>;

impl GlobalFunctionSymbol {
    #[must_use]
    pub fn add_param(&mut self, name: &str) -> FunctionParameterSymbol {
        let s = FunctionParameterSymbol::new_with_default(name, self.id);
        self.data.param_ids.push(s.id);
        s
    }
}



#[derive(Debug, Clone)]
pub struct MemberFunctionSymbolData {
    pub specifiers: HashSet<MemberFunctionSpecifier>,
    pub flavour: Option<MemberFunctionFlavour>,
    pub param_ids: Vec<Uuid>,
    pub return_type_id: Uuid
}

impl Default for MemberFunctionSymbolData {
    fn default() -> Self {
        Self { 
            specifiers: HashSet::new(), 
            flavour: None, 
            param_ids: Vec::new(), 
            return_type_id: ERROR_SYMBOL_ID
        }
    }
}

impl SymbolData for MemberFunctionSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::MemberFunction;
}

pub type MemberFunctionSymbol = Symbol<MemberFunctionSymbolData>;

impl MemberFunctionSymbol {
    #[must_use]
    pub fn add_param(&mut self, name: &str) -> FunctionParameterSymbol {
        let s = FunctionParameterSymbol::new_with_default(name, self.id);
        self.data.param_ids.push(s.id);
        s
    }
}



#[derive(Debug, Clone, Default)]
pub struct EventSymbolData {
    pub param_ids: Vec<Uuid>
}

impl SymbolData for EventSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::Event;
}

pub type EventSymbol = Symbol<EventSymbolData>;

impl EventSymbol {
    #[must_use]
    pub fn add_param(&mut self, name: &str) -> FunctionParameterSymbol {
        let s = FunctionParameterSymbol::new_with_default(name, self.id);
        self.data.param_ids.push(s.id);
        s
    }
}
