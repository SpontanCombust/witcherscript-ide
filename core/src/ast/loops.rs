use super::{expressions::*, functions::*};

pub struct ForLoop {
    pub init_expr: Option<Box<Expression>>,
    pub condition: Option<Box<Expression>>,
    pub iter_expr: Option<Box<Expression>>,
    pub body: Box<FunctionStatement> // can be NOP
}

pub struct WhileLoop {
    pub condition: Box<Expression>,
    pub body: Box<FunctionStatement>
}

pub struct DoWhileLoop {
    pub condition: Box<Expression>,
    pub body: Box<FunctionStatement>
}