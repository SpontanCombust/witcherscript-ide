use std::{collections::btree_map, path::Path};
use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


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
        self.iter
            .find(|(sympath, _)| sympath.starts_with(&self.parent_sympath) && sympath.components().count() == self.children_comp_count)
            .map(|(_, variant)| variant)
    }
}


pub enum ClassSymbolChild<'st> {
    Var(&'st MemberVarSymbol),
    Autobind(&'st AutobindSymbol),
    Method(&'st MemberFunctionSymbol),
    Event(&'st EventSymbol)
}

#[derive(Clone)]
pub struct ClassSymbolChildren<'st> {
    iter: SymbolChildren<'st>
}

impl<'st> ClassSymbolChildren<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, class_sympath: &SymbolPath) -> Self {
        Self {
            iter: SymbolChildren::new(symtab, class_sympath)
        }
    }
}

impl<'st> Iterator for ClassSymbolChildren<'st> {
    type Item = ClassSymbolChild<'st>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find_map(|v| match v {
                SymbolVariant::MemberVar(s) => Some(ClassSymbolChild::Var(s)),
                SymbolVariant::Autobind(s) => Some(ClassSymbolChild::Autobind(s)),
                SymbolVariant::MemberFunc(s) => Some(ClassSymbolChild::Method(s)),
                SymbolVariant::Event(s) => Some(ClassSymbolChild::Event(s)),
                _ => None
            })
    }
}


pub type StateSymbolChild<'st> = ClassSymbolChild<'st>;
pub type StateSymbolChildren<'st> = ClassSymbolChildren<'st>;


#[derive(Clone)]
pub struct StructSymbolChildren<'st> {
    iter: SymbolChildren<'st>
}

impl<'st> StructSymbolChildren<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, struct_sympath: &SymbolPath) -> Self {
        Self {
            iter: SymbolChildren::new(symtab, struct_sympath)
        }
    }
}

impl<'st> Iterator for StructSymbolChildren<'st> {
    type Item = &'st MemberVarSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find_map(|v| v.try_as_member_var_ref())
    }
}


pub enum CallableSymbolChild<'st> {
    Param(&'st FunctionParameterSymbol),
    LocalVar(&'st LocalVarSymbol)
}

#[derive(Clone)]
pub struct CallableSymbolChildren<'st> {
    iter: SymbolChildren<'st>
}

impl<'st> CallableSymbolChildren<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, func_sympath: &SymbolPath) -> Self {
        Self {
            iter: SymbolChildren::new(symtab, func_sympath)
        }
    }
}

impl<'st> Iterator for CallableSymbolChildren<'st> {
    type Item = CallableSymbolChild<'st>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find_map(|v| match v {
                SymbolVariant::FuncParam(s) => Some(CallableSymbolChild::Param(s)),
                SymbolVariant::LocalVar(s) => Some(CallableSymbolChild::LocalVar(s)),
                _ => None
            })
    }
}


#[derive(Clone)]
pub struct ArrayTypeSymbolChildren<'st> {
    iter: SymbolChildren<'st>
}

impl<'st> ArrayTypeSymbolChildren<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, array_sympath: &SymbolPath) -> Self {
        Self {
            iter: SymbolChildren::new(symtab, array_sympath)
        }
    }
}

impl<'st> Iterator for ArrayTypeSymbolChildren<'st> {
    type Item = &'st ArrayTypeFunctionSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find_map(|v| match v {
                SymbolVariant::ArrayFunc(s) => Some(s),
                _ => None
            })
    }
}


#[derive(Clone)]
pub struct ArrayTypeFunctionSymbolChildren<'st> {
    iter: SymbolChildren<'st>
}

impl<'st> ArrayTypeFunctionSymbolChildren<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, array_func_sympath: &SymbolPath) -> Self {
        Self {
            iter: SymbolChildren::new(symtab, array_func_sympath)
        }
    }
}

impl<'st> Iterator for ArrayTypeFunctionSymbolChildren<'st> {
    type Item = &'st ArrayTypeFunctionParameterSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find_map(|v| match v {
                SymbolVariant::ArrayFuncParam(s) => Some(s),
                _ => None
            })
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
