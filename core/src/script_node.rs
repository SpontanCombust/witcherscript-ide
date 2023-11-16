use ropey::Rope;
use tree_sitter::{Node, Range};
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptNode<'script, T> {
    pub(crate) tree_node: Node<'script>,
    pub(crate) rope: Rope, // ropes can be cloned cheaply
    pub(crate) phantom : PhantomData<T>
}

impl<'script, T> ScriptNode<'script, T> {
    pub(crate) fn new(tree_node: Node<'script>, rope: Rope) -> Self {
        Self {
            tree_node,
            rope,
            phantom: PhantomData,
        }
    }

    pub fn span(&self) -> Range {
        self.tree_node.range()
    }

    pub fn text(&self) -> String {
        let span = self.tree_node.start_byte() .. self.tree_node.end_byte();
        let slice = self.rope.byte_slice(span);
        slice.to_string()
    }
}
