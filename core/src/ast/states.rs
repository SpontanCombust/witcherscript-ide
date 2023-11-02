use bitmask_enum::bitmask;

use super::{identifier::Identifier, vars::TypeAnnotation, classes::ClassBody};

pub struct StateDeclaration {
    pub imported: bool,
    pub specifiers: StateSpecifiers,
    pub name: Identifier,
    pub parent_class: TypeAnnotation,
    pub extended_state: Option<TypeAnnotation>,
    pub body: ClassBody,
}

#[bitmask]
pub enum StateSpecifiers {
    Abstract
}