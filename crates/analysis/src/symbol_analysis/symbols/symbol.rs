use crate::symbol_analysis::symbol_path::SymbolPath;
use super::SymbolLocation;


pub trait Symbol {
    fn typ(&self) -> SymbolType;
    fn path(&self) -> &SymbolPath;

    /// Returns name of the last path component.
    /// If path is empty returns empty string.
    fn name(&self) -> &str {
        self.path().components().last().map(|c| c.name).unwrap_or("")
    }
}

/// Denotes a symbol, which location can be pin-pointed in a file
/// The range typically spans over the name label of the symbol 
pub trait LocatableSymbol: Symbol {
    fn location(&self) -> &SymbolLocation;
}

/// A symbol that can be used to group together an entire family of symbols.
/// This is used to associate symbols with source paths, even if those symbols cannot be located, but their primary symbol parents can.
pub trait PrimarySymbol: LocatableSymbol { }



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    // types
    Type,
    Enum,
    Struct,
    Class,
    State,
    Array,
    
    // callables
    GlobalFunction,
    MemberFunction,
    Event,
    Constructor,
    MemberFunctionInjector,
    MemberFunctionReplacer,
    GlobalFunctionReplacer,
    MemberFunctionWrapper,
    WrappedMethod,
    
    // data
    EnumVariant,
    Parameter,
    GlobalVar,
    MemberVar,
    Autobind,
    LocalVar,
    ThisVar,
    SuperVar,
    ParentVar,
    VirtualParentVar,
    MemberVarInjector,
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
            Type 
            | Enum
            | Struct
            | Class
            | State
            | Array   => SymbolCategory::Type,
            EnumVariant     
            | Parameter
            | GlobalVar
            | MemberVar
            | Autobind
            | LocalVar
            | ThisVar
            | SuperVar
            | ParentVar
            | VirtualParentVar 
            | MemberVarInjector => SymbolCategory::Data,
            GlobalFunction 
            | MemberFunction 
            | Event 
            | Constructor 
            | MemberFunctionInjector 
            | MemberFunctionReplacer 
            | GlobalFunctionReplacer 
            | MemberFunctionWrapper
            | WrappedMethod => SymbolCategory::Callable,
        }
    }
}
