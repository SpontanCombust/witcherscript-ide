use std::fmt::Debug;
use lsp_types as lsp;
use crate::{tokens::*, AnyNode, DebugMaybeAlternate, DebugRange, ErrorNode, NamedSyntaxNode, SyntaxNode};
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
    pub struct MemberAccessExpression;
    pub struct NewExpression;
    pub struct TypeCastExpression;
    pub struct UnaryOperationExpression;
    pub struct BinaryOperationExpression;
    pub struct AssignmentOperationExpression;
    pub struct TernaryConditionalExpression;
    pub struct ArrayInitializerExpression;
    pub struct ExpressionStatement;
}


pub type NestedExpressionNode<'script> = SyntaxNode<'script, tags::NestedExpression>;

impl NamedSyntaxNode for NestedExpressionNode<'_> {
    const NODE_KIND: &'static str = "nested_expr";
}

impl<'script> NestedExpressionNode<'script> {
    pub fn inner(&self) -> ExpressionNode<'script> {
        self.first_child(true).unwrap().unsafe_into()
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for NestedExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_nested_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::NestedExpression);

            for ch in self.children_detailed() {
                match ch {
                    Ok((inner, _)) if inner.is_named() && tp.traverse_inner => {
                        ctx.push(TraversalContext::NestedExpressionInner);

                        let inner: ExpressionNode = inner.unsafe_into();
                        inner.accept(visitor, ctx);

                        ctx.pop();
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx)
                    },
                    _ => {}
                }
            }

            ctx.pop();         
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ThisExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for SuperExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ParentExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for VirtualParentExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        visitor.visit_virtual_parent_expr(self, ctx);
    }
}



pub type FunctionCallExpressionNode<'script> = SyntaxNode<'script, tags::FunctionCallExpression>;

impl NamedSyntaxNode for FunctionCallExpressionNode<'_> {
    const NODE_KIND: &'static str = "func_call_expr";
}

impl<'script> FunctionCallExpressionNode<'script> {
    pub fn func(&self) -> ExpressionNode<'script> {
        self.field_child("func").unwrap().unsafe_into()
    }

    pub fn args(&self) -> Option<FunctionCallArgumentsNode<'script>> {
        self.field_child("args").map(|n| n.unsafe_into())
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for FunctionCallExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_func_call_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::FunctionCall);

            for ch in self.children_detailed() {
                match ch {
                    Ok((func, Some("func"))) if tp.traverse_func => {
                        ctx.push(TraversalContext::FunctionCallExpressionFunc);

                        let func: ExpressionNode = func.unsafe_into();
                        func.accept(visitor, ctx);

                        ctx.pop();
                    },
                    Ok((args, Some("args"))) if tp.traverse_args => {
                        let args: FunctionCallArgumentsNode = args.unsafe_into();

                        args.accept_with_policy(visitor, ctx, tp.clone());
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
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
        self.iter_result().filter_map(|n| n.ok())
    }

    pub fn iter_result(&self) -> impl Iterator<Item = Result<FunctionCallArgument<'script>, ErrorNode<'script>>> {
        let mut iter = self.children_detailed().must_be_named(false);
        let mut previous_was_comma = true;

        std::iter::from_fn(move || {
            iter.find_map(|ch| {
                match ch {
                    Ok((n, _)) => {
                        if n.is_named() {
                            let arg = FunctionCallArgument::Some(n.unsafe_into());
                            previous_was_comma = false;

                            Some(Ok(arg))
                        } else {
                            if previous_was_comma {
                                let arg = FunctionCallArgument::Omitted(n.range());
                                previous_was_comma = true;

                                Some(Ok(arg))
                            } else {
                                previous_was_comma = true;
                                
                                None
                            }
                        }
                    },
                    Err(e) => {
                        previous_was_comma = false;
                        Some(Err(e))
                    }
                }
            })
        })
    }

    fn accept_with_policy<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack, tp: FunctionCallExpressionTraversalPolicy) {
        let mut previous_was_comma = true;

        for ch in self.children_detailed().must_be_named(false) {
            match ch {
                Ok((arg, _)) if arg.is_named() => {
                    let arg = FunctionCallArgument::Some(arg.unsafe_into());
                    arg.accept(visitor, ctx);

                    previous_was_comma = false;
                },
                // comma is the only valid unnamed token in thi s context
                Ok((comma, _)) if !comma.is_named() => {
                    if previous_was_comma {
                        let arg = FunctionCallArgument::Omitted(comma.range());
                        arg.accept(visitor, ctx);
                    }

                    previous_was_comma = true;

                    if tp.traverse_unnamed {
                        let unnamed: UnnamedNode = comma.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    }
                },
                Err(e) if tp.traverse_errors => {
                    e.accept(visitor, ctx);

                    previous_was_comma = false;
                },
                _ => {}
            }
        }
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for FunctionCallArgumentsNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        // UNUSED
        self.iter().for_each(|n| n.accept(visitor, ctx))
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
        } else if r.start.line == position.line {
            r.start.character <= position.character && position.character <= r.end.character
        } else {
            false
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
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_func_call_arg(self, ctx);
        // the argument node is either an expression or nothing (an ommited optional arg)
        // there are no error nodes to look for at this point
        if tp.traverse_expr {
            if let FunctionCallArgument::Some(n) = self {
                ctx.push(TraversalContext::FunctionCallArg);
                n.accept(visitor, ctx);
                ctx.pop();
            }
        }
        visitor.exit_func_call_arg(self, ctx);
    }
}



