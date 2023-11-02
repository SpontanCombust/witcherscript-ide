use super::{functions::*, expressions::*};

pub struct IfConditional {
    pub condition: Box<Expression>,
    pub body: Box<FunctionStatement>, // can be a scope or a NOP
    // "else if" statement can be treated as regular if statement nested in the "else" statement
    pub else_body: Option<Box<FunctionStatement>>
}

pub struct SwitchConditional {
    pub matched_expr: Box<Expression>,
    pub cases: Vec<SwitchConditionalCase>,
    pub default: Option<FunctionBody>,
}

pub struct SwitchConditionalCase {
    pub value: Box<Expression>,
    pub body: FunctionBody,
}