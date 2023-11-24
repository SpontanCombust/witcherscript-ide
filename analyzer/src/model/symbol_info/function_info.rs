use uuid::Uuid;
use witcherscript::attribs::*;
use super::{FunctionParameterInfo, SymbolInfo, SymbolType, GlobalSymbolInfo, ChildSymbolInfo};


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
    pub fn new(script_id: Uuid, name: &str, return_type_id: Uuid) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            flavour: None,
            params: Vec::new(),
            return_type_id
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
    pub fn new(class_id: Uuid, name: &str, return_type_id: Uuid) -> Self {
        Self {
            class_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            flavour: None,
            params: Vec::new(),
            return_type_id
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