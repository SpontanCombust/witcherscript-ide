use std::fmt::Debug;
use std::str::FromStr;
use crate::{tokens::Keyword, AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};


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
        let s = self.first_child(false).unwrap().tree_node.kind();
        if let Ok(k) = Keyword::from_str(s) {
            match k {
                Keyword::Import => return StateSpecifier::Import,
                Keyword::Abstract => return StateSpecifier::Abstract,
                _ => {}
            }
        }

        panic!("Unknown state specifier: {} {}", s, self.range().debug());
    }
}

impl Debug for StateSpecifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.value(), self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StateSpecifierNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.is_named() && value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}