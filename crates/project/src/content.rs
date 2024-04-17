use std::any::Any;
use std::path::PathBuf;
use abs_path::AbsPath;
use thiserror::Error;

use crate::manifest::{self, Manifest};
use crate::source_tree::SourceTree;
use crate::{redkit, FileError};


/// Characteristics of a directory that contains a "scripts" folder.
pub trait Content : core::fmt::Debug + dyn_clone::DynClone + Send + Sync {
    fn try_from_dir(dir: &AbsPath) -> Result<Self, ContentScanError> where Self: Sized;

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


/// Just a directory that has "scripts" folder inside it, directly or in an intermediary "content" subfolder
#[derive(Debug, Clone)]
pub struct RawContentDirectory {
    path: AbsPath,
    script_root: AbsPath
}

impl Content for RawContentDirectory {
    fn try_from_dir(dir: &AbsPath) -> Result<Self, ContentScanError> where Self: Sized {
        let script_root = dir.join("scripts").unwrap();
        if script_root.exists() {
            let path = if dir.file_name().unwrap().to_str().unwrap() == "content" {
                dir.parent().unwrap()
            } else {
                dir.to_owned()
            };

            Ok(Self {
                path,
                script_root
            })
        } else {
            Err(ContentScanError::NotContent)
        }
    }


    fn path(&self) -> &AbsPath {
        &self.path
    }

