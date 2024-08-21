use witcherscript_project::SourceMask;
use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use super::*;


/// A type that can perform data fetching operations on many symbol tables
/// until that data is found.
/// Values are fetched from symbol tables in the order that they were submitted to the marcher.
/// 
/// Uses [`SourceMask`] to properly mask out script files that were already present in previously visited content's symbol table.
/// So when a marcher is composed of two tables: A and B, when table A contains script file "game/r4Game.ws" 
/// any data coming from a file at the same local path is ignored when searching through table B.
#[derive(Clone)]
pub struct SymbolTableMarcher<'a> {
    inner: Vec<MaskedSymbolTable<'a>>,
    start_idx: usize
}

impl<'a> SymbolTableMarcher<'a> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            start_idx: 0
        }
    }

    pub fn add_step(&mut self, symtab: &'a SymbolTable, mask: SourceMask) {
        let accum_mask = 
            self.inner.last()
            .map(|masked| masked.accum_mask.union(&masked.assoc_mask))
            .unwrap_or_default();

        let masked = MaskedSymbolTable {
            symtab,
            accum_mask,
            assoc_mask: mask
        };

        self.inner.push(masked);
    }

    /// Can be used to march only over dependencies while keeping proper source masking
    pub fn skip_first_step(mut self, skip: bool) -> Self {
        if skip {
            self.start_idx = 1;
        } else {
            self.start_idx = 0;
        }

        self
    }


    #[inline]
    pub fn test_contains_symbol(&self, path: &SymbolPath) -> Result<(), PathOccupiedError> {
        for i in self.start_idx..self.inner.len() {
            let masked = &self.inner[i];
            masked.test_contains_symbol(path)?;
        }

        Ok(())
    }

    #[inline]
    pub fn contains_symbol(&self, path: &SymbolPath) -> bool {
        self.march(|masked| if masked.contains_symbol(path) { Some(()) } else { None }).is_some()
    }

    #[inline]
    pub fn find_table_with_symbol_path(&self, path: &SymbolPath) -> Option<&'a SymbolTable> {
        self.march(|masked| if masked.contains_symbol(path) { Some(masked.symtab) } else { None })   
    }
    
    /// As opposed to [`SymbolTableMarcher::find_table_with_symbol_path`] it looks for the exact table
    /// that contains the given symbol. It does this by comparing the `scripts_root` in its location.
    /// Because of that for the exact symbol to be found it needs to be a [`LocatableSymbol`].
    /// This is in most part only useful when dealing with annotated symbols.
    /// 
    /// [`LocatableSymbol`]: crate::symbol_analysis::symbols::LocatableSymbol
    #[inline]
    pub fn find_table_with_symbol(&self, symvar: &SymbolVariant) -> Option<&'a SymbolTable> {
        self.march(|masked| {
            if masked.get_symbol(symvar.path_ref()).filter(|v| v.location() == symvar.location()).is_some() { 
                Some(masked.symtab) 
            } else { 
                None 
            }
        })   
    }

    #[inline]
    pub fn get_symbol(&self, path: &SymbolPath) -> Option<&'a SymbolVariant> {
        self.march(|masked| masked.get_symbol(path))
    }

    #[inline]
    pub fn get_symbol_with_table(&self, path: &SymbolPath) -> Option<(&'a SymbolTable, &'a SymbolVariant)> {
        self.march(|masked| {
            if let Some(symvar) = masked.get_symbol(path) {
                Some((masked.symtab, symvar))
            } else {
                None
            }
        })
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

    /// Iterate over callable symbols in the annotation chain accross the marcher.
    #[inline]
    pub fn annotation_chain_for_member_callable(&self, member_callable_sympath: &MemberCallableSymbolPath) -> AnnotationChain<'a> {
        AnnotationChain::for_member_callable(self.clone(), member_callable_sympath)
    }

    /// Iterate over callable symbols in the annotation chain accross the marcher.
    #[inline]
    pub fn annotation_chain_for_global_callable(&self, global_callable_sympath: &GlobalCallableSymbolPath) -> AnnotationChain<'a> {
        AnnotationChain::for_global_callable(self.clone(), global_callable_sympath)
    }


    fn march<T, F>(&self, mut f: F) -> Option<T> 
    where F: FnMut(&MaskedSymbolTable<'a>) -> Option<T> {
        for i in self.start_idx..self.inner.len() {
            let masked = &self.inner[i];
            if let Some(val) = f(masked) {
                return Some(val);
            }
        }

        None
    }

    fn into_iter(self) -> impl Iterator<Item = MaskedSymbolTable<'a>> {
        self.inner.into_iter().skip(self.start_idx)
    }
}


