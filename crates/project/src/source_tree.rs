use std::path::{Path, PathBuf};
use std::sync::Arc;
use abs_path::AbsPath;
use filetime::FileTime;
use crate::FileError;


#[derive(Debug, Clone)]
pub struct SourceTreePath {
    script_root: Arc<AbsPath>,
    abs_path: AbsPath
}

impl SourceTreePath {
    fn new(script_root: Arc<AbsPath>, abs_path: AbsPath) -> Self {
        Self {
            script_root,
            abs_path
        }
    }


    #[inline(always)]
    pub fn absolute(&self) -> &AbsPath {
        &self.abs_path
    }

    #[inline(always)]
    pub fn into_absolute(self) -> AbsPath {
        self.abs_path
    }

    #[inline(always)]
    pub fn local(&self) -> &Path {
        self.abs_path.strip_prefix(self.script_root.as_ref()).unwrap()
    }

    #[inline(always)]
    pub fn into_local(self) -> PathBuf {
        self.abs_path.strip_prefix(self.script_root.as_ref()).unwrap().to_owned()
    }

    #[inline(always)]
    pub fn script_root(&self) -> &AbsPath {
        self.script_root.as_ref()
    }
}

impl PartialEq for SourceTreePath {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.abs_path == other.abs_path
    }
}

impl Eq for SourceTreePath {}

impl PartialOrd for SourceTreePath {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.abs_path.partial_cmp(&other.abs_path)
    }
}

impl Ord for SourceTreePath {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.abs_path.cmp(&other.abs_path)
    }
}

impl std::hash::Hash for SourceTreePath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.abs_path.hash(state);
    }
}

impl std::borrow::Borrow<AbsPath> for SourceTreePath {
    fn borrow(&self) -> &AbsPath {
        &self.abs_path
    }
}

impl std::ops::Deref for SourceTreePath {
    type Target = AbsPath;

    fn deref(&self) -> &Self::Target {
        &self.abs_path
    }
}




#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceTreeFile {
    pub path: SourceTreePath,
    pub modified_timestamp: FileTime
}




#[derive(Debug, Clone)]
pub struct SourceTree {
    script_root: Arc<AbsPath>,
    tree: Vec<SourceTreeFile>,
    /// Errors encountered during scanning
    pub errors: Vec<FileError<std::io::Error>>
}

impl SourceTree {
    pub(crate) fn new(script_root: AbsPath) -> Self {
        let mut tree = Self {
            script_root: Arc::new(script_root),
            tree: Vec::new(),
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
                match std::fs::metadata(&path) {
                    Ok(metadata) => {
                        let modified_timestamp = FileTime::from_last_modification_time(&metadata);

                        self.tree.push(SourceTreeFile {
                            path: SourceTreePath::new(self.script_root.clone(), path),
                            modified_timestamp
                        });
                    },
                    Err(err) => {
                        self.errors.push(FileError::new(path.clone(), err));
                    },
                }
            }
        }
    }

    pub fn scan(&mut self) -> SourceTreeDifference {
        let old_tree = std::mem::replace(&mut self.tree, Vec::new());
        self.errors.clear();

        self.scan_visit_dir(self.script_root.as_ref().to_owned());
        self.tree.sort_by(|a, b| a.path.cmp(&b.path));

        let diff = SourceTreeDifference::from_comparison(old_tree, &self.tree);
            
        diff
    }

    
    pub fn script_root(&self) -> &AbsPath {
        &self.script_root
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    #[inline]
    pub fn find(&self, path: &AbsPath) -> Option<&SourceTreeFile> {
        self.tree.binary_search_by(|f| f.path.abs_path.cmp(path))
            .map(|idx| &self.tree[idx])
            .ok()
    }

    /// Searches for a file with a given absolute path
    #[inline]
    pub fn contains(&self, path: &AbsPath) -> bool {
        self.tree.binary_search_by(|f| f.path.abs_path.cmp(path)).is_ok()
    }

    #[inline]
    pub fn find_local(&self, path: &Path) -> Option<&SourceTreeFile> {
        self.tree.binary_search_by(|f| f.path.local().cmp(path))
            .map(|idx| &self.tree[idx])
            .ok()
    }

    /// Searches for a file with a given path relative to the tree root
    #[inline]
    pub fn contains_local(&self, path: &Path) -> bool {
        self.tree.binary_search_by(|f| f.path.local().cmp(path)).is_ok()
    }

    pub fn iter(&self) -> impl Iterator<Item = &SourceTreeFile> {
        self.tree.iter()
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
            iter: Box::new(self.tree.into_iter())
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
    pub fn from_comparison(old_tree: Vec<SourceTreeFile>, new_tree: &Vec<SourceTreeFile>) -> Self {
        let (mut i, mut j) = (0, 0);
        let mut diff = Self::default();

        while i < old_tree.len() && j < new_tree.len() {
            match old_tree[i].path.cmp(&new_tree[j].path) {
                std::cmp::Ordering::Less => {
                    diff.removed.push(old_tree[i].clone());
                    i += 1;
                },
                std::cmp::Ordering::Equal => {
                    if old_tree[i].modified_timestamp < new_tree[j].modified_timestamp {
                        diff.modified.push(old_tree[i].clone());
                    }
                    i += 1;
                    j += 1;
                },
                std::cmp::Ordering::Greater => {
                    diff.added.push(new_tree[j].clone());
                    j += 1;
                },
            }
        }
        while i < old_tree.len() {
            diff.removed.push(old_tree[i].clone());
            i += 1;
        }
        while j < new_tree.len() {
            diff.added.push(new_tree[j].clone());
            j += 1;
        }

        diff
    }

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