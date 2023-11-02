use bitmask_enum::bitmask;

use super::{identifier::Identifier, vars::TypeAnnotation, classes::ClassBody};


#[derive(Debug, Clone, PartialEq)]
pub struct StateDeclaration {
    pub imported: bool,
    pub specifiers: StateSpecifiers,
    pub name: Identifier,
    pub parent_class: TypeAnnotation,
    pub extended_state: Option<TypeAnnotation>,
    pub body: ClassBody,
}

#[bitmask(u8)]
pub enum StateSpecifiers {
    Abstract
}