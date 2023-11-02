use super::{expressions::*, functions::*};

pub struct ForLoop {
    pub init_expr: Option<Box<Expression>>,
    pub condition: Option<Box<Expression>>,
    pub iter_expr: Option<Box<Expression>>,
    pub body: Option<FunctionBody>
}

pub struct WhileLoop {
    pub condition: Box<Expression>,
    pub body: Option<FunctionBody>
}

pub struct DoWhileLoop {
    pub condition: Box<Expression>,
    pub body: FunctionBody
}