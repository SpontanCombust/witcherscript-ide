use std::fmt::Debug;
use crate::{SyntaxNode, NamedSyntaxNode, tokens::*, AnyNode};
use super::{StatementTraversal, ExpressionVisitor, ExpressionTraversal, StatementVisitor};


#[derive(Debug, Clone)]
pub struct NestedExpression;

pub type NestedExpressionNode<'script> = SyntaxNode<'script, NestedExpression>;

impl NamedSyntaxNode for NestedExpressionNode<'_> {
    const NODE_KIND: &'static str = "nested_expr";
}

impl NestedExpressionNode<'_> {
    pub fn value(&self) -> ExpressionNode {
        self.first_child(true).unwrap().into()
    }
}

impl Debug for NestedExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NestedExpression")
            .field(&self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for NestedExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for NestedExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.value().accept(visitor);
        visitor.visit_nested_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct ThisExpression;

pub type ThisExpressionNode<'script> = SyntaxNode<'script, ThisExpression>;

impl NamedSyntaxNode for ThisExpressionNode<'_> {
    const NODE_KIND: &'static str = "this_expr";
}

impl ThisExpressionNode<'_> {}

impl Debug for ThisExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThisExpression")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ThisExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for ThisExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_this_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct SuperExpression;

pub type SuperExpressionNode<'script> = SyntaxNode<'script, SuperExpression>;

impl NamedSyntaxNode for SuperExpressionNode<'_> {
    const NODE_KIND: &'static str = "super_expr";
}

impl SuperExpressionNode<'_> {}

impl Debug for SuperExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperExpression")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SuperExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for SuperExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_super_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct ParentExpression;

pub type ParentExpressionNode<'script> = SyntaxNode<'script, ParentExpression>;

impl NamedSyntaxNode for ParentExpressionNode<'_> {
    const NODE_KIND: &'static str = "parent_expr";
}

impl ParentExpressionNode<'_> {}

impl Debug for ParentExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParentExpression")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ParentExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for ParentExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_parent_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct VirtualParentExpression;

pub type VirtualParentExpressionNode<'script> = SyntaxNode<'script, VirtualParentExpression>;

impl NamedSyntaxNode for VirtualParentExpressionNode<'_> {
    const NODE_KIND: &'static str = "virtual_parent_expr";
}

impl VirtualParentExpressionNode<'_> {}

impl Debug for VirtualParentExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VirtualParentExpression")
    }
}

impl<'script> TryFrom<AnyNode<'script>> for VirtualParentExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for VirtualParentExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_virtual_parent_expr(self);
    }
}




#[derive(Debug, Clone)]
pub struct FunctionCallExpression;

pub type FunctionCallExpressionNode<'script> = SyntaxNode<'script, FunctionCallExpression>;

impl NamedSyntaxNode for FunctionCallExpressionNode<'_> {
    const NODE_KIND: &'static str = "func_call_expr";
}

impl FunctionCallExpressionNode<'_> {
    pub fn func(&self) -> IdentifierNode {
        self.field_child("func").unwrap().into()
    }

    pub fn args(&self) -> impl Iterator<Item = FuncCallArg> {
        func_args(self)
    }
}

impl Debug for FunctionCallExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionCallExpression")
            .field("func", &self.func())
            .field("args", &self.args().collect::<Vec<_>>())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionCallExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for FunctionCallExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.args().for_each(|arg| arg.accept(visitor));
        visitor.visit_func_call_expr(self);
    }
}


type FuncCallArg<'script> = Option<ExpressionNode<'script>>;

impl ExpressionTraversal for FuncCallArg<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        if let Some(expr) = self {
            expr.accept(visitor);
        }
        visitor.visit_func_call_arg(self);
    }
}

