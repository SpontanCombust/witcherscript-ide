use std::fmt::Debug;
use std::str::FromStr;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::Keyword};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructSpecifier {
    Import
}

impl NamedSyntaxNode for StructSpecifier {
    const NODE_NAME: &'static str = "struct_specifier";
}

impl SyntaxNode<'_, StructSpecifier> {
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

impl Debug for SyntaxNode<'_, StructSpecifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}