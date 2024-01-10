use crate::{SyntaxNode, AnyNode};


#[derive(Debug, Clone)]
pub struct Error;

pub type ErrorNode<'script> = SyntaxNode<'script, Error>;

impl<'script> TryFrom<AnyNode<'script>> for ErrorNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_error() {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}