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

dyn_clone::clone_trait_object!(Content);


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
    manifest: Manifest
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

pub fn try_make_content(path: &AbsPath) -> Result<Box<dyn Content>, ContentScanError> {
    if !path.exists() {
        return Err(FileError::new(path.to_owned(), std::io::Error::from(std::io::ErrorKind::NotFound)).into())
    }

    let manifest_path = path.join(Manifest::FILE_NAME).unwrap();
    if manifest_path.exists() {
        match ProjectDirectory::new(path.clone()) {
            Ok(proj) => {
                return Ok(Box::new(proj));
            },
            Err(err) => {
                return Err(FileError::new(manifest_path, err).into());
            }
        };
    } else if path.join("scripts").unwrap().exists() {
        return Ok(Box::new(UnpackedContentDirectory::new(path.clone())));
    } 
    
    let packed_path = path.join("content").unwrap();

    if packed_path.exists() {
        let manifest_path = packed_path.join(Manifest::FILE_NAME).unwrap();
        if manifest_path.exists() {
            match ProjectDirectory::new(packed_path.clone()) {
                Ok(proj) => {
                    return Ok(Box::new(proj));
                },
                Err(err) => {
                    return Err(FileError::new(manifest_path, err).into());
                }
            };
        } else if packed_path.join("scripts").unwrap().exists() {
            return Ok(Box::new(PackedContentDirectory::new(path.clone())));
        }
    }
    
    Err(ContentScanError::NotContent)
}




#[cfg(test)]
mod test {
    use std::sync::OnceLock;
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
        let path = test_assets().join("nonexistent").unwrap();
        let content = try_make_content(&path);
        assert!(matches!(content, Err(ContentScanError::Io(_))));

        let path = test_assets();
        let content = try_make_content(&path);
        assert!(matches!(content, Err(ContentScanError::NotContent)));

        let path = test_assets().join("proj1").unwrap();
        let content = try_make_content(&path).unwrap();
        let proj = content.as_any().downcast_ref::<ProjectDirectory>().unwrap();
        assert_eq!(proj.path(), &path);
        assert_eq!(proj.manifest_path(), &path.join(Manifest::FILE_NAME).unwrap());
        assert_eq!(proj.content_name(), "proj1");
        assert_eq!(proj.source_tree_root(), &path.join("scripts").unwrap());

        let path = test_assets().join("proj2").unwrap();
        let content = try_make_content(&path).unwrap();
        let proj = content.as_any().downcast_ref::<ProjectDirectory>().unwrap();
        assert_eq!(proj.path(), &path);
        assert_eq!(proj.manifest_path(), &path.join(Manifest::FILE_NAME).unwrap());
        assert_eq!(proj.content_name(), "proj2");
        assert_eq!(proj.source_tree_root(), &path.join("content/scripts").unwrap());

        let path = test_assets().join("raw1").unwrap();
        let content = try_make_content(&path).unwrap();
        let raw = content.as_any().downcast_ref::<UnpackedContentDirectory>().unwrap();
        assert_eq!(raw.path(), &path);
        assert_eq!(raw.content_name(), "raw1");
        assert_eq!(raw.source_tree_root(), &path.join("scripts").unwrap());

        let path = test_assets().join("nested/raw2").unwrap();
        let content = try_make_content(&path).unwrap();
        let raw = content.as_any().downcast_ref::<PackedContentDirectory>().unwrap();
        assert_eq!(raw.path(), &path);
        assert_eq!(raw.content_name(), "raw2");
        assert_eq!(raw.source_tree_root(), &path.join("content/scripts").unwrap());
    }   
}