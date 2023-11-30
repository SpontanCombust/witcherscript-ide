use std::fmt::Debug;
use crate::{SyntaxNode, NamedSyntaxNode, tokens::*};
use super::{StatementTraversal, ExpressionVisitor, ExpressionTraversal, StatementVisitor};


#[derive(Debug, Clone)]
pub struct NestedExpression;

impl NamedSyntaxNode for NestedExpression {
    const NODE_NAME: &'static str = "nested_expr";
}

impl SyntaxNode<'_, NestedExpression> {
    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.first_child(true).unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, NestedExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NestedExpression")
            .field(&self.value())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, NestedExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.value().accept(visitor);
        visitor.visit_nested_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct ThisExpression;

impl NamedSyntaxNode for ThisExpression {
    const NODE_NAME: &'static str = "this_expr";
}

impl SyntaxNode<'_, ThisExpression> {}

impl Debug for SyntaxNode<'_, ThisExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThisExpression")
    }
}

impl ExpressionTraversal for SyntaxNode<'_, ThisExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_this_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct SuperExpression;

impl NamedSyntaxNode for SuperExpression {
    const NODE_NAME: &'static str = "super_expr";
}

impl SyntaxNode<'_, SuperExpression> {}

impl Debug for SyntaxNode<'_, SuperExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperExpression")
    }
}

impl ExpressionTraversal for SyntaxNode<'_, SuperExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_super_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct ParentExpression;

impl NamedSyntaxNode for ParentExpression {
    const NODE_NAME: &'static str = "parent_expr";
}

impl SyntaxNode<'_, ParentExpression> {}

impl Debug for SyntaxNode<'_, ParentExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParentExpression")
    }
}

impl ExpressionTraversal for SyntaxNode<'_, ParentExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_parent_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct VirtualParentExpression;

impl NamedSyntaxNode for VirtualParentExpression {
    const NODE_NAME: &'static str = "virtual_parent_expr";
}

impl SyntaxNode<'_, VirtualParentExpression> {}

impl Debug for SyntaxNode<'_, VirtualParentExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VirtualParentExpression")
    }
}

impl ExpressionTraversal for SyntaxNode<'_, VirtualParentExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        visitor.visit_virtual_parent_expr(self);
    }
}




#[derive(Debug, Clone)]
pub struct FunctionCallExpression;

impl NamedSyntaxNode for FunctionCallExpression {
    const NODE_NAME: &'static str = "func_call_expr";
}

impl SyntaxNode<'_, FunctionCallExpression> {
    pub fn func(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("func").unwrap().into()
    }

    pub fn args(&self) -> impl Iterator<Item = FuncCallArg<'_>> {
        func_args(self)
    }
}

impl Debug for SyntaxNode<'_, FunctionCallExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionCallExpression")
            .field("func", &self.func())
            .field("args", &self.args().collect::<Vec<_>>())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, FunctionCallExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.args().for_each(|arg| arg.accept(visitor));
        visitor.visit_func_call_expr(self);
    }
}


