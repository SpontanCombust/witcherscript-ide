use std::rc::Rc;

use super::{literal::*, operators::*, identifier::Identifier};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Nested(Rc<Expression>),
    Literal(Literal),
    This,
    Super,
    Parent,
    VirtualParent,
    Identifier(Identifier),
    FunctionCall {
        func: Identifier, 
        args: Vec<Option<Rc<Expression>>> // arguments can be optional and can be skipped in the call (like func(arg0,,,arg3))
    },
    ArrayAccess {
        expr: Rc<Expression>, 
        index: Rc<Expression>
    },
    MemberAccess {
        expr: Rc<Expression>, 
        member: Identifier
    },
    MethodCall {
        expr: Rc<Expression>,
        func: Identifier,
        args: Vec<Option<Rc<Expression>>>
    },
    Instantiation {
        class: Identifier,
        lifetime_object: Rc<Expression>
    },
    TypeCast {
        target_type: Identifier,
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