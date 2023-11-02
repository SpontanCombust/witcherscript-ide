use super::{functions::*, expressions::*};

#[derive(Debug, Clone, PartialEq)]
pub struct IfConditional {
    pub condition: Box<Expression>,
    pub body: Box<FunctionStatement>, // can be a scope or a NOP
    // "else if" statement can be treated as regular if statement nested in the "else" statement
    pub else_body: Option<Box<FunctionStatement>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchConditional {
    pub matched_expr: Box<Expression>,
    pub cases: Vec<SwitchConditionalCase>,
    pub default: Option<FunctionBody>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchConditionalCase {
    pub value: Box<Expression>,
    pub body: FunctionBody,
}