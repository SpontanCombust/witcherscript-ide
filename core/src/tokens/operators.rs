use std::fmt::Debug;

use crate::SyntaxNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    BitNot,
    Negation,
    Plus
}

impl SyntaxNode<'_, UnaryOperator> {
    pub fn value(&self) -> UnaryOperator {
        match self.tree_node.kind() {
            "unary_op_neg" => UnaryOperator::Negation,
            "unary_op_not" => UnaryOperator::Not,
            "unary_op_bitnot" => UnaryOperator::BitNot,
            "unary_op_plus" => UnaryOperator::Plus,
            _ => panic!("Unknown unary operator: {}", self.tree_node.kind())
        }
    }
}

impl Debug for SyntaxNode<'_, UnaryOperator> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Mult,
    Div,
    Mod,
    Sum,
    Diff,
    BitAnd,
    BitOr,
    And,
    Or,
    Equal,
    NotEqual,
    Lesser,
    LesserOrEqual,
    Greater,
    GreaterOrEqual
}

impl SyntaxNode<'_, BinaryOperator> {
    pub fn value(&self) -> BinaryOperator {
        match self.tree_node.kind() {
            "binary_op_or" => BinaryOperator::Or,
            "binary_op_and" => BinaryOperator::And,
            "binary_op_bitor" => BinaryOperator::BitOr,
            "binary_op_bitand" => BinaryOperator::BitAnd,
            "binary_op_eq" => BinaryOperator::Equal,
            "binary_op_neq" => BinaryOperator::NotEqual,
            "binary_op_gt" => BinaryOperator::Greater,
            "binary_op_ge" => BinaryOperator::GreaterOrEqual,
            "binary_op_lt" => BinaryOperator::Lesser,
            "binary_op_le" => BinaryOperator::LesserOrEqual,
            "binary_op_diff" => BinaryOperator::Diff,
            "binary_op_sum" => BinaryOperator::Sum,
            "binary_op_mod" => BinaryOperator::Mod,
            "binary_op_div" => BinaryOperator::Div,
            "binary_op_mult" => BinaryOperator::Mult,
            _ => panic!("Unknown binary operator: {}", self.tree_node.kind())
        }
    }
}

impl Debug for SyntaxNode<'_, BinaryOperator> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    Direct,
    Mult,
    Div,
    Mod,
    Sum,
    Diff
}

impl SyntaxNode<'_, AssignmentOperator> {
    pub fn value(&self) -> AssignmentOperator {
        match self.tree_node.kind() {
            "assign_op_direct" => AssignmentOperator::Direct,
            "assign_op_sum" => AssignmentOperator::Sum,
            "assign_op_diff" => AssignmentOperator::Diff,
            "assign_op_mult" => AssignmentOperator::Mult,
            "assign_op_div" => AssignmentOperator::Div,
            "assign_op_mod" => AssignmentOperator::Mod,
            _ => panic!("Unknown assignment operator: {}", self.tree_node.kind())
        }
    }
}

impl Debug for SyntaxNode<'_, AssignmentOperator> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
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
            BinaryOperator::Mult  |
            BinaryOperator::Div     |
            BinaryOperator::Mod  |
            BinaryOperator::Sum     |
            BinaryOperator::Diff     => true,
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
            BinaryOperator::Lesser            |
            BinaryOperator::LesserOrEqual     |
            BinaryOperator::Greater         |
            BinaryOperator::GreaterOrEqual  => true,
            _ => false
        }
    }
}

impl OperatorTraits for AssignmentOperator {
    fn is_arithmetic(&self) -> bool {
        match self {
            AssignmentOperator::Mult  |
            AssignmentOperator::Div     |
            AssignmentOperator::Mod  |
            AssignmentOperator::Sum     |
            AssignmentOperator::Diff     => true,
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