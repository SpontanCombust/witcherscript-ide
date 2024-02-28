use std::{borrow::Cow, fs::File, io::{self, Read}, path::{Path, PathBuf}, sync::Arc};
use ropey::Rope;
use semver::Version;
use shrinkwraprs::Shrinkwrap;
use thiserror::Error;
use lsp_types as lsp;
use toml_span::{de_helpers::{expected, TableHelper}, Deserialize};


/// Deserialized WitcherScript manifest file containing project metadata.
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
    /// Version of this project, has to abide to semantic versioning
    pub version: Version,
    /// List of this project authors (optional)
    pub authors: Option<Vec<String>>,
    /// Version(s) of the game this project is compatible with 
    pub game_version: String, // CDPR's versioning system doesn't comply with semver, so string will have to do for now
}

#[derive(Debug, Clone, Shrinkwrap)]
pub struct Dependencies(Vec<DependencyEntry>);

#[derive(Debug, Clone)]
pub struct DependencyEntry {
    pub name: Ranged<String>,
    pub value: Ranged<DependencyValue>
}

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
    pub const FILE_NAME: &str = "witcherscript.toml";

    pub fn from_file<P>(path: P) -> Result<Self, ManifestParseError> 
    where P: AsRef<Path> {
        let mut f = File::open(&path).map_err(|err| Arc::new(err))?;

        let mut buff = String::new();
        // manifests are usually comparatively small, so reading it all at once shouldn't be that big of a deal
        f.read_to_string(&mut buff).map_err(|err| Arc::new(err))?;

        Self::from_str(&buff)
    }

    pub fn from_str(s: &str) -> Result<Self, ManifestParseError> {
        let rope = Rope::from_str(s);
        let raw = toml_span::parse(s)
            .map_err(|err| toml_span::DeserError::from(err))
            .and_then(|mut v| ManifestRaw::deserialize(&mut v));

        match raw {
            Ok(raw) => {
                Ok(Self {
                    content: raw.content,
                    dependencies: Dependencies::from_raw(raw.dependencies, &rope)
                })
            },
            Err(err) => {
                let first_error = err.errors.into_iter().next().unwrap();
                Err(ManifestParseError::Toml {
                    range: span_to_range(first_error.span, &rope),
                    msg: first_error.to_string()
                })
            },
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum ManifestParseError {
    #[error("file access error")]
    Io(#[from] Arc<io::Error>),
    #[error("TOML file parsing errors")]
    Toml {
        range: lsp::Range,
        msg: String
    }
}


// These "raw" types are the ones with toml_span's span type
// The proper type has range type from lsp_types
// Can't convert between those without accessing the entire file (here in a form of rope)
struct ManifestRaw {
    content: Content,
    dependencies: DependenciesRaw
}

impl<'de> toml_span::Deserialize<'de> for ManifestRaw {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, toml_span::DeserError> {
        let mut th = TableHelper::new(value)?;

        let content = th.required("content")?;
        let dependencies = th.required("dependencies")?;

        th.finalize(None)?;

        Ok(Self {
            content,
            dependencies
        })
    }
}


impl<'de> toml_span::Deserialize<'de> for Content {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, toml_span::DeserError> {
        let mut th = TableHelper::new(value)?;

        let name = th.required("name")?;
        let version_str: toml_span::Spanned<String> = th.required("version")?;
        let version = Version::parse(&version_str.value)
            .map_err(|e| toml_span::Error::from((toml_span::ErrorKind::Custom(Cow::Owned(e.to_string())), version_str.span)))?;
        let authors = th.optional("authors");
        let game_version = th.required("game_version")?;

        th.finalize(None)?;

        Ok(Content {
            name,
            version,
            authors,
            game_version
        })
    }
}


impl Dependencies {
    fn from_raw(raw: DependenciesRaw, rope: &Rope) -> Self {
        let mut entries = Vec::new();
        for (k, v) in raw.0 {
            let dep_name = Ranged::new(k.value, span_to_range(k.span, rope));
            let dep_val = Ranged::new(v.value, span_to_range(v.span, rope));
            entries.push(DependencyEntry{
                name: dep_name,
                value: dep_val
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


struct DependenciesRaw(Vec<(toml_span::Spanned<String>, toml_span::Spanned<DependencyValue>)>);

impl<'de> toml_span::Deserialize<'de> for DependenciesRaw {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, toml_span::DeserError> {
        let mut entries = Vec::new();

        let inner = value.take();
        if let toml_span::value::ValueInner::Table(tab) = inner {
            for (k, mut v) in tab {
                let dep_name = toml_span::Spanned::with_span(k.name.to_string(), k.span.clone());
                let dep_val = toml_span::Spanned::deserialize(&mut v)?;
                entries.push((dep_name, dep_val));
            }
    
            Ok(DependenciesRaw(entries))
        } else {
            Err(expected("table", inner, value.span).into())
        }
    }
}


impl<'de> toml_span::Deserialize<'de> for DependencyValue {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, toml_span::DeserError> {
        match value.take() {
            toml_span::value::ValueInner::Boolean(b) => {
                Ok(DependencyValue::FromRepo(b))
            },
            toml_span::value::ValueInner::Table(tab) => {
                let mut th = TableHelper::from((tab, value.span));

                let path_str: String = th.required("path")?;
                let path = PathBuf::from(path_str);

                th.finalize(None)?;

                Ok(DependencyValue::FromPath { path })
            },
            other => {
                Err(expected("bool or table", other, value.span).into())
            }
        }
    }
}



#[derive(Debug, Clone, Shrinkwrap)]
pub struct Ranged<T> {
    #[shrinkwrap(main_field)]
    value: T,
    range: lsp::Range
}

impl<T> Ranged<T> {
    fn new(val: T, range: lsp::Range) -> Self {
        Self {
            value: val,
            range
        }
    }

    pub fn inner(&self) -> &T {
        &self.value
    }

    pub fn range(&self) -> lsp::Range {
        self.range.clone()
    }

    pub fn into_tuple(self) -> (T, lsp::Range) {
        (self.value, self.range)
    }
}

impl<T: PartialEq> PartialEq for Ranged<T> {
    fn eq(&self, other: &Self) -> bool {
        // only value is taken as the comparison subject
        self.value == other.value
    }
}

impl<T: Eq> Eq for Ranged<T> {}


fn span_to_range(span: toml_span::Span, rope: &Rope) -> lsp::Range {
    let start_line = rope.char_to_line(span.start);
    let start_char = span.start - rope.line_to_char(start_line);
    let end_line = rope.char_to_line(span.end);
    let end_char = span.end - rope.line_to_char(end_line);

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



#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test() {
        let s = 
r#"
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
    

        assert_eq!(manifest.dependencies.len(), 2);

        let content0 = manifest.dependencies[0].clone();
        assert_eq!(content0.name.value, "content0".to_string());
        assert_eq!(content0.name.range, lsp::Range::new(lsp::Position::new(8, 0), lsp::Position::new(8, 8)));
        assert_eq!(content0.value.value, DependencyValue::FromPath { path: PathBuf::from("../Witcher 3/content/content0") });
        assert_eq!(content0.value.range, lsp::Range::new(lsp::Position::new(8, 11), lsp::Position::new(8, 53)));

        let shared_utils = manifest.dependencies[1].clone();
        assert_eq!(shared_utils.name.value, "shared_utils".to_string());
        assert_eq!(shared_utils.name.range, lsp::Range::new(lsp::Position::new(9, 0), lsp::Position::new(9, 12)));
        assert_eq!(shared_utils.value.value, DependencyValue::FromRepo(true));
        assert_eq!(shared_utils.value.range, lsp::Range::new(lsp::Position::new(9, 15), lsp::Position::new(9, 19)));
    }
}