pub type ArrayExpressionNode<'script> = SyntaxNode<'script, tags::ArrayExpression>;

impl NamedSyntaxNode for ArrayExpressionNode<'_> {
    const NODE_KIND: &'static str = "array_expr";
}

impl<'script> ArrayExpressionNode<'script> {
    pub fn accessor(&self) -> ExpressionNode<'script> {
        self.field_child("accessor").unwrap().unsafe_into()
    }

    pub fn index(&self) -> ExpressionNode<'script> {
        self.field_child("index").unwrap().unsafe_into()
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ArrayExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_array_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::ArrayExpression);

            for ch in self.children_detailed() {
                match ch {
                    Ok((accessor, Some("accessor"))) if tp.traverse_accessor => {
                        let accessor: ExpressionNode = accessor.unsafe_into();
                        
                        ctx.push(TraversalContext::ArrayExpressionAccessor);
                        accessor.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((index, Some("index"))) if tp.traverse_index => {
                        let index: ExpressionNode = index.unsafe_into();

                        ctx.push(TraversalContext::ArrayExpressionIndex);
                        index.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
        }

        visitor.exit_array_expr(self, ctx);
    }
}



pub type MemberAccessExpressionNode<'script> = SyntaxNode<'script, tags::MemberAccessExpression>;

impl NamedSyntaxNode for MemberAccessExpressionNode<'_> {
    const NODE_KIND: &'static str = "member_access_expr";
}

impl<'script> MemberAccessExpressionNode<'script> {
    pub fn accessor(&self) -> ExpressionNode<'script> {
        self.field_child("accessor").unwrap().unsafe_into()
    }

    pub fn member(&self) -> IdentifierNode<'script> {
        self.field_child("member").unwrap().unsafe_into()
    }
}

impl Debug for MemberAccessExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("MemberAccessExpression {}", self.range().debug()))
            .field("accessor", &self.accessor())
            .field("member", &self.member())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberAccessExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for MemberAccessExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_member_access_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::MemberAccessExpression);

            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((accessor, Some("accessor"))) if tp.traverse_accessor => {
                        let accessor: ExpressionNode = accessor.unsafe_into();

                        ctx.push(TraversalContext::MemberAccessExpressionAccessor);
                        accessor.accept(visitor, ctx);
                        ctx.pop();

                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
        }

        visitor.exit_member_access_expr(self, ctx);
    }
}



pub type NewExpressionNode<'script> = SyntaxNode<'script, tags::NewExpression>;

impl NamedSyntaxNode for NewExpressionNode<'_> {
    const NODE_KIND: &'static str = "new_expr";
}

