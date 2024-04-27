use std::fmt::Debug;
use lsp_types as lsp;
use crate::{tokens::*, AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct NestedExpression;
    pub struct ThisExpression;
    pub struct SuperExpression;
    pub struct ParentExpression;
    pub struct VirtualParentExpression;
    pub struct FunctionCallExpression;
    pub struct FunctionCallArguments;
    pub struct ArrayExpression;
    pub struct MemberFieldExpression;
    pub struct NewExpression;
    pub struct TypeCastExpression;
    pub struct UnaryOperationExpression;
    pub struct BinaryOperationExpression;
    pub struct AssignmentOperationExpression;
    pub struct TernaryConditionalExpression;
    pub struct ExpressionStatement;
}


pub type NestedExpressionNode<'script> = SyntaxNode<'script, tags::NestedExpression>;

impl NamedSyntaxNode for NestedExpressionNode<'_> {
    const NODE_KIND: &'static str = "nested_expr";
}

impl<'script> NestedExpressionNode<'script> {
    pub fn inner(&self) -> ExpressionNode<'script> {
        self.first_child(true).unwrap().into()
    }
}

impl Debug for NestedExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("NestedExpression {}", self.range().debug()))
            .field(&self.inner())
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

impl SyntaxNodeTraversal for NestedExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_nested_expr(self, ctx);
        if tp.traverse_inner {
            self.inner().accept(visitor, ExpressionTraversalContext::NestedExpressionInner);
        }
        visitor.exit_nested_expr(self, ctx);
    }
}



pub type ThisExpressionNode<'script> = SyntaxNode<'script, tags::ThisExpression>;

impl NamedSyntaxNode for ThisExpressionNode<'_> {
    const NODE_KIND: &'static str = "this_expr";
}

impl ThisExpressionNode<'_> {}

impl Debug for ThisExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThisExpression {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ThisExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ThisExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        visitor.visit_this_expr(self, ctx);
    }
}



pub type SuperExpressionNode<'script> = SyntaxNode<'script, tags::SuperExpression>;

impl NamedSyntaxNode for SuperExpressionNode<'_> {
    const NODE_KIND: &'static str = "super_expr";
}

impl SuperExpressionNode<'_> {}

impl Debug for SuperExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperExpression {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SuperExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for SuperExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        visitor.visit_super_expr(self, ctx);
    }
}



pub type ParentExpressionNode<'script> = SyntaxNode<'script, tags::ParentExpression>;

impl NamedSyntaxNode for ParentExpressionNode<'_> {
    const NODE_KIND: &'static str = "parent_expr";
}

impl ParentExpressionNode<'_> {}

impl Debug for ParentExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParentExpression {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ParentExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ParentExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        visitor.visit_parent_expr(self, ctx);
    }
}



pub type VirtualParentExpressionNode<'script> = SyntaxNode<'script, tags::VirtualParentExpression>;

impl NamedSyntaxNode for VirtualParentExpressionNode<'_> {
    const NODE_KIND: &'static str = "virtual_parent_expr";
}

impl VirtualParentExpressionNode<'_> {}

impl Debug for VirtualParentExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VirtualParentExpression {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for VirtualParentExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for VirtualParentExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        visitor.visit_virtual_parent_expr(self, ctx);
    }
}



pub type FunctionCallExpressionNode<'script> = SyntaxNode<'script, tags::FunctionCallExpression>;

impl NamedSyntaxNode for FunctionCallExpressionNode<'_> {
    const NODE_KIND: &'static str = "func_call_expr";
}

impl<'script> FunctionCallExpressionNode<'script> {
    pub fn func(&self) -> ExpressionNode<'script> {
        self.field_child("func").unwrap().into()
    }

    pub fn args(&self) -> Option<FunctionCallArgumentsNode<'script>> {
        self.field_child("args").map(|n| n.into())
    }
}

impl Debug for FunctionCallExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("FunctionCallExpression {}", self.range().debug()))
            .field("func", &self.func())
            .field("args", &self.args())
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

impl SyntaxNodeTraversal for FunctionCallExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_func_call_expr(self, ctx);
        if tp.traverse_func {
            self.func().accept(visitor, ExpressionTraversalContext::FunctionCallExpressionFunc);
        }
        if tp.traverse_args {
            self.args().map(|n| n.accept(visitor, ()));
        }
        visitor.exit_func_call_expr(self, ctx);
    }
}



pub type FunctionCallArgumentsNode<'script> = SyntaxNode<'script, tags::FunctionCallArguments>;

