use uuid::Uuid;
use witcherscript::attribs::*;
use super::{FunctionParameterInfo, SymbolInfo, SymbolType, GlobalSymbolInfo, ChildSymbolInfo};


pub struct GlobalFunctionInfo {
    script_id: Uuid,
    symbol_id: Uuid,
    name: String,
    specifiers: Vec<GlobalFunctionSpecifier>,
    flavour: Option<GlobalFunctionFlavour>,
    params: Vec<FunctionParameterInfo>,
    return_type_id: Uuid
}

impl GlobalFunctionInfo {
    pub fn new(script_id: Uuid, name: &str, flavour: Option<GlobalFunctionFlavour>, return_type_id: Uuid) -> Self {
        Self {
            script_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            flavour,
            params: Vec::new(),
            return_type_id
        }
    }


    pub fn specifiers(&self) -> &[GlobalFunctionSpecifier] {
        &self.specifiers
    }

    pub fn add_specifier(&mut self, specifier: GlobalFunctionSpecifier) {
        self.specifiers.push(specifier);
    }


    pub fn params(&self) -> &[FunctionParameterInfo] {
        &self.params
    }

    pub fn add_param(&mut self, param: FunctionParameterInfo) {
        self.params.push(param);
    }


    pub fn flavour(&self) -> Option<GlobalFunctionFlavour> {
        self.flavour
    }


    pub fn return_type_id(&self) -> Uuid {
        self.return_type_id
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
    specifiers: Vec<MemberFunctionSpecifier>,
    flavour: Option<MemberFunctionFlavour>,
    params: Vec<FunctionParameterInfo>,
    return_type_id: Uuid
}

impl MemberFunctionInfo {
    pub fn new(class_id: Uuid, name: &str, flavour: Option<MemberFunctionFlavour>, return_type_id: Uuid) -> Self {
        Self {
            class_id,
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            flavour,
            params: Vec::new(),
            return_type_id
        }
    }


    pub fn specifiers(&self) -> &[MemberFunctionSpecifier] {
        &self.specifiers
    }

    pub fn add_specifier(&mut self, specifier: MemberFunctionSpecifier) {
        self.specifiers.push(specifier);
    }


    pub fn params(&self) -> &[FunctionParameterInfo] {
        &self.params
    }

    pub fn add_param(&mut self, param: FunctionParameterInfo) {
        self.params.push(param);
    }


    pub fn flavour(&self) -> Option<MemberFunctionFlavour> {
        self.flavour
    }


    pub fn return_type_id(&self) -> Uuid {
        self.return_type_id
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
    params: Vec<FunctionParameterInfo>
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


    pub fn params(&self) -> &[FunctionParameterInfo] {
        &self.params
    }

    pub fn add_param(&mut self, param: FunctionParameterInfo) {
        self.params.push(param);
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