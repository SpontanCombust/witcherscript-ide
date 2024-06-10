use std::fmt::Debug;
use crate::tokens::Keyword;
use super::Specifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructSpecifier {
    Import
}

impl TryFrom<Specifier> for StructSpecifier {
    type Error = ();

    fn try_from(value: Specifier) -> Result<Self, Self::Error> {
        match value {
            Specifier::Import => Ok(Self::Import),
            _ => Err(())
        }
    }
}

impl From<StructSpecifier> for Keyword {
    fn from(value: StructSpecifier) -> Self {
        match value {
            StructSpecifier::Import => Keyword::Import,
        }
    }
}
