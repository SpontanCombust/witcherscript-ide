mod tests;

mod script;
pub use script::*;

mod syntax_node;
pub use syntax_node::*;

mod syntax_error;
pub use syntax_error::*;

pub mod tokens;
pub mod attribs;
pub mod ast;
pub mod script_document;



/// Purely utility trait to not repeat code when implementing Debug trait for printing regular debug string or pretty debug string
trait DebugMaybeAlternate {
    fn debug_maybe_alternate(&mut self, value: &dyn std::fmt::Debug) -> std::fmt::Result;
    fn debug_maybe_alternate_named(&mut self, name: &str, value: &dyn std::fmt::Debug) -> std::fmt::Result;
}

impl DebugMaybeAlternate for std::fmt::Formatter<'_> {
    fn debug_maybe_alternate(&mut self, value: &dyn std::fmt::Debug) -> std::fmt::Result {
        if self.alternate() {
            write!(self, "{value:#?}")
        } else {
            write!(self, "{value:?}")
        }
    }

    fn debug_maybe_alternate_named(&mut self, name: &str, value: &dyn std::fmt::Debug) -> std::fmt::Result {
        if self.alternate() {
            write!(self, "{name} {value:#?}")
        } else {
            write!(self, "{name} {value:?}")
        }
    }
}