use std::path::{Path, PathBuf};
use crate::manifest::{Manifest, ManifestError};
use crate::source_tree::SourceTree;


#[derive(Debug, Clone)]
pub struct ContentDirectory {
    path: PathBuf
}

impl ContentDirectory {
    pub fn new<P>(path: P) -> Self 
    where P: AsRef<Path> {
        let absolute = path.as_ref().canonicalize().unwrap();
        Self {
            path: absolute
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Projects without a manifest can be parsed, but their paths need to be explicitly specified.
    /// Manifest-less projects are supported purely out of user convenience, 
    /// so they don't have to manually create manifests for vanilla scripts or installed mods.
    pub fn has_manifest(&self) -> bool {
        let manifest_path = self.path.join(Manifest::FILE_NAME);
        manifest_path.exists()
    }

    pub fn manifest(&self) -> Option<Result<Manifest, ManifestError>> {
        let manifest_path = self.path.join(Manifest::FILE_NAME);
        if manifest_path.exists() {
            Some(Manifest::from_file(manifest_path))
        } else {
            None
        }
    }

    /// The returned source tree is pre-scanned
    pub fn source_tree(&self) -> SourceTree {
        let script_root = self.path.join("content").join("scripts");
        SourceTree::new(script_root)
    }
}
