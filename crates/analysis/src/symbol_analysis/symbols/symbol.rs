use std::path::Path;
use lsp_types as lsp;
use crate::symbol_analysis::symbol_path::SymbolPath;


pub trait Symbol {
    fn typ(&self) -> SymbolType;
    fn path(&self) -> &SymbolPath;

    /// If path is empty returns empty string
    fn name(&self) -> &str {
        self.path().components().last().map(|c| c.name).unwrap_or("")
    }
}

/// A symbol with no parent (its path has only a single component) and can be associated with a file it was declared in
pub trait PrimarySymbol: Symbol {
    fn local_source_path(&self) -> &Path;
}

/// Denotes a symbol, which location can be pin-pointed in a file
/// The range typically spans over the name label of the symbol 
pub trait LocatableSymbol: Symbol {
    fn range(&self) -> lsp::Range;
    fn label_range(&self) -> lsp::Range;
}



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
    
    // data
    EnumVariant,
    Parameter,
    GlobalVar,
    MemberVar,
    Autobind,
    LocalVar,
    SpecialVar
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
            EnumVariant | Parameter | GlobalVar | MemberVar | Autobind | LocalVar | SpecialVar => SymbolCategory::Data,
            GlobalFunction | MemberFunction | Event => SymbolCategory::Callable,
        }
    }
}
