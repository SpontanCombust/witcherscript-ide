#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    LogicalNot,
    BitwiseNot,
    Negation
}


#[derive(Debug, PartialEq)]
pub enum ArithmeticBinaryOperator {
    Multip,
    Div,
    Modulo,
    Add,
    Sub,
    BitwiseAnd,
    BitwiseOr
}

#[derive(Debug, PartialEq)]
pub enum LogicalBinaryOperator {
    And,
    Or
}

#[derive(Debug, PartialEq)]
pub enum RelationalBinaryOperator {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual
}

#[derive(Debug, PartialEq)]
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


#[derive(Debug, PartialEq)]
pub enum AssignmentOperator {
    Direct,
    Multip,
    Div,
    Modulo,
    Add,
    Sub
}
