use std::rc::Rc;

use super::{literal::Literal, operators::{UnaryOperator, BinaryOperator}};

pub enum Expression {
    Literal(Literal),
    Identifier(String),
    UnaryOperation(UnaryOperator, Rc<Expression>),
    BinaryOperation(Rc<Expression>, BinaryOperator, Rc<Expression>),
    TernaryConditional {
        condition: Rc<Expression>,
        expr_if_true: Rc<Expression>,
        expr_if_false: Rc<Expression>
    },
    MemberAccess {
        expr: Rc<Expression>, 
        member: String
    },
    Subscript {
        expr: Rc<Expression>, 
        index: Rc<Expression>
    },
    FunctionCall {
        expr: Rc<Expression>, 
        args: Vec<Rc<Expression>>
    },
    Instantiation {
        class: String,
        lifetime_object: Rc<Expression>
    },
    TypeCast {
        target_type: String,
        expr: Rc<Expression>
    },
    Nested(Rc<Expression>)
}