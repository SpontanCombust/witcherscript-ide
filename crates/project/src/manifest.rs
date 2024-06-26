use std::{fs::File, io::{self, Read}, path::PathBuf, str::FromStr, sync::Arc};
use abs_path::AbsPath;
use ropey::Rope;
use semver::Version;
use shrinkwraprs::Shrinkwrap;
use thiserror::Error;
use lsp_types as lsp;


/// WitcherScript manifest file containing project metadata.
#[derive(Debug, Clone)]
pub struct Manifest {
    /// Content metadata of this project
    pub content: Content,
    /// Dependencies needed by this project
    pub dependencies: Dependencies
}

#[derive(Debug, Clone)]
pub struct Content {
    /// Name of this project, for example SharedUtils
    pub name: String,
    pub name_range: lsp::Range,
    /// Short description of the project
    pub description: Option<String>,
    /// Version of this project, has to abide to semantic versioning
    pub version: Version,
    /// Version(s) of the game this project is compatible with 
    pub game_version: String, // CDPR's versioning system doesn't comply with semver, so string will have to do for now
    /// List of this project authors (optional)
    pub authors: Option<Vec<String>>,
    /// Relative path to the `scripts` directory. "./scripts" by default
    pub scripts_root: Option<PathBuf>
}

/// A list of dependency entries
#[derive(Debug, Clone, Shrinkwrap, PartialEq, Eq)]
pub struct Dependencies(Vec<DependencyEntry>);

// Dependency item as a key-value pair of dependency name and dependency source specifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyEntry {
    pub name: String,
    pub name_range: lsp::Range,
    pub value: DependencyValue,
    pub value_range: lsp::Range
}

/// Value of the dependency entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyValue {
    /// Get the dependency from one of project repositories by name
    FromRepo(bool),
    /// Get the dependency from a specific location
    FromPath {
        path: PathBuf
    }
}



impl Manifest {
    pub const FILE_NAME: &'static str = "witcherscript.toml";

    pub fn from_file(path: &AbsPath) -> Result<Self, Error> {
        let mut f = File::open(path).map_err(|err| Arc::new(err))?;

        let mut buff = String::new();
        // manifests are usually comparatively small, so reading it all at once shouldn't be that big of a deal
        f.read_to_string(&mut buff).map_err(|err| Arc::new(err))?;

        Self::from_str(&buff)
    }

    /// Returns true if the given name is a valid project content name. False otherwise.
    pub fn validate_content_name(name: &str) -> bool {
        let name_chars: Vec<_> = name.chars().collect();

        !name_chars.is_empty()
        && (name_chars[0].is_ascii_alphabetic() || name_chars[0] == '_')
        && name_chars.iter().all(|c| c.is_ascii_alphanumeric() || c == &'_')
    }
}

impl FromStr for Manifest {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rope = Rope::from_str(s);
        let raw: Result<raw::Manifest, toml::de::Error> = toml::from_str(s);

        if let Err(err) = raw {
            return Err(Error::Toml {
                range: lsp::Range::from_raw(err.span().unwrap_or_default(), &rope),
                msg: err.to_string()
            });
        }

        let manifest = Self::from_raw(raw.unwrap(), &rope);

        // validate content name
        if !Self::validate_content_name(&manifest.content.name) {
            return Err(Error::InvalidNameField {
                range: manifest.content.name_range.clone()
            })
        }

        Ok(manifest)
    }
}

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("file access error: {}", .0)]
    Io(#[from] Arc<io::Error>),
    #[error("TOML file parsing error: {msg}")]
    Toml {
        range: lsp::Range,
        msg: String
    },
    #[error("The `name` field in `[content]` table is invalid")]
    InvalidNameField {
        range: lsp::Range
    }
}


impl FromRaw for Manifest {
    type RawType = raw::Manifest;

    fn from_raw(raw: Self::RawType, rope: &Rope) -> Self {
        Self {
            content: Content::from_raw(raw.content, rope),
            dependencies: Dependencies::from_raw(raw.dependencies, rope)
        }
    }
}

impl FromRaw for Content {
    type RawType = raw::Content;

    fn from_raw(raw: Self::RawType, rope: &Rope) -> Self {
        Self {
           name: raw.name.get_ref().to_owned(),
           name_range: lsp::Range::from_raw(raw.name.span(), rope),
           description: raw.description,
           version: raw.version,
           authors: raw.authors,
           game_version: raw.game_version,
           scripts_root: raw.scripts_root
        }
    }
}

impl FromRaw for Dependencies {
    type RawType = raw::Dependencies;

    fn from_raw(raw: Self::RawType, rope: &Rope) -> Self {
        let mut entries = Vec::new();
        for (k, v) in raw {
            let dep_name = String::from_raw(k.get_ref().to_owned(), rope);
            let dep_name_range = lsp::Range::from_raw(k.span(), rope);
            let dep_val = DependencyValue::from_raw(v.get_ref().to_owned(), rope);
            let dep_val_range = lsp::Range::from_raw(v.span(), rope);
            entries.push(DependencyEntry { 
                name: dep_name, 
                value: dep_val,
                name_range: dep_name_range,
                value_range: dep_val_range
            });
        }

        Dependencies(entries)
    }
}

