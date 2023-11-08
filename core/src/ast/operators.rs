#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    BitNot,
    Negation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Multip,
    Div,
    Modulo,
    Add,
    Sub,
    BitAnd,
    BitOr,
    And,
    Or,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    Direct,
    Multip,
    Div,
    Modulo,
    Add,
    Sub
}



pub trait OperatorTraits {
    fn is_arithmetic(&self) -> bool;
    fn is_logical(&self) -> bool;   
    fn is_bitwise(&self) -> bool;
    fn is_relational(&self) -> bool;
}

impl OperatorTraits for UnaryOperator {
    fn is_arithmetic(&self) -> bool {
        match self {
            UnaryOperator::Negation => true,
            _ => false
        }
    }

    fn is_logical(&self) -> bool {
        match self {
            UnaryOperator::Not => true,
            _ => false
        }
    }

    fn is_bitwise(&self) -> bool {
        match self {
            UnaryOperator::BitNot => true,
            _ => false
        }
    }

    fn is_relational(&self) -> bool {
        false  
    }
}

impl OperatorTraits for BinaryOperator {
    fn is_arithmetic(&self) -> bool {
        match self {
            BinaryOperator::Multip  |
            BinaryOperator::Div     |
            BinaryOperator::Modulo  |
            BinaryOperator::Add     |
            BinaryOperator::Sub     => true,
            _ => false
        }
    }

    fn is_logical(&self) -> bool {
        match self {
            BinaryOperator::And |
            BinaryOperator::Or  => true,
            _ => false
        }
    }

    fn is_bitwise(&self) -> bool {
        match self {
            BinaryOperator::BitAnd  |
            BinaryOperator::BitOr   => true,
            _ => false
        }
    }

    fn is_relational(&self) -> bool {
        match self {
            BinaryOperator::Equal           |
            BinaryOperator::NotEqual        |
            BinaryOperator::Less            |
            BinaryOperator::LessOrEqual     |
            BinaryOperator::Greater         |
            BinaryOperator::GreaterOrEqual  => true,
            _ => false
        }
    }
}

impl OperatorTraits for AssignmentOperator {
    fn is_arithmetic(&self) -> bool {
        match self {
            AssignmentOperator::Multip  |
            AssignmentOperator::Div     |
            AssignmentOperator::Modulo  |
            AssignmentOperator::Add     |
            AssignmentOperator::Sub     => true,
            _ => false
        }
    }

    fn is_logical(&self) -> bool {
        false
    }

    fn is_bitwise(&self) -> bool {
        false
    }

    fn is_relational(&self) -> bool {
        false
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
    Assignment(AssignmentOperator)
}