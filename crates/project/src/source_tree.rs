use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::FileError;


#[derive(Debug, Clone)]
pub struct SourceTree {
    script_root: PathBuf,
    tree: BTreeSet<PathBuf>,
    /// Errors encountered during scanning
    pub errors: Vec<FileError>
}

impl SourceTree {
    /// `script_root` should be the `{content_name}/content/scripts` directory
    pub fn new<P: AsRef<Path>>(script_root: P) -> Self {
        let mut tree = Self {
            script_root: script_root.as_ref().into(),
            tree: BTreeSet::new(),
            errors: Vec::new()
        };

        if tree.script_root.is_dir() {
            tree.scan_visit_dir(tree.script_root.clone());
        }

        tree
    }

    fn scan_visit_dir(&mut self, path: PathBuf) {
        match std::fs::read_dir(&path) {
            Ok(iter) => {
                for entry in iter {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_dir() {
                                self.scan_visit_dir(path);
                            } else {
                                self.scan_visit_file(path);
                            }
                        },
                        Err(err) => {
                            self.errors.push(FileError {
                                path: path.clone(),
                                error: Arc::new(err)
                            });
                        }
                    }
                }
            },
            Err(err) => {
                self.errors.push(FileError {
                    path: path.clone(),
                    error: Arc::new(err)
                });
            },
        }
    }

    fn scan_visit_file(&mut self, path: PathBuf) {
        if let Some(ext) = path.extension() {
            if ext == "ws" {
                let relative = path.strip_prefix(&self.script_root).unwrap();
                self.tree.insert(relative.into());
            }
        }
    }


    pub fn script_root(&self) -> &Path {
        &self.script_root
    }

    pub fn contains<P: AsRef<Path>>(&self, path: P) -> bool {
        self.tree.contains(path.as_ref())
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Path> {
        self.tree.iter().map(|buf| buf.as_path())
    }
}