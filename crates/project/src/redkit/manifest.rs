use std::{fs::File, io::Read, str::FromStr, sync::Arc};
use serde::Deserialize;
use thiserror::Error;
use lsp_types as lsp;
use abs_path::AbsPath;


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedkitManifest {
    pub name: String,
    pub version: String,
    pub game_version: String,
    pub author: String,
    pub description: String
}

impl RedkitManifest {
    pub const EXTENSION: &'static str = "w3edit";

    pub fn from_file(path: &AbsPath) -> Result<Self, Error> {
        let mut f = File::open(path).map_err(|err| Arc::new(err))?;

        let mut buff = String::new();
        f.read_to_string(&mut buff).map_err(|err| Arc::new(err))?;

        Self::from_str(&buff)
    }
}

impl FromStr for RedkitManifest {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<Self>(s) {
            Ok(manifest) => {
                Ok(manifest)
            },
            Err(err) => {
                let position = lsp::Position::new(err.line().max(1) as u32 - 1, err.column().max(1) as u32 - 1);
                Err(Error::Json { 
                    position, 
                    msg: err.to_string() 
                })
            },
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("file access error: {}", .0)]
    Io(#[from] Arc<std::io::Error>),
    #[error("JSON parsing error: {msg}")]
    Json {
        position: lsp::Position,
        msg: String
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
            manifest_dir.join("assets/tests").unwrap()
        })
    }


    #[test]
    fn test() {
        let path = test_assets().join("dir1/redkit/redkit_proj.w3edit").unwrap();
        let manifest = RedkitManifest::from_file(&path).unwrap();

        assert_eq!(manifest.name, "redkit_proj");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.game_version, "4.04");
        assert_eq!(manifest.description, "Very cool mod!");
        assert_eq!(manifest.author, "Foo Bar");
    }
}