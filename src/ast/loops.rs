use std::rc::Rc;
use super::{expression::*, functions::*};

pub struct ForLoop {
    pub init_expr: Option<Rc<Expression>>,
    pub condition: Option<Rc<Expression>>,
    pub iter_expr: Option<Rc<Expression>>,
    pub body: Option<FunctionBody>
}

pub struct WhileLoop {
    pub condition: Rc<Expression>,
    pub body: Option<FunctionBody>
}

pub struct DoWhileLoop {
    pub condition: Rc<Expression>,
    pub body: FunctionBody
}