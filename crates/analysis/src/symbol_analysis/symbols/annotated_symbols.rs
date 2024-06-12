use shrinkwraprs::Shrinkwrap;
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


/// Corresponding to @addMethod(Class) functions
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct AddedMemberFunctionSymbol {
    pub inner: MemberFunctionSymbol
}

impl Symbol for AddedMemberFunctionSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::AddedMemberFunction
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for AddedMemberFunctionSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for AddedMemberFunctionSymbol { }

impl AddedMemberFunctionSymbol {
    pub fn new(path: MemberCallableSymbolPath, location: SymbolLocation) -> Self {
        Self {
            inner: MemberFunctionSymbol::new(path, location)
        }
    }
}



/// Corresponding to @replaceMethod(Class) functions
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct ReplacedMemberFunctionSymbol {
    pub inner: MemberFunctionSymbol
}

impl Symbol for ReplacedMemberFunctionSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::ReplacedMemberFunction
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for ReplacedMemberFunctionSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for ReplacedMemberFunctionSymbol { }

impl ReplacedMemberFunctionSymbol {
    pub fn new(path: MemberCallableSymbolPath, location: SymbolLocation) -> Self {
        Self {
            inner: MemberFunctionSymbol::new(path, location)
        }
    }
}



/// Corresponding to @replaceMethod functions
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct ReplacedGlobalFunctionSymbol {
    pub inner: GlobalFunctionSymbol
}

impl Symbol for ReplacedGlobalFunctionSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::ReplacedGlobalFunction
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for ReplacedGlobalFunctionSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for ReplacedGlobalFunctionSymbol { }

impl ReplacedGlobalFunctionSymbol {
    pub fn new(path: GlobalCallableSymbolPath, location: SymbolLocation) -> Self {
        Self {
            inner: GlobalFunctionSymbol::new(path, location)
        }
    }
}



/// Corresponding to @wrapMethod(Class) functions
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct WrappedMemberFunctionSymbol {
    pub inner: MemberFunctionSymbol
}

impl Symbol for WrappedMemberFunctionSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::WrappedMemberFunction
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for WrappedMemberFunctionSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for WrappedMemberFunctionSymbol { }

impl WrappedMemberFunctionSymbol {
    pub fn new(path: MemberCallableSymbolPath, location: SymbolLocation) -> Self {
        Self {
            inner: MemberFunctionSymbol::new(path, location)
        }
    }
}



/// Corresponding to @addField(Class) vars
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct AddedMemberVarSymbol {
    pub inner: MemberVarSymbol
}

impl Symbol for AddedMemberVarSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::AddedMemberVar
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for AddedMemberVarSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for AddedMemberVarSymbol { }

impl AddedMemberVarSymbol {
    pub fn new(path: MemberDataSymbolPath, location: SymbolLocation) -> Self {
        Self {
            inner: MemberVarSymbol::new(path, location)
        }
    }
}
