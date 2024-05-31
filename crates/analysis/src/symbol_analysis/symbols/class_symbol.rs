use witcherscript::attribs::{ClassSpecifier, AutobindSpecifier};
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct ClassSymbol {
    path: BasicTypeSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<ClassSpecifier>,
    pub base_path: Option<BasicTypeSymbolPath>
}

impl Symbol for ClassSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Class
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for ClassSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for ClassSymbol {
    
}

impl ClassSymbol {
    pub fn new(path: BasicTypeSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            specifiers: SymbolSpecifiers::new(),
            base_path: None
        }
    }

    pub fn base_name(&self) -> Option<&str> {
        self.base_path.as_ref().and_then(|p| p.components().next().map(|c| c.name))
    }
}



#[derive(Debug, Clone)]
pub struct AutobindSymbol {
    path: MemberDataSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<AutobindSpecifier>,
    pub type_path: TypeSymbolPath,
}

impl Symbol for AutobindSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Autobind
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for AutobindSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl AutobindSymbol {
    pub fn new(path: MemberDataSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            specifiers: SymbolSpecifiers::new(),
            type_path: TypeSymbolPath::unknown()
        }
    }

    pub fn type_name(&self) -> &str {
        self.type_path.components().next().map(|c| c.name).unwrap_or_default()
    }
}