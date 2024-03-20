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
    fn source_tree_root(&self) -> &Path;
    fn dependencies(&self) -> Option<&Dependencies>;
    
    fn as_any(self: Box<Self>) -> Box<dyn Any>;

    fn source_tree(&self) -> SourceTree {
        SourceTree::new(self.source_tree_root().to_owned())
    }
}


/// Directory that has "scripts" folder directly inside it i.e. content0.
#[derive(Debug, Clone)]
pub struct UnpackedContentDirectory {
    path: PathBuf,
    script_root: PathBuf
}

impl UnpackedContentDirectory {
    pub fn new(path: PathBuf) -> Self {
        let script_root = path.join("scripts");

        Self {
            path,
            script_root
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

    fn source_tree_root(&self) -> &Path {
        &self.script_root
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
    path: PathBuf,
    script_root: PathBuf
}

impl PackedContentDirectory {
    pub fn new(path: PathBuf) -> Self {
        let script_root = path.join("content").join("scripts");

        Self {
            path,
            script_root
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

    fn source_tree_root(&self) -> &Path {
        &self.script_root
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
    script_root: PathBuf,
    manifest: Manifest
}

impl ProjectDirectory {
    pub fn new(path: PathBuf) -> Result<Self, ManifestParseError> {
        let script_root = path.join("scripts");
        let manifest_path = path.join(Manifest::FILE_NAME);
        let manifest = Manifest::from_file(manifest_path)?;

        Ok(Self {
            path,
            script_root,
            manifest,
        })
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.path.join(Manifest::FILE_NAME)
    }
}

impl Content for ProjectDirectory {
    fn path(&self) -> &Path {
        &self.path
    }

    fn content_name(&self) -> &str {
        &self.manifest.content.name
    }

    fn source_tree_root(&self) -> &Path {
        &self.script_root
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

pub fn find_content_in_directory(path: &Path, scan_recursively: bool) -> (Vec<Box<dyn Content>>, Vec<ContentScanError>) {
    let mut contents = Vec::new();
    let mut errors = Vec::new();

    if path.is_dir() {
        if let Ok(content) = try_make_content(path) {
            contents.push(content);
        } else {
            _find_content_in_directory(path, scan_recursively, &mut contents, &mut errors);
        }
    }

    (contents, errors)
}

fn _find_content_in_directory(path: &Path, scan_recursively: bool, contents: &mut Vec<Box<dyn Content>>, errors: &mut Vec<ContentScanError>) {
    match std::fs::read_dir(&path) {
        Ok(iter) => {
            for entry in iter {
                match entry {
                    Ok(entry) => {
                        let candidate = entry.path();
                        if candidate.is_dir() {
                            match try_make_content(&candidate) {
                                Ok(content) => contents.push(content),
                                Err(err) => {
                                    if let (&ContentScanError::NotContent, true) = (&err, scan_recursively) {
                                        _find_content_in_directory(&candidate, scan_recursively, contents, errors)
                                    } else {
                                        errors.push(err);
                                    }
                                }
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
}

pub fn try_make_content(path: &Path) -> Result<Box<dyn Content>, ContentScanError> {
    let manifest_path = path.join(Manifest::FILE_NAME);
    if manifest_path.exists() {
        match ProjectDirectory::new(path.to_path_buf()) {
            Ok(proj) => {
                Ok(Box::new(proj))
            },
            Err(err) => {
                Err(FileError::new(manifest_path, err).into())
            },
        }
    } else if path.join("scripts").exists() {
        Ok(Box::new(UnpackedContentDirectory::new(path.to_path_buf())))
    } else if path.join("content").join("scripts").exists() {
        Ok(Box::new(PackedContentDirectory::new(path.to_path_buf())))
    } else {
        Err(ContentScanError::NotContent)
    }
}