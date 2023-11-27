use uuid::Uuid;
use witcherscript::attribs::StateSpecifier;
use super::{MemberVarSymbol, MemberFunctionSymbol, SymbolType, Symbol, GlobalSymbol, EventSymbol};


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
}

impl Symbol for StateSymbol {
    const TYPE: SymbolType = SymbolType::State;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbol for StateSymbol {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}