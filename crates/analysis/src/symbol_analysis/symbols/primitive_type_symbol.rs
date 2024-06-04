use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


/// For basic arithmetic and string-like types
#[derive(Debug, Clone)]
pub struct PrimitiveTypeSymbol {
    path: SymbolPathBuf,
    /// Most of the primitive types have a lowercase keyword-like name, e.g. `CName` has the `name` alias
    /// Real path allows to know this original type if we inspect the lowercase alias.
    pub real_path: Option<SymbolPathBuf>,
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
    pub fn new(name: &str, real_name: Option<&str>) -> Self {
        Self {
            path: SymbolPathBuf::new(name, SymbolCategory::Type),
            real_path: real_name.map(|n| SymbolPathBuf::new(n, SymbolCategory::Type))
        }
    }

    pub fn real_name(&self) -> Option<&str> {
        self.real_path.as_ref()
            .and_then(|p| p.components().next())
            .map(|comp| comp.name)
    }
}