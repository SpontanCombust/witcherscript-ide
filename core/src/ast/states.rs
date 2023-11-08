use bitmask_enum::bitmask;

use crate::lexing::{Identifier, Spanned};
use super::classes::ClassBody;


#[derive(Debug, Clone, PartialEq)]
pub struct StateDeclaration {
    pub imported: bool,
    pub specifiers: StateSpecifiers,
    pub name: Spanned<Identifier>,
    pub parent_class: Spanned<Identifier>,
    pub base_state: Option<Spanned<Identifier>>,
    pub body: Spanned<ClassBody>,
}

#[bitmask(u8)]
pub enum StateSpecifiers {
    Abstract
}