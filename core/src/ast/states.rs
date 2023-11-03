use bitmask_enum::bitmask;

use super::{identifier::Identifier, classes::ClassBody};


#[derive(Debug, Clone, PartialEq)]
pub struct StateDeclaration {
    pub imported: bool,
    pub specifiers: StateSpecifiers,
    pub name: Identifier,
    pub parent_class: Identifier,
    pub base_state: Option<Identifier>,
    pub body: ClassBody,
}

#[bitmask(u8)]
pub enum StateSpecifiers {
    Abstract
}