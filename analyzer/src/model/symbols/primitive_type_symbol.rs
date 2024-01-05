use crate::model::symbol_path::SymbolPath;
use super::*;


/// For basic arithmetic and string-like types
#[derive(Debug, Clone)]
pub struct PrimitiveTypeSymbol {
    path: SymbolPath,
    /// Most of the primitive types have a lowercase keyword name, e.g. `CName` has the `name` alias
    pub alias: Option<SymbolPath>,
}

impl Symbol for PrimitiveTypeSymbol {
    const SYMBOL_TYPE: SymbolType = SymbolType::Type;

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl PrimitiveTypeSymbol {
    pub fn new(name: &str, alias: Option<&str>) -> Self {
        Self {
            path: SymbolPath::new(name, SymbolCategory::Type),
            alias: alias.map(|a| SymbolPath::new(a, SymbolCategory::Type))
        }
    }
}