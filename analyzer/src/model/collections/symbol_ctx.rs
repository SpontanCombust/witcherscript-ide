use thiserror::Error;
use uuid::Uuid;
use std::{collections::HashMap, borrow::Borrow, hash::Hash};
use crate::model::symbols::*;


// All of this just to not have to allocate String on every map lookup and to not use lifetime annotations on the type

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    name: String,
    category: SymbolCategory
}

impl Key {
    fn new(name: &str, category: SymbolCategory) -> Self {
        Self {
            name: name.to_owned(),
            category
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct BorrowedKey<'s> {
    name: &'s str,
    category: SymbolCategory
}

impl<'s> BorrowedKey<'s> {
    fn new(name: &'s str, category: SymbolCategory) -> Self {
        Self {
            name, category
        }
    }
}

trait AsBorrowedKey {
    fn borrow(&self) -> BorrowedKey;
}

impl AsBorrowedKey for Key {
    fn borrow(&self) -> BorrowedKey {
        BorrowedKey { 
            name: &self.name, 
            category: self.category 
        }
    }
}

impl AsBorrowedKey for BorrowedKey<'_> {
    fn borrow(&self) -> BorrowedKey {
        *self
    }
}

impl<'s> Borrow<dyn AsBorrowedKey + 's> for Key {
    fn borrow(&self) -> &(dyn AsBorrowedKey + 's) {
        self
    }
}

impl PartialEq for (dyn AsBorrowedKey + '_) {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().eq(&other.borrow())
    }
}

impl Eq for (dyn AsBorrowedKey + '_) {}

impl Hash for (dyn AsBorrowedKey + '_) {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.borrow().hash(state)
    }
}



#[derive(Debug, Clone, Copy)]
pub struct SymbolPointer {
    /// Symbol Uuid
    pub id: Uuid,
    /// Type of the symbol
    pub typ: SymbolType,
}

impl SymbolPointer {
    pub fn new(id: Uuid, typ: SymbolType) -> Self {
        Self {
            id, typ
        }
    }
}


type Scope = HashMap<Key, SymbolPointer>;


/// Keeps track of all symbol identifiers that are valid and accessible in the current context.
/// Names on each deeper scope layer can overshadow the same name from higher layers.
#[derive(Debug, Clone)]
pub struct SymbolContext {
    stack: Vec<Scope>,
}


#[derive(Debug, Clone, Error)]
pub enum SymbolContextError {
    #[error("global var already exists for name {0:?}")]
    GlobalVarAlreadyExists(String, SymbolPointer),
    #[error("type already exists for name {0:?}")]
    TypeAlreadyExists(String, SymbolPointer),
    #[error("data already exists in the same scope for name {0:?}")]
    DataAlreadyExists(String, SymbolPointer),
    #[error("callable already exists in the same scope for name {0:?}")]
    CallableAlreadyExists(String, SymbolPointer),
    #[error("type with name {0:?} not found")]
    TypeDoesntExist(String),
}

impl SymbolContext {
    pub fn new() -> Self {
        Self {
            stack: vec![Scope::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.stack.push(Scope::new())
    }

    pub fn pop_scope(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    
    /// Get the value and relative scope it is contained in (0 means this scope, higher means parent scopes)
    fn get_with_rel_scope(&self, name: &str, category: SymbolCategory) -> Option<(&SymbolPointer, usize)> {
        let k = BorrowedKey::new(name, category);
        for (i, level) in self.stack.iter().rev().enumerate() {
            let v = level.get(&k as &dyn AsBorrowedKey);
            if v.is_some() {
                return v.map(|opt| (opt, i));
            }
        }

        None
    }

    pub fn contains(&self, name: &str, category: SymbolCategory) -> bool {
        self.get_with_rel_scope(name, category).is_some()
    }

    pub fn get(&self, name: &str, category: SymbolCategory) -> Option<&SymbolPointer> {
        self.get_with_rel_scope(name, category).map(|v| v.0)
    }
    
    /// If a symbol info with that configuration cannot be inserted, returns Err as to why that can't be done.
    pub fn can_insert(&self, name: &str, typ: SymbolType) -> Result<(), SymbolContextError> {
        use SymbolContextError::*;

        let exist_data = self.get_with_rel_scope(name, SymbolCategory::Data);
        if let Some((data_val, _)) = exist_data {
            // global var names are prohibited from being re-used in any context
            // as a sidenote, globals are actually already defined on the lexer level (in the WS compiler)
            if data_val.typ == SymbolType::GlobalVar {
                return Err(GlobalVarAlreadyExists(name.to_string(), data_val.clone()));
            }
        }
        
        let cat = typ.category();
        match cat {
            SymbolCategory::Type |
            SymbolCategory::Data => {
                // If there is a type defined with that name, always fail.
                if let Some((typ_val, _)) = self.get_with_rel_scope(name, SymbolCategory::Type) {
                    return Err(TypeAlreadyExists(name.to_string(), typ_val.clone()));
                }
                // If there is a var or constant defined, but it exists in a higher scope, allow to obstruct it.
                // In the case of types, they can only be defined in the global scope. So when they check against
                // data, they check against defined enum members.
                else if let Some((data_val, data_scope)) = exist_data {
                    if data_scope == 0 {
                        return Err(DataAlreadyExists(name.to_string(), data_val.clone()));
                    }
                } 
            },
            SymbolCategory::Callable => {
                // Callables only need to check against other callables in the same scope.
                // They have the advantage of being able to be easily distinguished from the other two categories
                // - they are always used with `()` operator. Functions in WS are not first-class objects afaik.
                if let Some((callable_val, callable_scope)) = self.get_with_rel_scope(name, SymbolCategory::Callable) {
                    if callable_scope == 0 {
                        return Err(CallableAlreadyExists(name.to_string(), callable_val.clone()));
                    }
                }                 
            },
        }

        Ok(())
    }

    /// Inserts a symbol mapping into the stack.
    /// Make sure to check with `can_insert` beforehand.
    pub fn insert<S: SymbolData>(&mut self, sym: &Symbol<S>) {
        let cat = sym.typ().category();
        let kv = (
            Key::new(sym.name(), cat), 
            SymbolPointer::new(sym.id(), sym.typ())
        );

        if cat == SymbolCategory::Type {
            // always insert types into the highest (global) scope
            self.stack.first_mut().unwrap().insert(kv.0, kv.1);
        } else {
            // the rest of symbols can be scope dependant
            self.stack.last_mut().unwrap().insert(kv.0, kv.1);
        }
    }

    /// Inserts a name alias for a symbol. Symbol must be a type.
    /// Used for primitives.
    pub fn insert_alias<S: SymbolData>(&mut self, sym: &Symbol<S>, alias: &str) -> Result<(), SymbolContextError> {
        if !self.contains(sym.name(), SymbolCategory::Type) {
            return Err(SymbolContextError::TypeDoesntExist(sym.name().to_string()));
        }

        let kv = (
            Key::new(alias, SymbolCategory::Type), 
            SymbolPointer::new(sym.id(), sym.typ())
        );

        self.stack.first_mut().unwrap().insert(kv.0, kv.1);

        Ok(())
    }
}
