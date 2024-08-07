use core::fmt::Debug;
use crate::{AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct Root;
}


#[derive(Clone)]
pub enum RootStatement<'script> {
    Function(FunctionDeclarationNode<'script>),
    Class(ClassDeclarationNode<'script>),
    State(StateDeclarationNode<'script>),
    Struct(StructDeclarationNode<'script>),
    Enum(EnumDeclarationNode<'script>),
    Var(MemberVarDeclarationNode<'script>), // needed for @addField
    Nop(NopNode<'script>)
}

impl Debug for RootStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(n) => f.debug_maybe_alternate(n),
            Self::Class(n) => f.debug_maybe_alternate(n),
            Self::State(n) => f.debug_maybe_alternate(n),
            Self::Struct(n) => f.debug_maybe_alternate(n),
            Self::Enum(n) => f.debug_maybe_alternate(n),
            Self::Var(n) => f.debug_maybe_alternate(n),
            Self::Nop(n) => f.debug_maybe_alternate(n),
        }
    }
}

pub type RootStatementNode<'script> = SyntaxNode<'script, RootStatement<'script>>;

impl<'script> RootStatementNode<'script> {
    pub fn value(self) -> RootStatement<'script> {
        let s = self.tree_node.kind();
        match s {
            FunctionDeclarationNode::NODE_KIND => RootStatement::Function(self.into()),
            ClassDeclarationNode::NODE_KIND => RootStatement::Class(self.into()),
            StateDeclarationNode::NODE_KIND => RootStatement::State(self.into()),
            StructDeclarationNode::NODE_KIND => RootStatement::Struct(self.into()),
            EnumDeclarationNode::NODE_KIND => RootStatement::Enum(self.into()),
            MemberVarDeclarationNode::NODE_KIND => RootStatement::Var(self.into()),
            NopNode::NODE_KIND => RootStatement::Nop(self.into()),
            _ => panic!("Unknown script statement: {} {}", s, self.range().debug())
        }
    }
}

impl Debug for RootStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.clone().value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for RootStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() {
            return Err(());
        }
        
        match value.tree_node.kind() {
            FunctionDeclarationNode::NODE_KIND          |
            ClassDeclarationNode::NODE_KIND             |
            StateDeclarationNode::NODE_KIND             |
            StructDeclarationNode::NODE_KIND            |
            EnumDeclarationNode::NODE_KIND              |
            MemberVarDeclarationNode::NODE_KIND         |
            NopNode::NODE_KIND                          => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl SyntaxNodeTraversal for RootStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        match self.clone().value() {
            RootStatement::Function(s) => s.accept(visitor, ctx),
            RootStatement::Class(s) => s.accept(visitor, ctx),
            RootStatement::State(s) => s.accept(visitor, ctx),
            RootStatement::Struct(s) => s.accept(visitor, ctx),
            RootStatement::Enum(s) => s.accept(visitor, ctx),
            RootStatement::Var(s) => s.accept(visitor, ctx),
            RootStatement::Nop(_) => {},
        }
    }
}



pub type RootNode<'script> = SyntaxNode<'script, tags::Root>;

impl NamedSyntaxNode for RootNode<'_> {
    const NODE_KIND: &'static str = "script";
}

impl<'script> RootNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = RootStatementNode> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for RootNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("Script {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for RootNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for RootNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_root(self);
        if tp.traverse {
            ctx.push(TraversalContext::Global);
            self.iter().for_each(|s| s.accept(visitor, ctx));
            ctx.pop();
        }
    }
}