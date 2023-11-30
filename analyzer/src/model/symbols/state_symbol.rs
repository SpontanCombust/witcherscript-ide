use uuid::Uuid;
use witcherscript::attribs::StateSpecifier;
use super::{MemberVarSymbol, MemberFunctionSymbol, SymbolType, Symbol, EventSymbol, AutobindSymbol, ERROR_SYMBOL_ID};


#[derive(Debug, Clone)]
pub struct StateSymbol {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<StateSpecifier>,
    pub parent_id: Uuid,
    pub base_id: Option<Uuid>,
    pub member_var_ids: Vec<Uuid>,
    pub autobind_ids: Vec<Uuid>,
    pub member_func_ids: Vec<Uuid>,
    pub event_ids: Vec<Uuid>,
}

impl StateSymbol {
    /// Use class_name to construct complete name
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            parent_id: ERROR_SYMBOL_ID,
            base_id: None,
            member_var_ids: Vec::new(),
            autobind_ids: Vec::new(),
            member_func_ids: Vec::new(),
            event_ids: Vec::new(),
        }
    }

    pub fn class_name(state_name: &str, parent_name: &str) -> String {
        format!("{}State{}", parent_name, state_name)
    }


    #[must_use]
    pub fn add_member_var(&mut self, name: &str) -> MemberVarSymbol {
        let s = MemberVarSymbol::new(self.symbol_id, name);
        self.member_var_ids.push(s.symbol_id());
        s
    }

    #[must_use]
    pub fn add_autobind(&mut self, name: &str) -> AutobindSymbol {
        let s = AutobindSymbol::new(self.symbol_id, name);
        self.autobind_ids.push(s.symbol_id());
        s
    }

    #[must_use]
    pub fn add_member_func(&mut self, name: &str) -> MemberFunctionSymbol {
        let s = MemberFunctionSymbol::new(self.symbol_id, name);
        self.member_func_ids.push(s.symbol_id());
        s
    }

    #[must_use]
    pub fn add_event(&mut self, name: &str) -> EventSymbol {
        let s = EventSymbol::new(self.symbol_id, name);
        self.event_ids.push(s.symbol_id());
        s
    }
}

impl Symbol for StateSymbol {
    const TYPE: SymbolType = SymbolType::State;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn symbol_name(&self) -> &str {
        self.name.as_str()
    }

    fn parent_symbol_id(&self) -> Uuid {
        self.script_id
    }
}