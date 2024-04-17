use abs_path::AbsPath;
use crate::{content::{try_make_content, ContentScanError, ProjectDirectory, RedkitProjectDirectory}, Content, FileError};


#[derive(Debug, Clone)]
pub struct ContentScanner {
    scan_root: AbsPath,
    recursive: bool,
    only_projects: bool
}

impl ContentScanner {
    pub fn new(scan_root: AbsPath) -> Result<Self, ContentScanError> {
        if !scan_root.is_dir() {
            return Err(ContentScanError::Io(FileError::new(
                scan_root, 
                std::io::Error::from(std::io::ErrorKind::NotFound)
            ))); 
        }

        Ok(Self {
            scan_root,
            recursive: false,
            only_projects: false
        })
    }

    pub fn recursive(self, val: bool) -> Self {
        Self {
            recursive: val,
            ..self
        }
    }

    pub fn only_projects(self, val: bool) -> Self {
        Self {
            only_projects: val,
            ..self
        }
    }


    pub fn scan(&self) -> (Vec<Box<dyn Content>>, Vec<ContentScanError>) {
        let mut contents = Vec::new();
        let mut errors = Vec::new();
        //FIXME Error not handled!
        if let Ok(content) = try_make_content(&self.scan_root) {
            contents.push(content);
        } else {
            self.find_content_in_directory(&self.scan_root, &mut contents, &mut errors);
        }

        if self.only_projects {
            contents.retain(|c| c.as_any().is::<ProjectDirectory>() || c.as_any().is::<RedkitProjectDirectory>());
        }

        (contents, errors)
    }

    fn find_content_in_directory(&self, path: &AbsPath, contents: &mut Vec<Box<dyn Content>>, errors: &mut Vec<ContentScanError>) {
        match std::fs::read_dir(path) {
            Ok(iter) => {
                for entry in iter {
                    match entry {
                        Ok(entry) => {
                            let candidate = AbsPath::resolve(entry.path(), None).unwrap();
                            if candidate.is_dir() {
                                match try_make_content(&candidate) {
                                    Ok(content) => {
                                        contents.push(content)
                                    },
                                    Err(err) => {
                                        if let &ContentScanError::NotContent = &err {
                                            if self.recursive {
                                                self.find_content_in_directory(&candidate, contents, errors)
                                            }
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
    fn test_repos() {
        let scan_dir = test_assets().join("dir1").unwrap();
        let scanner = ContentScanner::new(scan_dir.clone()).unwrap()
            .only_projects(false)
            .recursive(false);

        let (contents, errors) = scanner.scan();

        assert!(errors.is_empty());
        assert_eq!(contents.len(), 4);
        assert!(contents.iter().any(|c| c.path() == &scan_dir.join("proj1").unwrap()));
        assert!(contents.iter().any(|c| c.path() == &scan_dir.join("proj2").unwrap()));
        assert!(contents.iter().any(|c| c.path() == &scan_dir.join("raw1").unwrap()));
        assert!(contents.iter().any(|c| c.path() == &scan_dir.join("redkit").unwrap()));
    }

    #[test]
    fn test_workspaces() {
        let scan_dir = test_assets().join("dir1").unwrap();
        let scanner = ContentScanner::new(scan_dir.clone()).unwrap()
            .only_projects(true)
            .recursive(true);

        let (contents, errors) = scanner.scan();

        assert!(errors.is_empty());
        assert_eq!(contents.len(), 4);
        assert!(contents.iter().any(|c| c.path() == &scan_dir.join("proj1").unwrap()));
        assert!(contents.iter().any(|c| c.path() == &scan_dir.join("proj2").unwrap()));
        assert!(contents.iter().any(|c| c.path() == &scan_dir.join("nested/proj3").unwrap()));
        assert!(contents.iter().any(|c| c.path() == &scan_dir.join("redkit").unwrap()));
    }
}