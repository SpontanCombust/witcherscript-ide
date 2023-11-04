use bitmask_enum::bitmask;

use super::{
    identifier::Identifier, 
    classes::AccessModifier, 
    vars::*, 
    expressions::Expression, 
    loops::*,
    conditionals::*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub imported: bool,
    pub access_modifier: Option<AccessModifier>,
    pub specifiers: FunctionSpecifiers,
    pub speciality: Option<FunctionSpeciality>,

    pub name: Identifier,
    pub params: Vec<FunctionParameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Option<FunctionBody> // if there is no body it doesn't have a definition
}


#[bitmask(u8)]
pub enum FunctionSpecifiers {
    Final,
    Latent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionSpeciality {
    Entry,
    Event,
    Exec,
    Quest,
    Timer,
    Storyscene
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionParameter {
    pub name: Identifier,
    pub is_optional: bool,
    pub is_output: bool,
    pub param_type: TypeAnnotation
}


#[derive(Debug, Clone, PartialEq)]
pub enum FunctionStatement {
    Var(VarDeclaration),
    Expr(Box<Expression>),
    For(ForLoop),
    While(WhileLoop),
    DoWhile(DoWhileLoop),
    If(IfConditional),
    Switch(SwitchConditional),
    Break,
    Continue,
    Return(Option<Box<Expression>>),
    Delete(Box<Expression>),
    Scope(FunctionBody),
    Nop
}

pub type FunctionBody = Vec<FunctionStatement>;