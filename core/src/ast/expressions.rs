use std::error::Error;

use crate::tokens::*;
use crate::{SyntaxNode, NamedSyntaxNode};


#[derive(Debug, Clone)]
pub struct NestedExpression;

impl NamedSyntaxNode for NestedExpression {
    const NODE_NAME: &'static str = "nested_expr";
}

impl SyntaxNode<'_, NestedExpression> {
    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.first_child()
    }
}


impl NamedSyntaxNode for Literal {
    const NODE_NAME: &'static str = "literal";
}

impl SyntaxNode<'_, Literal> {
    pub fn value(&self) -> Result<Literal, Box<dyn Error>> {
        let child = self.first_child::<()>();
        let text = child.text();
        match child.tree_node.kind() {
            "literal_null" => Ok(Literal::Null),
            "literal_int" => {
                let i = Self::parse_int(&text)?;
                Ok(Literal::Int(i))
            },
            "literal_float" => {
                let f = Self::parse_float(&text)?;
                Ok(Literal::Float(f))
            },
            "literal_string" => {
                let s = Self::parse_string(text);
                Ok(Literal::String(s))
            },
            "literal_name" => {
                let n = Self::parse_name(text);
                Ok(Literal::Name(n))
            },
            "literal_bool" => {
                let b = Self::parse_bool(text);
                Ok(Literal::Bool(b))
            }
            _ => panic!("Unknown literal kind")
        }
    }

    fn parse_int(s: &str) -> Result<i32, impl Error> {
        s.parse::<i32>()
    }

    fn parse_float(s: &str) -> Result<f32, impl Error> {
        // trim the optional trailing 'f'
        let s = if s.chars().last().unwrap() == 'f' { 
            &s[..s.len() - 1] 
        } else { 
            s 
        };

        s.parse::<f32>()
    }

    fn parse_string(s: String) -> String {
        s[1..s.len()-1] // eliminate surrounding quotes
        .replace(r#"\""#, r#"""#) // eliminate escaped quotes
    }

    fn parse_name(s: String) -> String {
        s[1..s.len()-1].to_string() // eliminate surrounding quotes   
    }

    fn parse_bool(s: String) -> bool {
        match s.as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("Unknown bool value")
        }
    }
}


#[derive(Debug, Clone)]
pub struct ThisExpression;

impl NamedSyntaxNode for ThisExpression {
    const NODE_NAME: &'static str = "this_expr";
}

impl SyntaxNode<'_, ThisExpression> {}


#[derive(Debug, Clone)]
pub struct SuperExpression;

impl NamedSyntaxNode for SuperExpression {
    const NODE_NAME: &'static str = "super_expr";
}

impl SyntaxNode<'_, SuperExpression> {}


#[derive(Debug, Clone)]
pub struct ParentExpression;

impl NamedSyntaxNode for ParentExpression {
    const NODE_NAME: &'static str = "parent_expr";
}

impl SyntaxNode<'_, ParentExpression> {}


#[derive(Debug, Clone)]
pub struct VirtualParentExpression;

impl NamedSyntaxNode for VirtualParentExpression {
    const NODE_NAME: &'static str = "virtual_parent_expr";
}

impl SyntaxNode<'_, VirtualParentExpression> {}


impl NamedSyntaxNode for Identifier {
    const NODE_NAME: &'static str = "identifier";
}

impl SyntaxNode<'_, Identifier> {
    // use text() to get identifier name
}


#[derive(Debug, Clone)]
pub struct FunctionCallExpression;

impl NamedSyntaxNode for FunctionCallExpression {
    const NODE_NAME: &'static str = "func_call_expr";
}

impl SyntaxNode<'_, FunctionCallExpression> {
    pub fn func(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("func")
    }

    pub fn args(&self) -> impl Iterator<Item = Option<SyntaxNode<'_, Expression>>> {
        func_args(&self)
    }
}

