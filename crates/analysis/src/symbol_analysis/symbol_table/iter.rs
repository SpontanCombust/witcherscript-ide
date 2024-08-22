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
            iter: SymbolChildren::new(symtab, symbol.path_ref()),
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
    VarInjector(&'st MemberVarInjectorSymbol),
    Method(&'st MemberFunctionSymbol),
    Event(&'st EventSymbol),
    MethodInjector(&'st MemberFunctionInjectorSymbol),
    ThisVar(&'st ThisVarSymbol),
    SuperVar(&'st SuperVarSymbol)
}

impl<'a> TryFrom<&'a SymbolVariant> for ClassSymbolChild<'a> {
    type Error = ();

    fn try_from(value: &'a SymbolVariant) -> Result<Self, Self::Error> {
        match value {
            SymbolVariant::MemberVar(s) => Ok(ClassSymbolChild::Var(s)),
            SymbolVariant::Autobind(s) => Ok(ClassSymbolChild::Autobind(s)),
            SymbolVariant::MemberVarInjector(s) => Ok(ClassSymbolChild::VarInjector(s)),
            SymbolVariant::MemberFunc(s) => Ok(ClassSymbolChild::Method(s)),
            SymbolVariant::Event(s) => Ok(ClassSymbolChild::Event(s)),
            SymbolVariant::MemberFuncInjector(s) => Ok(ClassSymbolChild::MethodInjector(s)),
            SymbolVariant::ThisVar(s) => Ok(ClassSymbolChild::ThisVar(s)),
            SymbolVariant::SuperVar(s) => Ok(ClassSymbolChild::SuperVar(s)),
            _ => Err(())
        }
    }
}

impl<'a> ChildrenSymbolsFilter<'a> for ClassSymbol {
    type ChildRef = ClassSymbolChild<'a>;
}


pub enum StateSymbolChild<'st> {
    Var(&'st MemberVarSymbol),
    Autobind(&'st AutobindSymbol),
    VarInjector(&'st MemberVarInjectorSymbol),
    Method(&'st MemberFunctionSymbol),
    Event(&'st EventSymbol),
    MethodInjector(&'st MemberFunctionInjectorSymbol),
    ThisVar(&'st ThisVarSymbol),
    SuperVar(&'st SuperVarSymbol),
    ParentVar(&'st ParentVarSymbol),
    VirtualParentVar(&'st VirtualParentVarSymbol)
}

impl<'a> TryFrom<&'a SymbolVariant> for StateSymbolChild<'a> {
    type Error = ();

    fn try_from(value: &'a SymbolVariant) -> Result<Self, Self::Error> {
        match value {
            SymbolVariant::MemberVar(s) => Ok(StateSymbolChild::Var(s)),
            SymbolVariant::Autobind(s) => Ok(StateSymbolChild::Autobind(s)),
            SymbolVariant::MemberVarInjector(s) => Ok(StateSymbolChild::VarInjector(s)),
            SymbolVariant::MemberFunc(s) => Ok(StateSymbolChild::Method(s)),
            SymbolVariant::Event(s) => Ok(StateSymbolChild::Event(s)),
            SymbolVariant::MemberFuncInjector(s) => Ok(StateSymbolChild::MethodInjector(s)),
            SymbolVariant::ThisVar(s) => Ok(StateSymbolChild::ThisVar(s)),
            SymbolVariant::SuperVar(s) => Ok(StateSymbolChild::SuperVar(s)),
            SymbolVariant::ParentVar(s) => Ok(StateSymbolChild::ParentVar(s)),
            SymbolVariant::VirtualParentVar(s) => Ok(StateSymbolChild::VirtualParentVar(s)),
            _ => Err(())
        }
    }
}

impl<'a> ChildrenSymbolsFilter<'a> for StateSymbol {
    type ChildRef = StateSymbolChild<'a>;
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


pub enum FunctionWrapperSymbolChild<'a> {
    Param(&'a FunctionParameterSymbol),
    LocalVar(&'a LocalVarSymbol),
    WrappedMethod(&'a WrappedMethodSymbol)
}

impl<'a> TryFrom<&'a SymbolVariant> for FunctionWrapperSymbolChild<'a> {
    type Error = ();

    fn try_from(value: &'a SymbolVariant) -> Result<Self, Self::Error> {
        match value {
            SymbolVariant::WrappedMethod(s) => Ok(FunctionWrapperSymbolChild::WrappedMethod(s)),
            SymbolVariant::FuncParam(s) => Ok(FunctionWrapperSymbolChild::Param(s)),
            SymbolVariant::LocalVar(s) => Ok(FunctionWrapperSymbolChild::LocalVar(s)),
            _ => Err(())
        }
    }
}

impl<'a> ChildrenSymbolsFilter<'a> for MemberFunctionWrapperSymbol {
    type ChildRef = FunctionWrapperSymbolChild<'a>;
}



/// Iterate over primary symbols associated with a script file at a given path
pub struct FilePrimarySymbols<'st> {
    iter: Box<dyn Iterator<Item = &'st SymbolVariant> + Send + 'st>
}

impl<'st> FilePrimarySymbols<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, local_source_path: &Path) -> Self {
        let roots = symtab.source_path_assocs
            .get(local_source_path)
            .map(|v| v.as_slice())
            .unwrap_or_default();

        let iter = roots.iter()
            .filter_map(|root| symtab.symbols.get(root));

        Self {
            iter: Box::new(iter)
        }
    }
}

impl<'st> Iterator for FilePrimarySymbols<'st> {
    type Item = &'st SymbolVariant;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}


/// Iterate over symbols associated with a script file at a given path
pub struct FileSymbols<'st> {
    iter: Box<dyn Iterator<Item = &'st SymbolVariant> + Send + 'st>
}

impl<'st> FileSymbols<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, local_source_path: &Path) -> Self {
        let iter = 
            symtab.get_primary_symbols_for_source(local_source_path)
            .map(|prim_sym| {
                std::iter::once(prim_sym)
                .chain(symtab.get_symbol_descendants(prim_sym.path_ref(), true))
            })
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
pub struct SymbolDescendants<'st> {
    iter: Box<dyn Iterator<Item = &'st SymbolVariant> + Send + 'st>,
    skip_primary_children: bool
}

impl<'st> SymbolDescendants<'st> {
    pub(super) fn new(symtab: &'st SymbolTable, sympath: &SymbolPath, skip_primary_children: bool) -> Self {
        let parent_sympath = sympath.to_owned();

        let mut iter = symtab.symbols
            .range(sympath.to_owned()..)
            .take_while(move |(sympath, _)| sympath.starts_with(&parent_sympath))
            .map(|(_, symvar)| symvar);

        // prime the iterator to go to the first descendant
        // it is assumed this parent exists
        iter.next();

        Self {
            iter: Box::new(iter),
            skip_primary_children
        }
    }
}

impl<'st> Iterator for SymbolDescendants<'st> {
    type Item = &'st SymbolVariant;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut item) = self.iter.next() {
            while self.skip_primary_children && item.is_primary() {
                let prim_item_sympath = item.path_ref().to_owned();

                if let Some(skipped) = self.iter.find(move |v| !v.path_ref().starts_with(&prim_item_sympath)) {
                    item = skipped;
                } else {
                    return None;
                }
            }

            Some(item)
        } else {
            None
        }
    }
}