type FuncCallArg<'script> = Option<SyntaxNode<'script, Expression<'script>>>;

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
            // spacial care into handling commas.
            // If a node is named, some argument was passed. If a node is anonymous it is a comma.
            // If we encounter a comma right after a previous comma, we have a defaulted argument.
            if !n.is_error() {
                if n.is_named() {
                    v.push(Some(func_node.clone().replace_node(n).into()));
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

impl NamedSyntaxNode for ArrayExpression {
    const NODE_NAME: &'static str = "array_expr";
}

impl SyntaxNode<'_, ArrayExpression> {
    pub fn accessor(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("accessor").unwrap().into()
    }

    pub fn index(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("index").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, ArrayExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayExpression")
            .field("accessor", &self.accessor())
            .field("index", &self.index())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, ArrayExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.accessor().accept(visitor);
        self.index().accept(visitor);
        visitor.visit_array_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct MemberFieldExpression;

impl NamedSyntaxNode for MemberFieldExpression {
    const NODE_NAME: &'static str = "member_field_expr";
}

impl SyntaxNode<'_, MemberFieldExpression> {
    pub fn accessor(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("accessor").unwrap().into()
    }

    pub fn member(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("member").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, MemberFieldExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberFieldExpression")
            .field("accessor", &self.accessor())
            .field("member", &self.member())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, MemberFieldExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.accessor().accept(visitor);
        visitor.visit_member_field_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct MethodCallExpression;

impl NamedSyntaxNode for MethodCallExpression {
    const NODE_NAME: &'static str = "member_func_call_expr";
}

impl SyntaxNode<'_, MethodCallExpression> {
    pub fn accessor(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("accessor").unwrap().into()
    }

    pub fn func(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("func").unwrap().into()
    }

    pub fn args(&self) -> impl Iterator<Item = Option<SyntaxNode<'_, Expression>>> {
        func_args(self)
    }
}

impl Debug for SyntaxNode<'_, MethodCallExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionCallExpression")
            .field("accessor", &self.accessor())
            .field("func", &self.func())
            .field("args", &self.args().collect::<Vec<_>>())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, MethodCallExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.accessor().accept(visitor);
        self.args().for_each(|arg| arg.accept(visitor));
        visitor.visit_method_call_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct InstantiationExpression;

impl NamedSyntaxNode for InstantiationExpression {
    const NODE_NAME: &'static str = "new_expr";
}

impl SyntaxNode<'_, InstantiationExpression> {
    pub fn class(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("class").unwrap().into()
    }

    pub fn lifetime_obj(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("lifetime_obj").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, InstantiationExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstantiationExpression")
            .field("class", &self.class())
            .field("lifetime_obj", &self.lifetime_obj())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, InstantiationExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.lifetime_obj().accept(visitor);
        visitor.visit_instantiation_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct TypeCastExpression;

impl NamedSyntaxNode for TypeCastExpression {
    const NODE_NAME: &'static str = "cast_expr";
}

impl SyntaxNode<'_, TypeCastExpression> {
    pub fn target_type(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("type").unwrap().into()
    }

    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, TypeCastExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeCastExpression")
            .field("type", &self.target_type())
            .field("value", &self.value())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, TypeCastExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.value().accept(visitor);
        visitor.visit_type_cast_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct UnaryOperationExpression;

impl NamedSyntaxNode for UnaryOperationExpression {
    const NODE_NAME: &'static str = "unary_op_expr";
}

impl SyntaxNode<'_, UnaryOperationExpression> {
    pub fn op(&self) -> SyntaxNode<'_, UnaryOperator> {
        self.field_child("op").unwrap().into()
    }

    pub fn right(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("right").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, UnaryOperationExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnaryOperationExpression")
            .field("op", &self.op())
            .field("right", &self.right())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, UnaryOperationExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.right().accept(visitor);
        visitor.visit_unary_op_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct BinaryOperationExpression;

impl NamedSyntaxNode for BinaryOperationExpression {
    const NODE_NAME: &'static str = "binary_op_expr";
}

impl SyntaxNode<'_, BinaryOperationExpression> {
    pub fn op(&self) -> SyntaxNode<'_, BinaryOperator> {
        self.field_child("op").unwrap().into()
    }

    pub fn left(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("left").unwrap().into()
    }

    pub fn right(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("right").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, BinaryOperationExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryOperationExpression")
            .field("op", &self.op())
            .field("left", &self.left())
            .field("right", &self.right())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, BinaryOperationExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.left().accept(visitor);
        self.right().accept(visitor);
        visitor.visit_binary_op_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct AssignmentOperationExpression;

impl NamedSyntaxNode for AssignmentOperationExpression {
    const NODE_NAME: &'static str = "assign_op_expr";
}

impl SyntaxNode<'_, AssignmentOperationExpression> {
    pub fn op(&self) -> SyntaxNode<'_, AssignmentOperator> {
        self.field_child("op").unwrap().into()
    }

    pub fn left(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("left").unwrap().into()
    }

    pub fn right(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("right").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, AssignmentOperationExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssignmentOperationExpression")
            .field("op", &self.op())
            .field("left", &self.left())
            .field("right", &self.right())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, AssignmentOperationExpression> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        self.left().accept(visitor);
        self.right().accept(visitor);
        visitor.visit_assign_op_expr(self);
    }
}



#[derive(Debug, Clone)]
pub struct TernaryConditionalExpression;

impl NamedSyntaxNode for TernaryConditionalExpression {
    const NODE_NAME: &'static str = "ternary_cond_expr";
}

impl SyntaxNode<'_, TernaryConditionalExpression> {
    pub fn cond(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("cond").unwrap().into()
    }

    pub fn conseq(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("conseq").unwrap().into()
    }

    pub fn alt(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("alt").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, TernaryConditionalExpression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TernaryConditionalExpression")
            .field("cond", &self.cond())
            .field("conseq", &self.conseq())
            .field("alt", &self.alt())
            .finish()
    }
}

impl ExpressionTraversal for SyntaxNode<'_, TernaryConditionalExpression> {
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
    Nested(SyntaxNode<'script, NestedExpression>),
    Literal(SyntaxNode<'script, Literal<'script>>),
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
            AssignmentOperationExpression::NODE_NAME => Expression::AssignmentOperation(self.clone().into()),
            TernaryConditionalExpression::NODE_NAME => Expression::TernaryConditional(self.clone().into()),
            BinaryOperationExpression::NODE_NAME => Expression::BinaryOperation(self.clone().into()),
            InstantiationExpression::NODE_NAME => Expression::Instantiation(self.clone().into()),
            UnaryOperationExpression::NODE_NAME => Expression::UnaryOperation(self.clone().into()),
            TypeCastExpression::NODE_NAME => Expression::TypeCast(self.clone().into()),
            MethodCallExpression::NODE_NAME => Expression::MethodCall(self.clone().into()),
            MemberFieldExpression::NODE_NAME => Expression::MemberField(self.clone().into()),
            FunctionCallExpression::NODE_NAME => Expression::FunctionCall(self.clone().into()),
            ArrayExpression::NODE_NAME => Expression::Array(self.clone().into()),
            NestedExpression::NODE_NAME => Expression::Nested(self.clone().into()),
            ThisExpression::NODE_NAME => Expression::This(self.clone().into()),
            SuperExpression::NODE_NAME => Expression::Super(self.clone().into()),
            ParentExpression::NODE_NAME => Expression::Parent(self.clone().into()),
            VirtualParentExpression::NODE_NAME => Expression::VirtualParent(self.clone().into()),
            Identifier::NODE_NAME => Expression::Identifier(self.clone().into()),
            Literal::NODE_NAME => Expression::Literal(self.clone().into()),
            _ => panic!("Unknown expression type: {}", self.tree_node.kind())
        }
    }
}

impl Debug for SyntaxNode<'_, Expression<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}

impl ExpressionTraversal for SyntaxNode<'_, Expression<'_>> {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) {
        match self.value() {
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

impl NamedSyntaxNode for ExpressionStatement {
    const NODE_NAME: &'static str = "expr_stmt";
}

impl SyntaxNode<'_, ExpressionStatement> {
    pub fn expr(&self) -> SyntaxNode<'_, Expression<'_>> {
        self.first_child(true).unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, ExpressionStatement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ExpressionStatement")
            .field(&self.expr())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, ExpressionStatement> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_expr_stmt(self);
    }
}