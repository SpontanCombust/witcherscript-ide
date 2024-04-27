use std::borrow::Cow;
use std::fmt::Debug;
use shrinkwraprs::Shrinkwrap;
use crate::{script_document::ScriptDocument, AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};
use crate::ast::{SyntaxNodeTraversal, ExpressionTraversalContext, SyntaxNodeVisitor};


#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier<'d>(Cow<'d, str>);

impl std::fmt::Display for Identifier<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'d, S: AsRef<str>> PartialEq<S> for Identifier<'d> {
    #[inline]
    fn eq(&self, other: &S) -> bool {
        self.0 == other.as_ref()
    }
}


pub type IdentifierNode<'script> = SyntaxNode<'script, Identifier<'script>>;

impl NamedSyntaxNode for IdentifierNode<'_> {
    const NODE_KIND: &'static str = "ident";
}

impl IdentifierNode<'_> {
    /// Returns None if the node is marked as missing
    pub fn value<'d>(&self, doc: &'d ScriptDocument) -> Option<Identifier<'d>> {
        self.text(doc).map(|s| Identifier(s))
    }
}

impl Debug for IdentifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier {}", self.range().debug())
    }
}

impl SyntaxNodeTraversal for IdentifierNode<'_> {
    type TraversalCtx = ExpressionTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
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