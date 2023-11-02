use super::{functions::*, expressions::*};

pub struct IfConditional {
    pub condition: Box<Expression>,
    pub body: FunctionBody,
    // "else if" statements can be treated as regular if statements 
    // that go directly under previous if's "else" directive without the brackets {}
    pub else_body: Option<FunctionBody>
}

pub struct SwitchConditional {
    pub matched_expr: Box<Expression>,
    pub cases: Vec<SwitchConditionalCase>,
    pub default: Option<FunctionBody>,
}

pub struct SwitchConditionalCase {
    pub value: Box<Expression>,
    pub body: Option<FunctionBody>,
}