use std::str::FromStr;
use crate::{tokens::Keyword, SyntaxNode};


#[derive(Debug, Clone)]
pub enum Unnamed {
    Keyword(Keyword),
    Punctuation(&'static str)
}

pub type UnnamedNode<'script> = SyntaxNode<'script, Unnamed>;

impl UnnamedNode<'_> {
    pub fn value(&self) -> Unnamed {
        let kind = self.tree_node.kind();
        if let Ok(kw) = Keyword::from_str(kind) {
            Unnamed::Keyword(kw)
        } else {
            Unnamed::Punctuation(kind)
        }
    }
}