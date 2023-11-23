use uuid::Uuid;

use super::{SymbolInfo, SymbolType, GlobalSymbolInfo, NATIVE_SYMBOL_SCRIPT_ID};

pub struct BasicTypeInfo {
    symbol_id: Uuid,
    name: String
}

impl BasicTypeInfo {
    pub fn new(name: &str) -> Self {
        Self {
            symbol_id: Uuid::new_v4(),
            name: name.to_owned()
        }
    }
}

impl SymbolInfo for BasicTypeInfo {
    const TYPE: SymbolType = SymbolType::Type;
    
    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl GlobalSymbolInfo for BasicTypeInfo {
    fn script_id(&self) -> Uuid {
        NATIVE_SYMBOL_SCRIPT_ID
    }
}