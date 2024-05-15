use std::{collections::btree_map, path::Path};
use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::{ClassSymbol, StateSymbol, SymbolTable, SymbolVariant};


/// Iterate over direct children of a symbol in a symbol hierarchy
#[derive(Clone)]
pub struct SymbolChildren<'st> {
    iter: btree_map::Range<'st, SymbolPathBuf, SymbolVariant>,
    parent_sympath: SymbolPathBuf,
    children_comp_count: usize
}

impl<'st> SymbolChildren<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, sympath: &SymbolPath) -> Self {
        Self {
            iter: symtab.symbols.range(sympath.to_owned()..),
            parent_sympath: sympath.to_owned(),
            children_comp_count: sympath.components().count() + 1
        }
    }
}

impl<'st> Iterator for SymbolChildren<'st> {
    type Item = &'st SymbolVariant;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
            .filter(|(sympath, _)| sympath.starts_with(&self.parent_sympath) && sympath.components().count() == self.children_comp_count)
            .map(|(_, variant)| variant)
    }
}


/// Iterate over symbols associated with a script file at a given path
pub struct FileSymbols<'st> {
    iter: Box<dyn Iterator<Item = &'st SymbolVariant> + 'st>
}

impl<'st> FileSymbols<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, local_source_path: &Path) -> Self {
        let roots = symtab.source_path_assocs
            .get(local_source_path)
            .map(|v| v.as_slice())
            .unwrap_or_default();

        let iter = roots.iter()
            .map(|root| symtab.symbols.range(root.to_owned()..)
                            .take_while(|(p, _)| p.starts_with(root))
                            .map(|(_, v)| v))
            .flatten();

        Self {
            iter: Box::new(iter)
        }
    }
} 

impl<'st> Iterator for FileSymbols<'st> {
    type Item = &'st SymbolVariant;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}


/// Iterator going through all base classes of a given class symbol 
/// starting from the class pointed to by the `start_path` parameter (if it points to an existing class symbol).
#[derive(Clone)]
pub struct ClassHierarchy<'st> {
    symtab: &'st SymbolTable,
    current_path: SymbolPathBuf
}

impl<'st> ClassHierarchy<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, start_path: &SymbolPath) -> Self {
        Self {
            symtab,
            current_path: start_path.to_owned()
        }
    }
}

impl<'st> Iterator for ClassHierarchy<'st> {
    type Item = &'st ClassSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_path.is_empty() {
            None
        } else if let Some(class) = self.symtab.get(&self.current_path).and_then(|v| v.try_as_class_ref()) {
            self.current_path = class.base_path.as_ref().map(|p| p.clone().into()).unwrap_or_default();
            Some(class)
        } else {
            None
        }
    }
}


/// Iterator going through all base states of a given state symbol
/// starting from the state pointed to by the `start_path` parameter (if it points to an existing state symbol).
#[derive(Clone)]
pub struct StateHierarchy<'st> {
    symtab: &'st SymbolTable,
    current_path: SymbolPathBuf
}

impl<'st> StateHierarchy<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, start_path: &SymbolPath) -> Self {
        Self {
            symtab,
            current_path: start_path.to_owned()
        }
    }
}

impl<'st> Iterator for StateHierarchy<'st> {
    type Item = &'st StateSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_path.is_empty() {
            None
        } else if let Some(state) = self.symtab.get(&self.current_path).and_then(|v| v.try_as_state_ref()) {
            self.current_path = state.base_state_path.as_ref().map(|p| p.clone().into()).unwrap_or_default();
            Some(state)
        } else {
            None
        }
    }
}
