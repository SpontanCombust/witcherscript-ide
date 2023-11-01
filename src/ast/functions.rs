use std::rc::Rc;
use bitmask_enum::bitmask;

use super::{
    identifier::Identifier, 
    classes::AccessModifier, 
    vars::TypeAnnotation, 
    expression::Expression, 
    loops::*,
    conditionals::*,
};


pub struct FunctionDeclaration {
    pub imported: bool,
    pub access_modifier: Option<AccessModifier>,
    pub specifiers: FunctionSpecifiers,
    pub speciality: Option<FunctionSpeciality>,

    pub name: Identifier,
    pub return_type: TypeAnnotation,
    pub body: Option<FunctionBody> // if there is no body it doesn't have a definition
}


#[bitmask(u32)]
pub enum FunctionSpecifiers {
    Final,
    Latent,
}

pub enum FunctionSpeciality {
    Entry,
    Event,
    Exec,
    Quest,
    Timer,
    Storyscene
}

pub struct FunctionParameter {
    pub name: Identifier,
    pub is_optional: bool,
    pub is_output: bool,
    pub param_type: TypeAnnotation
}


pub enum FunctionStatement {
    Expr(Rc<Expression>),
    For(ForLoop),
    While(WhileLoop),
    DoWhile(DoWhileLoop),
    If(IfConditional),
    Switch(SwitchConditional),
    Break,
    Continue,
    Return(Option<Rc<Expression>>)
}

pub type FunctionBody = Vec<FunctionStatement>;