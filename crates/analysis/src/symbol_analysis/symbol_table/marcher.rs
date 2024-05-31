use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::{ClassSymbol, PathOccupiedError, StateSymbol, Symbol, SymbolLocation, SymbolTable, SymbolVariant};


//TODO needs additional symbol masking mechanism - search only in paths not present in previous symtabs
/// A type that can perform data fetching operations on many symbol tables
/// until that data is found.
/// Can be created from a iterator over symbol tables.
/// Values are fetched from symbol tables in the order that they are in the iterator.
#[derive(Clone)]
pub struct SymbolTableMarcher<'a> {
    inner : Vec<&'a SymbolTable>
}

impl<'a> SymbolTableMarcher<'a> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new()
        }
    }

    pub fn add_step(&mut self, symtab: &'a SymbolTable) {
        self.inner.push(symtab)
    }


    pub fn test_contains_symbol(&self, path: &SymbolPath) -> Result<(), PathOccupiedError> {
        for symtab in &self.inner {
            symtab.test_contains_symbol(path)?;
        }

        Ok(())
    }

    pub fn contains_symbol(&self, path: &SymbolPath) -> bool {
        self.inner.iter().any(|symtab| symtab.contains_symbol(path))
    }

    pub fn find_table_containing_symbol(&self, path: &SymbolPath) -> Option<&'a SymbolTable> {
        self.march(|symtab| if symtab.contains_symbol(path) { Some(symtab) } else { None })   
    }

    #[inline]
    pub fn get_symbol(&self, path: &SymbolPath) -> Option<&'a SymbolVariant> {
        self.march(|symtab| symtab.get_symbol(path))
    }

    #[inline]
    pub fn get_symbol_with_containing_table(&self, path: &SymbolPath) -> Option<(&'a SymbolTable, &'a SymbolVariant)> {
        self.march(|symtab| {
            if let Some(symvar) = symtab.get_symbol(path) {
                Some((symtab, symvar))
            } else {
                None
            }
        })
    }

    #[inline]
    pub fn locate_symbol(&self, path: &SymbolPath) -> Option<&'a SymbolLocation> {
        self.march(|symtab| symtab.locate_symbol(path))
    }

    #[inline]
    pub fn get_symbol_with_location(&self, path: &SymbolPath) -> Option<(&'a SymbolVariant, &'a SymbolLocation)> {
        self.march(|symtab| symtab.get_symbol_with_location(path))
    }

    #[inline]
    pub fn class_hierarchy(&self, class_path: &SymbolPath) -> ClassHierarchy<'a> {
        ClassHierarchy::new(self.clone(), class_path)
    }

    #[inline]
    pub fn class_states(&self, class_path: &SymbolPath) -> ClassStates<'a> {
        ClassStates::new(self.clone(), class_path)
    }

    /// Iterator over base states of the given state starting from it.
    /// Does not include the CScriptableState class, which all state types derive from.
    #[inline]
    pub fn state_hierarchy(&self, state_path: &SymbolPath) -> StateHierarchy<'a> {
        StateHierarchy::new(self.clone(), state_path)
    }


    fn march<T, F>(&self, mut f: F) -> Option<T> 
    where F: FnMut(&'a SymbolTable) -> Option<T> {
        for symtab in &self.inner {
            if let Some(val) = f(symtab) {
                return Some(val);
            }
        }

        None
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
        } else if let Some(class) = self.marcher.get_symbol(&self.current_path).and_then(|v| v.try_as_class_ref()) {
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
        let it = marcher.inner.into_iter().map(move |symtab| {
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
        
        if let Some(current_state_sym) = self.marcher.get_symbol(&self.current_state_path).and_then(|v| v.try_as_state_ref()) {
            self.current_state_path.clear();
            //FIXME switch to using `base_state_path` when that is possible
            if let Some(base_state_name) = &current_state_sym.base_state_name {
                for class in self.marcher.class_hierarchy(current_state_sym.parent_class_path()) {
                    for state in self.marcher.class_states(class.path()) {
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