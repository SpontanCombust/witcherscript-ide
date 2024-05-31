use witcherscript::attribs::StructSpecifier;
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct StructSymbol {
    path: BasicTypeSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<StructSpecifier>
}

impl Symbol for StructSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Struct
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for StructSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for StructSymbol {

}

impl StructSymbol {
    pub fn new(path: BasicTypeSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            specifiers: SymbolSpecifiers::new()
        }
    }
}


/// Struct constructor
#[derive(Debug, Clone)]
pub struct ConstructorSymbol {
    path: GlobalCallableSymbolPath,
    location: SymbolLocation,
    pub parent_type_path: BasicTypeSymbolPath
}

impl Symbol for ConstructorSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Constructor
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for ConstructorSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for ConstructorSymbol {
    
}

impl ConstructorSymbol {
    pub fn new(path: GlobalCallableSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            parent_type_path: BasicTypeSymbolPath::unknown()
        }
    }
}
