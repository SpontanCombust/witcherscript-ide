/// Module responsible for general gathering of data about WitcherScript and reacting to changes made to it.

mod content_indexing_tasks;
pub use content_indexing_tasks::*;

mod script_indexing_tasks;
pub use script_indexing_tasks::*;

mod script_analysis_tasks;
pub use script_analysis_tasks::*;