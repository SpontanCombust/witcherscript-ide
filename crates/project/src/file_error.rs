use std::{path::PathBuf, sync::Arc};
use thiserror::Error;


#[derive(Debug, Clone, Error)]
#[error("error for file or directory {}: {}", .path.display(), .error)]
pub struct FileError {
    pub path: PathBuf,
    #[source]
    pub error: Arc<std::io::Error>
}