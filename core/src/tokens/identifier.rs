use std::fmt::Debug;
use ropey::Rope;
use shrinkwraprs::Shrinkwrap;
use crate::{NamedSyntaxNode, SyntaxNode};


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(String);

impl NamedSyntaxNode for Identifier {
    const NODE_NAME: &'static str = "ident";
}

impl SyntaxNode<'_, Identifier> {
    pub fn value(&self, rope: &Rope) -> Identifier {
        Identifier(self.text(rope))
    }
}

impl Debug for SyntaxNode<'_, Identifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier {:?}", self.span())
    }
}