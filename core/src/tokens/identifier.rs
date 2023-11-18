use shrinkwraprs::Shrinkwrap;

use crate::{NamedSyntaxNode, SyntaxNode};


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(String);

impl NamedSyntaxNode for Identifier {
    const NODE_NAME: &'static str = "identifier";
}

impl SyntaxNode<'_, Identifier> {
    // use text() to get identifier name
}