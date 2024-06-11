mod syntax_error_visitor;
mod error_inference;
mod contextual_analysis;

pub use syntax_error_visitor::syntax_analysis;
pub use contextual_analysis::contextual_syntax_analysis;