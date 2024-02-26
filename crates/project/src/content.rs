use std::any::Any;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::manifest::{Dependencies, Manifest, ManifestParseError};
use crate::source_tree::SourceTree;
use crate::FileError;


/// Characteristics of a directory that contains a "scripts" folder.
pub trait Content : core::fmt::Debug + dyn_clone::DynClone + Send + Sync {
    fn path(&self) -> &Path;
    fn content_name(&self) -> &str;
    fn source_tree(&self) -> SourceTree;
    fn dependencies(&self) -> Option<&Dependencies>;

    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}


/// Directory that has "scripts" folder directly inside it i.e. content0.
#[derive(Debug, Clone)]
pub struct UnpackedContentDirectory {
    path: PathBuf
}

impl UnpackedContentDirectory {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path
        }
    }
}

impl Content for UnpackedContentDirectory {
    fn path(&self) -> &Path {
        &self.path
    }

    fn content_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    fn source_tree(&self) -> SourceTree {
        let script_root = self.path.join("scripts");
        SourceTree::new(script_root)
    }

    fn dependencies(&self) -> Option<&Dependencies> {
        None
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}


/// Directory that has an intermediary "content" folder inside it.
/// This means every packed mod folder inside "Mods" directory.
#[derive(Debug, Clone)]
pub struct PackedContentDirectory {
    path: PathBuf
}

impl PackedContentDirectory {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path
        }
    }
}

impl Content for PackedContentDirectory {
    fn path(&self) -> &Path {
        &self.path
    }

    fn content_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    fn source_tree(&self) -> SourceTree {
        let script_root = self.path.join("content").join("scripts");
        SourceTree::new(script_root)
    }

    fn dependencies(&self) -> Option<&Dependencies> {
        None
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}


/// Directory with a script project manifest.
#[derive(Debug, Clone)]
pub struct ProjectDirectory {
    path: PathBuf,
    manifest: Manifest
}

impl ProjectDirectory {
    pub fn new(path: PathBuf) -> Result<Self, ManifestParseError> {
        let manifest_path = path.join(Manifest::FILE_NAME);
        let manifest = Manifest::from_file(manifest_path)?;

        Ok(Self {
            path,
            manifest
        })
    }
}

impl Content for ProjectDirectory {
    fn path(&self) -> &Path {
        &self.path
    }

    fn content_name(&self) -> &str {
        &self.manifest.content.name
    }

    fn source_tree(&self) -> SourceTree {
        let script_root = self.path.join("scripts");
        SourceTree::new(script_root)
    }

    fn dependencies(&self) -> Option<&Dependencies> {
        Some(&self.manifest.dependencies)
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}





#[derive(Debug, Clone, Error)]
pub enum ContentScanError {
    #[error(transparent)]
    Io(FileError<std::io::Error>),
    #[error(transparent)]
    ManifestParse(FileError<ManifestParseError>),
    #[error("this is not content directory")]
    NotContent
}

impl From<FileError<std::io::Error>> for ContentScanError {
    fn from(value: FileError<std::io::Error>) -> Self {
        Self::Io(value)
    }
}

impl From<FileError<ManifestParseError>> for ContentScanError {
    fn from(value: FileError<ManifestParseError>) -> Self {
        Self::ManifestParse(value)
    }
}

pub fn find_content_in_directory(path: &Path) -> (Vec<Box<dyn Content>>, Vec<ContentScanError>) {
    let mut contents = Vec::new();
    let mut errors = Vec::new();

    match std::fs::read_dir(&path) {
        Ok(iter) => {
            for entry in iter {
                match entry {
                    Ok(entry) => {
                        let candidate = entry.path();
                        if candidate.is_dir() {
                            if let Some(candidate_result) = test_make_content(&candidate) {
                                match candidate_result {
                                    Ok(content) => contents.push(content),
                                    Err(err) => errors.push(err),
                                }
                            } else {
                                let (inner_contents, inner_errors) = find_content_in_directory(&candidate);
                                contents.extend(inner_contents);
                                errors.extend(inner_errors);
                            }
                        }
                    },
                    Err(err) => {
                        errors.push(FileError::new(path, err).into());
                    }
                }
            }
        },
        Err(err) => {
            errors.push(FileError::new(path, err).into());
        }
    }

    (contents, errors)
}

fn test_make_content(path: &Path) -> Option<Result<Box<dyn Content>, ContentScanError>> {
    let manifest_path = path.join(Manifest::FILE_NAME);
    if manifest_path.exists() {
        match ProjectDirectory::new(path.to_path_buf()) {
            Ok(proj) => {
                Some(Ok(Box::new(proj)))
            },
            Err(err) => {
                Some(Err(FileError::new(manifest_path, err).into()))
            },
        }
    } else if path.join("scripts").exists() {
        Some(Ok(Box::new(UnpackedContentDirectory::new(path.to_path_buf()))))
    } else if path.join("content").join("scripts").exists() {
        Some(Ok(Box::new(PackedContentDirectory::new(path.to_path_buf()))))
    } else {
        None
    }
}

pub fn make_content(path: &Path) -> Result<Box<dyn Content>, ContentScanError> {
    if let Some(content) = test_make_content(path) {
        content
    } else {
        Err(ContentScanError::NotContent)
    }
}