impl NamedSyntaxNode for FunctionCallArgumentsNode<'_> {
    const NODE_KIND: &'static str = "func_call_args";
}

impl<'script> FunctionCallArgumentsNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = FunctionCallArgument<'script>> {
        let children = self.children();

        let mut args = Vec::new();
        let mut previous_was_comma = true;
        for n in children {
            if n.tree_node.is_named() {
                args.push(FunctionCallArgument::Some(n.into()));
                previous_was_comma = false;
            } else {
                if previous_was_comma {
                    args.push(FunctionCallArgument::Omitted(n.range()));
                }
                previous_was_comma = true;
            }
        }

        args.into_iter()
    }
}

impl Debug for FunctionCallArgumentsNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("FunctionCallArguments {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionCallArgumentsNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for FunctionCallArgumentsNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        self.iter().for_each(|n| n.accept(visitor, ()))
    }
}


#[derive(Clone)]
pub enum FunctionCallArgument<'script> {
    Some(ExpressionNode<'script>),
    Omitted(lsp::Range)
}

impl FunctionCallArgument<'_> {
    #[inline]
    pub fn range(&self) -> lsp::Range {
        match self {
            FunctionCallArgument::Some(n) => n.range(),
            FunctionCallArgument::Omitted(r) => *r,
        }
    }

    #[inline]
    pub fn spans_position(&self, position: lsp::Position) -> bool {
        let r = self.range();
        if r.start.line < r.end.line {
            r.start.line <= position.line && position.line <= r.end.line
        } else {
            r.start.character <= position.character && position.character <= r.end.character
        }
    }
}

impl Debug for FunctionCallArgument<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(n) => f.debug_maybe_alternate(n),
            Self::Omitted(pos) => write!(f, "Omitted {}", pos.debug()),
        }
    }
}

impl SyntaxNodeTraversal for FunctionCallArgument<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        let tp = visitor.visit_func_call_arg(self);
        if tp.traverse_expr {
            if let FunctionCallArgument::Some(n) = self {
                n.accept(visitor, ExpressionTraversalContext::FunctionCallArg);
            }
        }
        visitor.exit_func_call_arg(self);
    }
}



pub type ArrayExpressionNode<'script> = SyntaxNode<'script, tags::ArrayExpression>;

impl NamedSyntaxNode for ArrayExpressionNode<'_> {
    const NODE_KIND: &'static str = "array_expr";
}

impl<'script> ArrayExpressionNode<'script> {
    pub fn accessor(&self) -> ExpressionNode<'script> {
        self.field_child("accessor").unwrap().into()
    }

    pub fn index(&self) -> ExpressionNode<'script> {
        self.field_child("index").unwrap().into()
    }
}

impl Debug for ArrayExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("ArrayExpression {}", self.range().debug()))
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

impl SyntaxNodeTraversal for ArrayExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_array_expr(self, ctx);
        if tp.traverse_accessor {
            self.accessor().accept(visitor, ExpressionTraversalContext::ArrayExpressionAccessor);
        }
        if tp.traverse_index {
            self.index().accept(visitor, ExpressionTraversalContext::ArrayExpressionIndex);
        }
        visitor.exit_array_expr(self, ctx);
    }
}



pub type MemberFieldExpressionNode<'script> = SyntaxNode<'script, tags::MemberFieldExpression>;

impl NamedSyntaxNode for MemberFieldExpressionNode<'_> {
    const NODE_KIND: &'static str = "member_field_expr";
}

impl<'script> MemberFieldExpressionNode<'script> {
    pub fn accessor(&self) -> ExpressionNode<'script> {
        self.field_child("accessor").unwrap().into()
    }

    pub fn member(&self) -> IdentifierNode<'script> {
        self.field_child("member").unwrap().into()
    }
}

impl Debug for MemberFieldExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("MemberFieldExpression {}", self.range().debug()))
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

impl SyntaxNodeTraversal for MemberFieldExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_member_field_expr(self, ctx);
        if tp.traverse_accessor {
            self.accessor().accept(visitor, ExpressionTraversalContext::MemberFieldExpressionAccessor);
        }
        visitor.exit_member_field_expr(self, ctx);
    }
}



pub type NewExpressionNode<'script> = SyntaxNode<'script, tags::NewExpression>;

impl NamedSyntaxNode for NewExpressionNode<'_> {
    const NODE_KIND: &'static str = "new_expr";
}

