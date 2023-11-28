use uuid::Uuid;
use super::{Symbol, SymbolType};


#[derive(Debug, Clone)]
pub struct TypeParameterSymbol {
    symbol_id: Uuid,
    name: String,
    owner_id: Uuid,
}

impl TypeParameterSymbol {
    pub fn new(owner_id: Uuid, name: &str) -> Self {
        Self {
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            owner_id
        }
    }
}

impl Symbol for TypeParameterSymbol {
    const TYPE: SymbolType = SymbolType::Type;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn symbol_name(&self) -> &str {
        self.name.as_str()
    }

    fn parent_symbol_id(&self) -> Uuid {
        self.owner_id
    }
}