use std::fmt::Debug;
use crate::tokens::Keyword;
use super::Specifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessModifier {
    Private,
    Protected,
    Public
}

impl TryFrom<Specifier> for AccessModifier {
    type Error = ();

    fn try_from(value: Specifier) -> Result<Self, Self::Error> {
        match value {
            Specifier::Private => Ok(Self::Private),
            Specifier::Protected => Ok(Self::Protected),
            Specifier::Public => Ok(Self::Public),
            _ => Err(())
        }
    }
}

impl From<AccessModifier> for Keyword {
    fn from(value: AccessModifier) -> Self {
        match value {
            AccessModifier::Private => Keyword::Private,
            AccessModifier::Protected => Keyword::Protected,
            AccessModifier::Public => Keyword::Public,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassSpecifier {
    Import,
    Abstract,
    Statemachine
}

impl TryFrom<Specifier> for ClassSpecifier {
    type Error = ();

    fn try_from(value: Specifier) -> Result<Self, Self::Error> {
        match value {
            Specifier::Abstract => Ok(Self::Abstract),
            Specifier::Import => Ok(Self::Import),
            Specifier::Statemachine => Ok(Self::Statemachine),
            _ => Err(())
        }
    }
}

impl From<ClassSpecifier> for Keyword {
    fn from(value: ClassSpecifier) -> Self {
        match value {
            ClassSpecifier::Import => Keyword::Import,
            ClassSpecifier::Abstract => Keyword::Abstract,
            ClassSpecifier::Statemachine => Keyword::Statemachine,
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutobindSpecifier {
    AccessModifier(AccessModifier),
    Optional
}

impl TryFrom<Specifier> for AutobindSpecifier {
    type Error = ();

    fn try_from(value: Specifier) -> Result<Self, Self::Error> {
        match value {
            Specifier::Optional => Ok(Self::Optional),
            Specifier::Private => Ok(Self::AccessModifier(AccessModifier::Private)),
            Specifier::Protected => Ok(Self::AccessModifier(AccessModifier::Protected)),
            Specifier::Public => Ok(Self::AccessModifier(AccessModifier::Public)),
            _ => Err(())
        }
    }
}

impl From<AutobindSpecifier> for Keyword {
    fn from(value: AutobindSpecifier) -> Self {
        match value {
            AutobindSpecifier::AccessModifier(am) => am.into(),
            AutobindSpecifier::Optional => Keyword::Optional,
        }
    }
}

impl From<AccessModifier> for AutobindSpecifier {
    fn from(value: AccessModifier) -> Self {
        Self::AccessModifier(value)
    }
}
