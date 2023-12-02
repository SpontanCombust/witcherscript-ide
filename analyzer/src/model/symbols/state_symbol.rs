use uuid::Uuid;
use witcherscript::attribs::StateSpecifier;
use super::{MemberFunctionSymbol, SymbolType, Symbol, EventSymbol, AutobindSymbol, ERROR_SYMBOL_ID, MemberVarSymbol, SymbolData};


#[derive(Debug, Clone)]
pub struct StateSymbolData {
    pub specifiers: Vec<StateSpecifier>,
    pub parent_id: Uuid,
    pub base_id: Option<Uuid>,
    pub member_var_ids: Vec<Uuid>,
    pub autobind_ids: Vec<Uuid>,
    pub member_func_ids: Vec<Uuid>,
    pub event_ids: Vec<Uuid>,
}

impl Default for StateSymbolData {
    fn default() -> Self {
        Self { 
            specifiers: Vec::new(), 
            parent_id: ERROR_SYMBOL_ID, 
            base_id: None, 
            member_var_ids: Vec::new(), 
            autobind_ids: Vec::new(), 
            member_func_ids: Vec::new(), 
            event_ids: Vec::new() 
        }
    }
}

impl SymbolData for StateSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::State;
}

pub type StateSymbol = Symbol<StateSymbolData>;

impl StateSymbol {
    /// Use this function to construct a class name for the state complete name
    pub fn class_name(state_name: &str, parent_name: &str) -> String {
        format!("{}State{}", parent_name, state_name)
    }


    #[must_use]
    pub fn add_member_var(&mut self, name: &str) -> MemberVarSymbol {
        let s = MemberVarSymbol::new_with_default(name, self.id);
        self.data.member_var_ids.push(s.id);
        s
    }

    #[must_use]
    pub fn add_autobind(&mut self, name: &str) -> AutobindSymbol {
        let s = AutobindSymbol::new_with_default(name, self.id);
        self.data.autobind_ids.push(s.id);
        s
    }

    #[must_use]
    pub fn add_member_func(&mut self, name: &str) -> MemberFunctionSymbol {
        let s = MemberFunctionSymbol::new_with_default(name, self.id);
        self.data.member_func_ids.push(s.id);
        s
    }

    #[must_use]
    pub fn add_event(&mut self, name: &str) -> EventSymbol {
        let s = EventSymbol::new_with_default(name, self.id);
        self.data.event_ids.push(s.id);
        s
    }
}