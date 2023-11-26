use uuid::Uuid;
use super::{Symbol, SymbolType, GlobalSymbol, NATIVE_SYMBOL_SCRIPT_ID};

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

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbol for BasicTypeSymbol {
    fn script_id(&self) -> Uuid {
        NATIVE_SYMBOL_SCRIPT_ID
    }
}