    fn content_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    fn source_tree_root(&self) -> &AbsPath {
        &self.script_root
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
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
    pub fn manifest_path(&self) -> &AbsPath {
        &self.manifest_path
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
}

impl Content for ProjectDirectory {     
    fn try_from_dir(dir: &AbsPath) -> Result<Self, ContentScanError> where Self: Sized {
        let manifest_path = dir.join(Manifest::FILE_NAME).unwrap();
        if manifest_path.exists() {
            match Manifest::from_file(&manifest_path) {
                Ok(manifest) => {
                    let manifest_script_root = manifest.content.scripts_root.clone().unwrap_or(PathBuf::from("scripts"));
                    let script_root = dir.join(manifest_script_root).unwrap();

                    Ok(Self {
                        path: dir.to_owned(),
                        manifest_path,
                        script_root,
                        manifest,
                    })
                },
                Err(err) => {
                    Err(FileError::new(manifest_path, err).into())
                },
            }
        } else {
            Err(ContentScanError::NotContent)
        }
    }


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


#[derive(Debug, Clone)]
pub struct RedkitProjectDirectory {
    path: AbsPath,
    manifest_path: AbsPath,
    script_root: AbsPath,
    manifest: redkit::RedkitManifest
}

impl RedkitProjectDirectory {
    fn find_manifest_file(dir: &AbsPath) -> Option<AbsPath> {
        if let Ok(iter) = std::fs::read_dir(dir) {
            iter.filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .find(|path| path.extension().filter(|ext| ext.to_string_lossy() == redkit::RedkitManifest::EXTENSION).is_some())
                .map(|path| AbsPath::resolve(path, None).unwrap())
        } else {
            None
        }
    }

    pub fn manifest_path(&self) -> &AbsPath {
        &self.manifest_path
    }

    pub fn manifest(&self) -> &redkit::RedkitManifest {
        &self.manifest
    }
}

impl Content for RedkitProjectDirectory {
    fn try_from_dir(dir: &AbsPath) -> Result<Self, ContentScanError> where Self: Sized {
        if let Some(manifest_path) = Self::find_manifest_file(dir) {
            match redkit::RedkitManifest::from_file(&manifest_path) {
                Ok(manifest) => {
                    let script_root = dir.join("workspace/scripts").unwrap();
                    
                    Ok(Self {
                        path: dir.to_owned(),
                        manifest_path,
                        manifest,
                        script_root
                    })
                },
                Err(err) => {
                    Err(FileError::new(manifest_path, err).into())
                },
            }
        } else {
            Err(ContentScanError::NotContent)
        }
    }


    fn path(&self) -> &AbsPath {
        &self.path
    }

    fn content_name(&self) -> &str {
        &self.manifest.name
    }

    fn source_tree_root(&self) -> &AbsPath {
        &self.script_root
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}


#[derive(Debug, Clone, Error)]
pub enum ContentScanError {
    #[error(transparent)]
    Io(#[from] FileError<std::io::Error>),
    #[error(transparent)]
    ManifestParse(#[from] FileError<manifest::Error>), //todo rename to ManifestRead
    #[error(transparent)]
    RedkitManifestRead(#[from] FileError<redkit::manifest::Error>),
    #[error("this is not content directory")]
    NotContent,
}

pub fn try_make_content(path: &AbsPath) -> Result<Box<dyn Content>, ContentScanError> {
    if !path.exists() {
        return Err(FileError::new(path.to_owned(), std::io::Error::from(std::io::ErrorKind::NotFound)).into())
    } else if !path.is_dir() {
        return Err(ContentScanError::NotContent);
    }

    if let Some(proj) = try_make_specific_content::<ProjectDirectory>(path) {
        return proj;
    }
    if let Some(redkit_proj) = try_make_specific_content::<RedkitProjectDirectory>(path) {
        return redkit_proj;
    }
    if let Some(raw) = try_make_specific_content::<RawContentDirectory>(path) {
        return raw;
    }

    let packed_path = path.join("content").unwrap();

    if packed_path.exists() {
        if let Some(proj) = try_make_specific_content::<ProjectDirectory>(&packed_path) {
            return proj;
        }
        if let Some(raw) = try_make_specific_content::<RawContentDirectory>(&packed_path) {
            return raw;
        }
    }
    
    Err(ContentScanError::NotContent)
}

fn try_make_specific_content<C: Content + 'static>(path: &AbsPath) -> Option<Result<Box<dyn Content>, ContentScanError>> {
    match C::try_from_dir(path) {
        Ok(content) => {
            Some(Ok(Box::new(content)))
        },
        Err(err) => {
            if matches!(err, ContentScanError::NotContent) {
                None
            } else {
                Some(Err(err))
            }
        },
    }
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
        let path = test_assets().join("dir1/nonexistent").unwrap();
        let content = try_make_content(&path);
        assert!(matches!(content, Err(ContentScanError::Io(_))));

        let path = test_assets();
        let content = try_make_content(&path);
        assert!(matches!(content, Err(ContentScanError::NotContent)));

        let path = test_assets().join("dir1/proj1").unwrap();
        let content = try_make_content(&path).unwrap();
        let proj = content.as_any().downcast_ref::<ProjectDirectory>().unwrap();
        assert_eq!(proj.path(), &path);
        assert_eq!(proj.manifest_path(), &path.join(Manifest::FILE_NAME).unwrap());
        assert_eq!(proj.content_name(), "proj1");
        assert_eq!(proj.source_tree_root(), &path.join("scripts").unwrap());

        let path = test_assets().join("dir1/proj2").unwrap();
        let content = try_make_content(&path).unwrap();
        let proj = content.as_any().downcast_ref::<ProjectDirectory>().unwrap();
        assert_eq!(proj.path(), &path);
        assert_eq!(proj.manifest_path(), &path.join(Manifest::FILE_NAME).unwrap());
        assert_eq!(proj.content_name(), "proj2");
        assert_eq!(proj.source_tree_root(), &path.join("content/scripts").unwrap());

        let path = test_assets().join("dir1/raw1").unwrap();
        let content = try_make_content(&path).unwrap();
        let raw = content.as_any().downcast_ref::<RawContentDirectory>().unwrap();
        assert_eq!(raw.path(), &path);
        assert_eq!(raw.content_name(), "raw1");
        assert_eq!(raw.source_tree_root(), &path.join("scripts").unwrap());

        let path = test_assets().join("dir1/nested/raw2").unwrap();
        let content = try_make_content(&path).unwrap();
        let raw = content.as_any().downcast_ref::<RawContentDirectory>().unwrap();
        assert_eq!(raw.path(), &path);
        assert_eq!(raw.content_name(), "raw2");
        assert_eq!(raw.source_tree_root(), &path.join("content/scripts").unwrap());

        let path = test_assets().join("dir1/redkit").unwrap();
        let content = try_make_content(&path).unwrap();
        let proj = content.as_any().downcast_ref::<RedkitProjectDirectory>().unwrap();
        assert_eq!(proj.path(), &path);
        assert_eq!(proj.manifest_path(), &path.join("redkit_proj.w3edit").unwrap());
        assert_eq!(proj.content_name(), "redkit_proj");
        assert_eq!(proj.source_tree_root(), &path.join("workspace/scripts").unwrap());
    }   
}