use uuid::Uuid;
use super::{SymbolInfo, SymbolType, ChildSymbolInfo};


#[derive(Debug, Clone)]
pub struct TypeParameterInfo {
    symbol_id: Uuid,
    name: String,
    owner_id: Uuid,
}

impl TypeParameterInfo {
    pub fn new(owner_id: Uuid, name: &str) -> Self {
        Self {
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            owner_id
        }
    }
}

impl SymbolInfo for TypeParameterInfo {
    const TYPE: SymbolType = SymbolType::Type;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ChildSymbolInfo for TypeParameterInfo {
    fn parent_symbol_id(&self) -> Uuid {
        self.owner_id
    }
}