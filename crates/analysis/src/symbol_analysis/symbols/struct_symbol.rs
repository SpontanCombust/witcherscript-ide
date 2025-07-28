use witcherscript_lang::attribs::StructSpecifier;
use super::*;


#[derive(Debug, Clone)]
pub struct StructSymbol {
    path: BasicTypeSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<StructSpecifier>
}

impl Symbol for StructSymbol {
    type PathType = BasicTypeSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::Struct
    }

    fn path(&self) -> &Self::PathType {
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
    type PathType = GlobalCallableSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::Constructor
    }

    fn path(&self) -> &Self::PathType {
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
