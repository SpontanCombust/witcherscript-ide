//! Symbol paths are not all uniform, especially for types.
//! Arrays for example are identified with `array<Type>`, where `array` and `Type` are distinct identifiers.
//! Following utility wrapper types exist to gather these exceptions under a single umbrella, 
//! detached from symbols themselves (in the sense of creating them).
//! They provide convenience constructors and some of them house extra fields.
//! 
//! SymbolPath is used as a means of uniquely identifying a symbol in a global name space. 
//! During script file parsing a given symbol has to be first checked whether it is not a duplicate or whatnot.
//! This means paths should be deduced before eventual symbol creation in case of an error.
//! 
//! Symbols that are not scanned from .ws files do not need dedicated path types as they exist regardless 
//! of which files are scanned.

use std::{ops::Deref, borrow::Borrow};
use shrinkwraprs::Shrinkwrap;
use witcherscript::tokens::Keyword;
use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::SymbolCategory;


#[derive(Debug, Clone, Shrinkwrap)]
pub struct GlobalDataSymbolPath(SymbolPathBuf);

impl GlobalDataSymbolPath {
    pub fn new(name: &str) -> Self {
        let path = SymbolPathBuf::new(name, SymbolCategory::Data);
        Self(path)
    }
}

impl From<GlobalDataSymbolPath> for SymbolPathBuf {
    fn from(value: GlobalDataSymbolPath) -> Self {
        value.0
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct MemberDataSymbolPath(SymbolPathBuf);

impl MemberDataSymbolPath {
    pub fn new(parent_path: &SymbolPath, name: &str) -> Self {
        let mut path = parent_path.to_owned();
        path.push(name, SymbolCategory::Data);
        Self(path)
    }
}

impl From<MemberDataSymbolPath> for SymbolPathBuf {
    fn from(value: MemberDataSymbolPath) -> Self {
        value.0
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct GlobalCallableSymbolPath(SymbolPathBuf);

impl GlobalCallableSymbolPath {
    pub fn new(name: &str) -> Self {
        let path = SymbolPathBuf::new(name, SymbolCategory::Callable);
        Self(path)
    }
}

impl From<GlobalCallableSymbolPath> for SymbolPathBuf {
    fn from(value: GlobalCallableSymbolPath) -> Self {
        value.0
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct MemberCallableSymbolPath(SymbolPathBuf);

impl MemberCallableSymbolPath {
    pub fn new(parent_path: &SymbolPath, name: &str) -> Self {
        let mut path = parent_path.to_owned();
        path.push(name, SymbolCategory::Callable);
        Self(path)
    }
}

impl From<MemberCallableSymbolPath> for SymbolPathBuf {
    fn from(value: MemberCallableSymbolPath) -> Self {
        value.0
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct BasicTypeSymbolPath(SymbolPathBuf);

impl BasicTypeSymbolPath {
    pub fn new(name: &str) -> Self {
        let path = SymbolPathBuf::new(name, SymbolCategory::Type);
        Self(path)
    }

    pub fn unknown() -> Self {
        Self(SymbolPathBuf::unknown(SymbolCategory::Type))
    }
}

impl From<BasicTypeSymbolPath> for SymbolPathBuf {
    fn from(value: BasicTypeSymbolPath) -> Self {
        value.0
    }
}

impl Default for BasicTypeSymbolPath {
    fn default() -> Self {
        Self::unknown()
    }
}


/// States are not uniquely identified by just the state name.
/// Internally WS compiler creates a new class, which has a name {parent_class_name}State{state_name}.
/// We identify the state type by that class name.
#[derive(Debug, Clone, Shrinkwrap)]
pub struct StateSymbolPath {
    #[shrinkwrap(main_field)]
    path: SymbolPathBuf,
    pub state_name: String,
    pub parent_class_path: BasicTypeSymbolPath
}

impl StateSymbolPath {
    pub fn new(state_name: &str, parent_class_name: &str) -> Self {
        let path = SymbolPathBuf::new(&format!("{}State{}", parent_class_name, state_name), SymbolCategory::Type);
        let parent_class_path = BasicTypeSymbolPath::new(parent_class_name);

        Self {
            path,
            state_name: state_name.to_string(),
            parent_class_path
        }
    }
}

impl From<StateSymbolPath> for BasicTypeSymbolPath {
    fn from(value: StateSymbolPath) -> Self {
        BasicTypeSymbolPath(value.path)
    }
}

impl From<StateSymbolPath> for SymbolPathBuf {
    fn from(value: StateSymbolPath) -> Self {
        value.path
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct ArrayTypeSymbolPath {
    #[shrinkwrap(main_field)]
    path: SymbolPathBuf,
    pub type_arg_path: Box<TypeSymbolPath>
}

impl ArrayTypeSymbolPath {
    pub fn new(type_arg_path: TypeSymbolPath) -> Self {
        let path = SymbolPathBuf::new(&format!("array<{}>", type_arg_path.to_string()), SymbolCategory::Type);

        Self {
            path,
            type_arg_path: Box::new(type_arg_path)
        }
    }

    pub fn unknown() -> Self {
        Self {
            path: SymbolPathBuf::unknown(SymbolCategory::Type),
            type_arg_path: Box::new(TypeSymbolPath::unknown())
        }
    }
}

impl From<ArrayTypeSymbolPath> for SymbolPathBuf {
    fn from(value: ArrayTypeSymbolPath) -> Self {
        value.path
    }
} 


#[derive(Debug, Clone)]
pub enum TypeSymbolPath {
    BasicOrState(BasicTypeSymbolPath),
    Array(ArrayTypeSymbolPath)
    // StateSymbolPath not included, because notation `state X in Y` 
    // is used only in state's declaration and not when its class is mentioned
}

impl TypeSymbolPath {
    pub fn unknown() -> Self {
        Self::BasicOrState(BasicTypeSymbolPath::unknown())
    }
}

impl Borrow<SymbolPathBuf> for TypeSymbolPath {
    fn borrow(&self) -> &SymbolPathBuf {
        match self {
            TypeSymbolPath::BasicOrState(basic) => &basic.0,
            TypeSymbolPath::Array(array) => &array.path,
        }
    }
}

impl Deref for TypeSymbolPath {
    type Target = SymbolPathBuf;

    fn deref(&self) -> &Self::Target {
        match self {
            TypeSymbolPath::BasicOrState(basic) => &basic.0,
            TypeSymbolPath::Array(array) => &array.path,
        }
    }
}

impl From<BasicTypeSymbolPath> for TypeSymbolPath {
    fn from(value: BasicTypeSymbolPath) -> Self {
        Self::BasicOrState(value)
    }
}

impl From<StateSymbolPath> for TypeSymbolPath {
    fn from(value: StateSymbolPath) -> Self {
        Self::BasicOrState(value.into())
    }
}

impl From<ArrayTypeSymbolPath> for TypeSymbolPath {
    fn from(value: ArrayTypeSymbolPath) -> Self {
        Self::Array(value)
    }
}

impl From<TypeSymbolPath> for SymbolPathBuf {
    fn from(value: TypeSymbolPath) -> Self {
        match value {
            TypeSymbolPath::BasicOrState(p) => p.into(),
            TypeSymbolPath::Array(p) => p.into(),
        }
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct ThisVarSymbolPath {
    path: SymbolPathBuf
}

impl ThisVarSymbolPath {
    pub fn new(parent_path: &SymbolPath) -> Self {
        let mut path = parent_path.to_owned();
        path.push(Keyword::This.as_ref(), SymbolCategory::Data);

        Self { path }
    }
}

impl From<ThisVarSymbolPath> for SymbolPathBuf {
    fn from(value: ThisVarSymbolPath) -> Self {
        value.path
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct SuperVarSymbolPath {
    path: SymbolPathBuf
}

impl SuperVarSymbolPath {
    pub fn new(parent_path: &SymbolPath) -> Self {
        let mut path = parent_path.to_owned();
        path.push(Keyword::Super.as_ref(), SymbolCategory::Data);

        Self { path }
    }
}

impl From<SuperVarSymbolPath> for SymbolPathBuf {
    fn from(value: SuperVarSymbolPath) -> Self {
        value.path
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct ParentVarSymbolPath {
    path: SymbolPathBuf
}

impl ParentVarSymbolPath {
    pub fn new(parent_path: &SymbolPath) -> Self {
        let mut path = parent_path.to_owned();
        path.push(Keyword::Parent.as_ref(), SymbolCategory::Data);

        Self { path }
    }
}

impl From<ParentVarSymbolPath> for SymbolPathBuf {
    fn from(value: ParentVarSymbolPath) -> Self {
        value.path
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct VirtualParentVarSymbolPath {
    path: SymbolPathBuf
}

impl VirtualParentVarSymbolPath {
    pub fn new(parent_path: &SymbolPath) -> Self {
        let mut path = parent_path.to_owned();
        path.push(Keyword::VirtualParent.as_ref(), SymbolCategory::Data);

        Self { path }
    }
}

impl From<VirtualParentVarSymbolPath> for SymbolPathBuf {
    fn from(value: VirtualParentVarSymbolPath) -> Self {
        value.path
    }
}



// Path suffixes that disambiguate annotated functions from each other and the un-annotated counterpart
pub const CALLABLE_WRAPPER_PATH_SUFFIX: &'static str = "@wrapped"; 
pub const CALLABLE_REPLACER_PATH_SUFFIX: &'static str = "@replaced"; 


#[derive(Debug, Clone, Shrinkwrap)]
pub struct GlobalCallableReplacerSymbolPath(SymbolPathBuf);

impl GlobalCallableReplacerSymbolPath {
    pub fn new(name: &str) -> Self {
        let path = SymbolPathBuf::new(&format!("{}{}", name, CALLABLE_REPLACER_PATH_SUFFIX), SymbolCategory::Callable);

        Self(path)
    }

    pub fn function_name(&self) -> &str {
        self.0.components().last().and_then(|c| c.name.strip_suffix(CALLABLE_REPLACER_PATH_SUFFIX)).unwrap_or_default()
    }
}

impl From<GlobalCallableReplacerSymbolPath> for SymbolPathBuf {
    fn from(value: GlobalCallableReplacerSymbolPath) -> Self {
        value.0
    }
}

impl From<GlobalCallableSymbolPath> for GlobalCallableReplacerSymbolPath {
    fn from(value: GlobalCallableSymbolPath) -> Self {
        let name = value.components()
            .last().unwrap()
            .name.to_string();

        Self::new(&name)
    }
}

impl From<GlobalCallableReplacerSymbolPath> for GlobalCallableSymbolPath {
    fn from(value: GlobalCallableReplacerSymbolPath) -> Self {
        let name = value.components()
            .next().unwrap()
            .name
            .strip_suffix(CALLABLE_REPLACER_PATH_SUFFIX).unwrap()
            .to_string();
        
        let mut path = value.0.clone();
        path.pop();
        path.push(&name, SymbolCategory::Callable);

        GlobalCallableSymbolPath(path)
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct MemberCallableReplacerSymbolPath(SymbolPathBuf);

impl MemberCallableReplacerSymbolPath {
    pub fn new(class_name: &str, name: &str) -> Self {
        let mut path = SymbolPathBuf::new(class_name, SymbolCategory::Type);
        path.push(&format!("{}{}", name, CALLABLE_REPLACER_PATH_SUFFIX), SymbolCategory::Callable);

        Self(path)
    }

    pub fn function_name(&self) -> &str {
        self.0.components().last().and_then(|c| c.name.strip_suffix(CALLABLE_REPLACER_PATH_SUFFIX)).unwrap_or_default()
    }
}

impl From<MemberCallableReplacerSymbolPath> for SymbolPathBuf {
    fn from(value: MemberCallableReplacerSymbolPath) -> Self {
        value.0
    }
}

impl From<MemberCallableSymbolPath> for MemberCallableReplacerSymbolPath {
    fn from(value: MemberCallableSymbolPath) -> Self {
        let name = value.components()
            .last().unwrap()
            .name.to_string();

        let mut path = value.0;
        path.pop();
        path.push(&format!("{}{}", name, CALLABLE_REPLACER_PATH_SUFFIX), SymbolCategory::Callable);

        MemberCallableReplacerSymbolPath(path)
    }
}

impl From<MemberCallableReplacerSymbolPath> for MemberCallableSymbolPath {
    fn from(value: MemberCallableReplacerSymbolPath) -> Self {
        let name = value.components()
            .last().unwrap()
            .name
            .strip_suffix(CALLABLE_REPLACER_PATH_SUFFIX).unwrap()
            .to_string();
        
        let mut path = value.0.clone();
        path.pop();
        path.push(&name, SymbolCategory::Callable);

        MemberCallableSymbolPath(path)
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct MemberCallableWrapperSymbolPath(SymbolPathBuf);

impl MemberCallableWrapperSymbolPath {
    pub fn new(class_name: &str, name: &str) -> Self {
        let mut path = SymbolPathBuf::new(class_name, SymbolCategory::Type);
        path.push(&format!("{}{}", name, CALLABLE_WRAPPER_PATH_SUFFIX), SymbolCategory::Callable);

        Self(path)
    }

    pub fn function_name(&self) -> &str {
        self.0.components().last().and_then(|c| c.name.strip_suffix(CALLABLE_WRAPPER_PATH_SUFFIX)).unwrap_or_default()
    }
}

impl From<MemberCallableWrapperSymbolPath> for SymbolPathBuf {
    fn from(value: MemberCallableWrapperSymbolPath) -> Self {
        value.0
    }
}

impl From<MemberCallableWrapperSymbolPath> for MemberCallableSymbolPath {
    fn from(value: MemberCallableWrapperSymbolPath) -> Self {
        let name = value.components()
            .last().unwrap()
            .name
            .strip_suffix(CALLABLE_WRAPPER_PATH_SUFFIX).unwrap()
            .to_string();
        
        let mut path = value.0.clone();
        path.pop();
        path.push(&name, SymbolCategory::Callable);

        MemberCallableSymbolPath(path)
    }
}

impl From<MemberCallableSymbolPath> for MemberCallableWrapperSymbolPath  {
    fn from(value: MemberCallableSymbolPath) -> Self {
        let name = value.components()
            .last().unwrap()
            .name.to_string();

        let mut path = value.0;
        path.pop();
        path.push(&format!("{}{}", name, CALLABLE_WRAPPER_PATH_SUFFIX), SymbolCategory::Callable);

        MemberCallableWrapperSymbolPath(path)
    }
}