use std::fs;
use std::path::{Path, PathBuf};
use crate::Content;


pub struct ContentRepository {
    path: PathBuf
}

impl ContentRepository {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().into()
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }


    pub fn find_content(&self) -> Result<Vec<Content>, std::io::Error> {
        let mut v = Vec::new();

        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && Content::maybe_content_dir(&path) {
                v.push(Content::new(path));
            }
        }

        Ok(v)
    }
}