impl<'script> NewExpressionNode<'script> {
    pub fn class(&self) -> IdentifierNode<'script> {
        self.field_child("class").unwrap().into()
    }

    pub fn lifetime_obj(&self) -> Option<ExpressionNode<'script>> {
        self.field_child("lifetime_obj").map(|n| n.into())
    }
}

impl Debug for NewExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("NewExpression {}", self.range().debug()))
            .field("class", &self.class())
            .field("lifetime_obj", &self.lifetime_obj())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for NewExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for NewExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_new_expr(self, ctx);
        if tp.traverse_lifetime_obj {
            self.lifetime_obj().map(|n| n.accept(visitor, ExpressionTraversalContext::NewExpressionLifetimeObj));
        }
        visitor.exit_new_expr(self, ctx);
    }
}



pub type TypeCastExpressionNode<'script> = SyntaxNode<'script, tags::TypeCastExpression>;

impl NamedSyntaxNode for TypeCastExpressionNode<'_> {
    const NODE_KIND: &'static str = "cast_expr";
}

impl<'script> TypeCastExpressionNode<'script> {
    pub fn target_type(&self) -> IdentifierNode<'script> {
        self.field_child("type").unwrap().into()
    }

    pub fn value(&self) -> ExpressionNode<'script> {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for TypeCastExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("TypeCastExpression {}", self.range().debug()))
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

impl SyntaxNodeTraversal for TypeCastExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_type_cast_expr(self, ctx);
        if tp.traverse_value {
            self.value().accept(visitor, ExpressionTraversalContext::TypeCastExpressionValue);
        }
        visitor.exit_type_cast_expr(self, ctx);
    }
}



pub type UnaryOperationExpressionNode<'script> = SyntaxNode<'script, tags::UnaryOperationExpression>;

impl NamedSyntaxNode for UnaryOperationExpressionNode<'_> {
    const NODE_KIND: &'static str = "unary_op_expr";
}

impl<'script> UnaryOperationExpressionNode<'script> {
    pub fn op(&self) -> UnaryOperatorNode<'script> {
        self.field_child("op").unwrap().into()
    }

    pub fn right(&self) -> ExpressionNode<'script> {
        self.field_child("right").unwrap().into()
    }
}

impl Debug for UnaryOperationExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("UnaryOperationExpression {}", self.range().debug()))
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

impl SyntaxNodeTraversal for UnaryOperationExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_unary_op_expr(self, ctx);
        if tp.traverse_right {
            self.right().accept(visitor, ExpressionTraversalContext::UnaryOperationExpressionRight);
        }
        visitor.exit_unary_op_expr(self, ctx);
    }
}



pub type BinaryOperationExpressionNode<'script> = SyntaxNode<'script, tags::BinaryOperationExpression>;

impl NamedSyntaxNode for BinaryOperationExpressionNode<'_> {
    const NODE_KIND: &'static str = "binary_op_expr";
}

impl<'script> BinaryOperationExpressionNode<'script> {
    pub fn op(&self) -> BinaryOperatorNode<'script> {
        self.field_child("op").unwrap().into()
    }

    pub fn left(&self) -> ExpressionNode<'script> {
        self.field_child("left").unwrap().into()
    }

    pub fn right(&self) -> ExpressionNode<'script> {
        self.field_child("right").unwrap().into()
    }
}

impl Debug for BinaryOperationExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("BinaryOperationExpression {}", self.range().debug()))
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

impl SyntaxNodeTraversal for BinaryOperationExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_binary_op_expr(self, ctx);
        if tp.traverse_left {
            self.left().accept(visitor, ExpressionTraversalContext::BinaryOperationExpressionLeft);
        }
        if tp.traverse_right {
            self.right().accept(visitor, ExpressionTraversalContext::BinaryOperationExpressionRight);
        }
        visitor.exit_binary_op_expr(self, ctx);
    }
}



pub type AssignmentOperationExpressionNode<'script> = SyntaxNode<'script, tags::AssignmentOperationExpression>;

impl NamedSyntaxNode for AssignmentOperationExpressionNode<'_> {
    const NODE_KIND: &'static str = "assign_op_expr";
}

impl<'script> AssignmentOperationExpressionNode<'script> {
    pub fn op(&self) -> AssignmentOperatorNode<'script> {
        self.field_child("op").unwrap().into()
    }

    pub fn left(&self) -> ExpressionNode<'script> {
        self.field_child("left").unwrap().into()
    }

    pub fn right(&self) -> ExpressionNode<'script> {
        self.field_child("right").unwrap().into()
    }
}

impl Debug for AssignmentOperationExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("AssignmentOperationExpression {}", self.range().debug()))
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

