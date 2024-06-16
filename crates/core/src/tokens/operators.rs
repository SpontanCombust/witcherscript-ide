use std::fmt::Debug;
use crate::{AnyNode, DebugMaybeAlternate, DebugRange, SyntaxNode};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    BitNot,
    Negation,
    Plus
}

pub type UnaryOperatorNode<'script> = SyntaxNode<'script, UnaryOperator>;

impl UnaryOperatorNode<'_> {
    pub fn value(&self) -> UnaryOperator {
        match self.tree_node.kind() {
            "unary_op_neg" => UnaryOperator::Negation,
            "unary_op_not" => UnaryOperator::Not,
            "unary_op_bitnot" => UnaryOperator::BitNot,
            "unary_op_plus" => UnaryOperator::Plus,
            _ => panic!("Unknown unary operator: {} {}", self.tree_node.kind(), self.range().debug())
        }
    }
}

impl Debug for UnaryOperatorNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.value())?;
        write!(f, " {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for UnaryOperatorNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() {
            return Err(());
        }

        match value.tree_node.kind() {
            "unary_op_neg"      | 
            "unary_op_not"      |
            "unary_op_bitnot"   |
            "unary_op_plus" =>  Ok(value.into()),
            _ => Err(())
        }
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
    BitXor,
    And,
    Or,
    Equal,
    NotEqual,
    Lesser,
    LesserOrEqual,
    Greater,
    GreaterOrEqual
}

pub type BinaryOperatorNode<'script> = SyntaxNode<'script, BinaryOperator>;

impl BinaryOperatorNode<'_> {
    pub fn value(&self) -> BinaryOperator {
        match self.tree_node.kind() {
            "binary_op_or" => BinaryOperator::Or,
            "binary_op_and" => BinaryOperator::And,
            "binary_op_bitor" => BinaryOperator::BitOr,
            "binary_op_bitand" => BinaryOperator::BitAnd,
            "binary_op_bitxor" => BinaryOperator::BitXor,
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
            _ => panic!("Unknown binary operator: {} {}", self.tree_node.kind(), self.range().debug())
        }
    }
}

impl Debug for BinaryOperatorNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.value())?;
        write!(f, " {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for BinaryOperatorNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() {
            return Err(());
        }

        match value.tree_node.kind() {
            "binary_op_or"      | 
            "binary_op_and"     |
            "binary_op_bitor"   |
            "binary_op_bitand"  |
            "binary_op_bitxor"  |
            "binary_op_eq"      |
            "binary_op_neq"     |
            "binary_op_gt"      |
            "binary_op_ge"      |
            "binary_op_lt"      |
            "binary_op_le"      |
            "binary_op_diff"    |
            "binary_op_sum"     |
            "binary_op_mod"     |
            "binary_op_div"     |
            "binary_op_mult" => Ok(value.into()),
            _ => Err(())
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    Direct,
    Mult,
    Div,
    Sum,
    Diff,
    BitAnd,
    BitOr
}

pub type AssignmentOperatorNode<'script> = SyntaxNode<'script, AssignmentOperator>;

impl AssignmentOperatorNode<'_> {
    pub fn value(&self) -> AssignmentOperator {
        match self.tree_node.kind() {
            "assign_op_direct" => AssignmentOperator::Direct,
            "assign_op_sum" => AssignmentOperator::Sum,
            "assign_op_diff" => AssignmentOperator::Diff,
            "assign_op_mult" => AssignmentOperator::Mult,
            "assign_op_div" => AssignmentOperator::Div,
            "assign_op_bitand" => AssignmentOperator::BitAnd,
            "assign_op_bitor" => AssignmentOperator::BitOr,
            _ => panic!("Unknown assignment operator: {} {}", self.tree_node.kind(), self.range().debug())
        }
    }
}

impl Debug for AssignmentOperatorNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.value())?;
        write!(f, " {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for AssignmentOperatorNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() {
            return Err(());
        }
        
        match value.tree_node.kind() {
            "assign_op_direct"  |
            "assign_op_sum"     |
            "assign_op_diff"    |
            "assign_op_mult"    |
            "assign_op_div"     |
            "assign_op_bitand"  |
            "assign_op_bitor" => Ok(value.into()),
            _ => Err(())
        }
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
            BinaryOperator::Lesser          |
            BinaryOperator::LesserOrEqual   |
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
            AssignmentOperator::Div   |
            AssignmentOperator::Sum   |
            AssignmentOperator::Diff  => true,
            _ => false
        }
    }

    fn is_logical(&self) -> bool {
        false
    }

    fn is_bitwise(&self) -> bool {
        match self {
            AssignmentOperator::BitAnd |
            AssignmentOperator::BitOr => true,
            _ => false
        }
    }

    fn is_relational(&self) -> bool {
        false
    }
}
