use std::{collections::HashMap, borrow::Borrow, hash::Hash};
use crate::model::{symbols::*, symbol_path::SymbolPathBuf};


// All of this just to not have to allocate String on every map lookup and to not use lifetime annotations on the type

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    name: String,
    category: SymbolCategory
}

impl Key {
    fn new(name: String, category: SymbolCategory) -> Self {
        Self {
            name,
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



#[derive(Debug, Clone)]
pub struct SymbolPointer {
    /// Symbol path
    pub path: SymbolPathBuf,
    /// Type of the symbol
    pub typ: SymbolType,
}

impl SymbolPointer {
    pub fn new(path: SymbolPathBuf, typ: SymbolType) -> Self {
        Self {
            path, typ
        }
    }
}


type Scope = HashMap<Key, SymbolPointer>;


/// Keeps track of all unqualified symbol identifiers that are valid and accessible in the current context.
/// Names on each deeper scope layer can overshadow the same name from higher layers.
/// It is used during function analysis, where identifiers may be used in an ambiguous way,
/// e.g. member var can be used without prefixing it with `this.`.
#[derive(Debug, Clone)]
pub struct SymbolContext {
    stack: Vec<Scope>
}

impl SymbolContext {
    pub fn new() -> Self {
        Self {
            stack: vec![Scope::new()]
        }
    }

    pub fn push_scope(&mut self) {
        self.stack.push(Scope::new())
    }

    pub fn pop_scope(&mut self) {
        // always keep at least one scope level
        if self.stack.len() > 1 {
            self.stack.pop();
        } else {
            // if there is one scope left, only clear its contents
            self.stack.last_mut().unwrap().clear();
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

    /// Inserts a symbol name mapping into the stack.
    pub fn insert(&mut self, sym: &impl Symbol) {
        self.stack.last_mut().unwrap().insert(
            Key::new(sym.name().to_string(), sym.typ().category()), 
            SymbolPointer::new(sym.path().to_sympath_buf(), sym.typ())
        );
    }

    /// Inserts a name alias for a symbol. Used for primitive type aliases.
    pub fn insert_alias(&mut self, sym: &impl Symbol, alias: SymbolPathBuf) {
        self.stack.last_mut().unwrap().insert(
            Key::new(alias.to_string(), sym.typ().category()), 
            SymbolPointer::new(sym.path().to_sympath_buf(), sym.typ())
        );
    }
}
