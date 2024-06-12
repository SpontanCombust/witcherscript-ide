use shrinkwraprs::Shrinkwrap;
use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


/// Corresponding to @addMethod(Class) functions
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct MemberFunctionInjectorSymbol {
    pub inner: MemberFunctionSymbol
}

impl Symbol for MemberFunctionInjectorSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunctionInjector
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for MemberFunctionInjectorSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for MemberFunctionInjectorSymbol { }

impl MemberFunctionInjectorSymbol {
    pub fn new(inner: MemberFunctionSymbol) -> Self {
        Self {
            inner
        }
    }
}



/// Corresponding to @replaceMethod(Class) functions
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct MemberFunctionReplacerSymbol {
    pub inner: MemberFunctionSymbol
}

impl Symbol for MemberFunctionReplacerSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunctionReplacer
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for MemberFunctionReplacerSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for MemberFunctionReplacerSymbol { }

impl MemberFunctionReplacerSymbol {
    pub fn new(inner: MemberFunctionSymbol) -> Self {
        Self {
            inner
        }
    }
}



/// Corresponding to @replaceMethod functions
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct GlobalFunctionReplacerSymbol {
    pub inner: GlobalFunctionSymbol
}

impl Symbol for GlobalFunctionReplacerSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::GlobalFunctionReplacer
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for GlobalFunctionReplacerSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for GlobalFunctionReplacerSymbol { }

impl GlobalFunctionReplacerSymbol {
    pub fn new(inner: GlobalFunctionSymbol) -> Self {
        Self {
            inner
        }
    }
}



/// Corresponding to @wrapMethod(Class) functions
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct MemberFunctionWrapperSymbol {
    pub inner: MemberFunctionSymbol
}

impl Symbol for MemberFunctionWrapperSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunctionWrapper
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for MemberFunctionWrapperSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for MemberFunctionWrapperSymbol { }

impl MemberFunctionWrapperSymbol {
    pub fn new(inner: MemberFunctionSymbol) -> Self {
        Self {
            inner
        }
    }
}



/// Corresponding to @addField(Class) vars
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct MemberVarInjectorSymbol {
    pub inner: MemberVarSymbol
}

impl Symbol for MemberVarInjectorSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberVarInjector
    }

    fn path(&self) -> &SymbolPath {
        &self.inner.path()
    }
}

impl LocatableSymbol for MemberVarInjectorSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.inner.location()
    }
}

impl PrimarySymbol for MemberVarInjectorSymbol { }

impl MemberVarInjectorSymbol {
    pub fn new(inner: MemberVarSymbol) -> Self {
        Self {
            inner
        }
    }
}
