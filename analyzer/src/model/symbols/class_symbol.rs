use uuid::Uuid;
use witcherscript::attribs::{ClassSpecifier, StateSpecifier, ClassAutobindSpecifier};
use super::{MemberVarSymbol, MemberFunctionSymbol, EventSymbol, Symbol, SymbolType, GlobalSymbol, ChildSymbol};


#[derive(Debug, Clone)]
pub struct ClassSymbol {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<ClassSpecifier>,
    pub base_id: Option<Uuid>,
    pub member_vars: Vec<MemberVarSymbol>,
    pub member_funcs: Vec<MemberFunctionSymbol>,
    pub events: Vec<EventSymbol>,
}

impl ClassSymbol {
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

impl Symbol for ClassSymbol {
    const TYPE: SymbolType = SymbolType::Class;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbol for ClassSymbol {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}


#[derive(Debug, Clone)]
pub struct AutobindSymbol {
    class_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<ClassAutobindSpecifier>,
    pub type_id: Uuid,
}

impl AutobindSymbol {
    pub fn new(class_id: Uuid, name: &str, type_id: Uuid) -> Self {
        Self {
            class_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            type_id,
            specifiers: Vec::new()
        }
    }
}

impl Symbol for AutobindSymbol {
    const TYPE: SymbolType = SymbolType::Field;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ChildSymbol for AutobindSymbol {
    fn parent_symbol_id(&self) -> Uuid {
        self.class_id
    }
}




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