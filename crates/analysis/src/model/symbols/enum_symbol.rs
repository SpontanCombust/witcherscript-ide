use std::collections::HashMap;
use abs_path::AbsPath;
use crate::model::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    path: BasicTypeSymbolPath,
    decl_file_path: AbsPath,
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
    pub fn new(path: BasicTypeSymbolPath, decl_file_path: AbsPath) -> Self {
        Self {
            path,
            decl_file_path,
            variants: HashMap::new()
        }
    }

    
    pub fn decl_file_path(&self) -> &AbsPath {
        &self.decl_file_path
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