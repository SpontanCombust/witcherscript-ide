use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::{ClassSymbol, PathOccupiedError, StateSymbol, SymbolLocation, SymbolTable, SymbolVariant};


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
where It: Iterator<Item = &'a SymbolTable> {
    pub fn contains(mut self, path: &SymbolPath) -> Result<(), PathOccupiedError> {
        while let Some(symtab) = self.it.next() {
            symtab.contains(path)?;
        }

        Ok(())
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
    pub fn class_hierarchy(self, sympath: &SymbolPath) -> ClassHierarchy<It> {
        ClassHierarchy::new(self, sympath)
    }

    #[inline]
    pub fn state_hierarchy(self, sympath: &SymbolPath) -> StateHierarchy<It> {
        StateHierarchy::new(self, sympath)
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
where It: Iterator<Item = &'a SymbolTable> + Clone {
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


#[derive(Clone)]
pub struct StateHierarchy<It> {
    marcher: SymbolTableMarcher<It>,
    current_path: SymbolPathBuf
}

impl<It> StateHierarchy<It> {
    fn new(marcher: SymbolTableMarcher<It>, start_path: &SymbolPath) -> Self {
        Self {
            marcher,
            current_path: start_path.to_owned()
        }
    }
}

impl<'a, It> Iterator for StateHierarchy<It> 
where It: Iterator<Item = &'a SymbolTable> + Clone {
    type Item = &'a StateSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_path.is_empty() {
            None
        } else if let Some(state) = self.marcher.clone().get(&self.current_path).and_then(|v| v.try_as_state_ref()) {
            self.current_path = state.base_state_path.as_ref().map(|p| p.clone().into()).unwrap_or_default();
            Some(state)
        } else {
            None
        }
    }
}