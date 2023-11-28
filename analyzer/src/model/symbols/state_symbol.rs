use uuid::Uuid;
use witcherscript::attribs::StateSpecifier;
use super::{MemberVarSymbol, MemberFunctionSymbol, SymbolType, Symbol, EventSymbol};


#[derive(Debug, Clone)]
pub struct StateSymbol {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<StateSpecifier>,
    pub parent_id: Uuid,
    pub base_id: Option<Uuid>,
    pub member_vars: Vec<MemberVarSymbol>,
    pub member_funcs: Vec<MemberFunctionSymbol>,
    pub events: Vec<EventSymbol>,
}

impl StateSymbol {
    pub fn new(script_id: Uuid, name: &str, parent_id: Uuid) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            parent_id,
            base_id: None,
            member_vars: Vec::new(),
            member_funcs: Vec::new(),
            events: Vec::new(),
        }
    }


    pub fn add_member_var(&mut self, name: &str) -> &mut MemberVarSymbol {
        self.member_vars.push(MemberVarSymbol::new(self.symbol_id, name));
        self.member_vars.last_mut().unwrap()
    }

    pub fn add_member_func(&mut self, name: &str) -> &mut MemberFunctionSymbol {
        self.member_funcs.push(MemberFunctionSymbol::new(self.symbol_id, name));
        self.member_funcs.last_mut().unwrap()
    }

    pub fn add_event(&mut self, name: &str) -> &mut EventSymbol {
        self.events.push(EventSymbol::new(self.symbol_id, name));
        self.events.last_mut().unwrap()
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