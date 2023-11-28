use uuid::Uuid;
use super::{Symbol, SymbolType, NATIVE_SYMBOL_SCRIPT_ID};

/// For basic arithmetic and string-like types
#[derive(Debug, Clone)]
pub struct PrimitiveTypeSymbol {
    symbol_id: Uuid,
    name: String,
    /// Most of the primitive types have a lowercase keyword name, e.g. `CName` has the `name` alias
    pub alias: Option<String>,
}

impl PrimitiveTypeSymbol {
    pub fn new(name: &str, alias: Option<&str>) -> Self {
        Self {
            symbol_id: Uuid::new_v4(),
            name: name.to_owned(),
            alias: alias.map(|s| s.to_owned())
        }
    }
}

impl Symbol for PrimitiveTypeSymbol {
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
