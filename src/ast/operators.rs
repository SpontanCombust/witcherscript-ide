pub enum UnaryOperator {
    LogicalNot,
    BitwiseNot,
    Minus
}

pub enum ArithmeticBinaryOperator {
    Multip,
    Div,
    Modulo,
    Add,
    Sub,
    BitwiseAnd,
    BitwiseOr
}

pub enum LogicalBinaryOperator {
    And,
    Or
}

pub enum RelationalBinaryOperator {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual
}

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
