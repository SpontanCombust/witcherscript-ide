use super::{Symbol, SymbolType, SymbolData, NATIVE_SYMBOL_SCRIPT_ID};

/// For basic arithmetic and string-like types
#[derive(Debug, Clone)]
pub struct PrimitiveTypeSymbolData {
    /// Most of the primitive types have a lowercase keyword name, e.g. `CName` has the `name` alias
    pub alias: Option<String>,
}

impl SymbolData for PrimitiveTypeSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::Type;
}

pub type PrimitiveTypeSymbol = Symbol<PrimitiveTypeSymbolData>;

impl PrimitiveTypeSymbol {
    pub fn new_with_alias(name: &str, alias: Option<&str>) -> Self {
        Self::new(
            name, 
            NATIVE_SYMBOL_SCRIPT_ID, 
            PrimitiveTypeSymbolData { 
                alias: alias.map(|a| a.to_string())
            }
        )
    }
}