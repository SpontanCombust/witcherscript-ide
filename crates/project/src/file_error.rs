use std::{path::{Path, PathBuf}, sync::Arc};
use thiserror::Error;


#[derive(Error)]
#[error("error for file or directory {}: {}", .path.display(), .error)]
pub struct FileError<T>
where T: std::error::Error {
    pub path: PathBuf,
    #[source]
    pub error: Arc<T>
}

impl<T> FileError<T> 
where T: std::error::Error {
    pub fn new<P: AsRef<Path>>(path: P, error: T) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            error: Arc::new(error)
        }
    }

    pub fn error(&self) -> &T {
        &self.error
    }
}

impl<T> std::fmt::Debug for FileError<T>
where T: std::fmt::Debug + std::error::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileError").field("path", &self.path).field("error", &self.error).finish()
    }
}

impl<T> Clone for FileError<T> 
where T: std::error::Error {
    fn clone(&self) -> Self {
        Self { path: self.path.clone(), error: self.error.clone() }
    }
}