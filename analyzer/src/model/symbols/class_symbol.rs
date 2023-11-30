use uuid::Uuid;
use witcherscript::attribs::{ClassSpecifier, AutobindSpecifier};
use super::{MemberVarSymbol, MemberFunctionSymbol, EventSymbol, Symbol, SymbolType, ERROR_SYMBOL_ID};


#[derive(Debug, Clone)]
pub struct ClassSymbol {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<ClassSpecifier>,
    pub base_id: Option<Uuid>,
    pub member_var_ids: Vec<Uuid>,
    pub autobind_ids: Vec<Uuid>,
    pub member_func_ids: Vec<Uuid>,
    pub event_ids: Vec<Uuid>,
}

impl ClassSymbol {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            base_id: None,
            member_var_ids: Vec::new(),
            autobind_ids: Vec::new(),
            member_func_ids: Vec::new(),
            event_ids: Vec::new(),
        }
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
    pub specifiers: Vec<AutobindSpecifier>,
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
    const TYPE: SymbolType = SymbolType::Autobind;

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
