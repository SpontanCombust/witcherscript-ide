use uuid::Uuid;
use witcherscript::attribs::{ClassSpecifier, ClassAutobindSpecifier};
use super::{MemberVarSymbol, MemberFunctionSymbol, EventSymbol, Symbol, SymbolType, ERROR_SYMBOL_ID};


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

impl Symbol for ClassSymbol {
    const TYPE: SymbolType = SymbolType::Class;

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



#[derive(Debug, Clone)]
pub struct AutobindSymbol {
    class_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<ClassAutobindSpecifier>,
    pub type_id: Uuid,
}

impl AutobindSymbol {
    pub fn new(class_id: Uuid, name: &str) -> Self {
        Self {
            class_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            type_id: ERROR_SYMBOL_ID,
            specifiers: Vec::new()
        }
    }
}

impl Symbol for AutobindSymbol {
    const TYPE: SymbolType = SymbolType::Field;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn symbol_name(&self) -> &str {
        self.name.as_str()
    }

    fn parent_symbol_id(&self) -> Uuid {
        self.class_id
    }
}
