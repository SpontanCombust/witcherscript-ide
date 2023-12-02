use uuid::Uuid;
use witcherscript::attribs::{ClassSpecifier, AutobindSpecifier};
use super::{MemberFunctionSymbol, EventSymbol, Symbol, SymbolType, ERROR_SYMBOL_ID, SymbolData, MemberVarSymbol};


#[derive(Debug, Clone, Default)]
pub struct ClassSymbolData {
    pub specifiers: Vec<ClassSpecifier>,
    pub base_id: Option<Uuid>,
    pub member_var_ids: Vec<Uuid>,
    pub autobind_ids: Vec<Uuid>,
    pub member_func_ids: Vec<Uuid>,
    pub event_ids: Vec<Uuid>,
}

impl SymbolData for ClassSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::Class;
}

pub type ClassSymbol = Symbol<ClassSymbolData>;

impl ClassSymbol {
    #[must_use]
    pub fn add_member_var(&mut self, name: &str) -> MemberVarSymbol {
        let s = MemberVarSymbol::new(name, self.id);
        self.data.member_var_ids.push(s.id);
        s
    }

    #[must_use]
    pub fn add_autobind(&mut self, name: &str) -> AutobindSymbol {
        let s = AutobindSymbol::new(name, self.id);
        self.data.autobind_ids.push(s.id);
        s
    }

    #[must_use]
    pub fn add_member_func(&mut self, name: &str) -> MemberFunctionSymbol {
        let s = MemberFunctionSymbol::new(name, self.id);
        self.data.member_func_ids.push(s.id);
        s
    }

    #[must_use]
    pub fn add_event(&mut self, name: &str) -> EventSymbol {
        let s = EventSymbol::new(name, self.id);
        self.data.event_ids.push(s.id);
        s
    }
}



#[derive(Debug, Clone)]
pub struct AutobindSymbolData {
    pub specifiers: Vec<AutobindSpecifier>,
    pub type_id: Uuid,
}

impl Default for AutobindSymbolData {
    fn default() -> Self {
        Self { 
            specifiers: Vec::new(), 
            type_id: ERROR_SYMBOL_ID 
        }
    }
}

impl SymbolData for AutobindSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::Autobind;
}

pub type AutobindSymbol = Symbol<AutobindSymbolData>;