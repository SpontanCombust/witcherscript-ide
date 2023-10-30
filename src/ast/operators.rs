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

impl Into<BinaryOperator> for ArithmeticBinaryOperator {
    fn into(self) -> BinaryOperator {
        BinaryOperator::Airthmetic(self)
    }
}

impl Into<BinaryOperator> for LogicalBinaryOperator {
    fn into(self) -> BinaryOperator {
        BinaryOperator::Logical(self)
    }
}

impl Into<BinaryOperator> for RelationalBinaryOperator {
    fn into(self) -> BinaryOperator {
        BinaryOperator::Relational(self)
    }
}


#[derive(Debug)]
pub enum AssignmentOperator {
    Direct,
    Multip,
    Div,
    Modulo,
    Add,
    Sub
}
