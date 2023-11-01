use super::{identifier::Identifier, vars::TypeAnnotation, classes::ClassBody};

pub struct StateDeclaration {
    pub imported: bool,
    pub specifiers: StateSpecifiers,
    pub name: Identifier,
    pub parent_class: TypeAnnotation,
    pub extended_state: Option<TypeAnnotation>,
    pub body: ClassBody,
}

pub enum StateSpecifiers {
    Abstract
}