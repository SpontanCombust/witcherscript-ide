use std::{collections::HashMap, fs::File, io::{self, Read}, path::{Path, PathBuf}, sync::Arc};
use ropey::Rope;
use semver::Version;
use serde::Deserialize;
use thiserror::Error;


/// Deserialized WitcherScript manifest file containing project metadata.
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub content: Content,
    pub dependencies: Dependencies
}

#[derive(Debug, Clone, Deserialize)]
pub struct Content {
    /// Name of this project, for example SharedUtils
    pub name: String,
    /// Version of this project, has to abide to semantic versioning
    pub version: Version,
    /// List of this project authors (optional)
    pub authors: Option<Vec<String>>,
    /// Version(s) of the game this project is compatible with 
    pub game_version: String, // CDPR's versioning system doesn't comply with semver, so string will have to do for now
}

pub type Dependencies = HashMap<String, DependencyValue>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum DependencyValue {
    /// Get the dependency from one of project repositories by name
    FromRepo(bool),
    /// Get the dependency from a specific location
    FromPath {
        path: PathBuf
    }
}


#[derive(Debug, Clone, Error)]
pub enum ManifestError {
    #[error("file access error")]
    Io(#[from] Arc<io::Error>),
    #[error("TOML file parsing error")]
    Toml {
        range: lsp_types::Range,
        msg: String
    }
}

impl Manifest {
    pub const FILE_NAME: &str = "witcherscript.toml";

    pub fn from_str(s: &str) -> Result<Self, ManifestError> {
        let rope = Rope::from_str(s);
        match toml::from_str(s) {
            Ok(toml) => Ok(toml),
            Err(err) => {
                let range = err.span().map(|r| {
                    let start_line = rope.char_to_line(r.start);
                    let start_char = r.start - rope.line_to_char(start_line);
                    let end_line = rope.char_to_line(r.end);
                    let end_char = r.end - rope.line_to_char(end_line);

                    lsp_types::Range { 
                        start: lsp_types::Position { 
                            line: start_line as u32, 
                            character: start_char as u32
                        }, 
                        end: lsp_types::Position { 
                            line: end_line as u32, 
                            character: end_char as u32 
                        }
                    }
                }).unwrap_or(
                    lsp_types::Range {
                        start: lsp_types::Position { 
                            line: 0, 
                            character: 0
                        }, 
                        end: lsp_types::Position { 
                            line: u32::MAX, 
                            character: u32::MAX
                        }
                    }
                );
                
                Err(ManifestError::Toml { 
                    range, 
                    msg: err.message().to_string() 
                })
            },
        }
    }

    pub fn from_file<P>(path: P) -> Result<Self, ManifestError> 
    where P: AsRef<Path> {
        let mut f = File::open(&path).map_err(|err| Arc::new(err))?;

        let mut buff = String::new();
        // manifests are usually comparatively small, so reading it all at once shouldn't be that big of a deal
        f.read_to_string(&mut buff).map_err(|err| Arc::new(err))?;

        Self::from_str(&buff)
    }
}



#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test() {
        let s = r#"
        [content]
        name = "ExampleMod"
        version = "0.9.0"
        authors = ["Rip Van Winkle"]
        game_version = "4.04"
    
        [dependencies]
        content0 = { path = "../Witcher 3/content/content0" }
        shared_utils = true
        "#;
    
        let manifest = Manifest::from_str(s).unwrap();
    
        assert_eq!(manifest.content.name, "ExampleMod");
        assert_eq!(manifest.content.version, Version::from_str("0.9.0").unwrap());
        assert_eq!(manifest.content.authors, Some(vec!["Rip Van Winkle".into()]));
        assert_eq!(manifest.content.game_version, String::from("4.04"));
    
        assert_eq!(manifest.dependencies, HashMap::from_iter([
            ("content0".into(), DependencyValue::FromPath { path: "../Witcher 3/content/content0".into() }),
            ("shared_utils".into(), DependencyValue::FromRepo(true)),
        ]));
    }
}