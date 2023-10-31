use std::rc::Rc;
use super::{
    classes::*, 
    expression::*,
    loops::*,
    type_annotation::*,
};
use bitmask_enum::bitmask;


pub struct FunctionDeclaration {
    pub access_modifier: Option<AccessModifier>,
    pub specifiers: FunctionSpecifiers,

    pub name: String,
    pub return_type: TypeAnnotation,
    pub body: Option<FunctionBody> // if there is no body it doesn't have a definition
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
    Expr(Rc<Expression>),
    For(Rc<ForLoop>),
    While(Rc<WhileLoop>),
    DoWhile(Rc<DoWhileLoop>),
    Break,
    Continue,
    Return(Option<Rc<Expression>>)
}

pub type FunctionBody = Vec<FunctionStatement>;