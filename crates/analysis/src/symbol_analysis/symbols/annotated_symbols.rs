use witcherscript::{ast::WRAPPED_METHOD_NAME, attribs::*};
use super::*;


/// Corresponding to @addMethod(Class) functions
#[derive(Debug, Clone)]
pub struct MemberFunctionInjectorSymbol {
    path: MemberCallableSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<MemberFunctionSpecifier>,
    pub flavour: Option<MemberFunctionFlavour>,
    pub return_type_path: TypeSymbolPath
}

impl Symbol for MemberFunctionInjectorSymbol {
    type PathType = MemberCallableSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunctionInjector
    }

    fn path(&self) -> &Self::PathType {
        &self.path
    }
}

impl LocatableSymbol for MemberFunctionInjectorSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for MemberFunctionInjectorSymbol { }

impl MemberFunctionInjectorSymbol {
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



/// Corresponding to @replaceMethod(Class) functions
#[derive(Debug, Clone)]
pub struct MemberFunctionReplacerSymbol {
    path: MemberCallableReplacerSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<MemberFunctionSpecifier>,
    pub flavour: Option<MemberFunctionFlavour>,
    pub return_type_path: TypeSymbolPath
}

impl Symbol for MemberFunctionReplacerSymbol {
    type PathType = MemberCallableReplacerSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunctionReplacer
    }

    fn path(&self) -> &Self::PathType {
        &self.path
    }
}

impl LocatableSymbol for MemberFunctionReplacerSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for MemberFunctionReplacerSymbol { }

impl MemberFunctionReplacerSymbol {
    pub fn new(path: MemberCallableReplacerSymbolPath, location: SymbolLocation) -> Self {
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

    #[inline]
    pub fn function_name(&self) -> &str {
        self.path.function_name()
    }
}



/// Corresponding to @replaceMethod functions
#[derive(Debug, Clone)]
pub struct GlobalFunctionReplacerSymbol {
    path: GlobalCallableReplacerSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<GlobalFunctionSpecifier>,
    pub flavour: Option<GlobalFunctionFlavour>,
    pub return_type_path: TypeSymbolPath
}

impl Symbol for GlobalFunctionReplacerSymbol {
    type PathType = GlobalCallableReplacerSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::GlobalFunctionReplacer
    }

    fn path(&self) -> &Self::PathType {
        &self.path
    }
}

impl LocatableSymbol for GlobalFunctionReplacerSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for GlobalFunctionReplacerSymbol { }

impl GlobalFunctionReplacerSymbol {
    pub fn new(path: GlobalCallableReplacerSymbolPath, location: SymbolLocation) -> Self {
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

    #[inline]
    pub fn function_name(&self) -> &str {
        self.path.function_name()
    }
}



/// Corresponding to @wrapMethod(Class) functions
#[derive(Debug, Clone)]
pub struct MemberFunctionWrapperSymbol {
    path: MemberCallableWrapperSymbolPath,
    location: SymbolLocation,
    // you don't put specifiers in the declaration of a wrapped method
    pub return_type_path: TypeSymbolPath
}

impl Symbol for MemberFunctionWrapperSymbol {
    type PathType = MemberCallableWrapperSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunctionWrapper
    }

    fn path(&self) -> &Self::PathType {
        &self.path
    }
}

impl LocatableSymbol for MemberFunctionWrapperSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for MemberFunctionWrapperSymbol { }

impl MemberFunctionWrapperSymbol {
    pub fn new(path: MemberCallableWrapperSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            return_type_path: TypeSymbolPath::unknown()
        }
    }

    pub fn return_type_name(&self) -> &str {
        self.return_type_path.components().next().map(|c| c.name).unwrap_or_default()
    }

    #[inline]
    pub fn function_name(&self) -> &str {
        self.path.function_name()
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
    pub fn new(wrapper_path: &MemberCallableWrapperSymbolPath) -> Self {
        Self {
            path: MemberCallableSymbolPath::new(&wrapper_path, WRAPPED_METHOD_NAME),
            wrapped_path: wrapper_path.to_owned().into(), // wrapped and wrapper paths are the same
        }
    }

    /// Path to the original method
    pub fn wrapped_path(&self) -> &MemberCallableSymbolPath {
        &self.wrapped_path
    }
}


/// Corresponding to @addField(Class) vars
#[derive(Debug, Clone)]
pub struct MemberVarInjectorSymbol {
    path: MemberDataSymbolPath,
    location: SymbolLocation,
    pub specifiers: SymbolSpecifiers<MemberVarSpecifier>,
    pub type_path: TypeSymbolPath
}

impl Symbol for MemberVarInjectorSymbol {
    type PathType = MemberDataSymbolPath;

    fn typ(&self) -> SymbolType {
        SymbolType::MemberVarInjector
    }

    fn path(&self) -> &Self::PathType {
        &self.path
    }
}

impl LocatableSymbol for MemberVarInjectorSymbol {
    fn location(&self) -> &SymbolLocation {
        &self.location
    }
}

impl PrimarySymbol for MemberVarInjectorSymbol { }

impl MemberVarInjectorSymbol {
    pub fn new(path: MemberDataSymbolPath, location: SymbolLocation) -> Self {
        Self {
            path,
            location,
            specifiers: SymbolSpecifiers::new(),
            type_path: TypeSymbolPath::unknown()
        }
    }

    pub fn type_name(&self) -> &str {
        self.type_path.components().next().map(|c| c.name).unwrap_or_default()
    }
}
