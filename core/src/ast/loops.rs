use crate::lexing::Spanned;
use super::{expressions::Expression, functions::FunctionStatement};

#[derive(Debug, Clone, PartialEq)]
pub struct ForLoop {
    pub init_expr: Option<Box<Spanned<Expression>>>,
    pub condition: Option<Box<Spanned<Expression>>>,
    pub iter_expr: Option<Box<Spanned<Expression>>>,
    pub body: Box<Spanned<FunctionStatement>> // can be NOP
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub condition: Box<Spanned<Expression>>,
    pub body: Box<Spanned<FunctionStatement>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoWhileLoop {
    pub condition: Box<Spanned<Expression>>,
    pub body: Box<Spanned<FunctionStatement>>
}