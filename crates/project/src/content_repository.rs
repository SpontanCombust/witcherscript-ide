use std::{path::{Path, PathBuf}, rc::Rc};
use thiserror::Error;
use crate::{ContentDirectory, Manifest};


/// Looks for content in provided "repository" directories.
/// Allows the end user to specify dependencies from outside the current workspace directory.
/// These dependencies do not necessairly need a manifest, but they are still required to have a correct file structure.
/// This is due to not forcing the user to create manifest for content inside game directory when it is obvious as to where this content should be looked for.
#[derive(Debug, Clone, Default)]
pub struct ContentRepositories {
    repository_paths: Vec<PathBuf>,
    found_content: Vec<ContentDirectory>,
    /// Errors encountered during scanning
    errors: Vec<ContentRepositoryScanError>
}

#[derive(Debug, Clone, Error)]
#[error("Failed to scan repository directory {}", .repo_path.display())]
pub struct ContentRepositoryScanError {
    pub repo_path: PathBuf,
    pub source: Rc<std::io::Error> // Rc is needed as the error itself is not clonable
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
            match std::fs::read_dir(repo) {
                Ok(iter) => {
                    for entry in iter {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                if path.is_dir() && is_content_dir(&path) {
                                    self.found_content.push(ContentDirectory::new(path));
                                }
                            },
                            Err(err) => {
                                self.errors.push(ContentRepositoryScanError {
                                    repo_path: repo.to_owned(),
                                    source: Rc::new(err)
                                });
                            }
                        }
                    }
                },
                Err(err) => {
                    self.errors.push(ContentRepositoryScanError {
                        repo_path: repo.to_owned(),
                        source: Rc::new(err)
                    });
                }
            }
        }
    }
}


fn is_content_dir(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }

    let manifest_path = path.join(Manifest::FILE_NAME);
    if manifest_path.exists() {
        return true;
    }

    let scripts_path = path.join("content").join("scripts");
    if scripts_path.is_dir() {
        return true;
    }

    false
}