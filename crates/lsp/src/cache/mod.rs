use tokio::sync::RwLock;

use crate::providers::workspace_symbols::WorkspaceSymbolCache;


pub struct Cache {
    pub workspace_symbols: RwLock<WorkspaceSymbolCache>
}

impl Cache {
    pub fn new() -> Self {
        Self {
            workspace_symbols: RwLock::new(WorkspaceSymbolCache::new()) 
        }
    }
}