use std::path::{Path, PathBuf};
use crate::manifest::{Manifest, ManifestError};
use crate::source_tree::SourceTree;


#[derive(Debug, Clone)]
pub struct Content {
    path: PathBuf,
    source_tree: SourceTree
}

impl Content {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let absolute = path.as_ref().canonicalize().unwrap();
        Self {
            source_tree: SourceTree::new(absolute.join("content").join("scripts")),
            path: absolute,
        }
    }

    pub fn maybe_content_dir(path: &Path) -> bool {
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


    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        // hopefully this won't ever have to panic
        self.path.file_name().unwrap().to_str().unwrap()
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

    pub fn source_tree(&self) -> &SourceTree {
        &self.source_tree
    }

    pub fn source_tree_mut(&mut self) -> &mut SourceTree {
        &mut self.source_tree
    }
}