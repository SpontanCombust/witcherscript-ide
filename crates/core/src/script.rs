use std::io;
use ropey::Rope;
use thiserror::Error;
use tree_sitter::{Parser, Tree, LanguageError};
use crate::{ast::{RootNode, SyntaxNodeTraversal, SyntaxNodeVisitor, TraversalContextStack}, script_document::ScriptDocument};


#[derive(Debug, Clone)]
pub struct Script {
    current_tree: Tree,
    prev_tree: Option<Tree>
}

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("file failed to open")]
    FileIOError(#[source] io::Error),
    #[error("parser failed to initialize")]
    ParserInitError(#[source] LanguageError)
}

impl Script {
    pub fn new(doc: &ScriptDocument) -> Result<Self, ScriptError> {
        let parse_tree = Self::parse_rope(&doc.rope, None)?;

        Ok(Self {
            current_tree: parse_tree,
            prev_tree: None
        })
    }

    /// Reparses AST based on the previous script state and changes made to the document.
    /// Clear document's edit history.
    pub fn update(&mut self, doc: &mut ScriptDocument) -> Result<(), ScriptError> {
        for edit in &doc.edits {
            self.current_tree.edit(edit);
        }

        let current_tree = Self::parse_rope(&doc.rope, Some(&self.current_tree))?;
        let prev_tree = std::mem::replace(&mut self.current_tree, current_tree);
        self.prev_tree = Some(prev_tree);

        doc.edits.clear();

        Ok(())
    }

    /// Reparses AST based on the script document alone.
    /// The range of the entire script will be different after the operation.
    pub fn refresh(&mut self, doc: &ScriptDocument) -> Result<(), ScriptError> {
        let current_tree = Self::parse_rope(&doc.rope, None)?;
        let prev_tree = std::mem::replace(&mut self.current_tree, current_tree);
        self.prev_tree = Some(prev_tree);

        Ok(())
    }

    fn parse_rope(rope: &Rope, prev_tree: Option<&Tree>) -> Result<Tree, ScriptError> {
        use ScriptError::*;

        let mut parser = Parser::new();
        parser.set_language(tree_sitter_witcherscript::language()).map_err(ParserInitError)?;

        let parse_tree = parser.parse_with(&mut |byte, _| {
            if byte <= rope.len_bytes() {
                let (chunk, start_byte, _, _) = rope.chunk_at_byte(byte);
                &chunk.as_bytes()[byte - start_byte..]
            } else {
                &[]
            }
        }, prev_tree).unwrap();

        Ok(parse_tree)
    }


    #[inline(always)]
    pub fn root_node(&self) -> RootNode {
        RootNode::new(self.current_tree.root_node())
    }

    pub fn visit_nodes<V: SyntaxNodeVisitor>(&self, visitor: &mut V) {
        let mut ctx = TraversalContextStack::new();
        self.root_node().accept(visitor, &mut ctx);
    }
}