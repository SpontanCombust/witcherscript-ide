use ropey::Rope;
use shrinkwraprs::Shrinkwrap;

use crate::{NamedSyntaxNode, SyntaxNode};


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(String);

impl NamedSyntaxNode for Identifier {
    const NODE_NAME: &'static str = "identifier";
}

impl SyntaxNode<'_, Identifier> {
    pub fn value(&self, rope: &Rope) -> Identifier {
        Identifier(self.text(rope))
    }
}