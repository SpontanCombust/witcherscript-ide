use std::marker::PhantomData;
use smallvec::SmallVec;
use witcherscript::attribs::*;


/// Cheap to store and clone type that can contain symbol specifier information
#[derive(Clone, PartialEq, Eq)]
pub struct SymbolSpecifiers<S: SymbolSpecifier> {
    vec: SmallVec<S::BackingArray>,
    phantom: PhantomData<S>
}

impl<S: SymbolSpecifier> SymbolSpecifiers<S> {
    /// Returns an empty container
    #[inline]
    pub fn new() -> Self {
        Self {
            vec: SmallVec::new(),
            phantom: PhantomData
        }
    }

    /// Returns whether the value was newly inserted
    #[inline]
    pub fn insert(&mut self, spec: S) -> bool {
        if self.vec.contains(&spec) {
            false
        } else {
            self.vec.push(spec);
            true
        }
    }

    #[inline]
    pub fn contains(&self, spec: S) -> bool {
        self.vec.contains(&spec)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = S> + '_ {
        self.vec.iter().map(|s| *s)
    }
}

impl<S> std::fmt::Debug for SymbolSpecifiers<S> 
where S: SymbolSpecifier + std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vec.fmt(f)
    }
}


pub trait SymbolSpecifier: std::cmp::PartialEq + Copy {
    type BackingArray: smallvec::Array<Item = Self>;
}

impl SymbolSpecifier for ClassSpecifier {
    type BackingArray = [Self; 3];
}

impl SymbolSpecifier for AutobindSpecifier {
    type BackingArray = [Self; 2];
}

impl SymbolSpecifier for FunctionParameterSpecifier {
    type BackingArray = [Self; 2];
}

impl SymbolSpecifier for GlobalFunctionSpecifier {
    type BackingArray = [Self; 2];
}

impl SymbolSpecifier for MemberFunctionSpecifier {
    type BackingArray = [Self; 4];
}

impl SymbolSpecifier for StateSpecifier {
    type BackingArray = [Self; 2];
}

impl SymbolSpecifier for StructSpecifier {
    type BackingArray = [Self; 1];
}

impl SymbolSpecifier for MemberVarSpecifier {
    type BackingArray = [Self; 6];
}
