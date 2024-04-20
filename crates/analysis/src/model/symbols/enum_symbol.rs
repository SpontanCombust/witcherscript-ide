use std::collections::HashMap;
use abs_path::AbsPath;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    path: BasicTypeSymbolPath,
    decl_file_path: AbsPath,
    pub variants: HashMap<SymbolPathBuf, EnumVariantSymbol>
}

impl Symbol for EnumSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Enum
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl PrimarySymbol for EnumSymbol {
    fn decl_file_path(&self) -> &AbsPath {
        &self.decl_file_path
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
}



#[derive(Debug, Clone)]
pub struct EnumVariantSymbol {
    path: DataSymbolPath
}

impl Symbol for EnumVariantSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::EnumVariant
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