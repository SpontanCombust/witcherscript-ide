use std::rc::Rc;
use super::{
    expression::*,
    type_annotation::*,
    classes::*
};
use bitmask_enum::bitmask;


pub struct FunctionDeclaration {
    access_modifier: Option<AccessModifier>,
    specifiers: FunctionSpecifiers,

    name: String,
    return_type: TypeAnnotation,
    body: Option<Vec<FunctionStatement>> // if there is no body it doesn't have a definition
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
    name: String,
    is_optional: bool,
    is_output: bool,
    param_type: TypeAnnotation
}

pub enum FunctionStatement {
    Expr(Rc<Expression>)
    //TODO function statements
}