impl IntoIterator for Dependencies {
    type Item = DependencyEntry;
    type IntoIter = <Vec::<DependencyEntry> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


impl FromRaw for DependencyValue {
    type RawType = raw::DependencyValue;

    fn from_raw(raw: Self::RawType, _: &Rope) -> Self {
        match raw {
            raw::DependencyValue::FromRepo(b) => Self::FromRepo(b),
            raw::DependencyValue::FromPath { path } => Self::FromPath { path },
        }
    }
}

impl FromRaw for String {
    type RawType = String;

    fn from_raw(raw: Self::RawType, _: &Rope) -> Self {
        raw
    }
}

impl FromRaw for lsp::Range {
    type RawType = std::ops::Range<usize>;

    fn from_raw(raw: Self::RawType, rope: &Rope) -> Self {
        let start_line = rope.char_to_line(raw.start);
        let start_char = raw.start - rope.line_to_char(start_line);
        let end_line = rope.char_to_line(raw.end);
        let end_char = raw.end - rope.line_to_char(end_line);

        lsp::Range { 
            start: lsp::Position { 
                line: start_line as u32, 
                character: start_char as u32
            }, 
            end: lsp::Position { 
                line: end_line as u32, 
                character: end_char as u32 
            }
        }
    }
}



// These "raw" types are the ones with toml's span type
// This is the form that can be directly passed into serde
// The proper type has range type from lsp_types
// Can't convert between those without knowing contents of the entire file (here in a form of rope)
mod raw {
    use std::{collections::BTreeMap, path::PathBuf};
    use semver::Version;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Manifest {
        pub content: Content,
        pub dependencies: Dependencies
    }

    #[derive(Serialize, Deserialize)]
    pub struct Content {
        pub name: toml::Spanned<String>,
        pub description: Option<String>,
        pub version: Version,
        pub authors: Option<Vec<String>>,
        pub game_version: String,
        pub scripts_root: Option<PathBuf>
    }

    pub type Dependencies = BTreeMap<toml::Spanned<String>, toml::Spanned<DependencyValue>>;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum DependencyValue {
        FromRepo(bool),
        FromPath {
            path: PathBuf
        }
    }
}

trait FromRaw {
    type RawType;

    fn from_raw(raw: Self::RawType, rope: &Rope) -> Self;
}



#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_all() {
        let s = 
r#"
[content]
name = "ExampleMod"
description = "Short mod description"
version = "0.9.0"
authors = ["Rip Van Winkle"]
game_version = "4.04"
scripts_root = "./content/scripts"

[dependencies]
content0 = { path = "../Witcher 3/content/content0" }
shared_utils = true
"#;
    
        let manifest = Manifest::from_str(s).unwrap();
    
        assert_eq!(manifest.content.name, "ExampleMod");
        assert_eq!(manifest.content.description, Some("Short mod description".into()));
        assert_eq!(manifest.content.version, Version::from_str("0.9.0").unwrap());
        assert_eq!(manifest.content.authors, Some(vec!["Rip Van Winkle".into()]));
        assert_eq!(manifest.content.game_version, String::from("4.04"));
        assert_eq!(manifest.content.scripts_root, Some(PathBuf::from_str("./content/scripts").unwrap()));
    

        assert_eq!(manifest.dependencies.len(), 2);

        let content0 = manifest.dependencies[0].clone();
        assert_eq!(content0.name, "content0".to_string());
        assert_eq!(content0.name_range, lsp::Range::new(lsp::Position::new(10, 0), lsp::Position::new(10, 8)));
        assert_eq!(content0.value, DependencyValue::FromPath { path: PathBuf::from("../Witcher 3/content/content0") });
        assert_eq!(content0.value_range, lsp::Range::new(lsp::Position::new(10, 11), lsp::Position::new(10, 53)));

        let shared_utils = manifest.dependencies[1].clone();
        assert_eq!(shared_utils.name, "shared_utils".to_string());
        assert_eq!(shared_utils.name_range, lsp::Range::new(lsp::Position::new(11, 0), lsp::Position::new(11, 12)));
        assert_eq!(shared_utils.value, DependencyValue::FromRepo(true));
        assert_eq!(shared_utils.value_range, lsp::Range::new(lsp::Position::new(11, 15), lsp::Position::new(11, 19)));
    }

    #[test]
    fn test_optional() {
        let s = 
r#"
[content]
name = "ExampleMod"
version = "1.0.0"
game_version = "4.04"

[dependencies]
"#;

        let manifest = Manifest::from_str(s).unwrap();

        assert_eq!(manifest.content.description, None);
        assert_eq!(manifest.content.authors, None);
        assert_eq!(manifest.content.scripts_root, None);
        assert_eq!(*manifest.dependencies, vec![]);
    }
}