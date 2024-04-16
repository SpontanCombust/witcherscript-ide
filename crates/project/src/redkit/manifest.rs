use std::{fs::File, io::Read, str::FromStr, sync::Arc};
use serde::Deserialize;
use thiserror::Error;
use abs_path::AbsPath;


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedkitProjectManifest {
    pub name: String,
    pub version: String,
    pub game_version: String,
    pub author: String,
    pub description: String
}

impl RedkitProjectManifest {
    pub const EXTENSION: &'static str = "w3edit";

    pub fn from_file(path: &AbsPath) -> Result<Self, Error> {
        let mut f = File::open(path).map_err(|err| Arc::new(err))?;

        let mut buff = String::new();
        f.read_to_string(&mut buff).map_err(|err| Arc::new(err))?;

        Self::from_str(&buff)
    }
}

impl FromStr for RedkitProjectManifest {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manifest = serde_json::from_str(s).map_err(|err| Arc::new(err))?;
        Ok(manifest)
    }
}

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("file access error: {}", .0)]
    Io(#[from] Arc<std::io::Error>),
    #[error("json parsing error: {}", .0)]
    Json(#[from] Arc<serde_json::Error>)
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
        let path = test_assets().join("redkit/redkit_proj.w3edit").unwrap();
        let manifest = RedkitProjectManifest::from_file(&path).unwrap();

        assert_eq!(manifest.name, "redkit_proj");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.game_version, "4.04");
        assert_eq!(manifest.description, "Very cool mod!");
        assert_eq!(manifest.author, "Foo Bar");
    }
}