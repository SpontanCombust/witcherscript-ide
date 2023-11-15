use crate::tokens::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Nested(Box<Spanned<Expression>>),
    Literal(Literal),
    This,
    Super,
    Parent,
    VirtualParent,
    Identifier(Identifier),
    FunctionCall {
        func: Spanned<Identifier>,
        args: Spanned<Vec<FunctionCallArg>> 
    },
    ArrayAccess {
        expr: Box<Spanned<Expression>>, 
        index: Box<Spanned<Expression>>
    },
    MemberAccess {
        expr: Box<Spanned<Expression>>, 
        member: Spanned<Identifier>
    },
    MethodCall {
        expr: Box<Spanned<Expression>>,
        func: Spanned<Identifier>,
        args: Spanned<Vec<FunctionCallArg>>
    },
    Instantiation {
        class: Spanned<Identifier>,
        lifetime_object: Box<Spanned<Expression>>
    },
    TypeCast {
        target_type: Spanned<Identifier>,
        expr: Box<Spanned<Expression>>
    },
    UnaryOperation(UnaryOperator, Box<Spanned<Expression>>),
    BinaryOperation(Box<Spanned<Expression>>, BinaryOperator, Box<Spanned<Expression>>),
    AssignmentOperation(Box<Spanned<Expression>>, AssignmentOperator, Box<Spanned<Expression>>),
    TernaryConditional {
        condition: Box<Spanned<Expression>>,
        expr_if_true: Box<Spanned<Expression>>,
        expr_if_false: Box<Spanned<Expression>>
    },
}

pub type FunctionCallArg = Option<Box<Spanned<Expression>>>; // arguments can be optional and can be skipped in the call (like func(arg0,,,arg3))