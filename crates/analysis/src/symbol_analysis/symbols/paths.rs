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
use super::{SpecialVarSymbolKind, SymbolCategory};


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

    pub fn empty() -> Self {
        Self(SymbolPathBuf::empty())
    }
}

impl From<BasicTypeSymbolPath> for SymbolPathBuf {
    fn from(value: BasicTypeSymbolPath) -> Self {
        value.0
    }
}

impl Default for BasicTypeSymbolPath {
    fn default() -> Self {
        Self::empty()
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

    pub fn empty() -> Self {
        Self {
            path: SymbolPathBuf::empty(),
            type_arg_path: Box::new(TypeSymbolPath::empty())
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
    pub fn empty() -> Self {
        Self::BasicOrState(BasicTypeSymbolPath::empty())
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
pub struct SpecialVarSymbolPath {
    #[shrinkwrap(main_field)]
    path: SymbolPathBuf,
    pub kind: SpecialVarSymbolKind
}

impl SpecialVarSymbolPath {
    pub fn new(parent_path: &SymbolPath, kind: SpecialVarSymbolKind) -> Self {
        let name = match kind {
            SpecialVarSymbolKind::This => Keyword::This.as_ref(),
            SpecialVarSymbolKind::Super => Keyword::Super.as_ref(),
            SpecialVarSymbolKind::Parent => Keyword::Parent.as_ref(),
            SpecialVarSymbolKind::VirtualParent => Keyword::VirtualParent.as_ref(),
        };

        let mut path = parent_path.to_owned();
        path.push(name, SymbolCategory::Data);

        Self {
            path,
            kind
        }
    }
}

impl From<SpecialVarSymbolPath> for SymbolPathBuf {
    fn from(value: SpecialVarSymbolPath) -> Self {
        value.path
    }
}