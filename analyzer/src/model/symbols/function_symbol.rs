use uuid::Uuid;
use witcherscript::attribs::*;
use super::{FunctionParameterSymbol, Symbol, SymbolType, ERROR_SYMBOL_ID};


#[derive(Debug, Clone)]
pub struct GlobalFunctionSymbol {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<GlobalFunctionSpecifier>,
    pub flavour: Option<GlobalFunctionFlavour>,
    pub param_ids: Vec<Uuid>,
    pub return_type_id: Uuid
}

impl GlobalFunctionSymbol {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            flavour: None,
            param_ids: Vec::new(),
            return_type_id: ERROR_SYMBOL_ID
        }
    }


    #[must_use]
    pub fn add_param(&mut self, name: &str) -> FunctionParameterSymbol {
        let s = FunctionParameterSymbol::new(self.symbol_id, name);
        self.param_ids.push(s.symbol_id());
        s
    }
}

impl Symbol for GlobalFunctionSymbol {
    const TYPE: SymbolType = SymbolType::GlobalFunction;

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
pub struct MemberFunctionSymbol {
    class_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<MemberFunctionSpecifier>,
    pub flavour: Option<MemberFunctionFlavour>,
    pub param_ids: Vec<Uuid>,
    pub return_type_id: Uuid
}

impl MemberFunctionSymbol {
    pub fn new(class_id: Uuid, name: &str) -> Self {
        Self {
            class_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            flavour: None,
            param_ids: Vec::new(),
            return_type_id: ERROR_SYMBOL_ID
        }
    }


    #[must_use]
    pub fn add_param(&mut self, name: &str) -> FunctionParameterSymbol {
        let s = FunctionParameterSymbol::new(self.symbol_id, name);
        self.param_ids.push(s.symbol_id());
        s
    }
}

impl Symbol for MemberFunctionSymbol {
    const TYPE: SymbolType = SymbolType::MemberFunction;

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




#[derive(Debug, Clone)]
pub struct EventSymbol {
    class_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub param_ids: Vec<Uuid>
}

impl EventSymbol {
    pub fn new(class_id: Uuid, name: &str) -> Self {
        Self {
            class_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            param_ids: Vec::new()
        }
    }

    #[must_use]
    pub fn add_param(&mut self, name: &str) -> FunctionParameterSymbol {
        let s = FunctionParameterSymbol::new(self.symbol_id, name);
        self.param_ids.push(s.symbol_id());
        s
    }
}

impl Symbol for EventSymbol {
    const TYPE: SymbolType = SymbolType::Event;

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