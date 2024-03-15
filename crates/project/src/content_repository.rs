use std::path::{Path, PathBuf};
use crate::{content::ContentScanError, find_content_in_directory, Content};


/// A collection of directories in which content directories can be found.
/// Only direct directory descendants are checked for being content directories.
/// Mainly used repositories are `Witcher 3/content` and `Witcher 3/Mods`.
#[derive(Debug, Default)]
pub struct ContentRepositories {
    repository_paths: Vec<PathBuf>,
    found_content: Vec<Box<dyn Content>>,
    /// Errors encountered during scanning
    pub errors: Vec<ContentScanError>
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

    pub fn found_content(&self) -> &[Box<dyn Content>] {
        &self.found_content
    }

    pub fn scan(&mut self) {
        self.found_content.clear();
        self.errors.clear();

        for repo in &self.repository_paths {
            let (contents, errors) = find_content_in_directory(repo, false);
            self.found_content.extend(contents);
            self.errors.extend(errors);
        }
    }
}