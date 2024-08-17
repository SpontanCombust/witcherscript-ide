use witcherscript::ast::WRAPPED_METHOD_NAME;
use super::*;


/// Corresponding to @addMethod(Class) functions
#[derive(Debug, Clone)]
pub struct MemberFunctionInjectorSymbol {
    pub backer: MemberFunctionSymbol
}

impl Symbol for MemberFunctionInjectorSymbol {
    type PathType = MemberCallableSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunctionInjector
    }

    fn path(&self) -> &Self::PathType {
        &self.backer.path()
    }
}

impl LocatableSymbol for MemberFunctionInjectorSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.backer.location()
    }
}

impl PrimarySymbol for MemberFunctionInjectorSymbol { }

impl MemberFunctionInjectorSymbol {
    pub fn new(backer: MemberFunctionSymbol) -> Self {
        Self {
            backer
        }
    }
}



/// Corresponding to @replaceMethod(Class) functions
#[derive(Debug, Clone)]
pub struct MemberFunctionReplacerSymbol {
    pub backer: MemberFunctionSymbol
}

impl Symbol for MemberFunctionReplacerSymbol {
    type PathType = MemberCallableSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunctionReplacer
    }

    fn path(&self) -> &Self::PathType {
        &self.backer.path()
    }
}

impl LocatableSymbol for MemberFunctionReplacerSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.backer.location()
    }
}

impl PrimarySymbol for MemberFunctionReplacerSymbol { }

impl MemberFunctionReplacerSymbol {
    pub fn new(backer: MemberFunctionSymbol) -> Self {
        Self {
            backer
        }
    }
}



/// Corresponding to @replaceMethod functions
#[derive(Debug, Clone)]
pub struct GlobalFunctionReplacerSymbol {
    pub backer: GlobalFunctionSymbol
}

impl Symbol for GlobalFunctionReplacerSymbol {
    type PathType = GlobalCallableSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::GlobalFunctionReplacer
    }

    fn path(&self) -> &Self::PathType {
        &self.backer.path()
    }
}

impl LocatableSymbol for GlobalFunctionReplacerSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.backer.location()
    }
}

impl PrimarySymbol for GlobalFunctionReplacerSymbol { }

impl GlobalFunctionReplacerSymbol {
    pub fn new(backer: GlobalFunctionSymbol) -> Self {
        Self {
            backer
        }
    }
}



/// Corresponding to @wrapMethod(Class) functions
#[derive(Debug, Clone)]
pub struct MemberFunctionWrapperSymbol {
    pub backer: MemberFunctionSymbol
}

impl Symbol for MemberFunctionWrapperSymbol {
    type PathType = MemberCallableSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunctionWrapper
    }

    fn path(&self) -> &Self::PathType {
        &self.backer.path()
    }
}

impl LocatableSymbol for MemberFunctionWrapperSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.backer.location()
    }
}

impl PrimarySymbol for MemberFunctionWrapperSymbol { }

impl MemberFunctionWrapperSymbol {
    pub fn new(backer: MemberFunctionSymbol) -> Self {
        Self {
            backer
        }
    }
}


/// Corresponding to the special `wrappedMethod()` function valid inside @wrapMethod function's scope
#[derive(Debug, Clone)]
pub struct WrappedMethodSymbol {
    path: MemberCallableSymbolPath,
    wrapped_path: MemberCallableSymbolPath
}

impl Symbol for WrappedMethodSymbol {
    type PathType = MemberCallableSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::WrappedMethod
    }

    fn path(&self) -> &Self::PathType {
        &self.path
    }
}

impl WrappedMethodSymbol {
    pub fn new(wrapper_path: &MemberCallableSymbolPath) -> Self {
        Self {
            path: MemberCallableSymbolPath::new(&wrapper_path, WRAPPED_METHOD_NAME),
            wrapped_path: wrapper_path.to_owned(), // wrapped and wrapper paths are the same
        }
    }

    pub fn wrapped_path(&self) -> &MemberCallableSymbolPath {
        &self.wrapped_path
    }
}



/// Corresponding to @addField(Class) vars
#[derive(Debug, Clone)]
pub struct MemberVarInjectorSymbol {
    pub backer: MemberVarSymbol
}

impl Symbol for MemberVarInjectorSymbol {
    type PathType = MemberDataSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::MemberVarInjector
    }

    fn path(&self) -> &Self::PathType {
        &self.backer.path()
    }
}

impl LocatableSymbol for MemberVarInjectorSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.backer.location()
    }
}

impl PrimarySymbol for MemberVarInjectorSymbol { }

impl MemberVarInjectorSymbol {
    pub fn new(backer: MemberVarSymbol) -> Self {
        Self {
            backer
        }
    }
}
