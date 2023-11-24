use uuid::Uuid;
use witcherscript::attribs::FunctionParameterSpecifier;
use super::{SymbolInfo, SymbolType, ChildSymbolInfo};

pub struct FunctionParameterInfo {
    func_id: Uuid,
    symbol_id: Uuid,
    name: String,
    specifiers: Vec<FunctionParameterSpecifier>,
    type_id: Uuid
}

impl FunctionParameterInfo {
    pub fn new(func_info: &impl SymbolInfo, name: &str, type_id: Uuid) -> Self {
        Self {
            func_id: func_info.symbol_id(),
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            type_id
        }
    }

    
    pub fn specifiers(&self) -> &[FunctionParameterSpecifier] {
        &self.specifiers
    }

    pub fn add_specifier(&mut self, specifier: FunctionParameterSpecifier) {
        self.specifiers.push(specifier)
    }


    pub fn type_id(&self) -> Uuid {
        self.type_id
    }
}

impl SymbolInfo for FunctionParameterInfo {
    const TYPE: super::SymbolType = SymbolType::Parameter;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ChildSymbolInfo for FunctionParameterInfo {
    fn parent_symbol_id(&self) -> Uuid {
        self.func_id
    }
}