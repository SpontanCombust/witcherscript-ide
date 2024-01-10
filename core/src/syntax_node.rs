use lsp_types::{Range, Position};
use ropey::Rope;
use std::{marker::PhantomData, fmt::Debug};
use crate::{SyntaxError, tokens::UnnamedNode};


/// Represents a WitcherScript syntax tree node
/// 
/// It is a backbone of the strong typed layer for AST that instead of being represented by structs with data is represented by 
/// functions through which you can traverse said tree.
/// This way parsed data is retrieved only on demand and not stored anywhere else than in tree-sitter. 
/// 
/// It works as an adapter for tree-sitter's nodes. Generic parameter T denotes the type of node, e.g. `Identifier`. 
/// It can be just a marker type. What is important is to have a distinct type for a given node type in the parsed tree.
#[derive(Clone)]
pub struct SyntaxNode<'script, T> {
    pub(crate) tree_node: tree_sitter::Node<'script>,
    // TODO for later - see if storing RefCell<TreeCursor> would make any improvement in parsing speed
    pub(crate) phantom : PhantomData<T>
}

impl<'script, T> SyntaxNode<'script, T> {
    /// Constructs a completely new node from a tree-sitter node and a rope 
    pub(crate) fn new(tree_node: tree_sitter::Node<'script>) -> Self {
        Self {
            tree_node,
            phantom: PhantomData,
        }
    }

    /// Interpret this node into a node with a different underlying type.
    /// Gives no guarantees as to whether that target type is actually valid, so it's not exposed by the crate
    pub(crate) fn into<U>(self) -> SyntaxNode<'script, U> {
        SyntaxNode::new(self.tree_node)
    }

    pub fn into_any(self) -> AnyNode<'script> {
        AnyNode::new(self.tree_node)
    }

    /// Returns an iterator over non-error children of this node as AnyNodes
    pub fn children(&self) -> impl Iterator<Item = AnyNode> {
        let mut cursor = self.tree_node.walk();
        let name_nodes = self.tree_node
            .children(&mut cursor)
            .filter(|n| !n.is_error() && !n.is_extra())
            .collect::<Vec<_>>();

        name_nodes.into_iter()
            .map(|n| AnyNode::new(n))
    }

    /// Returns an iterator over non-error named children of this node as AnyNodes
    pub(crate) fn named_children(&self) -> impl Iterator<Item = AnyNode> {
        self.children()
            .filter(|n| n.tree_node.is_named())
    }

    /// Returns the first non-error child of this node as an AnyNodes
    pub(crate) fn first_child(&self, must_be_named: bool) -> Option<AnyNode> {
        self.children()
            .filter(|n| 
                if must_be_named { 
                    n.tree_node.is_named() 
                } else { 
                    true 
                }
            ).next()
    }

    /// Returns the first non-error child of this node with a given field name as an AnyNodes
    pub(crate) fn field_child(&self, field: &'static str) -> Option<AnyNode> {
        self.field_children(field).next()
    }

    /// Returns an iterator over named, non-error children of this node with a given field name
    pub(crate) fn field_children(&self, field: &'static str) -> impl Iterator<Item = AnyNode> {
        let mut cursor = self.tree_node.walk();
        let name_nodes = self.tree_node
            .children_by_field_name(field, &mut cursor)
            .filter(|n| !n.is_error() && n.is_named())
            .collect::<Vec<_>>();

        name_nodes.into_iter()
            .map(|n| AnyNode::new(n))
    }


    /// Whether any nodes descending from this node are errors
    pub fn has_errors(&self) -> bool {
        let mut cursor = self.tree_node.walk();

        let any_errors = self.tree_node
                            .children(&mut cursor)
                            .any(|child| child.has_error());

        any_errors
    }

    /// Returns an iterator over ERROR or missing children nodes
    pub fn errors(&self) -> impl Iterator<Item = SyntaxError> {
        let mut errors = Vec::new();

        let mut cursor = self.tree_node.walk();
        for n in self.tree_node.children(&mut cursor) {
            if n.is_error() {
                errors.push(SyntaxError::Invalid(AnyNode::new(n)));
            } else if n.is_missing() {
                errors.push(SyntaxError::Missing(AnyNode::new(n)));
            }
        }

        errors.into_iter()
    }

    pub fn unnamed_children(&self) -> impl Iterator<Item = UnnamedNode> {
        let mut cursor = self.tree_node.walk();

        let unnamed_nodes = self.tree_node
            .children(&mut cursor)
            .filter(|n| !n.is_named() && !n.is_extra())
            .collect::<Vec<_>>();

        unnamed_nodes.into_iter().map(|n| UnnamedNode::new(n))
    }


    /// Returns the span at which this node is located in the text document
    pub fn span(&self) -> Range {
        let r = self.tree_node.range();
        Range::new(
            Position::new(r.start_point.row as u32, r.start_point.column as u32),
            Position::new(r.end_point.row as u32, r.end_point.column as u32)
        )
    }

    pub fn is_missing(&self) -> bool {
        self.tree_node.is_missing()
    }

    /// Returns text that this node spans in the text document
    /// If the node is missing returns None
    pub fn text(&self, rope: &Rope) -> Option<String> {
        if self.is_missing() {
            None
        } else {
            let pos_span = self.tree_node.start_position() .. self.tree_node.end_position();
            let char_span = rope.line_to_char(pos_span.start.row) + pos_span.start.column .. rope.line_to_char(pos_span.end.row) + pos_span.end.column;
            let slice = rope.slice(char_span);
            Some(slice.to_string())
        }
    }
}

impl<T> PartialEq for SyntaxNode<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.tree_node == other.tree_node
    }
}

impl<T> Eq for SyntaxNode<'_, T> {}



/// Default opaque node type not possessing any additional capabilities.
pub type AnyNode<'script> = SyntaxNode<'script, ()>;


impl Debug for AnyNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxNode")
            .field("tree_node", &self.tree_node)
            .finish()
    }
}



/// Describes the name, by which a node is identified in tree-sitter's grammar
pub trait NamedSyntaxNode {
    const NODE_KIND: &'static str;
}
