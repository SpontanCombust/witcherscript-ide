use std::fmt::Debug;
use std::str::FromStr;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::Keyword, AnyNode, DebugMaybeAlternate};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructSpecifier {
    Import
}

pub type StructSpecifierNode<'script> = SyntaxNode<'script, StructSpecifier>;

impl NamedSyntaxNode for StructSpecifierNode<'_> {
    const NODE_KIND: &'static str = "struct_specifier";
}

impl StructSpecifierNode<'_> {
    pub fn value(&self) -> StructSpecifier {
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Import => return StructSpecifier::Import,
                _ => {}
            }
        }

        panic!("Unknown struct specifier: {}", s)
    }
}

impl Debug for StructSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StructSpecifierNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}