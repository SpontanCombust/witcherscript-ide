use uuid::Uuid;
use witcherscript::attribs::FunctionParameterSpecifier;
use super::{Symbol, SymbolType, ERROR_SYMBOL_ID};


#[derive(Debug, Clone)]
pub struct FunctionParameterSymbol {
    func_id: Uuid,
    symbol_id: Uuid,
    name: String,
    pub specifiers: Vec<FunctionParameterSpecifier>,
    pub type_id: Uuid
}

impl FunctionParameterSymbol {
    pub fn new(func_info: &impl Symbol, name: &str) -> Self {
        Self {
            func_id: func_info.symbol_id(),
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            specifiers: Vec::new(),
            type_id: ERROR_SYMBOL_ID
        }
    }

    /// Used in handling array type.
    pub fn with_type_substituted(&self, func_id: Uuid, substitute_id: Uuid) -> Self {
        Self {
            func_id,
            symbol_id: Uuid::new_v4(),
            type_id: substitute_id,
            ..self.clone()
        }
    }
}

impl Symbol for FunctionParameterSymbol {
    const TYPE: SymbolType = SymbolType::Parameter;

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