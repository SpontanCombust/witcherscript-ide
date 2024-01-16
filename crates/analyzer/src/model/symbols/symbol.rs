use crate::model::symbol_path::SymbolPath;


pub trait Symbol {
    fn typ(&self) -> SymbolType;
    fn path(&self) -> &SymbolPath;

    fn name(&self) -> &str {
        self.path().components().last().unwrap().name
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Type,
    Enum,
    Struct,
    Class,
    State,
    Array,
    
    EnumMember,

    GlobalFunction,
    MemberFunction,
    Event,

    Parameter,

    GlobalVar,
    MemberVar,
    Autobind,
    LocalVar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolCategory {
    Type,
    Data,
    Callable
}

impl SymbolType {
    pub fn category(&self) -> SymbolCategory {
        use SymbolType::*;
        match self {
            Type | Enum | Struct | Class | State | Array => SymbolCategory::Type,
            EnumMember | Parameter | GlobalVar | MemberVar | Autobind | LocalVar => SymbolCategory::Data,
            GlobalFunction | MemberFunction | Event => SymbolCategory::Callable,
        }
    }
}
