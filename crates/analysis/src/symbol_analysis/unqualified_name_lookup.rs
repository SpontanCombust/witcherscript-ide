use std::{collections::HashMap, borrow::Borrow, hash::Hash};
use super::{symbol_path::{SymbolPath, SymbolPathBuf}, symbols::SymbolCategory};


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


type Scope = HashMap<Key, SymbolPathBuf>;


/// Keeps track of all unqualified symbol identifiers that are valid and accessible in the current context.
/// Names on each deeper scope layer can overshadow the same name from higher layers.
/// It is used during function analysis, where identifiers may be used in an ambiguous way,
/// e.g. member var can be used without prefixing it with `this.`.
#[derive(Debug, Clone)]
pub struct UnqualifiedNameLookup {
    stack: Vec<Scope>
}

impl UnqualifiedNameLookup {
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
    
    pub fn contains(&self, name: &str, category: SymbolCategory) -> bool {
        let k = BorrowedKey::new(name, category);
        for level in self.stack.iter().rev() {
            if level.contains_key(&k as &dyn AsBorrowedKey) {
                return true;
            }
        }

        false
    }

    pub fn get(&self, name: &str, category: SymbolCategory) -> Option<&SymbolPath> {
        let k = BorrowedKey::new(name, category);
        for level in self.stack.iter().rev() {
            if let Some(path) = level.get(&k as &dyn AsBorrowedKey) {
                return Some(path);
            }
        }

        None
    }

    /// Inserts a symbol name mapping into the stack.
    pub fn insert(&mut self, path: &SymbolPath) {
        let comp = path.components().last().unwrap();

        self.stack.last_mut().unwrap().insert(
            Key::new(comp.name.to_string(), comp.category),
            path.to_owned()
        );
    }
}
