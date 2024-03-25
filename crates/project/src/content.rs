use std::any::Any;
use std::path::PathBuf;
use abs_path::AbsPath;
use thiserror::Error;

use crate::manifest::{Manifest, ManifestParseError};
use crate::source_tree::SourceTree;
use crate::FileError;


/// Characteristics of a directory that contains a "scripts" folder.
pub trait Content : core::fmt::Debug + dyn_clone::DynClone + Send + Sync {
    fn path(&self) -> &AbsPath;
    fn content_name(&self) -> &str;
    fn source_tree_root(&self) -> &AbsPath;
    
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    fn source_tree(&self) -> SourceTree {
        SourceTree::new(self.source_tree_root().to_owned())
    }
}


/// Directory that has "scripts" folder directly inside it i.e. content0.
#[derive(Debug, Clone)]
pub struct UnpackedContentDirectory {
    path: AbsPath,
    script_root: AbsPath
}

impl UnpackedContentDirectory {
    pub fn new(path: AbsPath) -> Self {
        let script_root = path.join("scripts").unwrap();

        Self {
            path,
            script_root
        }
    }
}

impl Content for UnpackedContentDirectory {
    fn path(&self) -> &AbsPath {
        &self.path
    }

    fn content_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    fn source_tree_root(&self) -> &AbsPath {
        &self.script_root
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


/// Directory that has an intermediary "content" folder inside it.
/// This means every packed mod folder inside "Mods" directory.
#[derive(Debug, Clone)]
pub struct PackedContentDirectory {
    path: AbsPath,
    script_root: AbsPath
}

impl PackedContentDirectory {
    pub fn new(path: AbsPath) -> Self {
        let script_root = path.join("content/scripts").unwrap();

        Self {
            path,
            script_root
        }
    }
}

impl Content for PackedContentDirectory {
    fn path(&self) -> &AbsPath {
        &self.path
    }

    fn content_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    fn source_tree_root(&self) -> &AbsPath {
        &self.script_root
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


/// Directory with a script project manifest.
#[derive(Debug, Clone)]
pub struct ProjectDirectory {
    path: AbsPath,
    manifest_path: AbsPath,
    script_root: AbsPath,
    manifest: Manifest //FIXME when manifest of a project changes it is not registered in LSP, because content graph would still contain the same objects without reparsing the manifest
}

impl ProjectDirectory {
    pub fn new(path: AbsPath) -> Result<Self, ManifestParseError> {
        let manifest_path = path.join(Manifest::FILE_NAME).unwrap();
        let manifest = Manifest::from_file(&manifest_path)?;

        let manifest_script_root = manifest.content.scripts_root.clone().unwrap_or(PathBuf::from("scripts"));
        let script_root = path.join(manifest_script_root).unwrap();
            
        Ok(Self {
            path,
            manifest_path,
            script_root,
            manifest,
        })
    }

    pub fn manifest_path(&self) -> &AbsPath {
        &self.manifest_path
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
}

impl Content for ProjectDirectory {
    fn path(&self) -> &AbsPath {
        &self.path
    }

    fn content_name(&self) -> &str {
        &self.manifest.content.name
    }

    fn source_tree_root(&self) -> &AbsPath {
        &self.script_root
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}





#[derive(Debug, Clone, Error)]
pub enum ContentScanError {
    #[error(transparent)]
    Io(#[from] FileError<std::io::Error>),
    #[error(transparent)]
    ManifestParse(#[from] FileError<ManifestParseError>),
    #[error("this is not content directory")]
    NotContent,
}

pub fn find_content_in_directory(path: &AbsPath, scan_recursively: bool) -> (Vec<Box<dyn Content>>, Vec<ContentScanError>) {
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

fn _find_content_in_directory(path: &AbsPath, scan_recursively: bool, contents: &mut Vec<Box<dyn Content>>, errors: &mut Vec<ContentScanError>) {
    match std::fs::read_dir(path) {
        Ok(iter) => {
            for entry in iter {
                match entry {
                    Ok(entry) => {
                        let candidate = AbsPath::resolve(entry.path(), None).unwrap();
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
                        errors.push(FileError::new(path.clone(), err).into());
                    }
                }
            }
        },
        Err(err) => {
            errors.push(FileError::new(path.clone(), err).into());
        }
    }
}

pub fn try_make_content(path: &AbsPath) -> Result<Box<dyn Content>, ContentScanError> {
    let manifest_path = path.join(Manifest::FILE_NAME).unwrap();
    if manifest_path.exists() {
        match ProjectDirectory::new(path.clone()) {
            Ok(proj) => {
                Ok(Box::new(proj))
            },
            Err(err) => {
                Err(FileError::new(manifest_path, err).into())
            },
        }
    } else if path.join("scripts").unwrap().exists() {
        Ok(Box::new(UnpackedContentDirectory::new(path.clone())))
    } else if path.join("content/scripts").unwrap().exists() {
        Ok(Box::new(PackedContentDirectory::new(path.clone())))
    } else {
        Err(ContentScanError::NotContent)
    }
}