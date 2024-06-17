use std::fmt::Debug;
use crate::{AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    // Empty type essentially representing an orphaned/trailing semicolon
    pub struct Nop;
}


pub type NopNode<'script> = SyntaxNode<'script, tags::Nop>;

impl NamedSyntaxNode for NopNode<'_> {
    const NODE_KIND: &'static str = "nop";
}

impl NopNode<'_> {}

impl Debug for NopNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nop {}", self.range().debug())
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

impl SyntaxNodeTraversal for NopNode<'_> {
    fn accept<V: super::SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        visitor.visit_nop_stmt(self, ctx);
    }
}