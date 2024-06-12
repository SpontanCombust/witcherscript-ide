use witcherscript::attribs::*;
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


pub const DEFAULT_FUNCTION_RETURN_TYPE_NAME: &'static str = "void";


#[derive(Debug, Clone)]
pub struct GlobalFunctionSymbol {
    path: GlobalCallableSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<GlobalFunctionSpecifier>,
    pub flavour: Option<GlobalFunctionFlavour>,
    pub return_type_path: TypeSymbolPath
}

impl Symbol for GlobalFunctionSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::GlobalFunction
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for GlobalFunctionSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for GlobalFunctionSymbol {

}

impl GlobalFunctionSymbol {
    pub fn new(path: GlobalCallableSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            specifiers: SymbolSpecifiers::new(),
            flavour: None,
            return_type_path: TypeSymbolPath::unknown()
        }
    }

    pub fn return_type_name(&self) -> &str {
        self.return_type_path.components().next().map(|c| c.name).unwrap_or_default()
    }
}



#[derive(Debug, Clone)]
pub struct MemberFunctionSymbol {
    path: MemberCallableSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<MemberFunctionSpecifier>,
    pub flavour: Option<MemberFunctionFlavour>,
    pub return_type_path: TypeSymbolPath
}

impl Symbol for MemberFunctionSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunction
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for MemberFunctionSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl MemberFunctionSymbol {
    pub fn new(path: MemberCallableSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            specifiers: SymbolSpecifiers::new(),
            flavour: None,
            return_type_path: TypeSymbolPath::unknown()
        }
    }

    pub fn return_type_name(&self) -> &str {
        self.return_type_path.components().next().map(|c| c.name).unwrap_or_default()
    }
}



#[derive(Debug, Clone)]
pub struct EventSymbol {
    path: MemberCallableSymbolPath,
    location: SymbolLocation
}

impl Symbol for EventSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Event
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for EventSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl EventSymbol {
    pub fn new(path: MemberCallableSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location
        }
    }
}