impl SyntaxNodeTraversal for AssignmentOperationExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_assign_op_expr(self, ctx);
        if tp.traverse_left {
            self.left().accept(visitor, ExpressionTraversalContext::AssignmentOperationExpressionLeft);
        }
        if tp.traverse_right {
            self.right().accept(visitor, ExpressionTraversalContext::AssignmentOperationExpressionRight);
        }
        visitor.exit_assign_op_expr(self, ctx);
    }
}



pub type TernaryConditionalExpressionNode<'script> = SyntaxNode<'script, tags::TernaryConditionalExpression>;

impl NamedSyntaxNode for TernaryConditionalExpressionNode<'_> {
    const NODE_KIND: &'static str = "ternary_cond_expr";
}

impl<'script> TernaryConditionalExpressionNode<'script> {
    pub fn cond(&self) -> ExpressionNode<'script> {
        self.field_child("cond").unwrap().into()
    }

    pub fn conseq(&self) -> ExpressionNode<'script> {
        self.field_child("conseq").unwrap().into()
    }

    pub fn alt(&self) -> ExpressionNode<'script> {
        self.field_child("alt").unwrap().into()
    }
}

impl Debug for TernaryConditionalExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("TernaryConditionalExpression {}", self.range().debug()))
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

impl SyntaxNodeTraversal for TernaryConditionalExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_ternary_cond_expr(self, ctx);
        if tp.traverse_cond {
            self.cond().accept(visitor, ExpressionTraversalContext::TernaryConditionalExpressionCond);
        }
        if tp.traverse_conseq {
            self.conseq().accept(visitor, ExpressionTraversalContext::TernaryConditionalExpressionConseq);
        }
        if tp.traverse_alt {
            self.alt().accept(visitor, ExpressionTraversalContext::TernaryConditionalExpressionAlt);
        }
        visitor.exit_ternary_cond_expr(self, ctx);
    }
}



