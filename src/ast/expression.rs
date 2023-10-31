use std::rc::Rc;

use super::{literal::*, operators::*};

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    This,
    Super,
    Parent,
    VirtualParent,
    Identifier(String),
    FunctionCall {
        func: String, 
        args: Vec<Option<Rc<Expression>>> // arguments can be optional and can be skipped in the call (like func(arg0,,,arg3))
    },
    ArrayAccess {
        expr: Rc<Expression>, 
        index: Rc<Expression>
    },
    MemberAccess {
        expr: Rc<Expression>, 
        member: String
    },
    MethodCall {
        expr: Rc<Expression>,
        func: String,
        args: Vec<Option<Rc<Expression>>>
    },
    Instantiation {
        class: String,
        lifetime_object: Rc<Expression>
    },
    TypeCast {
        target_type: String,
        expr: Rc<Expression>
    },
    UnaryOperation(UnaryOperator, Rc<Expression>),
    BinaryOperation(Rc<Expression>, BinaryOperator, Rc<Expression>),
    AssignmentOperation(Rc<Expression>, AssignmentOperator, Rc<Expression>),
    TernaryConditional {
        condition: Rc<Expression>,
        expr_if_true: Rc<Expression>,
        expr_if_false: Rc<Expression>
    },
}