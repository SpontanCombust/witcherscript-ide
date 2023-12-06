use thiserror::Error;
use uuid::Uuid;
use std::{collections::HashMap, borrow::Cow};
use super::symbols::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SymbolTableKey<'t> {
    name: Cow<'t, str>,
    category: SymbolCategory
}

impl<'t> SymbolTableKey<'t> {
    pub fn new<S>(name: S, category: SymbolCategory) -> Self 
    where S: Into<Cow<'t, str>> {
        Self {
            name: name.into(),
            category
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct SymbolTableValue {
    /// Symbol Uuid
    pub id: Uuid,
    /// Type of the symbol
    pub typ: SymbolType,
}

impl SymbolTableValue {
    pub fn new(id: Uuid, typ: SymbolType) -> Self {
        Self {
            id, typ
        }
    }
}


type SymbolTableScope<'t> = HashMap<SymbolTableKey<'t>, SymbolTableValue>;


/// A scope aware map of symbols with access to their identifiers by name and symbol category.
#[derive(Debug, Clone)]
pub struct SymbolTable<'t> {
    stack: Vec<SymbolTableScope<'t>>,
}


#[derive(Debug, Clone, Error)]
pub enum SymbolTableError {
    #[error("global var already exists for name {0:?}")]
    GlobalVarAlreadyExists(String, SymbolTableValue),
    #[error("type already exists for name {0:?}")]
    TypeAlreadyExists(String, SymbolTableValue),
    #[error("data already exists in the same scope for name {0:?}")]
    DataAlreadyExists(String, SymbolTableValue),
    #[error("callable already exists in the same scope for name {0:?}")]
    CallableAlreadyExists(String, SymbolTableValue),
}

impl<'t> SymbolTable<'t> {
    pub fn new() -> Self {
        Self {
            stack: vec![SymbolTableScope::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.stack.push(SymbolTableScope::new())
    }

    pub fn pop_scope(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    
    /// Get the value and relative scope it is contained in (0 means this scope, higher means parent scopes)
    fn get_with_rel_scope<S>(&self, name: S, category: SymbolCategory) -> Option<(&SymbolTableValue, usize)> 
    where S: Into<Cow<'t, str>> {
        let k = SymbolTableKey::new(name, category);
        for (i, level) in self.stack.iter().rev().enumerate() {
            let v = level.get(&k);
            if v.is_some() {
                return v.map(|opt| (opt, i));
            }
        }

        None
    }

    pub fn contains<S>(&self, name: S, category: SymbolCategory) -> bool
    where S: Into<Cow<'t, str>> {
        self.get_with_rel_scope(name, category).is_some()
    }

    pub fn get<S>(&self, name: S, category: SymbolCategory) -> Option<&SymbolTableValue> 
    where S: Into<Cow<'t, str>> {
        self.get_with_rel_scope(name, category).map(|v| v.0)
    }
    
    /// If a symbol with that configuration can be inserted, returns None.
    /// Otherwise, returns the reason as to why that can't be done.
    pub fn can_insert(&self, name: &str, typ: SymbolType) -> Option<SymbolTableError> {
        use SymbolTableError::*;

        let exist_data = self.get_with_rel_scope(name, SymbolCategory::Data);
        if let Some((data_val, _)) = exist_data {
            // global var names are prohibited from being re-used in any context
            // globals are actually already defined on the lexel level (in the WS compiler)
            if data_val.typ == SymbolType::GlobalVar {
                return Some(GlobalVarAlreadyExists(name.to_string(), data_val.clone()));
            }
        }
        
        let cat = typ.category();
        match cat {
            SymbolCategory::Type |
            SymbolCategory::Data => {
                // If there is a type defined with that name, always fail.
                if let Some((typ_val, _)) = self.get_with_rel_scope(name, SymbolCategory::Type) {
                    return Some(TypeAlreadyExists(name.to_string(), typ_val.clone()));
                }
                // If there is a var or constant defined, but it exists in a higher scope, allow to obstruct it.
                // In the case of types, they can only be defined in the global scope. So when they check against
                // data, they check against defined enum members.
                else if let Some((data_val, data_scope)) = exist_data {
                    if data_scope == 0 {
                        return Some(DataAlreadyExists(name.to_string(), data_val.clone()));
                    }
                } 
            },
            SymbolCategory::Callable => {
                // Callables only need to check against other callables in the same scope.
                // They have the advantage of being able to be easily distinguished from the other two categories
                // - they are always used with `()` operator. Functions in WS are not first-class objects afaik.
                if let Some((callable_val, callable_scope)) = self.get_with_rel_scope(name, SymbolCategory::Callable) {
                    if callable_scope == 0 {
                        return Some(CallableAlreadyExists(name.to_string(), callable_val.clone()));
                    }
                }                 
            },
        }

        None
    }

    /// Inserts a symbol mapping into the table.
    /// Make sure to check with `can_insert` beforehand.
    pub fn insert(&mut self, name: &str, id: Uuid, typ: SymbolType) {
        let cat = typ.category();
        if cat == SymbolCategory::Type {
            // always insert types into the highest (global) scope
            self.stack.first_mut().unwrap().insert(
                SymbolTableKey::new(name.to_string(), cat), 
                SymbolTableValue::new(id, typ)
            );
        } else {
            // the rest of symbols can be scope dependant
            self.stack.last_mut().unwrap().insert(
                SymbolTableKey::new(name.to_string(), cat), 
                SymbolTableValue::new(id, typ)
            );
        }
    }
}



#[derive(Debug, Clone, Default)]
pub struct SymbolDb {
    pub primitives: HashMap<Uuid, PrimitiveTypeSymbol>,
    pub enums: HashMap<Uuid, EnumSymbol>,
    pub structs: HashMap<Uuid, StructSymbol>,
    pub classes: HashMap<Uuid, ClassSymbol>,
    pub states: HashMap<Uuid, StateSymbol>,
    pub arrays: HashMap<Uuid, ArrayTypeSymbol>,

    pub enum_members: HashMap<Uuid, EnumMemberSymbol>,

    pub global_funcs: HashMap<Uuid, GlobalFunctionSymbol>,
    pub member_funcs: HashMap<Uuid, MemberFunctionSymbol>,
    pub events: HashMap<Uuid, EventSymbol>,

    pub params: HashMap<Uuid, FunctionParameterSymbol>,

    pub global_vars: HashMap<Uuid, GlobalVarSymbol>,
    pub member_vars: HashMap<Uuid, MemberVarSymbol>,
    pub autobinds: HashMap<Uuid, AutobindSymbol>,
    pub local_vars: HashMap<Uuid, LocalVarSymbol>
}

impl SymbolDb {
    pub fn new() -> Self {
        Self::default()
    }
}