use crate::{SyntaxNode, NamedSyntaxNode};

// Empty type essentially representing an orphaned/trailing semicolon
#[derive(Debug, Clone)]
pub struct Nop;

impl NamedSyntaxNode for Nop {
    const NODE_NAME: &'static str = "nop";
}

impl SyntaxNode<'_, Nop> {}