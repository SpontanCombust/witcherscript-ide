mod inject_native_symbols;
pub use inject_native_symbols::{inject_primitives, inject_globals};

mod scan_symbols;
pub use scan_symbols::scan_symbols;

mod syntax_analysis;
pub use syntax_analysis::syntax_analysis;

mod merge_symtabs;
pub use merge_symtabs::merge_symbol_tables;