use witcherscript_lang::attribs::FunctionParameterSpecifier;
use super::*;


#[derive(Debug, Clone)]
pub struct FunctionParameterSymbol {
    path: MemberDataSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<FunctionParameterSpecifier>,
    pub type_path: TypeSymbolPath,
    pub ordinal: usize
}

impl Symbol for FunctionParameterSymbol {
    type PathType = MemberDataSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::Parameter
    }

    fn path(&self) -> &Self::PathType {
        &self.path
    }
}

impl LocatableSymbol for FunctionParameterSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl FunctionParameterSymbol {
    pub fn new(path: MemberDataSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            specifiers: SymbolSpecifiers::new(),
            type_path: TypeSymbolPath::unknown(),
            ordinal: 0
        }
    }

    pub fn type_name(&self) -> &str {
        self.type_path.components().next().map(|c| c.name).unwrap_or_default()
    }
}