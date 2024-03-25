use std::borrow::Borrow;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::path::Path;
use std::sync::Arc;
use abs_path::AbsPath;
use crate::FileError;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceFilePath {
    script_root: Arc<AbsPath>,
    abs_path: AbsPath
}

impl SourceFilePath {
    fn new(script_root: Arc<AbsPath>, abs_path: AbsPath) -> Self {
        Self {
            script_root,
            abs_path
        }
    }

    /// A full path to the file
    pub fn absolute(&self) -> &AbsPath {
        &self.abs_path
    }

    /// Path relative tp the root
    pub fn local(&self) -> &Path {
        self.abs_path.strip_prefix(self.script_root.as_ref()).unwrap()
    }

    /// A full path to the "scripts" directory
    pub fn root(&self) -> &AbsPath {
        self.script_root.as_ref()
    }
}

impl Borrow<AbsPath> for SourceFilePath {
    fn borrow(&self) -> &AbsPath {
        &self.abs_path
    }
}

impl Display for SourceFilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abs_path)
    }
}


#[derive(Debug, Clone)]
pub struct SourceTree {
    script_root: Arc<AbsPath>,
    tree: BTreeSet<SourceFilePath>,
    /// Errors encountered during scanning
    pub errors: Vec<FileError<std::io::Error>>
}

impl SourceTree {
    pub(crate) fn new(script_root: AbsPath) -> Self {
        let mut tree = Self {
            script_root: Arc::new(script_root),
            tree: BTreeSet::new(),
            errors: Vec::new()
        };

        tree.scan();

        tree
    }

    fn scan_visit_dir(&mut self, path: AbsPath) {
        match std::fs::read_dir(&path) {
            Ok(iter) => {
                for entry in iter {
                    match entry {
                        Ok(entry) => {
                            let path = AbsPath::resolve(entry.path(), None).unwrap();
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
                self.errors.push(FileError::new(path.clone(), err));
            },
        }
    }

    fn scan_visit_file(&mut self, path: AbsPath) {
        if let Some(ext) = path.extension() {
            if ext == "ws" {
                self.tree.insert(SourceFilePath::new(self.script_root.clone(), path));
            }
        }
    }

    pub fn scan(&mut self) -> SourceTreeDifference {
        let mut diff = SourceTreeDifference::default();
        let old_tree = std::mem::replace(&mut self.tree, BTreeSet::new());
        self.errors.clear();

        if self.script_root.is_dir() {
            self.scan_visit_dir(self.script_root.as_ref().to_owned());

            diff.added.extend(self.tree.difference(&old_tree).cloned());
            diff.removed.extend(old_tree.difference(&self.tree).cloned());
        }
        
        diff
    }

    
    pub fn script_root(&self) -> &AbsPath {
        &self.script_root
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn contains(&self, path: &AbsPath) -> bool {
        self.tree.contains(path)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SourceFilePath> {
        self.tree.iter()
    }    
}

impl IntoIterator for SourceTree {
    type Item = SourceFilePath;
    type IntoIter = <BTreeSet<SourceFilePath> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.tree.into_iter()
    }
}

//TODO remake as an enum, add Changed variant, add timestamp to SourceFilePath
#[derive(Debug, Clone, Default)]
pub struct SourceTreeDifference {
    pub added: Vec<SourceFilePath>,
    pub removed: Vec<SourceFilePath>
}

impl SourceTreeDifference {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
}
