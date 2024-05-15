use std::marker::PhantomData;
use witcherscript::attribs::*;


/// Cheap to store and copy bitmask type that can contain symbol specifier information
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SpecifierBitmask<S: IntoSpecifierBitmask> {
    bits: u8,
    phantom: PhantomData<S>
}

impl<S: IntoSpecifierBitmask> SpecifierBitmask<S> {
    /// Returns an empty bitmask
    #[inline]
    pub fn new() -> Self {
        Self {
            bits: 0u8,
            phantom: PhantomData
        }
    }

    /// Returns whether the value was newly inserted
    #[inline]
    pub fn insert(&mut self, spec: S) -> bool {
        let b = spec.into_bitmask();
        let had = (self.bits & b) == b;
        self.bits |= b;
        !had
    }

    #[inline]
    pub fn contains(&self, spec: S) -> bool {
        let b = spec.into_bitmask();
        (self.bits & b) == b
    }
}

pub trait IntoSpecifierBitmask {
    fn into_bitmask(self) -> u8;
}

macro_rules! debug_specifier_bitmask {
    ($spec_ty:ty, $($specs:expr),+) => {
        impl std::fmt::Debug for SpecifierBitmask<$spec_ty> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut v = Vec::with_capacity(8);
                $(if self.contains($specs) { v.push($specs); })+
                v.fmt(f)
            }
        }
    }
}


impl IntoSpecifierBitmask for ClassSpecifier {
    #[inline]
    fn into_bitmask(self) -> u8 {
        match self {
            ClassSpecifier::Import       => 1 << 0,
            ClassSpecifier::Abstract     => 1 << 1,
            ClassSpecifier::Statemachine => 1 << 2,
        }
    }
}

debug_specifier_bitmask!(ClassSpecifier,
    ClassSpecifier::Import,
    ClassSpecifier::Abstract,
    ClassSpecifier::Statemachine
);


impl IntoSpecifierBitmask for AutobindSpecifier {
    #[inline]
    fn into_bitmask(self) -> u8 {
        match self {
            AutobindSpecifier::AccessModifier(AccessModifier::Private)   => 1 << 0,
            AutobindSpecifier::AccessModifier(AccessModifier::Protected) => 1 << 1,
            AutobindSpecifier::AccessModifier(AccessModifier::Public)    => 1 << 2,
            AutobindSpecifier::Optional                                  => 1 << 3,
        }
    }
}

debug_specifier_bitmask!(AutobindSpecifier,
    AutobindSpecifier::AccessModifier(AccessModifier::Private),
    AutobindSpecifier::AccessModifier(AccessModifier::Protected), 
    AutobindSpecifier::AccessModifier(AccessModifier::Public),
    AutobindSpecifier::Optional    
);


impl IntoSpecifierBitmask for FunctionParameterSpecifier {
    #[inline]
    fn into_bitmask(self) -> u8 {
        match self {
            FunctionParameterSpecifier::Optional => 1 << 0,
            FunctionParameterSpecifier::Out      => 1 << 1,
        }
    }
}

debug_specifier_bitmask!(FunctionParameterSpecifier,
    FunctionParameterSpecifier::Optional,
    FunctionParameterSpecifier::Out
);


impl IntoSpecifierBitmask for GlobalFunctionSpecifier {
    #[inline]
    fn into_bitmask(self) -> u8 {
        match self {
            GlobalFunctionSpecifier::Import => 1 << 0,
            GlobalFunctionSpecifier::Latent => 1 << 1,
        }
    }
}

debug_specifier_bitmask!(GlobalFunctionSpecifier,
    GlobalFunctionSpecifier::Import,
    GlobalFunctionSpecifier::Latent
);


impl IntoSpecifierBitmask for MemberFunctionSpecifier {
    #[inline]
    fn into_bitmask(self) -> u8 {
        match self {
            MemberFunctionSpecifier::AccessModifier(AccessModifier::Private)   => 1 << 0,
            MemberFunctionSpecifier::AccessModifier(AccessModifier::Protected) => 1 << 1,
            MemberFunctionSpecifier::AccessModifier(AccessModifier::Public)    => 1 << 2,
            MemberFunctionSpecifier::Import                                    => 1 << 3,
            MemberFunctionSpecifier::Final                                     => 1 << 4,
            MemberFunctionSpecifier::Latent                                    => 1 << 5,
        }
    }
}

debug_specifier_bitmask!(MemberFunctionSpecifier,
    MemberFunctionSpecifier::AccessModifier(AccessModifier::Private),
    MemberFunctionSpecifier::AccessModifier(AccessModifier::Protected),
    MemberFunctionSpecifier::AccessModifier(AccessModifier::Public),
    MemberFunctionSpecifier::Import,  
    MemberFunctionSpecifier::Final,
    MemberFunctionSpecifier::Latent
);


impl IntoSpecifierBitmask for StateSpecifier {
    #[inline]
    fn into_bitmask(self) -> u8 {
        match self {
            StateSpecifier::Import   => 1 << 0,
            StateSpecifier::Abstract => 1 << 1,
        }
    }
}

debug_specifier_bitmask!(StateSpecifier,
    StateSpecifier::Import,
    StateSpecifier::Abstract
);


impl IntoSpecifierBitmask for StructSpecifier {
    #[inline]
    fn into_bitmask(self) -> u8 {
        match self {
            StructSpecifier::Import => 1 << 0,
        }
    }
}

debug_specifier_bitmask!(StructSpecifier,
    StructSpecifier::Import
);


impl IntoSpecifierBitmask for MemberVarSpecifier {
    #[inline]
    fn into_bitmask(self) -> u8 {
        match self {
            MemberVarSpecifier::AccessModifier(AccessModifier::Private)   => 1 << 0,
            MemberVarSpecifier::AccessModifier(AccessModifier::Protected) => 1 << 1,
            MemberVarSpecifier::AccessModifier(AccessModifier::Public)    => 1 << 2,
            MemberVarSpecifier::Const                                     => 1 << 3,
            MemberVarSpecifier::Editable                                  => 1 << 4,
            MemberVarSpecifier::Import                                    => 1 << 5,
            MemberVarSpecifier::Inlined                                   => 1 << 6,
            MemberVarSpecifier::Saved                                     => 1 << 7,
        }
    }
}

debug_specifier_bitmask!(MemberVarSpecifier,
    MemberVarSpecifier::AccessModifier(AccessModifier::Private),
    MemberVarSpecifier::AccessModifier(AccessModifier::Protected),
    MemberVarSpecifier::AccessModifier(AccessModifier::Public),
    MemberVarSpecifier::Const,
    MemberVarSpecifier::Editable,
    MemberVarSpecifier::Import,
    MemberVarSpecifier::Inlined,
    MemberVarSpecifier::Saved
);