// Represents the unnamed $._expr node
#[derive(Clone)]
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
    New(NewExpressionNode<'script>),
    TypeCast(TypeCastExpressionNode<'script>),
    UnaryOperation(UnaryOperationExpressionNode<'script>),
    BinaryOperation(BinaryOperationExpressionNode<'script>),
    AssignmentOperation(AssignmentOperationExpressionNode<'script>),
    TernaryConditional(TernaryConditionalExpressionNode<'script>),
}

impl Debug for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nested(n) => f.debug_maybe_alternate(n),
            Self::Literal(n) => f.debug_maybe_alternate(n),
            Self::This(n) => f.debug_maybe_alternate(n),
            Self::Super(n) => f.debug_maybe_alternate(n),
            Self::Parent(n) => f.debug_maybe_alternate(n),
            Self::VirtualParent(n) => f.debug_maybe_alternate(n),
            Self::Identifier(n) => f.debug_maybe_alternate(n),
            Self::FunctionCall(n) => f.debug_maybe_alternate(n),
            Self::Array(n) => f.debug_maybe_alternate(n),
            Self::MemberField(n) => f.debug_maybe_alternate(n),
            Self::New(n) => f.debug_maybe_alternate(n),
            Self::TypeCast(n) => f.debug_maybe_alternate(n),
            Self::UnaryOperation(n) => f.debug_maybe_alternate(n),
            Self::BinaryOperation(n) => f.debug_maybe_alternate(n),
            Self::AssignmentOperation(n) => f.debug_maybe_alternate(n),
            Self::TernaryConditional(n) => f.debug_maybe_alternate(n),
        }
    }
}

pub type ExpressionNode<'script> = SyntaxNode<'script, Expression<'script>>;

impl<'script> ExpressionNode<'script> {
    pub fn value(self) -> Expression<'script> {
        match self.tree_node.kind() {
            AssignmentOperationExpressionNode::NODE_KIND => Expression::AssignmentOperation(self.into()),
            TernaryConditionalExpressionNode::NODE_KIND => Expression::TernaryConditional(self.into()),
            BinaryOperationExpressionNode::NODE_KIND => Expression::BinaryOperation(self.into()),
            NewExpressionNode::NODE_KIND => Expression::New(self.into()),
            UnaryOperationExpressionNode::NODE_KIND => Expression::UnaryOperation(self.into()),
            TypeCastExpressionNode::NODE_KIND => Expression::TypeCast(self.into()),
            MemberFieldExpressionNode::NODE_KIND => Expression::MemberField(self.into()),
            FunctionCallExpressionNode::NODE_KIND => Expression::FunctionCall(self.into()),
            ArrayExpressionNode::NODE_KIND => Expression::Array(self.into()),
            NestedExpressionNode::NODE_KIND => Expression::Nested(self.into()),
            ThisExpressionNode::NODE_KIND => Expression::This(self.into()),
            SuperExpressionNode::NODE_KIND => Expression::Super(self.into()),
            ParentExpressionNode::NODE_KIND => Expression::Parent(self.into()),
            VirtualParentExpressionNode::NODE_KIND => Expression::VirtualParent(self.into()),
            IdentifierNode::NODE_KIND => Expression::Identifier(self.into()),
            LiteralIntNode::NODE_KIND       |
            LiteralHexNode::NODE_KIND       |
            LiteralFloatNode::NODE_KIND     |
            LiteralBoolNode::NODE_KIND      |
            LiteralStringNode::NODE_KIND    |
            LiteralNameNode::NODE_KIND      |
            LiteralNullNode::NODE_KIND      => Expression::Literal(self.into()),
            _ => panic!("Unknown expression type: {} {}", self.tree_node.kind(), self.range().debug())
        }
    }
}

impl Debug for ExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.clone().value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() {
            return Err(());
        }

        match value.tree_node.kind() {
            AssignmentOperationExpressionNode::NODE_KIND    |
            TernaryConditionalExpressionNode::NODE_KIND     |
            BinaryOperationExpressionNode::NODE_KIND        |
            NewExpressionNode::NODE_KIND          |
            UnaryOperationExpressionNode::NODE_KIND         |
            TypeCastExpressionNode::NODE_KIND               |
            MemberFieldExpressionNode::NODE_KIND            |
            FunctionCallExpressionNode::NODE_KIND           |
            ArrayExpressionNode::NODE_KIND                  |
            NestedExpressionNode::NODE_KIND                 |
            ThisExpressionNode::NODE_KIND                   |
            SuperExpressionNode::NODE_KIND                  |
            ParentExpressionNode::NODE_KIND                 |
            VirtualParentExpressionNode::NODE_KIND          |
            IdentifierNode::NODE_KIND                       |
            LiteralIntNode::NODE_KIND                       |
            LiteralHexNode::NODE_KIND                       |
            LiteralFloatNode::NODE_KIND                     |
            LiteralBoolNode::NODE_KIND                      |
            LiteralStringNode::NODE_KIND                    |
            LiteralNameNode::NODE_KIND                      |
            LiteralNullNode::NODE_KIND                       => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl SyntaxNodeTraversal for ExpressionNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        match self.clone().value() {
            Expression::Nested(n) => n.accept(visitor, ctx),
            Expression::Literal(n) => n.accept(visitor, ctx),
            Expression::This(n) => n.accept(visitor, ctx),
            Expression::Super(n) => n.accept(visitor, ctx),
            Expression::Parent(n) => n.accept(visitor, ctx),
            Expression::VirtualParent(n) => n.accept(visitor, ctx),
            Expression::Identifier(n) => n.accept(visitor, ctx),
            Expression::FunctionCall(n) => n.accept(visitor, ctx),
            Expression::Array(n) => n.accept(visitor, ctx),
            Expression::MemberField(n) => n.accept(visitor, ctx),
            Expression::New(n) => n.accept(visitor, ctx),
            Expression::TypeCast(n) => n.accept(visitor, ctx),
            Expression::UnaryOperation(n) => n.accept(visitor, ctx),
            Expression::BinaryOperation(n) => n.accept(visitor, ctx),
            Expression::AssignmentOperation(n) => n.accept(visitor, ctx),
            Expression::TernaryConditional(n) => n.accept(visitor, ctx),
        }
    }
}



pub type ExpressionStatementNode<'script> = SyntaxNode<'script, tags::ExpressionStatement>;

impl NamedSyntaxNode for ExpressionStatementNode<'_> {
    const NODE_KIND: &'static str = "expr_stmt";
}

impl<'script> ExpressionStatementNode<'script> {
    pub fn expr(&self) -> ExpressionNode<'script> {
        self.first_child(true).unwrap().into()
    }
}

impl Debug for ExpressionStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("ExpressionStatement {}", self.range().debug()))
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

impl SyntaxNodeTraversal for ExpressionStatementNode<'_> {
    type TraversalCtx = StatementTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        let tp = visitor.visit_expr_stmt(self, ctx);
        if tp.traverse_expr {
            self.expr().accept(visitor, ExpressionTraversalContext::ExpressionStatement);
        }
        visitor.exit_expr_stmt(self, ctx);
    }
}