use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::fs::read_dir;


#[derive(Debug, Clone)]
pub struct SourceTree {
    script_root: PathBuf,
    tree: BTreeSet<PathBuf>
}

impl SourceTree {
    /// `script_root` should be the `{content_name}/content/scripts` directory
    pub fn new<P: AsRef<Path>>(script_root: P) -> Self {
        Self {
            script_root: script_root.as_ref().into(),
            tree: BTreeSet::new()
        }
    }


    pub fn build(&mut self) -> Result<(), std::io::Error> {
        self.tree.clear();   

        if self.script_root.is_dir() {
            self.build_visit_dir(self.script_root.clone())?;
        }

        Ok(())
    }

    fn build_visit_dir(&mut self, path: PathBuf) -> Result<(), std::io::Error> {
        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.build_visit_dir(path)?;
            } else {
                self.build_visit_file(path);
            }
        }

        Ok(())
    }

    fn build_visit_file(&mut self, path: PathBuf) {
        if let Some(ext) = path.extension() {
            if ext == "ws" {
                let relative = path.strip_prefix(&self.script_root).unwrap();
                self.tree.insert(relative.into());
            }
        }
    }


    pub fn script_root(&self) -> &Path {
        &self.script_root
    }

    pub fn contains<P: AsRef<Path>>(&self, path: P) -> bool {
        self.tree.contains(path.as_ref())
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Path> {
        self.tree.iter().map(|buf| buf.as_path())
    }
}