fn func_args<'script, T: Clone>(func_node: &'script SyntaxNode<'_, T>) -> impl Iterator<Item = FuncCallArg<'script>> {
    if let Some(args_node) = func_node.tree_node.child_by_field_name("args") {
        let mut cursor = args_node.walk();
        cursor.goto_first_child();
        
        let mut v = vec![];
        let mut previous_was_comma = true;
        
        loop {
            let n = cursor.node();
            // Because of how default parameters in WitcherScript work we can't simply do a delimited list, 
            // because the values in that list can be empty space or no space at all. We need to put 
            // special care into handling commas.
            // If a node is named, some argument was passed. If a node is anonymous it is a comma.
            // If we encounter a comma right after a previous comma, we have a defaulted argument.
            if !n.is_error() {
                if n.is_named() {
                    v.push(Some(SyntaxNode::new(n)));
                    previous_was_comma = false;
                } else {
                    if previous_was_comma {
                        v.push(None);
                    }
                    previous_was_comma = true;
                }
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

pub type ArrayExpressionNode<'script> = SyntaxNode<'script, ArrayExpression>;

impl NamedSyntaxNode for ArrayExpressionNode<'_> {
    const NODE_KIND: &'static str = "array_expr";
}

impl ArrayExpressionNode<'_> {
    pub fn accessor(&self) -> ExpressionNode {
        self.field_child("accessor").unwrap().into()
    }

    pub fn index(&self) -> ExpressionNode {
        self.field_child("index").unwrap().into()
    }
}

impl Debug for ArrayExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayExpression")
            .field("accessor", &self.accessor())
            .field("index", &self.index())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ArrayExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for ArrayExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.accessor().accept(visitor);
        self.index().accept(visitor);
        visitor.visit_array_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct MemberFieldExpression;

pub type MemberFieldExpressionNode<'script> = SyntaxNode<'script, MemberFieldExpression>;

impl NamedSyntaxNode for MemberFieldExpressionNode<'_> {
    const NODE_KIND: &'static str = "member_field_expr";
}

impl MemberFieldExpressionNode<'_> {
    pub fn accessor(&self) -> ExpressionNode {
        self.field_child("accessor").unwrap().into()
    }

    pub fn member(&self) -> IdentifierNode {
        self.field_child("member").unwrap().into()
    }
}

impl Debug for MemberFieldExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberFieldExpression")
            .field("accessor", &self.accessor())
            .field("member", &self.member())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberFieldExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for MemberFieldExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.accessor().accept(visitor);
        visitor.visit_member_field_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct MethodCallExpression;

pub type MethodCallExpressionNode<'script> = SyntaxNode<'script, MethodCallExpression>;

impl NamedSyntaxNode for MethodCallExpressionNode<'_> {
    const NODE_KIND: &'static str = "member_func_call_expr";
}

impl MethodCallExpressionNode<'_> {
    pub fn accessor(&self) -> ExpressionNode {
        self.field_child("accessor").unwrap().into()
    }

    pub fn func(&self) -> IdentifierNode {
        self.field_child("func").unwrap().into()
    }

    pub fn args(&self) -> impl Iterator<Item = FuncCallArg> {
        func_args(self)
    }
}

impl Debug for MethodCallExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionCallExpression")
            .field("accessor", &self.accessor())
            .field("func", &self.func())
            .field("args", &self.args().collect::<Vec<_>>())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MethodCallExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for MethodCallExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.accessor().accept(visitor);
        self.args().for_each(|arg| arg.accept(visitor));
        visitor.visit_method_call_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct InstantiationExpression;

pub type InstantiationExpressionNode<'script> = SyntaxNode<'script, InstantiationExpression>;

impl NamedSyntaxNode for InstantiationExpressionNode<'_> {
    const NODE_KIND: &'static str = "new_expr";
}

impl InstantiationExpressionNode<'_> {
    pub fn class(&self) -> IdentifierNode {
        self.field_child("class").unwrap().into()
    }

    pub fn lifetime_obj(&self) -> ExpressionNode {
        self.field_child("lifetime_obj").unwrap().into()
    }
}

impl Debug for InstantiationExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstantiationExpression")
            .field("class", &self.class())
            .field("lifetime_obj", &self.lifetime_obj())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for InstantiationExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for InstantiationExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.lifetime_obj().accept(visitor);
        visitor.visit_instantiation_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct TypeCastExpression;

pub type TypeCastExpressionNode<'script> = SyntaxNode<'script, TypeCastExpression>;

impl NamedSyntaxNode for TypeCastExpressionNode<'_> {
    const NODE_KIND: &'static str = "cast_expr";
}

impl TypeCastExpressionNode<'_> {
    pub fn target_type(&self) -> IdentifierNode {
        self.field_child("type").unwrap().into()
    }

    pub fn value(&self) -> ExpressionNode {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for TypeCastExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeCastExpression")
            .field("type", &self.target_type())
            .field("value", &self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for TypeCastExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for TypeCastExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.value().accept(visitor);
        visitor.visit_type_cast_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct UnaryOperationExpression;

pub type UnaryOperationExpressionNode<'script> = SyntaxNode<'script, UnaryOperationExpression>;

impl NamedSyntaxNode for UnaryOperationExpressionNode<'_> {
    const NODE_KIND: &'static str = "unary_op_expr";
}

impl UnaryOperationExpressionNode<'_> {
    pub fn op(&self) -> UnaryOperatorNode {
        self.field_child("op").unwrap().into()
    }

    pub fn right(&self) -> ExpressionNode {
        self.field_child("right").unwrap().into()
    }
}

