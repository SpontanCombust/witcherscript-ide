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
    pub params: Vec<FunctionParameterSymbol>,
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
            params: Vec::new(),
            return_type_id: ERROR_SYMBOL_ID
        }
    }
}

impl Symbol for GlobalFunctionSymbol {
    const TYPE: SymbolType = SymbolType::Function;

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
    pub params: Vec<FunctionParameterSymbol>,
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
            params: Vec::new(),
            return_type_id: ERROR_SYMBOL_ID
        }
    }

    /// Returns a new instance with all occurances of `type_id` replaced with `substitute_id`.
    /// Used in handling the array type.
    pub fn with_type_substituted(&self, class_id: Uuid, type_id: Uuid, substitute_id: Uuid) -> Self {
        let symbol_id = Uuid::new_v4();

        let subst_params = self.params.iter()
                                .filter(|p| p.type_id == type_id) // doesn't take into account nested types like Nasted<mapping.0> as there are no such cases needed to be checked for now
                                .map(|p| p.with_type_substituted(symbol_id, substitute_id))
                                .collect::<Vec<_>>();

        let subst_ret_type = if self.return_type_id == type_id {
            substitute_id
        } else {
            self.return_type_id
        };

        Self {
            class_id,
            symbol_id,
            params: subst_params,
            return_type_id: subst_ret_type,
            ..self.clone()
        }
    }
}

impl Symbol for MemberFunctionSymbol {
    const TYPE: SymbolType = SymbolType::Method;

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
    pub params: Vec<FunctionParameterSymbol>
}

impl EventSymbol {
    pub fn new(class_id: Uuid, name: &str) -> Self {
        Self {
            class_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            params: Vec::new()
        }
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