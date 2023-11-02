use super::{literal::*, operators::*, identifier::Identifier};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Nested(Box<Expression>),
    Literal(Literal),
    This,
    Super,
    Parent,
    VirtualParent,
    Identifier(Identifier),
    FunctionCall {
        func: Identifier, 
        args: Vec<Option<Box<Expression>>> // arguments can be optional and can be skipped in the call (like func(arg0,,,arg3))
    },
    ArrayAccess {
        expr: Box<Expression>, 
        index: Box<Expression>
    },
    MemberAccess {
        expr: Box<Expression>, 
        member: Identifier
    },
    MethodCall {
        expr: Box<Expression>,
        func: Identifier,
        args: Vec<Option<Box<Expression>>>
    },
    Instantiation {
        class: Identifier,
        lifetime_object: Box<Expression>
    },
    TypeCast {
        target_type: Identifier,
        expr: Box<Expression>
    },
    UnaryOperation(UnaryOperator, Box<Expression>),
    BinaryOperation(Box<Expression>, BinaryOperator, Box<Expression>),
    AssignmentOperation(Box<Expression>, AssignmentOperator, Box<Expression>),
    TernaryConditional {
        condition: Box<Expression>,
        expr_if_true: Box<Expression>,
        expr_if_false: Box<Expression>
    },
}