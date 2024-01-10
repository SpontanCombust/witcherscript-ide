use crate::SyntaxNode;


#[derive(Debug, Clone)]
pub struct Error;

pub type ErrorNode<'script> = SyntaxNode<'script, Error>;