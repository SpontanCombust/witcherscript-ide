use std::collections::HashMap;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    path: BasicTypeSymbolPath,
    pub members: HashMap<SymbolPath, EnumMemberSymbol>
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
            members: HashMap::new()
        }
    }
}



#[derive(Debug, Clone)]
pub struct EnumMemberSymbol {
    path: DataSymbolPath
}

impl Symbol for EnumMemberSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::EnumMember
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl EnumMemberSymbol {
    pub fn new(path: DataSymbolPath) -> Self {
        Self {
            path
        }
    }
}