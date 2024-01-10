use crate::{SyntaxNode, NamedSyntaxNode, AnyNode};

// Empty type essentially representing an orphaned/trailing semicolon
#[derive(Debug, Clone)]
pub struct Nop;

pub type NopNode<'script> = SyntaxNode<'script, Nop>;

impl NamedSyntaxNode for NopNode<'_> {
    const NODE_KIND: &'static str = "nop";
}

impl NopNode<'_> {}

impl<'script> TryFrom<AnyNode<'script>> for NopNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}