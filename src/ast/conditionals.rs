use std::rc::Rc;
use super::{functions::*, expression::*};

pub struct IfConditional {
    pub condition: Rc<Expression>,
    pub body: FunctionBody,
    // "else if" statements can be treated as regular if statements 
    // that go directly under previous if's "else" directive without the brackets {}
    pub else_body: Option<FunctionBody>
}

pub struct SwitchConditional {
    pub matched_expr: Rc<Expression>,
    pub cases: Vec<SwitchConditionalCase>,
    pub default: Option<FunctionBody>,
}

pub struct SwitchConditionalCase {
    pub value: Rc<Expression>,
    pub body: Option<FunctionBody>,
}