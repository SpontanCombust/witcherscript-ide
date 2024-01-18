use std::fmt::Debug;
use crate::{SyntaxNode, NamedSyntaxNode, AnyNode};
use super::StatementTraversal;


// Empty type essentially representing an orphaned/trailing semicolon
#[derive(Debug, Clone)]
pub struct Nop;

pub type NopNode<'script> = SyntaxNode<'script, Nop>;

impl NamedSyntaxNode for NopNode<'_> {
    const NODE_KIND: &'static str = "nop";
}

impl NopNode<'_> {}

impl Debug for NopNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nop")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for NopNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for NopNode<'_> {
    fn accept<V: super::StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_nop_stmt(self);
    }
}