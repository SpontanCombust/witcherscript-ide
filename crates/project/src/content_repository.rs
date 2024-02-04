use std::fs;
use std::path::Path;
use crate::{Content, Manifest};


pub fn find_content_in_repository<P: AsRef<Path>>(path: P) -> Result<Vec<Content>, std::io::Error> {
    let mut v = Vec::new();

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && maybe_content_dir(&path) {
            v.push(Content::new(path));
        }
    }

    Ok(v)
}

fn maybe_content_dir(path: &Path) -> bool {
    let manifest_path = path.join(Manifest::FILE_NAME);
    if manifest_path.exists() {
        return true;
    }

    let scripts_path = path.join("content").join("scripts");
    if scripts_path.is_dir() {
        return true;
    }

    false
}


