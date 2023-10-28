use std::rc::Rc;

use super::{literal::Literal, operators::{UnaryOperator, BinaryOperator}};

pub enum Expression {
    Literal(Literal),
    Identifier(String),
    UnaryOperation(UnaryOperator, Rc<Expression>),
    BinaryOperation(Rc<Expression>, BinaryOperator, Rc<Expression>),
    MemberAccess {
        exp: Rc<Expression>, 
        member_ident: String
    },
    Subscript {
        exp: Rc<Expression>, 
        index: Rc<Expression>
    },
    Call {
        exp: Rc<Expression>, 
        args: Vec<Rc<Expression>>
    },
    Instantiation {
        class: String,
        lifetime_object: Rc<Expression>
    },
    TypeCast {
        target_type: String,
        exp: Rc<Expression>
    },
    Nested(Rc<Expression>)
}