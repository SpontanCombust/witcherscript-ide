use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::{ClassSymbol, PathOccupiedError, StateSymbol, Symbol, SymbolLocation, SymbolTable, SymbolVariant};

//TODO needs additional symbol masking mechanism - search only in paths not present in previous symtabs
/// A type that can perform data fetching operations on many symbol tables
/// until that data is found.
/// Can be created from a iterator over symbol tables.
/// Values are attempted to be fetched from symbol tables in the order that they are in the iterator.
pub struct SymbolTableMarcher<It> {
    it : It
}

impl<It> Clone for SymbolTableMarcher<It>
where It: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self { it: self.it.clone() }
    }
}

impl<'a, It> SymbolTableMarcher<It> 
where It: Iterator<Item = &'a SymbolTable> + 'a {
    pub fn test_contains(mut self, path: &SymbolPath) -> Result<(), PathOccupiedError> {
        while let Some(symtab) = self.it.next() {
            symtab.test_contains(path)?;
        }

        Ok(())
    }

    pub fn contains(mut self, path: &SymbolPath) -> bool {
        self.it.find(|symtab| symtab.contains(path)).is_some()
    }

    pub fn find_containing(self, path: &SymbolPath) -> Option<&'a SymbolTable> {
        self.march(|symtab| if symtab.contains(path) { Some(symtab) } else { None })   
    }

    #[inline]
    pub fn get(self, path: &SymbolPath) -> Option<&'a SymbolVariant> {
        self.march(|symtab| symtab.get(path))
    }

    #[inline]
    pub fn locate(self, path: &SymbolPath) -> Option<SymbolLocation> {
        self.march(|symtab| symtab.locate(path))
    }

    #[inline]
    pub fn get_with_location(self, path: &SymbolPath) -> Option<(&'a SymbolVariant, SymbolLocation)> {
        self.march(|symtab| symtab.get_with_location(path))
    }

    #[inline]
    pub fn class_hierarchy(self, class_path: &SymbolPath) -> ClassHierarchy<It> {
        ClassHierarchy::new(self, class_path)
    }

    #[inline]
    pub fn class_states(self, class_path: &SymbolPath) -> ClassStates<'a>
    where It: 'a {
        ClassStates::new(self, class_path)
    }

    /// Iterator over base states of the given state starting from it.
    /// Does not include the CScriptableState class, which all state types derive from.
    #[inline]
    pub fn state_hierarchy(self, state_path: &SymbolPath) -> StateHierarchy<It> {
        StateHierarchy::new(self, state_path)
    }


    fn march<T, F>(mut self, mut f: F) -> Option<T> 
    where F: FnMut(&'a SymbolTable) -> Option<T> {
        while let Some(symtab) = self.it.next() {
            if let Some(val) = f(symtab) {
                return Some(val);
            }
        }

        None
    }
}


pub trait IntoSymbolTableMarcher {
    fn into_marcher(self) -> SymbolTableMarcher<Self> where Self: Sized;
}

impl<'a, It> IntoSymbolTableMarcher for It
where It: Iterator<Item = &'a SymbolTable> {
    fn into_marcher(self) -> SymbolTableMarcher<Self> where Self: Sized {
        SymbolTableMarcher { it: self }
    }
}


#[derive(Clone)]
pub struct ClassHierarchy<It> {
    marcher: SymbolTableMarcher<It>,
    current_path: SymbolPathBuf
}

impl<It> ClassHierarchy<It> {
    fn new(marcher: SymbolTableMarcher<It>, start_path: &SymbolPath) -> Self {
        Self {
            marcher,
            current_path: start_path.to_owned()
        }
    }
}

impl<'a, It> Iterator for ClassHierarchy<It> 
where It: Iterator<Item = &'a SymbolTable> + 'a + Clone {
    type Item = &'a ClassSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_path.is_empty() {
            None
        } else if let Some(class) = self.marcher.clone().get(&self.current_path).and_then(|v| v.try_as_class_ref()) {
            self.current_path = class.base_path.as_ref().map(|p| p.clone().into()).unwrap_or_default();
            Some(class)
        } else {
            None
        }
    }
}


pub struct ClassStates<'st> {
    it: Box<dyn Iterator<Item = &'st StateSymbol> + 'st>
}

impl<'st> ClassStates<'st> {
    fn new<It>(marcher: SymbolTableMarcher<It>, class_path: &SymbolPath) -> Self 
    where It: Iterator<Item = &'st SymbolTable> + 'st {
        let class_path = class_path.to_owned();
        let it = marcher.it.map(move |symtab| {
            let class_path = class_path.to_owned();
            symtab.symbols.iter()
                .filter_map(|(_, symvar)| symvar.try_as_state_ref())
                .filter(move |state_sym| state_sym.parent_class_path() == &class_path)
            })
            .flatten();

        Self { it: Box::new(it) }
    }
}

impl<'st> Iterator for ClassStates<'st> {
    type Item = &'st StateSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}


/// Iterator over base states of the given state starting from it.
/// Does not include the CScriptableState class, which all state types derive from.
#[derive(Clone)]
pub struct StateHierarchy<It> {
    marcher: SymbolTableMarcher<It>,
    current_state_path: SymbolPathBuf
}

impl<It> StateHierarchy<It> {
    fn new(marcher: SymbolTableMarcher<It>, state_path: &SymbolPath) -> Self {
        Self {
            marcher,
            current_state_path: state_path.to_owned()
        }
    }
}

impl<'a, It> Iterator for StateHierarchy<It> 
where It: Iterator<Item = &'a SymbolTable> + 'a + Clone {
    type Item = &'a StateSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_state_path.is_empty() {
            return None;
        } 
        
        if let Some(current_state_sym) = self.marcher.clone().get(&self.current_state_path).and_then(|v| v.try_as_state_ref()) {
            self.current_state_path.clear();

            if let Some(base_state_name) = &current_state_sym.base_state_name {
                for class in self.marcher.clone().class_hierarchy(current_state_sym.parent_class_path()) {
                    for state in self.marcher.clone().class_states(class.path()) {
                        if state.state_name() == base_state_name {
                            state.path().clone_into(&mut self.current_state_path);
                        }
                    }
                }
            }

            Some(current_state_sym)
        } else {
            None
        }
    }
}