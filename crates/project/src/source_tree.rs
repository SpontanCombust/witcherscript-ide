use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use crate::FileError;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceFilePath {
    script_root: Rc<PathBuf>,
    abs_path: PathBuf
}

impl SourceFilePath {
    fn new(script_root: Rc<PathBuf>, abs_path: PathBuf) -> Self {
        Self {
            script_root,
            abs_path
        }
    }

    /// A full path to the file
    pub fn absolute(&self) -> &Path {
        &self.abs_path
    }

    /// A path to the file relative to the "scripts" directory
    pub fn local(&self) -> &Path {
        self.abs_path.strip_prefix(self.script_root.as_ref()).unwrap()
    }

    /// A full path to the root "scripts" directory
    pub fn root(&self) -> &Path {
        self.script_root.as_ref()
    }
}


#[derive(Debug, Clone)]
pub struct SourceTree {
    script_root: Rc<PathBuf>,
    tree: BTreeSet<SourceFilePath>,
    /// Errors encountered during scanning
    pub errors: Vec<FileError<std::io::Error>>
}

impl SourceTree {
    /// `script_root` should be the `{content_name}/content/scripts` directory
    pub(crate) fn new(script_root: PathBuf) -> Self {
        let mut tree = Self {
            script_root: Rc::new(script_root),
            tree: BTreeSet::new(),
            errors: Vec::new()
        };

        if tree.script_root.is_dir() {
            tree.scan_visit_dir(tree.script_root.to_path_buf());
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
                            self.errors.push(FileError::new(path.clone(), err));
                        }
                    }
                }
            },
            Err(err) => {
                self.errors.push(FileError::new(path, err));
            },
        }
    }

    fn scan_visit_file(&mut self, path: PathBuf) {
        if let Some(ext) = path.extension() {
            if ext == "ws" {
                self.tree.insert(SourceFilePath::new(self.script_root.clone(), path));
            }
        }
    }


    pub fn script_root(&self) -> &Path {
        &self.script_root
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &SourceFilePath> {
        self.tree.iter()
    }
}