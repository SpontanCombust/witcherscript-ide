use crate::lexing::{Identifier, Spanned};
use super::{
    classes::AccessModifier, 
    vars::*, 
    expressions::Expression, 
    loops::*,
    conditionals::*, 
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub imported: bool,
    pub access_modifier: Option<Spanned<AccessModifier>>,
    pub specifiers: Spanned<Vec<Spanned<FunctionSpecifier>>>,
    pub speciality: Spanned<FunctionSpeciality>,

    pub name: Spanned<Identifier>,
    pub params: Spanned<Vec<FunctionParameter>>,
    pub return_type: Option<Spanned<TypeAnnotation>>,
    pub body: Option<Spanned<FunctionBody>> // if there is no body it doesn't have a definition
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionSpecifier {
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
    Storyscene,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter { //TODO turn into FunctionParameterGroup, similar in notion to VarDeclaration
    pub name: Spanned<Identifier>,
    pub is_optional: bool,
    pub is_output: bool,
    pub param_type: Spanned<TypeAnnotation>
}


#[derive(Debug, Clone, PartialEq)]
pub enum FunctionStatement {
    Var(VarDeclaration),
    Expr(Box<Spanned<Expression>>),
    For(ForLoop),
    While(WhileLoop),
    DoWhile(DoWhileLoop),
    If(IfConditional),
    Switch(SwitchConditional),
    Break,
    Continue,
    Return(Option<Box<Spanned<Expression>>>),
    Delete(Box<Spanned<Expression>>),
    Scope(Spanned<FunctionBody>),
    Nop
}

pub type FunctionBody = Vec<Spanned<FunctionStatement>>;