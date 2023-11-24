use uuid::Uuid;
use witcherscript::attribs::{ClassSpecifier, StateSpecifier};
use super::{MemberVarInfo, MemberFunctionInfo, EventInfo, SymbolInfo, SymbolType, GlobalSymbolInfo};

pub struct ClassInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<ClassSpecifier>,
    pub base_id: Option<Uuid>,
    pub member_vars: Vec<MemberVarInfo>,
    pub member_funcs: Vec<MemberFunctionInfo>,
    pub events: Vec<EventInfo>,
}

impl ClassInfo {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            base_id: None,
            member_vars: Vec::new(),
            member_funcs: Vec::new(),
            events: Vec::new(),
        }
    }
}

impl SymbolInfo for ClassInfo {
    const TYPE: SymbolType = SymbolType::Class;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbolInfo for ClassInfo {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}



pub struct StateInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<StateSpecifier>,
    pub parent_id: Uuid,
    pub base_id: Option<Uuid>,
    pub member_vars: Vec<MemberVarInfo>,
    pub member_funcs: Vec<MemberFunctionInfo>,
    pub events: Vec<EventInfo>,
}

impl StateInfo {
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

impl SymbolInfo for StateInfo {
    const TYPE: SymbolType = SymbolType::State;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbolInfo for StateInfo {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}