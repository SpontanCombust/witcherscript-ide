use std::fmt::Debug;
use shrinkwraprs::Shrinkwrap;
use crate::{script_document::ScriptDocument, AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};
use crate::ast::{ExpressionTraversal, ExpressionTraversalContext, ExpressionVisitor};


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(String);

impl Into<String> for Identifier {
    fn into(self) -> String {
        self.0
    }
}

pub type IdentifierNode<'script> = SyntaxNode<'script, Identifier>;

impl NamedSyntaxNode for IdentifierNode<'_> {
    const NODE_KIND: &'static str = "ident";
}

impl IdentifierNode<'_> {
    /// Returns None if the node is marked as missing
    pub fn value(&self, doc: &ScriptDocument) -> Option<Identifier> {
        self.text(doc).map(|s| Identifier(s))
    }
}

impl Debug for IdentifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier {}", self.range().debug())
    }
}

impl ExpressionTraversal for IdentifierNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        visitor.visit_identifier_expr(self, ctx);
    }
}

impl<'script> TryFrom<AnyNode<'script>> for IdentifierNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}