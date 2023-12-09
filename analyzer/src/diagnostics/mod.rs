mod diagnostic;
mod errors;
mod warnings;
mod infos;

pub use diagnostic::*;
pub use errors::*;
pub use warnings::*;
pub use infos::*;

pub mod syntax_analysis;