#[derive(Clone)]
struct MaskedSymbolTable<'a> {
    symtab: &'a SymbolTable,
    accum_mask: SourceMask,
    assoc_mask: SourceMask
}

impl<'a> MaskedSymbolTable<'a> {
    fn into_iter(self) -> impl Iterator<Item = (&'a SymbolPath, &'a SymbolVariant)> {
        self.symtab.iter().filter(move |(_, v)| mask_symbol(v, &self.accum_mask).is_some())
    }

    fn get_symbol(&self, path: &SymbolPath) -> Option<&'a SymbolVariant> {
        self.symtab.get_symbol(path).and_then(|symvar| mask_symbol(symvar, &self.accum_mask))
    }


    fn contains_symbol(&self, path: &SymbolPath) -> bool {
        self.get_symbol(path).is_some()
    }

    fn test_contains_symbol(&self, path: &SymbolPath) -> Result<(), PathOccupiedError> {
        if let Some(occupying) = self.get_symbol(path) {
            Err(PathOccupiedError {
                occupied_path: occupying.path_ref().to_sympath_buf(),
                occupied_location: occupying.location().cloned(),
                occupied_typ: occupying.typ()
            })
        } else {
            Ok(())
        }
    }
}

#[inline]
fn mask_symbol<'a>(symvar: &'a SymbolVariant, mask: &SourceMask) -> Option<&'a SymbolVariant> {
    if let Some(loc) = symvar.location() {
        if mask.test(&loc.local_source_path) {
            Some(symvar)
        } else {
            None
        }
    } else {
        Some(symvar)
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
        let it = marcher.into_iter().map(move |symtab| {
            let class_path = class_path.to_owned();
            symtab.into_iter()
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
            if let Some(base_state_name) = &current_state_sym.base_state_name {
                'classes: for class in self.marcher.class_hierarchy(current_state_sym.parent_class_path()) {
                    for state in self.marcher.class_states(class.path()) {
                        if state.state_name() == base_state_name {
                            state.path_ref().clone_into(&mut self.current_state_path);
                            break 'classes;
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



pub struct AnnotationChain<'a> {
    symtabs: Vec<MaskedSymbolTable<'a>>,
    symtab_idx: usize,

    regular_sympath: SymbolPathBuf,
    replaced_sympath: SymbolPathBuf,
    wrapped_sympath: Option<SymbolPathBuf>,

    regular_visited: bool,
    replaced_visited: bool,
    wrapped_visited: bool
}

impl<'a> AnnotationChain<'a> {
    fn for_member_callable(marcher: SymbolTableMarcher<'a>, path: &MemberCallableSymbolPath) -> Self {
        Self {
            symtabs: marcher.inner,
            symtab_idx: 0,

            regular_sympath: path.to_owned().into(),
            replaced_sympath: MemberCallableReplacerSymbolPath::from(path.to_owned()).into(),
            wrapped_sympath: Some(MemberCallableWrapperSymbolPath::from(path.to_owned()).into()),

            regular_visited: false,
            replaced_visited: false,
            wrapped_visited: false
        }
    }

    fn for_global_callable(marcher: SymbolTableMarcher<'a>, path: &GlobalCallableSymbolPath) -> Self {
        Self {
            symtabs: marcher.inner,
            symtab_idx: 0,

            regular_sympath: path.to_owned().into(),
            replaced_sympath: GlobalCallableReplacerSymbolPath::from(path.to_owned()).into(),
            wrapped_sympath: None,

            regular_visited: false,
            replaced_visited: false,
            wrapped_visited: false
        }
    }
}

impl<'a> Iterator for AnnotationChain<'a> {
    type Item = &'a SymbolVariant;

    fn next(&mut self) -> Option<Self::Item> {
        while self.symtab_idx < self.symtabs.len() {
            if !self.wrapped_visited {
                self.wrapped_visited = true;
                if let Some(wrapped_sympath) = &self.wrapped_sympath {
                    let symvar = self.symtabs[self.symtab_idx].get_symbol(wrapped_sympath);
        
                    if symvar.is_some() {
                        return symvar;
                    }
                }
            }

            if !self.replaced_visited {
                self.replaced_visited = true;
                let symvar = self.symtabs[self.symtab_idx].get_symbol(&self.replaced_sympath);

                if symvar.is_some() {
                    return symvar;
                }
            }

            if !self.regular_visited {
                self.regular_visited = true;
                let symvar = self.symtabs[self.symtab_idx].get_symbol(&self.regular_sympath);

                if symvar.is_some() {
                    return symvar;
                }
            }

            self.wrapped_visited = false;
            self.replaced_visited = false;
            self.regular_visited = false;

            self.symtab_idx += 1;
        }

        None
    }
}