fn func_args<'script, T>(func_node: &'script SyntaxNode<'_, T>) -> impl Iterator<Item = Option<SyntaxNode<'script, Expression<'script>>>> {
    if let Some(args_node) = func_node.tree_node.child_by_field_name("args") {
        let mut cursor = args_node.walk();
        cursor.goto_first_child();
        
        let mut v = vec![];
        let mut previous_was_comma = true;
        
        loop {
            let n = cursor.node();
            // Because of how default parameters in WitcherScript work we can't simply do a delimited list, 
            // because the values in that list can be empty space or no space at all. We need to put 
            // spacial care into handling commas.
            // If a node is named, some argument was passed. If a node is anonymous it is a comma.
            // If we encounter a comma right after a previous comma, we have a defaulted argument.
            if n.is_named() {
                v.push(Some(func_node.clone_as_with(n)));
                previous_was_comma = false;
            } else {
                if previous_was_comma {
                    v.push(None);
                }
                previous_was_comma = true;
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }

        if previous_was_comma {
            v.push(None);
        }
        
        v.into_iter()
    } else {
        // If the argument list is empty we don't know whether it actually takes no arguments
        // or all the arguments it takes are optional. It is difficult to figure that out
        // without looking at this function's declaration.
        vec![].into_iter()
    }
}


#[derive(Debug, Clone)]
pub struct ArrayExpression;

impl NamedSyntaxNode for ArrayExpression {
    const NODE_NAME: &'static str = "array_expr";
}

impl SyntaxNode<'_, ArrayExpression> {
    pub fn accessor(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("accessor")
    }

    pub fn index(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("index")
    }
}


#[derive(Debug, Clone)]
pub struct MemberFieldExpression;

impl NamedSyntaxNode for MemberFieldExpression {
    const NODE_NAME: &'static str = "member_field_expr";
}

impl SyntaxNode<'_, MemberFieldExpression> {
    pub fn accessor(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("accessor")
    }

    pub fn member(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("member")
    }
}


#[derive(Debug, Clone)]
pub struct MethodCallExpression;

impl NamedSyntaxNode for MethodCallExpression {
    const NODE_NAME: &'static str = "member_func_call_expr";
}

impl SyntaxNode<'_, MethodCallExpression> {
    pub fn accessor(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("accessor")
    }

    pub fn func(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("func")
    }

    pub fn args(&self) -> impl Iterator<Item = Option<SyntaxNode<'_, Expression>>> {
        func_args(&self)
    }
}


#[derive(Debug, Clone)]
pub struct InstantiationExpression;

impl NamedSyntaxNode for InstantiationExpression {
    const NODE_NAME: &'static str = "new_expr";
}

impl SyntaxNode<'_, InstantiationExpression> {
    pub fn class(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("class")
    }

    pub fn lifetime_obj(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("func")
    }
}


#[derive(Debug, Clone)]
pub struct TypeCastExpression;

impl NamedSyntaxNode for TypeCastExpression {
    const NODE_NAME: &'static str = "cast_expr";
}

impl SyntaxNode<'_, TypeCastExpression> {
    pub fn target_type(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("type")
    }

    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("value")
    }
}


#[derive(Debug, Clone)]
pub struct UnaryOperationExpression;

impl NamedSyntaxNode for UnaryOperationExpression {
    const NODE_NAME: &'static str = "unary_op_expr";
}

impl SyntaxNode<'_, UnaryOperationExpression> {
    pub fn op(&self) -> UnaryOperator {
        let child = self.field_child::<()>("op");
        match child.tree_node.kind() {
            "unary_op_neg" => UnaryOperator::Negation,
            "unary_op_not" => UnaryOperator::Not,
            "unary_op_bitnot" => UnaryOperator::BitNot,
            "unary_op_plus" => UnaryOperator::Plus,
            _ => panic!("Unknown unary operator")
        }
    }

    pub fn right(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("right")
    }
}


#[derive(Debug, Clone)]
pub struct BinaryOperationExpression;

impl NamedSyntaxNode for BinaryOperationExpression {
    const NODE_NAME: &'static str = "binary_op_expr";
}

impl SyntaxNode<'_, BinaryOperationExpression> {
    pub fn op(&self) -> BinaryOperator {
        let child = self.field_child::<()>("op");
        match child.tree_node.kind() {
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
            _ => panic!("Unknown binary operator")
        }
    }

    pub fn left(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("left")
    }

    pub fn right(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("right")
    }
}


#[derive(Debug, Clone)]
pub struct AssignmentOperationExpression;

impl NamedSyntaxNode for AssignmentOperationExpression {
    const NODE_NAME: &'static str = "assign_op_expr";
}

