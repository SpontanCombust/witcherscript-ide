pub enum UnaryOperator {
    LogicalNot,
    BitwiseNot,
    Minus
}

pub enum ArithmeticOperator {
    Multip,
    Div,
    Modulo,
    Add,
    Sub,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr
}

pub enum RelationalOperator {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual
}

pub enum BinaryOperator {
    Airthmetic(ArithmeticOperator),
    Relational(RelationalOperator)
}

pub enum AssignmentOperator {
    Direct,
    Multip,
    Div,
    Modulo,
    Add,
    Sub
}
