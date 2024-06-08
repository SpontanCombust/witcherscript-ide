use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct EnumSymbol {
    path: BasicTypeSymbolPath,
    location: SymbolLocation
}

impl Symbol for EnumSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Enum
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for EnumSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for EnumSymbol {

}

impl EnumSymbol {
    pub fn new(path: BasicTypeSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location
        }
    }
}



/// Enum variants/members are not technically primary symbols, but because they are visible in the global scope
/// and can be used anywhere as unqualified identifiers, we treat them as such. Enums in WS seem to behave just like in C. 
/// In that they are an exception to the symbol path system - you can't query for members of an enum by checking the path.
#[derive(Debug, Clone)]
pub struct EnumVariantSymbol {
    path: GlobalDataSymbolPath,
    location: SymbolLocation,
    pub parent_enum_path: BasicTypeSymbolPath,
    pub value: i32
}

impl Symbol for EnumVariantSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::EnumVariant
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for EnumVariantSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for EnumVariantSymbol {

}

impl EnumVariantSymbol {
    pub fn new(path: GlobalDataSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            parent_enum_path: BasicTypeSymbolPath::unknown(),
            value: 0
        }
    }
}