impl<'script> NewExpressionNode<'script> {
    pub fn class(&self) -> IdentifierNode<'script> {
        self.field_child("class").unwrap().unsafe_into()
    }

    pub fn lifetime_obj(&self) -> Option<ExpressionNode<'script>> {
        self.field_child("lifetime_obj").map(|n| n.unsafe_into())
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for NewExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_new_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::NewExpression);

            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((lifetime_obj, Some("lifetime_obj"))) if tp.traverse_lifetime_obj => {
                        let lifetime_obj: ExpressionNode = lifetime_obj.unsafe_into();

                        ctx.push(TraversalContext::NewExpressionLifetimeObj);
                        lifetime_obj.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    }
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
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
        self.field_child("type").unwrap().unsafe_into()
    }

    pub fn value(&self) -> ExpressionNode<'script> {
        self.field_child("value").unwrap().unsafe_into()
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for TypeCastExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_type_cast_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::TypeCastExpression);

            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((value, Some("value"))) if tp.traverse_value => {
                        let value: ExpressionNode = value.unsafe_into();

                        ctx.push(TraversalContext::TypeCastExpressionValue);
                        value.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    }
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
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
        self.field_child("op").unwrap().unsafe_into()
    }

    pub fn right(&self) -> ExpressionNode<'script> {
        self.field_child("right").unwrap().unsafe_into()
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for UnaryOperationExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_unary_op_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::UnaryOperationExpression);

            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((right, Some("right"))) if tp.traverse_right => {
                        let right: ExpressionNode = right.unsafe_into();

                        ctx.push(TraversalContext::UnaryOperationExpressionRight);
                        right.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
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
        self.field_child("op").unwrap().unsafe_into()
    }

    pub fn left(&self) -> ExpressionNode<'script> {
        self.field_child("left").unwrap().unsafe_into()
    }

    pub fn right(&self) -> ExpressionNode<'script> {
        self.field_child("right").unwrap().unsafe_into()
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for BinaryOperationExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_binary_op_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::BinaryOperationExpression);

            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((left, Some("left"))) if tp.traverse_left => {
                        let left: ExpressionNode = left.unsafe_into();

                        ctx.push(TraversalContext::BinaryOperationExpressionLeft);
                        left.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((right, Some("right"))) if tp.traverse_right => {
                        let right: ExpressionNode = right.unsafe_into();

                        ctx.push(TraversalContext::BinaryOperationExpressionRight);
                        right.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
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
        self.field_child("op").unwrap().unsafe_into()
    }

    pub fn left(&self) -> ExpressionNode<'script> {
        self.field_child("left").unwrap().unsafe_into()
    }

    pub fn right(&self) -> ExpressionNode<'script> {
        self.field_child("right").unwrap().unsafe_into()
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for AssignmentOperationExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_assign_op_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::AssignmentOperationExpression);

            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((left, Some("left"))) if tp.traverse_left => {
                        let left: ExpressionNode = left.unsafe_into();

                        ctx.push(TraversalContext::AssignmentOperationExpressionLeft);
                        left.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((right, Some("right"))) if tp.traverse_right => {
                        let right: ExpressionNode = right.unsafe_into();

                        ctx.push(TraversalContext::AssignmentOperationExpressionRight);
                        right.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
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
        self.field_child("cond").unwrap().unsafe_into()
    }

    pub fn conseq(&self) -> ExpressionNode<'script> {
        self.field_child("conseq").unwrap().unsafe_into()
    }

    pub fn alt(&self) -> ExpressionNode<'script> {
        self.field_child("alt").unwrap().unsafe_into()
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for TernaryConditionalExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_ternary_cond_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::TernaryConditionalExpression);

            for ch in self.children_detailed() {
                match ch {
                    Ok((cond, Some("cond"))) if tp.traverse_cond => {
                        let cond: ExpressionNode = cond.unsafe_into();

                        ctx.push(TraversalContext::TernaryConditionalExpressionCond);
                        cond.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((conseq, Some("conseq"))) if tp.traverse_conseq => {
                        let conseq: ExpressionNode = conseq.unsafe_into();

                        ctx.push(TraversalContext::TernaryConditionalExpressionConseq);
                        conseq.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((alt, Some("alt"))) if tp.traverse_alt => {
                        let alt: ExpressionNode = alt.unsafe_into();

                        ctx.push(TraversalContext::TernaryConditionalExpressionAlt);
                        alt.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
        }

        visitor.exit_ternary_cond_expr(self, ctx);
    }
}



pub type ArrayInitializerExpressionNode<'script> = SyntaxNode<'script, tags::ArrayInitializerExpression>;

impl NamedSyntaxNode for ArrayInitializerExpressionNode<'_> {
    const NODE_KIND: &'static str = "array_init_expr";
}

impl<'script> ArrayInitializerExpressionNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = ExpressionNode<'script>> {
        self.named_children().map(|ch| ch.unsafe_into())
    }
}

