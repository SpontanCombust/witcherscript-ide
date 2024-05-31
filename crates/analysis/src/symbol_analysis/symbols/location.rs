use std::{path::Path, sync::Arc};
use abs_path::AbsPath;
use lsp_types as lsp;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolLocation {
    pub scripts_root: Arc<AbsPath>,
    pub local_source_path: Arc<Path>,
    pub range: lsp::Range,
    pub label_range: lsp::Range
}

impl SymbolLocation {    
    #[inline]
    pub fn abs_source_path(&self) -> AbsPath {
        self.scripts_root.join(self.local_source_path.as_ref()).unwrap()
    }
}