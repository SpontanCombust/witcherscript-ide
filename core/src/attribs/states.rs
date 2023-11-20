use std::str::FromStr;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::Keyword};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpecifier {
    Import,
    Abstract
}

impl NamedSyntaxNode for StateSpecifier {
    const NODE_NAME: &'static str = "state_specifier";
}

impl SyntaxNode<'_, StateSpecifier> {
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