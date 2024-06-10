use std::str::FromStr;

use crate::{tokens::Keyword, AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Specifier {
    Abstract,
    Const,
    Editable,
    Final,
    Import,
    Inlined,
    Latent,
    Optional,
    Out,
    Private,
    Protected,
    Public,
    Saved,
    Statemachine
}

impl From<Specifier> for Keyword {
    fn from(value: Specifier) -> Self {
        match value {
            Specifier::Abstract => Keyword::Abstract,
            Specifier::Const => Keyword::Const,
            Specifier::Editable => Keyword::Editable,
            Specifier::Final => Keyword::Final,
            Specifier::Import => Keyword::Import,
            Specifier::Inlined => Keyword::Inlined,
            Specifier::Latent => Keyword::Latent,
            Specifier::Optional => Keyword::Optional,
            Specifier::Out => Keyword::Out,
            Specifier::Private => Keyword::Private,
            Specifier::Protected => Keyword::Protected,
            Specifier::Public => Keyword::Public,
            Specifier::Saved => Keyword::Saved,
            Specifier::Statemachine => Keyword::Statemachine,
        }
    }
}

pub type SpecifierNode<'script> = SyntaxNode<'script, Specifier>;

impl NamedSyntaxNode for SpecifierNode<'_> {
    const NODE_KIND: &'static str = "specifier";
}

impl SpecifierNode<'_> {
    pub fn value(&self) -> Specifier {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Abstract => return Specifier::Abstract,
                Keyword::Const => return Specifier::Const,
                Keyword::Editable => return Specifier::Editable,
                Keyword::Final => return Specifier::Final,
                Keyword::Import => return Specifier::Import,
                Keyword::Inlined => return Specifier::Inlined,
                Keyword::Latent => return Specifier::Latent,
                Keyword::Optional => return Specifier::Optional,
                Keyword::Out => return Specifier::Out,
                Keyword::Private => return Specifier::Private,
                Keyword::Protected => return Specifier::Protected,
                Keyword::Public => return Specifier::Public,
                Keyword::Saved => return Specifier::Saved,
                Keyword::Statemachine => return Specifier::Statemachine,
                _ => {}
            }
        }

        panic!("Unknown specifier: {} {}", s, self.range().debug())
    }
}

impl std::fmt::Debug for SpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.value(), self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SpecifierNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}