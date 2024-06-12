use std::{collections::btree_map, marker::PhantomData, path::Path};
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


/// Iterate over direct children of a symbol in a symbol hierarchy with type filtration
#[derive(Clone)]
pub struct FilteredSymbolChildren<'st, F> {
    iter: SymbolChildren<'st>,
    filter_phantom: PhantomData<F>
}

impl<'st, F> FilteredSymbolChildren<'st, F> 
where F: ChildrenSymbolsFilter<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, symbol: &F) -> Self {
        Self {
            iter: SymbolChildren::new(symtab, symbol.path()),
            filter_phantom: PhantomData
        }
    }
}

impl<'st, F> Iterator for FilteredSymbolChildren<'st, F> 
where F: ChildrenSymbolsFilter<'st> {
    type Item = F::ChildRef;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|symvar| symvar.try_into().ok())
    }
}


pub trait ChildrenSymbolsFilter<'a>: Symbol {
    type ChildRef: TryFrom<&'a SymbolVariant> + 'a;
}


pub enum ClassSymbolChild<'st> {
    Var(&'st MemberVarSymbol),
    Autobind(&'st AutobindSymbol),
    Method(&'st MemberFunctionSymbol),
    Event(&'st EventSymbol)
}

impl<'a> TryFrom<&'a SymbolVariant> for ClassSymbolChild<'a> {
    type Error = ();

    fn try_from(value: &'a SymbolVariant) -> Result<Self, Self::Error> {
        match value {
            SymbolVariant::MemberVar(s) => Ok(ClassSymbolChild::Var(s)),
            SymbolVariant::Autobind(s) => Ok(ClassSymbolChild::Autobind(s)),
            SymbolVariant::MemberFunc(s) => Ok(ClassSymbolChild::Method(s)),
            SymbolVariant::Event(s) => Ok(ClassSymbolChild::Event(s)),
            _ => Err(())
        }
    }
}

impl<'a> ChildrenSymbolsFilter<'a> for ClassSymbol {
    type ChildRef = ClassSymbolChild<'a>;
}


pub type StateSymbolChild<'st> = ClassSymbolChild<'st>;

impl<'a> ChildrenSymbolsFilter<'a> for StateSymbol {
    type ChildRef = StateSymbolChild<'a>;
}


impl<'a> TryFrom<&'a SymbolVariant> for &'a MemberVarSymbol {
    type Error = ();

    fn try_from(value: &'a SymbolVariant) -> Result<Self, Self::Error> {
        value.try_as_member_var_ref().ok_or(())
    }
}

impl<'a> ChildrenSymbolsFilter<'a> for StructSymbol {
    type ChildRef = &'a MemberVarSymbol;
}


pub enum CallableSymbolChild<'st> {
    Param(&'st FunctionParameterSymbol),
    LocalVar(&'st LocalVarSymbol)
}

impl<'a> TryFrom<&'a SymbolVariant> for CallableSymbolChild<'a> {
    type Error = ();

    fn try_from(value: &'a SymbolVariant) -> Result<Self, Self::Error> {
        match value {
            SymbolVariant::FuncParam(s) => Ok(CallableSymbolChild::Param(s)),
            SymbolVariant::LocalVar(s) => Ok(CallableSymbolChild::LocalVar(s)),
            _ => Err(())
        }
    }
}

impl<'a> ChildrenSymbolsFilter<'a> for GlobalFunctionSymbol {
    type ChildRef = CallableSymbolChild<'a>;
}

impl<'a> ChildrenSymbolsFilter<'a> for MemberFunctionSymbol {
    type ChildRef = CallableSymbolChild<'a>;
}

impl<'a> ChildrenSymbolsFilter<'a> for EventSymbol {
    type ChildRef = CallableSymbolChild<'a>;
}


impl<'a> TryFrom<&'a SymbolVariant> for &'a FunctionParameterSymbol {
    type Error = ();

    fn try_from(value: &'a SymbolVariant) -> Result<Self, Self::Error> {
        value.try_as_func_param_ref().ok_or(())
    }
}

impl<'a> ChildrenSymbolsFilter<'a> for ConstructorSymbol {
    type ChildRef = &'a FunctionParameterSymbol;
}


impl<'a> TryFrom<&'a SymbolVariant> for &'a ArrayTypeFunctionSymbol {
    type Error = ();

    fn try_from(value: &'a SymbolVariant) -> Result<Self, Self::Error> {
        value.try_as_array_func_ref().ok_or(())
    }
}

impl<'a> ChildrenSymbolsFilter<'a> for ArrayTypeSymbol {
    type ChildRef = &'a ArrayTypeFunctionSymbol;
}

impl<'a> TryFrom<&'a SymbolVariant> for &'a ArrayTypeFunctionParameterSymbol {
    type Error = ();

    fn try_from(value: &'a SymbolVariant) -> Result<Self, Self::Error> {
        value.try_as_array_func_param_ref().ok_or(())
    }
}

impl<'a> ChildrenSymbolsFilter<'a> for ArrayTypeFunctionSymbol {
    type ChildRef = &'a ArrayTypeFunctionParameterSymbol;
}

impl<'a> ChildrenSymbolsFilter<'a> for MemberFunctionInjectorSymbol {
    type ChildRef = CallableSymbolChild<'a>;
}

impl<'a> ChildrenSymbolsFilter<'a> for MemberFunctionReplacerSymbol {
    type ChildRef = CallableSymbolChild<'a>;
}

impl<'a> ChildrenSymbolsFilter<'a> for GlobalFunctionReplacerSymbol {
    type ChildRef = CallableSymbolChild<'a>;
}

impl<'a> ChildrenSymbolsFilter<'a> for MemberFunctionWrapperSymbol {
    type ChildRef = CallableSymbolChild<'a>;
}



/// Iterate over primary symbols associated with a script file at a given path
pub struct FilePrimarySymbols<'st> {
    symtab: &'st SymbolTable,
    primary_paths: Vec<SymbolPathBuf>,
    primary_path_idx: usize
}

impl<'st> FilePrimarySymbols<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, local_source_path: &Path) -> Self {
        let primary_paths = 
            symtab.source_path_assocs
            .get(local_source_path)
            .cloned()
            .unwrap_or_default();

        Self {
            symtab,
            primary_paths,
            primary_path_idx: 0
        }
    }
}

impl<'st> Iterator for FilePrimarySymbols<'st> {
    type Item = &'st SymbolVariant;

    fn next(&mut self) -> Option<Self::Item> {
        let sympath = self.primary_paths.get(self.primary_path_idx)?;
        let item = self.symtab.get_symbol(sympath);
        self.primary_path_idx += 1;
        item
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


/// Iterator of all symbols descending from a given parent symbol.
/// If you want an iterator going over only direct children use [`SymbolChildren`].
#[derive(Clone)]
pub struct SymbolDescendants<'st> {
    iter: btree_map::Range<'st, SymbolPathBuf, SymbolVariant>,
    parent_sympath: SymbolPathBuf
}

impl<'st> SymbolDescendants<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, sympath: &SymbolPath) -> Self {
        let mut iter = symtab.symbols.range(sympath.to_owned()..);
        // prime the iterator to go to the first descendant
        // it is assumed this parent exists
        iter.next();

        Self {
            iter,
            parent_sympath: sympath.to_owned()
        }
    }
}

impl<'st> Iterator for SymbolDescendants<'st> {
    type Item = &'st SymbolVariant;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|(sympath, _)| sympath.starts_with(&self.parent_sympath))
            .map(|(_, variant)| variant)
    }
}