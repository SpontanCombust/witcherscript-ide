//! Tree-sitter is an error-tolerant parser. Whenever it stumbles upon a sequence of tokens it is not able to parse into a valid node
//! it creates an ERROR node that encompasses these invalid tokens. This is however the full extent of what it does.
//! By itself it does not provide any concrete means as to **why** exactly it created an ERROR node.
//! This is why these nodes have to be poked around by hand and have useful information extracted from them.

//TODO implement syntax error node inference, use it when checking SyntaxError::Invalid in SyntaxErrorVisitor::check_errors
// Possible solution: flatten the error node into a vector of tokens or leaf node kinds.
// Go over over the elements until you encounter the first truly unexpected token and create a MissingElement or UnexpectedElement diagnostic