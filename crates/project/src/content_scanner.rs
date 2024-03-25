use abs_path::AbsPath;
use crate::{content::{try_make_content, ContentScanError, ProjectDirectory}, Content, FileError};


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
                std::io::Error::new(std::io::ErrorKind::NotFound, "Path is not an existing directory")
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

        if let Ok(content) = try_make_content(&self.scan_root) {
            contents.push(content);
        } else {
            self.find_content_in_directory(&self.scan_root, &mut contents, &mut errors);
        }

        if self.only_projects {
            contents.retain(|c| c.as_any().is::<ProjectDirectory>());
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
                                        if let (&ContentScanError::NotContent, true) = (&err, self.recursive) {
                                            self.find_content_in_directory(&candidate, contents, errors)
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
