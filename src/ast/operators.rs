#[derive(Debug)]
pub enum UnaryOperator {
    LogicalNot,
    BitwiseNot,
    Minus
}

#[derive(Debug)]
pub enum ArithmeticBinaryOperator {
    Multip,
    Div,
    Modulo,
    Add,
    Sub,
    BitwiseAnd,
    BitwiseOr
}

#[derive(Debug)]
pub enum LogicalBinaryOperator {
    And,
    Or
}

#[derive(Debug)]
pub enum RelationalBinaryOperator {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual
}

#[derive(Debug)]
pub enum BinaryOperator {
    Airthmetic(ArithmeticBinaryOperator),
    Logical(LogicalBinaryOperator),
    Relational(RelationalBinaryOperator)
}

pub enum AssignmentOperator {
    Direct,
    Multip,
    Div,
    Modulo,
    Add,
    Sub
}
