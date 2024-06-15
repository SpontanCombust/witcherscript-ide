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
    /// Will return [`crate::MISSING_TEXT`] if the node is marked as missing
    pub fn value<'d>(&self, doc: &'d ScriptDocument) -> Identifier<'d> {
        Identifier(self.text(doc))
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




#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnnotationIdentifier<'d>(Cow<'d, str>);

impl std::fmt::Display for AnnotationIdentifier<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'d, S: AsRef<str>> PartialEq<S> for AnnotationIdentifier<'d> {
    #[inline]
    fn eq(&self, other: &S) -> bool {
        self.0 == other.as_ref()
    }
}


pub type AnnotationIdentifierNode<'script> = SyntaxNode<'script, AnnotationIdentifier<'script>>;

impl NamedSyntaxNode for AnnotationIdentifierNode<'_> {
    const NODE_KIND: &'static str = "annotation_ident";
}

impl AnnotationIdentifierNode<'_> {
    pub fn value<'d>(&self, doc: &'d ScriptDocument) -> AnnotationIdentifier<'d> {
        AnnotationIdentifier(self.text(doc))
    }
}

impl Debug for AnnotationIdentifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnnotationIdentifier {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for AnnotationIdentifierNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}