impl Debug for ArrayInitializerExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("ArrayInitializerExpression {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ArrayInitializerExpressionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ArrayInitializerExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_array_initializer_expr(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::ArrayInitializerExpression);

            for ch in self.children_detailed() {
                match ch {
                    Ok((item, _)) if item.is_named() && tp.traverse_items => {
                        let item: ExpressionNode = item.unsafe_into();
                        item.accept(visitor, ctx);
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
        }

        visitor.exit_array_initializer_expr(self, ctx);
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
    MemberAccess(MemberAccessExpressionNode<'script>),
    New(NewExpressionNode<'script>),
    TypeCast(TypeCastExpressionNode<'script>),
    UnaryOperation(UnaryOperationExpressionNode<'script>),
    BinaryOperation(BinaryOperationExpressionNode<'script>),
    AssignmentOperation(AssignmentOperationExpressionNode<'script>),
    TernaryConditional(TernaryConditionalExpressionNode<'script>),
    ArrayInitializer(ArrayInitializerExpressionNode<'script>)
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
            Self::MemberAccess(n) => f.debug_maybe_alternate(n),
            Self::New(n) => f.debug_maybe_alternate(n),
            Self::TypeCast(n) => f.debug_maybe_alternate(n),
            Self::UnaryOperation(n) => f.debug_maybe_alternate(n),
            Self::BinaryOperation(n) => f.debug_maybe_alternate(n),
            Self::AssignmentOperation(n) => f.debug_maybe_alternate(n),
            Self::TernaryConditional(n) => f.debug_maybe_alternate(n),
            Self::ArrayInitializer(n) => f.debug_maybe_alternate(n),
        }
    }
}

pub type ExpressionNode<'script> = SyntaxNode<'script, Expression<'script>>;

impl<'script> ExpressionNode<'script> {
    pub fn value(self) -> Expression<'script> {
        match self.tree_node.kind() {
            AssignmentOperationExpressionNode::NODE_KIND => Expression::AssignmentOperation(self.unsafe_into()),
            TernaryConditionalExpressionNode::NODE_KIND => Expression::TernaryConditional(self.unsafe_into()),
            BinaryOperationExpressionNode::NODE_KIND => Expression::BinaryOperation(self.unsafe_into()),
            NewExpressionNode::NODE_KIND => Expression::New(self.unsafe_into()),
            UnaryOperationExpressionNode::NODE_KIND => Expression::UnaryOperation(self.unsafe_into()),
            TypeCastExpressionNode::NODE_KIND => Expression::TypeCast(self.unsafe_into()),
            MemberAccessExpressionNode::NODE_KIND => Expression::MemberAccess(self.unsafe_into()),
            FunctionCallExpressionNode::NODE_KIND => Expression::FunctionCall(self.unsafe_into()),
            ArrayExpressionNode::NODE_KIND => Expression::Array(self.unsafe_into()),
            NestedExpressionNode::NODE_KIND => Expression::Nested(self.unsafe_into()),
            ThisExpressionNode::NODE_KIND => Expression::This(self.unsafe_into()),
            SuperExpressionNode::NODE_KIND => Expression::Super(self.unsafe_into()),
            ParentExpressionNode::NODE_KIND => Expression::Parent(self.unsafe_into()),
            VirtualParentExpressionNode::NODE_KIND => Expression::VirtualParent(self.unsafe_into()),
            IdentifierNode::NODE_KIND => Expression::Identifier(self.unsafe_into()),
            LiteralIntNode::NODE_KIND       |
            LiteralHexNode::NODE_KIND       |
            LiteralFloatNode::NODE_KIND     |
            LiteralBoolNode::NODE_KIND      |
            LiteralStringNode::NODE_KIND    |
            LiteralNameNode::NODE_KIND      |
            LiteralNullNode::NODE_KIND      => Expression::Literal(self.unsafe_into()),
            ArrayInitializerExpressionNode::NODE_KIND => Expression::ArrayInitializer(self.unsafe_into()),
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
            NewExpressionNode::NODE_KIND                    |
            UnaryOperationExpressionNode::NODE_KIND         |
            TypeCastExpressionNode::NODE_KIND               |
            MemberAccessExpressionNode::NODE_KIND           |
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
            LiteralNullNode::NODE_KIND                      |
            ArrayInitializerExpressionNode::NODE_KIND       => Ok(value.unsafe_into()),
            _ => Err(())
        }
    }
}

impl<'script> From<NestedExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: NestedExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<LiteralNode<'script>> for ExpressionNode<'script> {
    fn from(value: LiteralNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<ThisExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: ThisExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<SuperExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: SuperExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<ParentExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: ParentExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<VirtualParentExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: VirtualParentExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<IdentifierNode<'script>> for ExpressionNode<'script> {
    fn from(value: IdentifierNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<FunctionCallExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: FunctionCallExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<ArrayExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: ArrayExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<MemberAccessExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: MemberAccessExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<NewExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: NewExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<TypeCastExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: TypeCastExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<UnaryOperationExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: UnaryOperationExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<BinaryOperationExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: BinaryOperationExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<AssignmentOperationExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: AssignmentOperationExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<TernaryConditionalExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: TernaryConditionalExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl<'script> From<ArrayInitializerExpressionNode<'script>> for ExpressionNode<'script> {
    fn from(value: ArrayInitializerExpressionNode<'script>) -> Self {
        value.unsafe_into()
    }
}

impl SyntaxNodeTraversal for ExpressionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
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
            Expression::MemberAccess(n) => n.accept(visitor, ctx),
            Expression::New(n) => n.accept(visitor, ctx),
            Expression::TypeCast(n) => n.accept(visitor, ctx),
            Expression::UnaryOperation(n) => n.accept(visitor, ctx),
            Expression::BinaryOperation(n) => n.accept(visitor, ctx),
            Expression::AssignmentOperation(n) => n.accept(visitor, ctx),
            Expression::TernaryConditional(n) => n.accept(visitor, ctx),
            Expression::ArrayInitializer(n) => n.accept(visitor, ctx),
        }
    }
}



pub type ExpressionStatementNode<'script> = SyntaxNode<'script, tags::ExpressionStatement>;

impl NamedSyntaxNode for ExpressionStatementNode<'_> {
    const NODE_KIND: &'static str = "expr_stmt";
}

impl<'script> ExpressionStatementNode<'script> {
    pub fn expr(&self) -> ExpressionNode<'script> {
        self.first_child(true).unwrap().unsafe_into()
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ExpressionStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_expr_stmt(self, ctx);

        if tp.any() {
            ctx.push(TraversalContext::ExpressionStatement);

            for ch in self.children_detailed() {
                match ch {
                    Ok((expr, _)) if expr.is_named() && tp.traverse_expr => {
                        let expr: ExpressionNode = expr.unsafe_into();
                        expr.accept(visitor, ctx);                        
                    },
                    Ok((unnamed, _)) if !unnamed.is_named() && tp.traverse_unnamed => {
                        let unnamed: UnnamedNode = unnamed.unsafe_into();
                        unnamed.accept(visitor, ctx);
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }

            ctx.pop();
        }

        visitor.exit_expr_stmt(self, ctx);
    }
}