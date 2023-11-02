use super::{expressions::*, functions::*};

#[derive(Debug, Clone, PartialEq)]
pub struct ForLoop {
    pub init_expr: Option<Box<Expression>>,
    pub condition: Option<Box<Expression>>,
    pub iter_expr: Option<Box<Expression>>,
    pub body: Box<FunctionStatement> // can be NOP
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub condition: Box<Expression>,
    pub body: Box<FunctionStatement>
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoWhileLoop {
    pub condition: Box<Expression>,
    pub body: Box<FunctionStatement>
}