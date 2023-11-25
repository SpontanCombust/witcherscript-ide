use uuid::Uuid;
use witcherscript::attribs::*;
use super::{FunctionParameterInfo, SymbolInfo, SymbolType, GlobalSymbolInfo, ChildSymbolInfo, ERROR_SYMBOL_ID};


#[derive(Debug, Clone)]
pub struct GlobalFunctionInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<GlobalFunctionSpecifier>,
    pub flavour: Option<GlobalFunctionFlavour>,
    pub params: Vec<FunctionParameterInfo>,
    pub return_type_id: Uuid
}

impl GlobalFunctionInfo {
    pub fn new(script_id: Uuid, name: &str) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            flavour: None,
            params: Vec::new(),
            return_type_id: ERROR_SYMBOL_ID //TODO change to void when that's set as const Uuid
        }
    }
}

impl SymbolInfo for GlobalFunctionInfo {
    const TYPE: SymbolType = SymbolType::Function;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbolInfo for GlobalFunctionInfo {
    fn script_id(&self) -> Uuid {
        self.script_id
    }
}




#[derive(Debug, Clone)]
pub struct MemberFunctionInfo {
    class_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<MemberFunctionSpecifier>,
    pub flavour: Option<MemberFunctionFlavour>,
    pub params: Vec<FunctionParameterInfo>,
    pub return_type_id: Uuid
}

impl MemberFunctionInfo {
    pub fn new(class_id: Uuid, name: &str) -> Self {
        Self {
            class_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            flavour: None,
            params: Vec::new(),
            return_type_id: ERROR_SYMBOL_ID //TODO change to void when that's set as const Uuid
        }
    }

    /// Returns a new instance with all occurances of mapping.0 replaced with mapping.1.
    /// Used in handling the array type.
    pub fn with_type_substituted(&self, class_id: Uuid, mapping: (Uuid, Uuid)) -> Self {
        let symbol_id = Uuid::new_v4();

        let subst_params = self.params.iter()
                                .filter(|p| p.type_id == mapping.0)
                                .map(|p| p.with_type_substituted(symbol_id, mapping.1))
                                .collect::<Vec<_>>();

        let subst_ret_type = if self.return_type_id == mapping.0 {
            mapping.1
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

impl SymbolInfo for MemberFunctionInfo {
    const TYPE: SymbolType = SymbolType::Method;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ChildSymbolInfo for MemberFunctionInfo {
    fn parent_symbol_id(&self) -> Uuid {
        self.class_id
    }
}




#[derive(Debug, Clone)]
pub struct EventInfo {
    class_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub params: Vec<FunctionParameterInfo>
}

impl EventInfo {
    pub fn new(class_id: Uuid, name: &str) -> Self {
        Self {
            class_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            params: Vec::new()
        }
    }
}

impl SymbolInfo for EventInfo {
    const TYPE: SymbolType = SymbolType::Event;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ChildSymbolInfo for EventInfo {
    fn parent_symbol_id(&self) -> Uuid {
        self.class_id
    }
}