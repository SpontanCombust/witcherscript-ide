use std::{collections::HashSet, ops::BitOr, path::{Path, PathBuf}};


/// A mask that can be tested against local paths in a source tree.
/// 
/// Multiple connected contents can modify a script file that is located at the same local path in a source tree.
/// In the case of mods this is most often seen when a mod modifies a vanilla script. Multiple mods doing this to the same
/// file it create a **conflict**. 
/// 
/// Studying relationships between different contents that depend on each other requires a way to handle such cases.
/// There are two ways of doing this:
/// 1. Dynamically create merges of these files
/// 2. Make one or the other win the conflict
/// 
/// Since script modding implies constant changes to a source code the first approach is out of the question, 
/// since it not only adds processing overhead to the end result, it also just isn't able to solve some conflicts reliably.
/// (Manual conflicts in kdiff3).
/// The second approach is preferred by WitcherScript IDE mainly because developing mods doesn't involve big dependency trees.
/// In fact in majority of cases it's just a single mod modifying a content0 script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMask {
    paths: HashSet<PathBuf>
}

impl SourceMask {
    pub fn empty() -> Self {
        Self { paths: HashSet::new() }
    }

    /// Returns false if the mask contains given path, true otherwise
    pub fn test(&self, path: &Path) -> bool {
        !self.paths.contains(path)
    }

    /// Join two masks together to create a new one.
    pub fn union(&self, other: &Self) -> Self {
        Self {
            paths: self.paths.union(&other.paths).map(|p| p.to_owned()).collect()
        }
    }
}

impl Default for SourceMask {
    fn default() -> Self {
        Self::empty()
    }
}

impl BitOr for &SourceMask {
    type Output = SourceMask;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl FromIterator<PathBuf> for SourceMask {
    fn from_iter<T: IntoIterator<Item = PathBuf>>(iter: T) -> Self {
        Self {
            paths: iter.into_iter().collect()
        }
    }
}