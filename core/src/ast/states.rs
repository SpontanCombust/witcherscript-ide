use crate::tokens::{Identifier, Spanned};
use super::classes::ClassBody;


#[derive(Debug, Clone, PartialEq)]
pub struct StateDeclaration {
    pub imported: bool,
    pub specifiers: Spanned<Vec<Spanned<StateSpecifier>>>,
    pub name: Spanned<Identifier>,
    pub parent_class: Spanned<Identifier>,
    pub base_state: Option<Spanned<Identifier>>,
    pub body: Spanned<ClassBody>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpecifier {
    Abstract
}