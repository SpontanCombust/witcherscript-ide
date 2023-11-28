use uuid::Uuid;
use super::{Symbol, SymbolType, NATIVE_SYMBOL_SCRIPT_ID};

#[derive(Debug, Clone)]
pub struct BasicTypeSymbol {
    symbol_id: Uuid,
    name: String
}

impl BasicTypeSymbol {
    pub fn new(name: &str) -> Self {
        Self {
            symbol_id: Uuid::new_v4(),
            name: name.to_owned()
        }
    }
}

impl Symbol for BasicTypeSymbol {
    const TYPE: SymbolType = SymbolType::Type;
    
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