impl Debug for UnaryOperationExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnaryOperationExpression")
            .field("op", &self.op())
            .field("right", &self.right())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for UnaryOperationExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for UnaryOperationExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.right().accept(visitor);
        visitor.visit_unary_op_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct BinaryOperationExpression;

pub type BinaryOperationExpressionNode<'script> = SyntaxNode<'script, BinaryOperationExpression>;

impl NamedSyntaxNode for BinaryOperationExpressionNode<'_> {
    const NODE_KIND: &'static str = "binary_op_expr";
}

impl BinaryOperationExpressionNode<'_> {
    pub fn op(&self) -> BinaryOperatorNode {
        self.field_child("op").unwrap().into()
    }

    pub fn left(&self) -> ExpressionNode {
        self.field_child("left").unwrap().into()
    }

    pub fn right(&self) -> ExpressionNode {
        self.field_child("right").unwrap().into()
    }
}

impl Debug for BinaryOperationExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryOperationExpression")
            .field("op", &self.op())
            .field("left", &self.left())
            .field("right", &self.right())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for BinaryOperationExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for BinaryOperationExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.left().accept(visitor);
        self.right().accept(visitor);
        visitor.visit_binary_op_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct AssignmentOperationExpression;

pub type AssignmentOperationExpressionNode<'script> = SyntaxNode<'script, AssignmentOperationExpression>;

impl NamedSyntaxNode for AssignmentOperationExpressionNode<'_> {
    const NODE_KIND: &'static str = "assign_op_expr";
}

impl AssignmentOperationExpressionNode<'_> {
    pub fn op(&self) -> AssignmentOperatorNode {
        self.field_child("op").unwrap().into()
    }

    pub fn left(&self) -> ExpressionNode {
        self.field_child("left").unwrap().into()
    }

    pub fn right(&self) -> ExpressionNode {
        self.field_child("right").unwrap().into()
    }
}

impl Debug for AssignmentOperationExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssignmentOperationExpression")
            .field("op", &self.op())
            .field("left", &self.left())
            .field("right", &self.right())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for AssignmentOperationExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for AssignmentOperationExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.left().accept(visitor);
        self.right().accept(visitor);
        visitor.visit_assign_op_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct TernaryConditionalExpression;

pub type TernaryConditionalExpressionNode<'script> = SyntaxNode<'script, TernaryConditionalExpression>;

impl NamedSyntaxNode for TernaryConditionalExpressionNode<'_> {
    const NODE_KIND: &'static str = "ternary_cond_expr";
}

impl TernaryConditionalExpressionNode<'_> {
    pub fn cond(&self) -> ExpressionNode {
        self.field_child("cond").unwrap().into()
    }

    pub fn conseq(&self) -> ExpressionNode {
        self.field_child("conseq").unwrap().into()
    }

    pub fn alt(&self) -> ExpressionNode {
        self.field_child("alt").unwrap().into()
    }
}

impl Debug for TernaryConditionalExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TernaryConditionalExpression")
            .field("cond", &self.cond())
            .field("conseq", &self.conseq())
            .field("alt", &self.alt())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for TernaryConditionalExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl ExpressionTraversal for TernaryConditionalExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.cond().accept(visitor);
        self.conseq().accept(visitor);
        self.alt().accept(visitor);
        visitor.visit_ternary_cond_expr(self);
    }
}



