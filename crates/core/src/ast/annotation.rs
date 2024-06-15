use strum_macros::{EnumString, Display, AsRefStr};
use crate::{AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};
use crate::tokens::{AnnotationIdentifierNode, IdentifierNode};


mod tags {
    pub struct Annotation;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, AsRefStr)]
pub enum AnnotationKind {
    #[strum(serialize="@addMethod")]
    AddMethod,
    #[strum(serialize="@addField")]
    AddField,
    #[strum(serialize="@replaceMethod")]
    ReplaceMethod,
    #[strum(serialize="@wrapMethod")]
    WrapMethod
}

impl AnnotationKind {
    pub fn requires_arg(&self) -> bool {
        match self {
            AnnotationKind::AddMethod => true,
            AnnotationKind::AddField => true,
            AnnotationKind::ReplaceMethod => false,
            AnnotationKind::WrapMethod => true,
        }
    }

    pub fn arg_type(&self) -> Option<&'static str> {
        match self {
            AnnotationKind::AddMethod => Some("a class identifier"),
            AnnotationKind::AddField => Some("a class identifier"),
            AnnotationKind::ReplaceMethod => Some("a class identifier"),
            AnnotationKind::WrapMethod => Some("a class identifier"),
        }
    }
}

pub const WRAPPED_METHOD_NAME: &'static str = "wrappedMethod";


pub type AnnotationNode<'script> = SyntaxNode<'script, tags::Annotation>;

impl NamedSyntaxNode for AnnotationNode<'_> {
    const NODE_KIND: &'static str = "annotation";
}

impl<'script> AnnotationNode<'script> {
    pub fn name(&self) -> AnnotationIdentifierNode<'script> {
        self.field_child("name").unwrap().into()
    }

    pub fn arg(&self) -> Option<IdentifierNode<'script>> {
        self.field_child("arg").map(|n| n.into())
    }
}

impl std::fmt::Debug for AnnotationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Annotation {}", self.range().debug()))
            .field("name", &self.name())
            .field("arg", &self.arg())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for AnnotationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}