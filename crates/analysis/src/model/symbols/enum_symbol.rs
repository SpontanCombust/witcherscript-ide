use std::collections::HashMap;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    path: BasicTypeSymbolPath,
    pub variants: HashMap<SymbolPath, EnumVariantSymbol>
}

impl Symbol for EnumSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Enum
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl EnumSymbol {
    pub fn new(path: BasicTypeSymbolPath) -> Self {
        Self {
            path,
            variants: HashMap::new()
        }
    }
}



#[derive(Debug, Clone)]
pub struct EnumVariantSymbol {
    path: DataSymbolPath
}

impl Symbol for EnumVariantSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::EnumMember
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl EnumVariantSymbol {
    pub fn new(path: DataSymbolPath) -> Self {
        Self {
            path
        }
    }
}