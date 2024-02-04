use std::path::{Path, PathBuf};
use crate::manifest::{Manifest, ManifestError};
use crate::source_tree::SourceTree;


#[derive(Debug, Clone)]
pub struct Content {
    path: PathBuf,
    source_tree: SourceTree,
    manifest: Option<Manifest>
}

impl Content {
    pub fn new<P>(path: P) -> Self 
    where P: AsRef<Path> {
        let absolute = path.as_ref().canonicalize().unwrap();
        Self {
            source_tree: SourceTree::new(absolute.join("content").join("scripts")),
            path: absolute,
            manifest: None
        }
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
        self.manifest.is_some()
    }

    pub fn manifest(&self) -> Option<&Manifest> {
        self.manifest.as_ref()
    }

    pub fn source_tree(&self) -> &SourceTree {
        &self.source_tree
    }


    pub fn build_source_tree(&mut self) -> Result<(), std::io::Error> {
        self.source_tree.build()
    }

    pub fn scan_manifest(&mut self) -> Option<Result<(), ManifestError>> {
        let manifest_path = self.path.join(Manifest::FILE_NAME);
        if manifest_path.exists() {
            Some(match Manifest::from_file(manifest_path) {
                Ok(manifest) => {
                    self.manifest = Some(manifest);
                    Ok(())
                },
                Err(err) => {
                    Err(err)
                }
            })
        } else {
            None
        }
    }
}