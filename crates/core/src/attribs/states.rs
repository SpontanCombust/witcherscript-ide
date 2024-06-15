use std::fmt::Debug;
use crate::tokens::Keyword;
use super::Specifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateSpecifier {
    Import,
    Abstract
}

impl TryFrom<Specifier> for StateSpecifier {
    type Error = ();

    fn try_from(value: Specifier) -> Result<Self, Self::Error> {
        match value {
            Specifier::Import => Ok(Self::Import),
            Specifier::Abstract => Ok(Self::Abstract),
            _ => Err(())
        }
    }
}

impl From<StateSpecifier> for Keyword {
    fn from(value: StateSpecifier) -> Self {
        match value {
            StateSpecifier::Import => Keyword::Import,
            StateSpecifier::Abstract => Keyword::Abstract,
        }
    }
}
