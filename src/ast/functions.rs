use std::rc::Rc;
use super::{
    expression::*,
    type_annotation::*,
    classes::*
};
use bitmask_enum::bitmask;


pub struct FunctionDeclaration {
    pub access_modifier: Option<AccessModifier>,
    pub specifiers: FunctionSpecifiers,

    pub name: String,
    pub return_type: TypeAnnotation,
    pub body: Option<Vec<FunctionStatement>> // if there is no body it doesn't have a definition
}

#[bitmask(u32)]
pub enum FunctionSpecifiers {
    Entry,
    Event,
    Exec,
    Final,
    Import,
    Latent,
    Quest,
    Timer,
    Storyscene
}

pub struct FunctionParameter {
    pub name: String,
    pub is_optional: bool,
    pub is_output: bool,
    pub param_type: TypeAnnotation
}

pub enum FunctionStatement {
    Expr(Rc<Expression>)
    //TODO function statements
}
