use crate::SyntaxNode;


pub struct Error;

pub type ErrorNode<'script> = SyntaxNode<'script, Error>;