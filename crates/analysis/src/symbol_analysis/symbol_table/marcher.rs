use dyn_clone::{clone_trait_object, DynClone};
use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::{ClassSymbol, PathOccupiedError, StateSymbol, Symbol, SymbolLocation, SymbolTable, SymbolVariant};


trait SymbolTableIter<'a>: Iterator<Item = &'a SymbolTable> + 'a + DynClone {}
clone_trait_object!(SymbolTableIter<'_>);

impl<'a, It> SymbolTableIter<'a> for It
where It: Iterator<Item = &'a SymbolTable> + 'a + Clone {}


//TODO needs additional symbol masking mechanism - search only in paths not present in previous symtabs
/// A type that can perform data fetching operations on many symbol tables
/// until that data is found.
/// Can be created from a iterator over symbol tables.
/// Values are attempted to be fetched from symbol tables in the order that they are in the iterator.
#[derive(Clone)]
pub struct SymbolTableMarcher<'a> {
    it : Box<dyn SymbolTableIter<'a>>
}

impl<'a> SymbolTableMarcher<'a> {
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
    pub fn class_hierarchy(self, class_path: &SymbolPath) -> ClassHierarchy<'a> {
        ClassHierarchy::new(self, class_path)
    }

    #[inline]
    pub fn class_states(self, class_path: &SymbolPath) -> ClassStates<'a> {
        ClassStates::new(self, class_path)
    }

    /// Iterator over base states of the given state starting from it.
    /// Does not include the CScriptableState class, which all state types derive from.
    #[inline]
    pub fn state_hierarchy(self, state_path: &SymbolPath) -> StateHierarchy<'a> {
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


pub trait IntoSymbolTableMarcher<'a> {
    fn into_marcher(self) -> SymbolTableMarcher<'a> where Self: Sized;
}

impl<'a, It> IntoSymbolTableMarcher<'a> for It
where It: SymbolTableIter<'a> {
    fn into_marcher(self) -> SymbolTableMarcher<'a> where Self: Sized {
        SymbolTableMarcher { it: Box::new(self) }
    }
}


#[derive(Clone)]
pub struct ClassHierarchy<'a> {
    marcher: SymbolTableMarcher<'a>,
    current_path: SymbolPathBuf
}

impl<'a> ClassHierarchy<'a> {
    fn new(marcher: SymbolTableMarcher<'a>, start_path: &SymbolPath) -> Self {
        Self {
            marcher,
            current_path: start_path.to_owned()
        }
    }
}

impl<'a> Iterator for ClassHierarchy<'a> {
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


pub struct ClassStates<'a> {
    it: Box<dyn Iterator<Item = &'a StateSymbol> + 'a>
}

impl<'a> ClassStates<'a> {
    fn new(marcher: SymbolTableMarcher<'a>, class_path: &SymbolPath) -> Self {
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

impl<'a> Iterator for ClassStates<'a> {
    type Item = &'a StateSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}


/// Iterator over base states of the given state starting from it.
/// Does not include the CScriptableState class, which all state types derive from.
#[derive(Clone)]
pub struct StateHierarchy<'a> {
    marcher: SymbolTableMarcher<'a>,
    current_state_path: SymbolPathBuf
}

impl<'a> StateHierarchy<'a> {
    fn new(marcher: SymbolTableMarcher<'a>, state_path: &SymbolPath) -> Self {
        Self {
            marcher,
            current_state_path: state_path.to_owned()
        }
    }
}

impl<'a> Iterator for StateHierarchy<'a> {
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