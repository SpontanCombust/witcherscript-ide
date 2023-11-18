use ropey::Rope;
use tree_sitter::{Node, Range};
use std::marker::PhantomData;

/// Represents a WitcherScript syntax tree node
/// 
/// It is a backbone of the convenience layer for AST that instead of being represented by structs with data is represented by 
/// functions through which you can traverse said tree.
/// This way parsed data is retrieved only on demand and not stored anywhere else than in tree-sitter. 
/// 
/// It works as an adapter for tree-sitter's nodes. Generic parameter T denotes the type of node, e.g. `Identifier`. 
/// It can also be just a marker type. What is important is to have a distinct type for a given node type in the parsed tree.
/// Traits can be blanket-implemented for SyntaxNode by accessing the marker type.
/// 
/// ## Arguments
/// * T - marker for the concrete type of the node; () means it can be any node type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxNode<'script, T = ()> {
    pub(crate) tree_node: Node<'script>,
    pub(crate) rope: Rope, // ropes can be cloned cheaply
    pub(crate) phantom : PhantomData<T>
}

impl<'script, T> SyntaxNode<'script, T> {
    pub(crate) fn new(tree_node: Node<'script>, rope: Rope) -> Self {
        Self {
            tree_node,
            rope,
            phantom: PhantomData,
        }
    }

    pub(crate) fn clone_as<U>(&self) -> SyntaxNode<'_, U> {
        SyntaxNode::<'_, U> {
            tree_node: self.tree_node.clone(),
            rope: self.rope.clone(),
            phantom: PhantomData
        }
    }

    pub(crate) fn clone_as_with<U>(&self, node: Node<'script>) -> SyntaxNode<'_, U> {
        SyntaxNode::<'_, U> {
            tree_node: node,
            rope: self.rope.clone(),
            phantom: PhantomData
        }
    } 

    pub(crate) fn first_child<U>(&self) -> SyntaxNode<'_, U> {
        self.clone_as_with(self.tree_node.named_child(0).unwrap())
    }

    pub(crate) fn field_child<U>(&self, field: &'static str) -> SyntaxNode<'_, U> {
        self.clone_as_with(self.tree_node.child_by_field_name(field).unwrap())
    }


    pub fn span(&self) -> Range {
        self.tree_node.range()
    }

    pub fn text(&self) -> String {
        let pos_span = self.tree_node.start_position() .. self.tree_node.end_position();
        let byte_span = self.rope.line_to_char(pos_span.start.row) + pos_span.start.column .. self.rope.line_to_char(pos_span.end.row) + pos_span.end.column;
        let slice = self.rope.slice(byte_span);
        slice.to_string()
    }
}


pub trait NamedSyntaxNode {
    const NODE_NAME: &'static str;
}

impl<T> NamedSyntaxNode for SyntaxNode<'_, T> where T: NamedSyntaxNode {
    const NODE_NAME: &'static str = T::NODE_NAME;
}