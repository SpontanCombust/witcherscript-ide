use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


/// For basic arithmetic and string-like types
#[derive(Debug, Clone)]
pub struct PrimitiveTypeSymbol {
    path: SymbolPathBuf,
    /// Most of the primitive types have a lowercase keyword name, e.g. `CName` has the `name` alias
    pub alias: Option<SymbolPathBuf>,
}

impl Symbol for PrimitiveTypeSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Type
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl PrimitiveTypeSymbol {
    pub fn new(name: &str, alias: Option<&str>) -> Self {
        Self {
            path: SymbolPathBuf::new(name, SymbolCategory::Type),
            alias: alias.map(|a| SymbolPathBuf::new(a, SymbolCategory::Type))
        }
    }
}