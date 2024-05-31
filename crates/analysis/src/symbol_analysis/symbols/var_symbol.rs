use witcherscript::{attribs::MemberVarSpecifier, tokens::Keyword};
use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


#[derive(Debug, Clone)]
pub struct MemberVarSymbol {
    path: MemberDataSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<MemberVarSpecifier>,
    pub type_path: TypeSymbolPath,
    pub ordinal: usize // used in the context of struct constructors
}

impl Symbol for MemberVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for MemberVarSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl MemberVarSymbol {
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



#[derive(Debug, Clone)]
pub struct LocalVarSymbol {
    path: MemberDataSymbolPath,
    location: SymbolLocation,
    pub type_path: TypeSymbolPath,
}

impl Symbol for LocalVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::LocalVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl LocatableSymbol for LocalVarSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl LocalVarSymbol {
    pub fn new(path: MemberDataSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            type_path: TypeSymbolPath::unknown()
        }
    }

    pub fn type_name(&self) -> &str {
        self.type_path.components().next().map(|c| c.name).unwrap_or_default()
    }
}



#[derive(Debug, Clone)]
pub struct GlobalVarSymbol {
    path: SymbolPathBuf,
    type_path: BasicTypeSymbolPath
}

impl Symbol for GlobalVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::GlobalVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl GlobalVarSymbol {
    // there is a fixed amount of predefined globals, so a standard 'new' is not required
    pub fn new(name: &str, type_path: BasicTypeSymbolPath) -> Self {
        Self {
            path: SymbolPathBuf::new(name, SymbolCategory::Data),
            type_path
        }
    }

    pub fn type_path(&self) -> &SymbolPathBuf {
        &self.type_path
    }

    pub fn type_name(&self) -> &str {
        self.type_path.components().next().map(|c| c.name).unwrap_or_default()
    }
}



#[derive(Debug, Clone)]
pub struct SpecialVarSymbol {
    path: SpecialVarSymbolPath,
    type_path: BasicTypeSymbolPath
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialVarSymbolKind {
    This,
    Super,
    Parent,
    VirtualParent
}

impl From<SpecialVarSymbolKind> for Keyword {
    fn from(value: SpecialVarSymbolKind) -> Self {
        match value {
            SpecialVarSymbolKind::This => Keyword::This,
            SpecialVarSymbolKind::Super => Keyword::Super,
            SpecialVarSymbolKind::Parent => Keyword::Parent,
            SpecialVarSymbolKind::VirtualParent => Keyword::VirtualParent,
        }
    }
}

impl Symbol for SpecialVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::GlobalVar
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl SpecialVarSymbol {
    pub fn new(path: SpecialVarSymbolPath, type_path: BasicTypeSymbolPath) -> Self {
        Self {
            path,
            type_path
        }
    }

    pub fn type_path(&self) -> &SymbolPathBuf {
        &self.type_path
    }

    pub fn type_name(&self) -> &str {
        &self.type_path.components().next().map(|c| c.name).unwrap_or_default()
    }

    pub fn kind(&self) -> SpecialVarSymbolKind {
        self.path.kind
    }
}
