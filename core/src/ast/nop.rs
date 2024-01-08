use crate::{SyntaxNode, NamedSyntaxNode};

// Empty type essentially representing an orphaned/trailing semicolon
#[derive(Debug, Clone)]
pub struct Nop;

pub type NopNode<'script> = SyntaxNode<'script, Nop>;

impl NamedSyntaxNode for NopNode<'_> {
    const NODE_KIND: &'static str = "nop";
}

impl NopNode<'_> {}