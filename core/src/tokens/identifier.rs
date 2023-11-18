use shrinkwraprs::Shrinkwrap;

use crate::{NamedSyntaxNode, SyntaxNode};


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(String);

impl NamedSyntaxNode for Identifier {
    const NODE_NAME: &'static str = "identifier";
}

impl SyntaxNode<'_, Identifier> {
    pub fn value(&self) -> Identifier {
        Identifier(self.text())
    }
}