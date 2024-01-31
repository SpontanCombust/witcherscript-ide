use std::{collections::HashMap, fs::File, io::{self, Read}, path::{Path, PathBuf}};
use semver::Version;
use serde::Deserialize;
use thiserror::Error;


#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: HashMap<String, PathBuf> // for now just use a path as a value
}

#[derive(Debug, Clone, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub authors: Option<Vec<String>>,
    pub game_version: String, // CDPR's versioning system doesn't comply with semver, so string will have to do for now
}


#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("File access error")]
    Io(#[from] io::Error),
    #[error("TOML file parsing error")]
    Toml(#[from] toml::de::Error)
}

impl Manifest {
    pub fn from_str(s: &str) -> Result<Self, ManifestError> {
        let toml: Manifest = toml::from_str(s)?;
        Ok(toml)
    }

    pub fn from_file<P>(path: P) -> Result<Self, ManifestError> 
    where P: AsRef<Path> {
        let mut f = File::open(&path)?;

        let mut buff = String::new();
        // manifests are usually comparatively small, so reading it all at once shouldn't be that big of a deal
        f.read_to_string(&mut buff)?;

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
        [package]
        name = "ExampleMod"
        version = "0.9.0"
        authors = ["Rip Van Winkle"]
        game_version = "4.04"
    
        [dependencies]
        content0 = "../Witcher 3/content/content0"
        shared_utils = "../Witcher 3/Mods/modSharedUtils"
        "#;
    
        let manifest = Manifest::from_str(s).unwrap();
    
        assert_eq!(manifest.package.name, "ExampleMod");
        assert_eq!(manifest.package.version, Version::from_str("0.9.0").unwrap());
        assert_eq!(manifest.package.authors, Some(vec!["Rip Van Winkle".into()]));
        assert_eq!(manifest.package.game_version, String::from("4.04"));
    
        assert_eq!(manifest.dependencies, HashMap::from_iter([
            ("content0".into(), "../Witcher 3/content/content0".into()),
            ("shared_utils".into(), "../Witcher 3/Mods/modSharedUtils".into()),
        ]));
    }
}