// Represents the anonymous $._expr node
#[derive(Debug, Clone)]
pub enum Expression<'script> {
    Nested(NestedExpressionNode<'script>),
    Literal(LiteralNode<'script>),
    This(ThisExpressionNode<'script>),
    Super(SuperExpressionNode<'script>),
    Parent(ParentExpressionNode<'script>),
    VirtualParent(VirtualParentExpressionNode<'script>),
    Identifier(IdentifierNode<'script>),
    FunctionCall(FunctionCallExpressionNode<'script>),
    Array(ArrayExpressionNode<'script>),
    MemberField(MemberFieldExpressionNode<'script>),
    MethodCall(MethodCallExpressionNode<'script>),
    Instantiation(InstantiationExpressionNode<'script>),
    TypeCast(TypeCastExpressionNode<'script>),
    UnaryOperation(UnaryOperationExpressionNode<'script>),
    BinaryOperation(BinaryOperationExpressionNode<'script>),
    AssignmentOperation(AssignmentOperationExpressionNode<'script>),
    TernaryConditional(TernaryConditionalExpressionNode<'script>),
}

pub type ExpressionNode<'script> = SyntaxNode<'script, Expression<'script>>;

impl<'script> ExpressionNode<'script> {
    pub fn value(self) -> Expression<'script> {
        match self.tree_node.kind() {
            AssignmentOperationExpressionNode::NODE_KIND => Expression::AssignmentOperation(self.into()),
            TernaryConditionalExpressionNode::NODE_KIND => Expression::TernaryConditional(self.into()),
            BinaryOperationExpressionNode::NODE_KIND => Expression::BinaryOperation(self.into()),
            InstantiationExpressionNode::NODE_KIND => Expression::Instantiation(self.into()),
            UnaryOperationExpressionNode::NODE_KIND => Expression::UnaryOperation(self.into()),
            TypeCastExpressionNode::NODE_KIND => Expression::TypeCast(self.into()),
            MethodCallExpressionNode::NODE_KIND => Expression::MethodCall(self.into()),
            MemberFieldExpressionNode::NODE_KIND => Expression::MemberField(self.into()),
            FunctionCallExpressionNode::NODE_KIND => Expression::FunctionCall(self.into()),
            ArrayExpressionNode::NODE_KIND => Expression::Array(self.into()),
            NestedExpressionNode::NODE_KIND => Expression::Nested(self.into()),
            ThisExpressionNode::NODE_KIND => Expression::This(self.into()),
            SuperExpressionNode::NODE_KIND => Expression::Super(self.into()),
            ParentExpressionNode::NODE_KIND => Expression::Parent(self.into()),
            VirtualParentExpressionNode::NODE_KIND => Expression::VirtualParent(self.into()),
            IdentifierNode::NODE_KIND => Expression::Identifier(self.into()),
            LiteralNode::NODE_KIND => Expression::Literal(self.into()),
            _ => panic!("Unknown expression type: {}", self.tree_node.kind())
        }
    }
}

impl Debug for ExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.clone().value())
        } else {
            write!(f, "{:?}", self.clone().value())
        }
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        match value.tree_node.kind() {
            AssignmentOperationExpressionNode::NODE_KIND    |
            TernaryConditionalExpressionNode::NODE_KIND     |
            BinaryOperationExpressionNode::NODE_KIND        |
            InstantiationExpressionNode::NODE_KIND          |
            UnaryOperationExpressionNode::NODE_KIND         |
            TypeCastExpressionNode::NODE_KIND               |
            MethodCallExpressionNode::NODE_KIND             |
            MemberFieldExpressionNode::NODE_KIND            |
            FunctionCallExpressionNode::NODE_KIND           |
            ArrayExpressionNode::NODE_KIND                  |
            NestedExpressionNode::NODE_KIND                 |
            ThisExpressionNode::NODE_KIND                   |
            SuperExpressionNode::NODE_KIND                  |
            ParentExpressionNode::NODE_KIND                 |
            VirtualParentExpressionNode::NODE_KIND          |
            IdentifierNode::NODE_KIND                       |
            LiteralNode::NODE_KIND                          => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl ExpressionTraversal for ExpressionNode<'_> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        match self.clone().value() {
            Expression::Nested(n) => n.accept(visitor),
            Expression::Literal(n) => n.accept(visitor),
            Expression::This(n) => n.accept(visitor),
            Expression::Super(n) => n.accept(visitor),
            Expression::Parent(n) => n.accept(visitor),
            Expression::VirtualParent(n) => n.accept(visitor),
            Expression::Identifier(n) => n.accept(visitor),
            Expression::FunctionCall(n) => n.accept(visitor),
            Expression::Array(n) => n.accept(visitor),
            Expression::MemberField(n) => n.accept(visitor),
            Expression::MethodCall(n) => n.accept(visitor),
            Expression::Instantiation(n) => n.accept(visitor),
            Expression::TypeCast(n) => n.accept(visitor),
            Expression::UnaryOperation(n) => n.accept(visitor),
            Expression::BinaryOperation(n) => n.accept(visitor),
            Expression::AssignmentOperation(n) => n.accept(visitor),
            Expression::TernaryConditional(n) => n.accept(visitor),
        }
    }
}



#[derive(Debug, Clone)]
pub struct ExpressionStatement;

pub type ExpressionStatementNode<'script> = SyntaxNode<'script, ExpressionStatement>;

impl NamedSyntaxNode for ExpressionStatementNode<'_> {
    const NODE_KIND: &'static str = "expr_stmt";
}

impl ExpressionStatementNode<'_> {
    pub fn expr(&self) -> ExpressionNode {
        self.first_child(true).unwrap().into()
    }
}

impl Debug for ExpressionStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ExpressionStatement")
            .field(&self.expr())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ExpressionStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for ExpressionStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_expr_stmt(self);
    }
}