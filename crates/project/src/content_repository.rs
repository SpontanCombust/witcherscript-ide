use std::path::{Path, PathBuf};
use crate::{ContentDirectory, FileError};


/// Looks for content in provided "repository" directories.
/// Allows the end user to specify dependencies from outside the current workspace directory.
/// These dependencies do not necessairly need a manifest, but they are still required to have a correct file structure.
/// This is due to not forcing the user to create manifest for content inside game directory when it is obvious as to where this content should be looked for.
#[derive(Debug, Clone, Default)]
pub struct ContentRepositories {
    repository_paths: Vec<PathBuf>,
    found_content: Vec<ContentDirectory>,
    /// Errors encountered during scanning
    errors: Vec<FileError>
}

impl ContentRepositories {
    pub fn new() -> Self {
        Self {
            repository_paths: Vec::new(),
            found_content: Vec::new(),
            errors: Vec::new()
        }
    }

    pub fn add_repository<P>(&mut self, path: P) 
    where P: AsRef<Path> {
        let pathbuf = path.as_ref().to_path_buf();
        if !self.repository_paths.contains(&pathbuf) {
            self.repository_paths.push(pathbuf);
        }
    }

    pub fn found_content(&self) -> &[ContentDirectory] {
        &self.found_content
    }

    pub fn scan(&mut self) {
        self.found_content.clear();
        self.errors.clear();

        for repo in &self.repository_paths {
            let (contents, errors) = ContentDirectory::find_in(repo, false, false);
            self.found_content.extend(contents);
            self.errors.extend(errors);
        }
    }
}