use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::manifest::{Manifest, ManifestError};
use crate::source_tree::SourceTree;
use crate::FileError;


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

    pub fn find_in<P>(path: P, manifest_required: bool, scan_recursively: bool) -> (Vec<Self>, Vec<FileError>)
    where P: AsRef<Path> {
        let mut contents = Vec::new();
        let mut errors = Vec::new();

        match std::fs::read_dir(path.as_ref()) {
            Ok(iter) => {
                for entry in iter {
                    match entry {
                        Ok(entry) => {
                            let content_path_candidate = entry.path();
                            if is_content_dir(&content_path_candidate, manifest_required) {
                                contents.push(ContentDirectory::new(content_path_candidate));
                            } else if content_path_candidate.is_dir() && scan_recursively {
                                let (inner_contents, inner_errors) = Self::find_in(content_path_candidate, manifest_required, scan_recursively);
                                contents.extend(inner_contents);
                                errors.extend(inner_errors);
                            }
                        },
                        Err(err) => {
                            errors.push(FileError {
                                path: path.as_ref().to_owned(),
                                error: Arc::new(err)
                            });
                        }
                    }
                }
            },
            Err(err) => {
                errors.push(FileError {
                    path: path.as_ref().to_owned(),
                    error: Arc::new(err)
                });
            }
        }

        (contents, errors)
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


fn is_content_dir(path: &Path, manifest_required: bool) -> bool {
    if !path.is_dir() {
        return false;
    }

    let manifest_path = path.join(Manifest::FILE_NAME);
    if manifest_path.exists() {
        return true;
    }
    if manifest_required {
        return false;
    }

    let scripts_path = path.join("content").join("scripts");
    if scripts_path.is_dir() {
        return true;
    }

    false
}