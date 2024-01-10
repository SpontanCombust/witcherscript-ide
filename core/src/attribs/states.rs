use std::fmt::Debug;
use std::str::FromStr;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::Keyword, AnyNode};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateSpecifier {
    Import,
    Abstract
}

pub type StateSpecifierNode<'script> = SyntaxNode<'script, StateSpecifier>;

impl NamedSyntaxNode for StateSpecifierNode<'_> {
    const NODE_KIND: &'static str = "state_specifier";
}

impl StateSpecifierNode<'_> {
    pub fn value(&self) -> StateSpecifier {
        let s = self.tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Import => return StateSpecifier::Import,
                Keyword::Abstract => return StateSpecifier::Abstract,
                _ => {}
            }
        }

        panic!("Unknown state specifier: {}", s);
    }
}

impl Debug for StateSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StateSpecifierNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}