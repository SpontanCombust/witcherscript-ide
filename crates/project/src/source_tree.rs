use std::borrow::Borrow;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use abs_path::AbsPath;
use filetime::FileTime;
use shrinkwraprs::Shrinkwrap;
use crate::FileError;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceTreeFile {
    script_root: Arc<AbsPath>,
    abs_path: AbsPath,
    modified_timestamp: FileTime
}

impl SourceTreeFile {
    /// A full path to the file
    pub fn absolute_path(&self) -> &AbsPath {
        &self.abs_path
    }

    pub fn into_absolute_path(self) -> AbsPath {
        self.abs_path
    }

    /// Path relative tp the root
    pub fn local_path(&self) -> &Path {
        self.abs_path.strip_prefix(self.script_root.as_ref()).unwrap()
    }

    /// A full path to the "scripts" directory
    pub fn script_root(&self) -> &AbsPath {
        self.script_root.as_ref()
    }

    pub fn modified_timestamp(&self) -> FileTime {
        self.modified_timestamp
    }
}

impl Borrow<AbsPath> for SourceTreeFile {
    fn borrow(&self) -> &AbsPath {
        &self.abs_path
    }
}

impl Display for SourceTreeFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abs_path)
    }
}


/// A wrapper type for SourceTreeFile made specifically to compare individual objects in the set only based on abs_path.
/// This in turn means that set operations only take abs_path into account and not modified_timestamp.
#[derive(Debug, Clone, Shrinkwrap)]
struct SourceTreeFileComparatorWrapper {
    inner: SourceTreeFile
}

impl Borrow<AbsPath> for SourceTreeFileComparatorWrapper {
    fn borrow(&self) -> &AbsPath {
        &self.inner.abs_path
    }
}

impl PartialEq for SourceTreeFileComparatorWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.inner.abs_path == other.inner.abs_path
    }
}

impl Eq for SourceTreeFileComparatorWrapper {}

impl PartialOrd for SourceTreeFileComparatorWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.abs_path.partial_cmp(&other.inner.abs_path)
    }
}

impl Ord for SourceTreeFileComparatorWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.abs_path.cmp(&other.inner.abs_path)
    }
}

impl From<SourceTreeFile> for SourceTreeFileComparatorWrapper {
    fn from(value: SourceTreeFile) -> Self {
        Self {
            inner: value
        }
    }
}

impl From<SourceTreeFileComparatorWrapper> for SourceTreeFile {
    fn from(value: SourceTreeFileComparatorWrapper) -> Self {
        value.inner
    }
}


#[derive(Debug, Clone)]
pub struct SourceTree {
    script_root: Arc<AbsPath>,
    tree: BTreeSet<SourceTreeFileComparatorWrapper>,
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
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        let timestamp = FileTime::from_last_modification_time(&metadata);

                        self.tree.insert(SourceTreeFile {
                            script_root: self.script_root.clone(),
                            abs_path: path,
                            modified_timestamp: timestamp
                        }.into());
                    },
                    Err(err) => {
                        self.errors.push(FileError::new(path.clone(), err));
                    },
                }
            }
        }
    }

    pub fn scan(&mut self) -> SourceTreeDifference {
        let old_tree = std::mem::replace(&mut self.tree, BTreeSet::new());
        self.errors.clear();

        self.scan_visit_dir(self.script_root.as_ref().to_owned());

        let diff_added = self.tree.difference(&old_tree).cloned().map(|f| f.inner);
        let diff_removed = old_tree.difference(&self.tree).cloned().map(|f| f.inner);
        let diff_modified = self.tree.iter().filter(|self_file| {
            if let Some(old_file) = old_tree.get(self_file.absolute_path()) {
                self_file.modified_timestamp > old_file.modified_timestamp
            } else {
                false
            }
        }).cloned().map(|f| f.inner);
            
        SourceTreeDifference { 
            added: diff_added.collect(), 
            removed: diff_removed.collect(), 
            modified: diff_modified.collect()
        }
    }

    
    pub fn script_root(&self) -> &AbsPath {
        &self.script_root
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    /// Searches for a file with a given absolute path
    pub fn contains(&self, path: &AbsPath) -> bool {
        self.tree.contains(path)
    }

    /// Searches for a file with a given path relative to the tree root
    pub fn contains_local(&self, path: &Path) -> bool {
        self.tree.iter().any(move |f| {
            let local = f.local_path();
            local == path
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &SourceTreeFile> {
        self.tree.iter().map(|f| f.as_ref())
    }    
}


pub struct IntoIter {
    iter: Box<dyn Iterator<Item = SourceTreeFile>>
}

impl Iterator for IntoIter {
    type Item = SourceTreeFile;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl IntoIterator for SourceTree {
    type Item = SourceTreeFile;
    type IntoIter = self::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: Box::new(self.tree.into_iter().map(|f| f.inner))
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct SourceTreeDifference {
    pub added: Vec<SourceTreeFile>,
    pub removed: Vec<SourceTreeFile>,
    pub modified: Vec<SourceTreeFile>
}

impl SourceTreeDifference {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && 
        self.removed.is_empty() && 
        self.modified.is_empty()
    }
}





#[cfg(test)]
mod test {
    use std::sync::OnceLock;
    use crate::try_make_content;
    use super::*;


    fn test_assets() -> &'static AbsPath {
        static TEST_ASSETS: OnceLock<AbsPath> = OnceLock::new();
        TEST_ASSETS.get_or_init(|| {
            let manifest_dir = AbsPath::resolve(env!("CARGO_MANIFEST_DIR"), None).unwrap();
            manifest_dir.join("../../test_assets/project").unwrap()
        })
    }

    #[test]
    fn test() {
        let path = test_assets().join("dir1/proj1").unwrap();
        let content = try_make_content(&path).unwrap();
        let tree = content.source_tree();
        assert_eq!(tree.len(), 6);
        assert!(tree.contains_local(&Path::new("core").join("2DArray.ws")));
        assert!(tree.contains_local(&Path::new("core").join("states.ws")));
        assert!(tree.contains_local(&Path::new("core").join("string.ws")));
        assert!(tree.contains_local(&Path::new("engine").join("behavior.ws")));
        assert!(tree.contains_local(&Path::new("engine").join("entity.ws")));
        assert!(tree.contains_local(&Path::new("local").join("my_local.ws")));

        let path = test_assets().join("dir1/nested/raw2").unwrap();
        let content = try_make_content(&path).unwrap();
        let tree = content.source_tree();
        assert_eq!(tree.len(), 0);
    }   
}