impl SyntaxNode<'_, AssignmentOperationExpression> {
    pub fn op(&self) -> AssignmentOperator {
        let child = self.field_child::<()>("op");
        match child.tree_node.kind() {
            "assign_op_direct" => AssignmentOperator::Direct,
            "assign_op_sum" => AssignmentOperator::Sum,
            "assign_op_diff" => AssignmentOperator::Diff,
            "assign_op_mult" => AssignmentOperator::Mult,
            "assign_op_div" => AssignmentOperator::Div,
            "assign_op_mod" => AssignmentOperator::Mod,
            _ => panic!("Unknown assignment operator")
        }
    }

    pub fn left(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("left")
    }

    pub fn right(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("right")
    }
}


#[derive(Debug, Clone)]
pub struct TernaryConditionalExpression;

impl NamedSyntaxNode for TernaryConditionalExpression {
    const NODE_NAME: &'static str = "ternary_cond_expr";
}

impl SyntaxNode<'_, TernaryConditionalExpression> {
    pub fn cond(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("cond")
    }

    pub fn conseq(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("conseq")
    }

    pub fn alt(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("alt")
    }
}


// Represents the anonymous $._expr node
#[derive(Debug, Clone)]
pub enum Expression<'script> {
    Nested(SyntaxNode<'script, NestedExpression>),
    Literal(SyntaxNode<'script, Literal>),
    This(SyntaxNode<'script, ThisExpression>),
    Super(SyntaxNode<'script, SuperExpression>),
    Parent(SyntaxNode<'script, ParentExpression>),
    VirtualParent(SyntaxNode<'script, VirtualParentExpression>),
    Identifier(SyntaxNode<'script, Identifier>),
    FunctionCall(SyntaxNode<'script, FunctionCallExpression>),
    Array(SyntaxNode<'script, ArrayExpression>),
    MemberField(SyntaxNode<'script, MemberFieldExpression>),
    MethodCall(SyntaxNode<'script, MethodCallExpression>),
    Instantiation(SyntaxNode<'script, InstantiationExpression>),
    TypeCast(SyntaxNode<'script, TypeCastExpression>),
    UnaryOperation(SyntaxNode<'script, UnaryOperationExpression>),
    BinaryOperation(SyntaxNode<'script, BinaryOperationExpression>),
    AssignmentOperation(SyntaxNode<'script, AssignmentOperationExpression>),
    TernaryConditional(SyntaxNode<'script, TernaryConditionalExpression>),
}

impl SyntaxNode<'_, Expression<'_>> {
    pub fn value(&self) -> Expression {
        match self.tree_node.kind() {
            AssignmentOperationExpression::NODE_NAME => Expression::AssignmentOperation(self.clone_as()),
            TernaryConditionalExpression::NODE_NAME => Expression::TernaryConditional(self.clone_as()),
            BinaryOperationExpression::NODE_NAME => Expression::BinaryOperation(self.clone_as()),
            InstantiationExpression::NODE_NAME => Expression::Instantiation(self.clone_as()),
            UnaryOperationExpression::NODE_NAME => Expression::UnaryOperation(self.clone_as()),
            TypeCastExpression::NODE_NAME => Expression::TypeCast(self.clone_as()),
            MethodCallExpression::NODE_NAME => Expression::MethodCall(self.clone_as()),
            MemberFieldExpression::NODE_NAME => Expression::MemberField(self.clone_as()),
            FunctionCallExpression::NODE_NAME => Expression::FunctionCall(self.clone_as()),
            ArrayExpression::NODE_NAME => Expression::Array(self.clone_as()),
            NestedExpression::NODE_NAME => Expression::Nested(self.clone_as()),
            ThisExpression::NODE_NAME => Expression::This(self.clone_as()),
            SuperExpression::NODE_NAME => Expression::Super(self.clone_as()),
            ParentExpression::NODE_NAME => Expression::Parent(self.clone_as()),
            VirtualParentExpression::NODE_NAME => Expression::VirtualParent(self.clone_as()),
            Identifier::NODE_NAME => Expression::Identifier(self.clone_as()),
            Literal::NODE_NAME => Expression::Literal(self.clone_as()),
            _ => panic!("Unknown expression type")
        }
    }
}