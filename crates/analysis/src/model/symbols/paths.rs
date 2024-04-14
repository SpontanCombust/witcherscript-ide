/// Symbol paths are not all uniform, especially for types.
/// Arrays for example are identified with `array<Type>`, where `array` and `Type` are distinct identifiers.
/// Following utility wrapper types exist to gather these exceptions under a single umbrella, 
/// detached from symbols themselves (in the sense of creating them).
/// They provide convenience constructors and some of them house extra fields.
/// 
/// SymbolPath is used as a means of uniquely identifying a symbol in a global name space. 
/// During script file parsing a given symbol has to be first checked whether it is not a duplicate or whatnot.
/// This means paths should be deduced before eventual symbol creation in case of an error.
/// 
/// Symbols that are not scanned from .ws files do not need dedicated path types as they exist regardless 
/// of which files are scanned.


use std::{ops::Deref, borrow::Borrow};
use shrinkwraprs::Shrinkwrap;
use crate::model::symbol_path::{SymbolPath, SymbolPathBuf};
use super::SymbolCategory;


#[derive(Debug, Clone, Shrinkwrap)]
pub struct DataSymbolPath(SymbolPathBuf);

impl DataSymbolPath {
    /// Data is always a child of some data structure or function (except globals)
    pub fn new(parent_path: &SymbolPath, name: &str) -> Self {
        Self(parent_path.join(&SymbolPathBuf::new(name, SymbolCategory::Data)))
    }

    pub fn empty() -> Self {
        Self(SymbolPathBuf::empty())
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct GlobalCallableSymbolPath(SymbolPathBuf);

impl GlobalCallableSymbolPath {
    pub fn new(name: &str) -> Self {
        Self(SymbolPathBuf::new(name, SymbolCategory::Callable))
    }

    pub fn empty() -> Self {
        Self(SymbolPathBuf::empty())
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct MemberCallableSymbolPath(SymbolPathBuf);

impl MemberCallableSymbolPath {
    pub fn new(parent_path: &SymbolPath, name: &str) -> Self {
        Self(parent_path.join(&SymbolPathBuf::new(name, SymbolCategory::Callable)))
    }

    pub fn empty() -> Self {
        Self(SymbolPathBuf::empty())
    }
}


#[derive(Debug, Clone, Shrinkwrap)]
pub struct BasicTypeSymbolPath(SymbolPathBuf);

impl BasicTypeSymbolPath {
    pub fn new(name: &str) -> Self {
        Self(SymbolPathBuf::new(name, SymbolCategory::Type))
    }

    pub fn empty() -> Self {
        Self(SymbolPathBuf::empty())
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
    pub fn new(state_name: &str, parent_class_path: BasicTypeSymbolPath) -> Self {
        Self {
            path: SymbolPathBuf::new(&format!("{}State{}", parent_class_path.to_string(), state_name), SymbolCategory::Type),
            state_name: state_name.to_string(),
            parent_class_path
        }
    }

    pub fn empty() -> Self {
        Self {
            path: SymbolPathBuf::empty(),
            state_name: String::new(),
            parent_class_path: BasicTypeSymbolPath::empty()
        }
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
        Self {
            path: SymbolPathBuf::new(&format!("array<{}>", type_arg_path.to_string()), SymbolCategory::Type),
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


#[derive(Debug, Clone)]
pub enum TypeSymbolPath {
    Basic(BasicTypeSymbolPath),
    Array(ArrayTypeSymbolPath)
    // StateSymbolPath not included, because notation `state X in Y` 
    // is used only in state's declaration and not when its class is mentioned
}

impl TypeSymbolPath {
    pub fn empty() -> Self {
        Self::Basic(BasicTypeSymbolPath::empty())
    }
}

impl Borrow<SymbolPathBuf> for TypeSymbolPath {
    fn borrow(&self) -> &SymbolPathBuf {
        match self {
            TypeSymbolPath::Basic(basic) => &basic.0,
            TypeSymbolPath::Array(array) => &array.path,
        }
    }
}

impl Deref for TypeSymbolPath {
    type Target = SymbolPathBuf;

    fn deref(&self) -> &Self::Target {
        match self {
            TypeSymbolPath::Basic(basic) => &basic.0,
            TypeSymbolPath::Array(array) => &array.path,
        }
    }
}