use std::ops::{DerefMut, Deref};

use crate::{NamedSyntaxNode, SyntaxNode};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(String);

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
