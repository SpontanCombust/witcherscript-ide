use lsp_types::{Range, Position};
use std::{marker::PhantomData, fmt::Debug};
use crate::{SyntaxError, script_document::ScriptDocument};


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
            .filter(|n| n.tree_node.is_named() && !n.tree_node.is_extra())
    }

    /// Returns the first non-error child of this node as an AnyNodes
    pub(crate) fn first_child(&self, must_be_named: bool) -> Option<AnyNode> {
        self.children()
            .filter(|n| 
                if must_be_named { 
                    n.tree_node.is_named() && !n.tree_node.is_extra()
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
            .filter(|n| !n.is_error() && n.is_named() && !n.is_extra())
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
    pub fn errors(&self) -> Vec<SyntaxError> {
        let mut errors = Vec::new();

        let mut cursor = self.tree_node.walk();
        for n in self.tree_node.children(&mut cursor) {
            if n.is_error() {
                errors.push(SyntaxError::Invalid(AnyNode::new(n)));
            } else if n.is_missing() {
                errors.push(SyntaxError::Missing(AnyNode::new(n)));
            }
        }

        errors
    }


    /// Returns the range at which this node is located in the text document
    pub fn range(&self) -> Range {
        let r = self.tree_node.range();
        Range::new(
            Position::new(r.start_point.row as u32, r.start_point.column as u32),
            Position::new(r.end_point.row as u32, r.end_point.column as u32)
        )
    }

    pub fn is_missing(&self) -> bool {
        // More reliable way than tree_sitter::Node::is_missing().
        // That's because in the node tree only leaves can be marked as missing.
        // TS is also a bit annoying when it comes to those leave nodes.
        // Named nodes can never be leaves, they always contain an unnamed node inside them,
        // even if this node corresponds to a single token.
        let range = self.range();
        range.start == range.end
    }

    /// Returns text that this node spans in the text document
    /// If the node is missing returns None
    pub fn text(&self, doc: &ScriptDocument) -> Option<String> {
        if self.is_missing() {
            None
        } else {
            Some(doc.text_at(self.range()))
        }
    }


    /// Returns tree-sitter's node structure in a form of XML.
    /// Use for debugging purposes.
    pub fn debug_ts_tree(&self, doc: &ScriptDocument) -> String {
        let mut buf = String::new();
        let mut cursor = self.tree_node.walk();

        let mut needs_newline = false;
        let mut indent_level = 0;
        let mut did_visit_children = false;
        let mut tags: Vec<&str> = Vec::new();

        loop {
            let node = cursor.node();
            let is_named = node.is_named();
            if did_visit_children {
                if is_named {
                    let tag = tags.pop();
                    buf += &format!("</{}>\n", tag.expect("there is a tag"));
                    needs_newline = true;
                }
                if cursor.goto_next_sibling() {
                    did_visit_children = false;
                } else if cursor.goto_parent() {
                    did_visit_children = true;
                    indent_level -= 1;
                } else {
                    break;
                }
            } else {
                if is_named {
                    if needs_newline {
                        buf += &format!("\n");
                    }
                    for _ in 0..indent_level {
                        buf += &format!("  ");
                    }
                    buf += &format!("<{}", node.kind());
                    if let Some(field_name) = cursor.field_name() {
                        buf += &format!(" type=\"{}\"", field_name);
                    }
                    buf += &format!(">");
                    tags.push(node.kind());
                    needs_newline = true;
                }
                if cursor.goto_first_child() {
                    did_visit_children = false;
                    indent_level += 1;
                } else {
                    let node_range = node.range();
                    let lsp_range = Range::new(
                        Position::new(node_range.start_point.row as u32, node_range.start_point.column as u32),
                        Position::new(node_range.end_point.row as u32, node_range.end_point.column as u32)
                    );

                    buf += &doc.text_at(lsp_range);
                    did_visit_children = true;
                }
                if node.is_missing() {
                    buf += &format!("MISSING");
                }
            }
        }

        buf
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
