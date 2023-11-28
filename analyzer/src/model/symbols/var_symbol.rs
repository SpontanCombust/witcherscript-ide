use uuid::Uuid;
use witcherscript::attribs::MemberVarSpecifier;
use super::{Symbol, SymbolType, NATIVE_SYMBOL_SCRIPT_ID, ERROR_SYMBOL_ID};


#[derive(Debug, Clone)]
pub struct MemberVarSymbol {
    owner_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<MemberVarSpecifier>,
    pub type_id: Uuid,
}

impl MemberVarSymbol {
    pub fn new(owner_id: Uuid, name: &str) -> Self {
        Self {
            owner_id,
            symbol_id: Uuid::new_v4(),
            specifiers: Vec::new(),
            name: name.to_owned(),
            type_id: ERROR_SYMBOL_ID,
        }
    }
}

impl Symbol for MemberVarSymbol {
    const TYPE: SymbolType = SymbolType::Field;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn symbol_name(&self) -> &str {
        self.name.as_str()
    }

    fn parent_symbol_id(&self) -> Uuid {
        self.owner_id
    }
}



#[derive(Debug, Clone)]
pub struct LocalVarSymbol {
    func_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub type_id: Uuid,
}

impl LocalVarSymbol {
    pub fn new(func_id: Uuid, name: &str, type_id: Uuid) -> Self {
        Self {
            func_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            type_id
        }
    }
}

impl Symbol for LocalVarSymbol {
    const TYPE: SymbolType = SymbolType::Variable;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn symbol_name(&self) -> &str {
        self.name.as_str()
    }

    fn parent_symbol_id(&self) -> Uuid {
        self.func_id
    }
}



#[derive(Debug, Clone)]
pub struct GlobalVarSymbol {
    symbol_id: Uuid,
    name: String,
    type_id: Uuid
}

impl GlobalVarSymbol {
    pub fn new(name: &str, type_id: Uuid) -> Self {
        Self {
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            type_id
        }
    }


    pub fn type_id(&self) -> Uuid {
        self.type_id
    }
}

impl Symbol for GlobalVarSymbol {
    const TYPE: SymbolType = SymbolType::Variable;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn symbol_name(&self) -> &str {
        self.name.as_str()
    }

    fn parent_symbol_id(&self) -> Uuid {
        NATIVE_SYMBOL_SCRIPT_ID
    }
}
