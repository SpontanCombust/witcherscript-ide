use std::fmt::Debug;
use crate::tokens::Keyword;
use super::{AccessModifier, Specifier};

//TODO split to ClassFieldSpecifier and StructFieldSpecifier (AccessModifier is not allowed in structs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberVarSpecifier {
    AccessModifier(AccessModifier),
    Const,
    Editable,
    Import,
    Inlined,
    Saved,
}

impl TryFrom<Specifier> for MemberVarSpecifier {
    type Error = ();

    fn try_from(value: Specifier) -> Result<Self, Self::Error> {
        match value {
            Specifier::Const => Ok(Self::Const),
            Specifier::Editable => Ok(Self::Editable),
            Specifier::Import => Ok(Self::Import),
            Specifier::Inlined => Ok(Self::Inlined),
            Specifier::Private => Ok(Self::AccessModifier(AccessModifier::Private)),
            Specifier::Protected => Ok(Self::AccessModifier(AccessModifier::Protected)),
            Specifier::Public => Ok(Self::AccessModifier(AccessModifier::Public)),
            Specifier::Saved => Ok(Self::Saved),
            _ => Err(())
        }
    }
}

impl From<MemberVarSpecifier> for Keyword {
    fn from(value: MemberVarSpecifier) -> Self {
        match value {
            MemberVarSpecifier::AccessModifier(am) => am.into(),
            MemberVarSpecifier::Const => Keyword::Const,
            MemberVarSpecifier::Editable => Keyword::Editable,
            MemberVarSpecifier::Import => Keyword::Import,
            MemberVarSpecifier::Inlined => Keyword::Inlined,
            MemberVarSpecifier::Saved => Keyword::Saved,
        }
    }
}

impl From<AccessModifier> for MemberVarSpecifier {
    fn from(value: AccessModifier) -> Self {
        Self::AccessModifier(value)
    }
}
