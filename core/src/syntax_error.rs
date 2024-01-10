use super::AnyNode;


#[derive(Debug, Clone)]
pub enum SyntaxError<'script> {
    /// Corresponds to a named or unnamed leaf node that was inserted by tree-sitter to recover from syntax error.
    Missing(AnyNode<'script>),
    /// Corresponds to a parent node of at least one node that could not fit into the syntax.
    Invalid(AnyNode<'script>)
}
