use crate::lexing::Spanned;
use super::{functions::*, expressions::Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct IfConditional {
    pub condition: Box<Spanned<Expression>>,
    pub body: Box<Spanned<FunctionStatement>>, // can be a scope or a NOP
    // "else if" statement can be treated as regular if statement nested in the "else" statement
    pub else_body: Option<Box<Spanned<FunctionStatement>>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchConditional {
    pub matched_expr: Box<Spanned<Expression>>,
    pub cases: Spanned<SwitchConditionalBody>,
    pub default: Option<Spanned<FunctionBody>>,
}

type SwitchConditionalBody = Vec<Spanned<SwitchConditionalCase>>;

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchConditionalCase {
    pub value: Box<Spanned<Expression>>,
    pub body: Spanned<FunctionBody>,
}