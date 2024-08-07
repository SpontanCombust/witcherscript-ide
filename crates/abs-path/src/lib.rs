use std::{fmt::Display, path::{Component, Path, PathBuf, Prefix, MAIN_SEPARATOR_STR}, sync::OnceLock};
use serde::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;
use thiserror::Error;


/// A type that guarantees to be an absolute file path.
/// This gives a type-safe way to mark a given parameter as needing of an absolute path.
/// It does not access the file system to create the path.
/// Also corrects separators into proper ones for host OS. 
/// 
/// For Windows paths removes verbatim and UNC prefixes, so only local paths will work.
/// Does not support symbolic links.
#[derive(Debug, Clone, Shrinkwrap, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AbsPath {
    inner: PathBuf
}

impl AbsPath {
    /// If the path is relative uses the `cwd` parameter to resolve it. 
    /// If `cwd` is not supplied uses the current working directory of the process.
    pub fn resolve<P: AsRef<Path>>(path: P, cwd: Option<&Self>) -> Result<Self, Error> {
        let path = path.as_ref();
        
        // not necessairly normalized, because `is_absolute` only checks the beginning of the path
        let unnormalized = if path.is_absolute() {
            path.to_path_buf()
        } else {
            let cwd = cwd.unwrap_or(self::current_dir());
            cwd.inner.join(path)
        };

        let (root, stem) = Self::split(&unnormalized)?;
        
        let mut abs_path = root;
        if !stem.as_os_str().is_empty() {
            let stem = Self::normalize_stem(stem)?;
            abs_path.push(stem);
        }

        Ok(Self { inner: abs_path })
    }

    /// Creates a new AbsPath by joining to it a `path`.
    /// If `path` is absolute it returns it instead of the expected sum of paths.
    pub fn join<P: AsRef<Path>>(&self, path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        if path.as_os_str().is_empty() {
            Ok(self.clone())
        } else if path.is_absolute() {
            Self::resolve(path, None)
        } else {
            let (root, stem) = Self::split(&self.inner)?;
            let stem = Self::normalize_stem(&stem.join(path))?;

            Ok(Self { inner: root.join(stem) })
        }
    }

    pub fn parent(&self) -> Option<Self> {
        if self.components().any(|c| matches!(c, Component::Normal(_))) {
            Some(Self { inner: self.inner.parent().unwrap().to_path_buf() })
        } else {
            None
        }
    }

    pub fn to_uri(&self) -> lsp_types::Url {
        lsp_types::Url::from_file_path(self).unwrap()
    }


    /// Divides the given path into a root and stem (relative path after the root)
    /// On Windows the root also includes the drive prefix
    /// `unnormalized` should be an absolute, possibly unnormalized path
    fn split(unnormalized: &Path) -> Result<(PathBuf, &Path), Error> {
        let mut components = unnormalized.components();
        let mut root = PathBuf::new();

        while let Some(comp) = components.next() {
            match comp {
                Component::Prefix(prefix_comp) => {
                    match prefix_comp.kind() {
                        Prefix::Disk(disk) | Prefix::VerbatimDisk(disk) => {
                            root.push(&format!("{}:", (disk as char).to_ascii_uppercase()));
                        },
                        _ => return Err(Error::UnsupportedPrefix)
                    };
                }
                Component::RootDir => {
                    root.push(MAIN_SEPARATOR_STR);
                    break;
                }
                _ => panic!("Path was not absolute")
            }
        }

        Ok((root, components.as_path()))
    }

    fn normalize_stem(path: &Path) -> Result<PathBuf, Error> {
        let mut normalized = PathBuf::with_capacity(path.as_os_str().len());

        for component in path.components() {
            match component {
                Component::ParentDir => { 
                    if !normalized.pop() {
                        return Err(Error::ImpossiblePath);
                    }
                },
                Component::Normal(comp) => {
                    normalized.push(comp);
                },
                _ => {}
            }
        }

        Ok(normalized)
    }
}

impl AsRef<AbsPath> for AbsPath {
    fn as_ref(&self) -> &AbsPath {
        self
    }
}

impl AsRef<Path> for AbsPath {
    fn as_ref(&self) -> &Path {
        self.inner.as_ref()
    }
}

impl Display for AbsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.as_os_str().to_string_lossy().as_ref())
    }
}


impl TryFrom<lsp_types::Url> for AbsPath {
    type Error = ();

    fn try_from(value: lsp_types::Url) -> Result<Self, Self::Error> {
        let path = value.to_file_path()?;
        Ok(Self::resolve(path, None).unwrap())
    }
}

impl TryFrom<PathBuf> for AbsPath {
    type Error = self::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::resolve(value, None)
    }
}

impl Into<lsp_types::Url> for AbsPath {
    fn into(self) -> lsp_types::Url {
        lsp_types::Url::from_file_path(self.inner).unwrap()
    }
}

impl Into<PathBuf> for AbsPath {
    fn into(self) -> PathBuf {
        self.inner
    }
}


#[derive(Debug, Error, Clone)]
pub enum Error {
    // Appears for paths like "/../../" which can never be possible, because they go outside of the root path
    #[error("Path points to a resource outside of root directory")]
    ImpossiblePath,
    // Windows only. 
    #[error("Windows path non-disk prefixes are not supported")]
    UnsupportedPrefix
}

impl Into<std::io::Error> for Error {
    fn into(self) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::NotFound, self)
    }
}


/// Returns the absolute path to Current Working Directory of the process
/// The result is always the same so changing process's cwd will not change it.
pub fn current_dir() -> &'static AbsPath {
    static CWD: OnceLock<AbsPath> = OnceLock::new();
    CWD.get_or_init(|| {
        AbsPath { inner: std::env::current_dir().unwrap() }
    })
}



#[cfg(test)]
mod test {
    use std::path::Path;
    use super::*;

    #[test]
    fn cwd() {
        let p = current_dir();
        assert_eq!(p.inner, std::env::current_dir().unwrap());
    }

    #[test]
    fn empty_path() {
        let cwd = current_dir();
        let abs = AbsPath::resolve("", None).unwrap();
        assert_eq!(abs.inner, cwd.inner);
    }

    #[test]
    fn pass_absolute() {
        if cfg!(windows) {
            let p = Path::new(r"C:\foo\bar baz\alpha\");
            let abs = AbsPath::resolve(p, None).unwrap();
            assert_eq!(abs.inner, Path::new(r"C:\foo\bar baz\alpha"));
        } else {
            let p = Path::new(r"/foo/bar baz/alpha/");
            let abs = AbsPath::resolve(p, None).unwrap();
            assert_eq!(abs.inner, Path::new(r"/foo/bar baz/alpha"));
        }
    }

    #[test]
    fn pass_absolute_with_cwd() {
        if cfg!(windows) {
            let cwd = AbsPath::resolve(r"D:\", None).unwrap();
            let p = Path::new(r"C:\foo\bar baz\alpha\");
            let abs = AbsPath::resolve(p, Some(&cwd)).unwrap();
            assert_eq!(abs.inner, Path::new(r"C:\foo\bar baz\alpha"));
        } else {
            let cwd = AbsPath::resolve(r"/", None).unwrap();
            let p = Path::new(r"/foo/bar baz/alpha/");
            let abs = AbsPath::resolve(p, Some(&cwd)).unwrap();
            assert_eq!(abs.inner, Path::new(r"/foo/bar baz/alpha"));
        }
    }

    #[test]
    fn pass_relative() {
        if cfg!(windows) {
            let p = Path::new(r"bar baz\alpha\");
            let abs = AbsPath::resolve(p, None).unwrap();
            assert_eq!(abs.inner, current_dir().inner.join(r"bar baz\alpha"));
        } else {
            let p = Path::new(r"bar baz/alpha/");
            let abs = AbsPath::resolve(p, None).unwrap();
            assert_eq!(abs.inner, current_dir().inner.join(r"bar baz/alpha"));
        }
    }

    #[test]
    fn pass_relative_with_cwd() {
        if cfg!(windows) {
            let cwd = AbsPath::resolve(r"D:\beta", None).unwrap();
            let p = Path::new(r"bar baz\alpha\");
            let abs = AbsPath::resolve(p, Some(&cwd)).unwrap();
            assert_eq!(abs.inner, Path::new(r"D:\beta\bar baz\alpha"));
        } else {
            let cwd = AbsPath::resolve(r"/beta", None).unwrap();
            let p = Path::new(r"bar baz/alpha/");
            let abs = AbsPath::resolve(p, Some(&cwd)).unwrap();
            assert_eq!(abs.inner, Path::new(r"/beta/bar baz/alpha"));
        }
    }

    #[test]
    fn fix_slashes() {
        if cfg!(windows) {
            let p = Path::new(r"C:\Foo/Bar Baz/Echo/1234\charlie.exe");
            let abs = AbsPath::resolve(p, None).unwrap();
            assert_eq!(abs.inner, Path::new(r"C:\Foo\Bar Baz\Echo\1234\charlie.exe"));
        } else {
            // On unix a backslash in a path is treated as an escape character instead of a separator
            // and so it doesn't get corrected to a forward slash.
            // Unlike how forward slashes are converted to backslashes on Windows.

            // let p = Path::new(r"/foo/Bar Baz\Echo/1234\charlie");
            // let abs = AbsPath::resolve(p, None).unwrap();
            // assert_eq!(abs.inner, Path::new(r"/foo/Bar Baz/Echo/1234/charlie"));
        }
    }

    #[test]
    fn normalization() {
        if cfg!(windows) {
            let cwd = AbsPath::resolve(r"E:\omega\..", None).unwrap();
            let p = Path::new(r"alpha_beta\.\gamma\..\.zeta\.");
            let abs = AbsPath::resolve(p, Some(&cwd)).unwrap();
            assert_eq!(abs.inner, Path::new(r"E:\alpha_beta\.zeta"));

            assert!(AbsPath::resolve(r"..\", Some(&cwd)).is_err());
        } else {
            let cwd = AbsPath::resolve(r"/omega/..", None).unwrap();
            let p = Path::new(r"alpha_beta/./gamma/../.zeta/.");
            let abs = AbsPath::resolve(p, Some(&cwd)).unwrap();
            assert_eq!(abs.inner, Path::new(r"/alpha_beta/.zeta"));

            assert!(AbsPath::resolve(r"../", Some(&cwd)).is_err());
        }
    }

    #[test]
    fn joins() {
        if cfg!(windows) {
            let p = AbsPath::resolve(r"C:\", None).unwrap();

            let abs = p.join(r"foo\.\bar").unwrap();
            assert_eq!(abs.inner, Path::new(r"C:\foo\bar"));

            let abs = p.join(r"D:\baz").unwrap();
            assert_eq!(abs.inner, Path::new(r"D:\baz"));

            let abs = p.join(r"foo\..\..");
            assert!(abs.is_err());
        } else {
            let p = AbsPath::resolve(r"/", None).unwrap();

            let abs = p.join(r"foo/./bar").unwrap();
            assert_eq!(abs.inner, Path::new(r"/foo/bar"));

            let abs = p.join(r"/baz").unwrap();
            assert_eq!(abs.inner, Path::new(r"/baz"));

            let abs = p.join(r"foo/../..");
            assert!(abs.is_err());
        }
    }

    #[test]
    fn windows_handle_prefixes() {
        if cfg!(windows) {
            let p = Path::new(r"\\?\c:\i\hate\windows\paths");
            let abs = AbsPath::resolve(p, None).unwrap();
            assert_eq!(abs.inner, Path::new(r"C:\i\hate\windows\paths"));
        }
    }

    #[test]
    fn parent() {
        if cfg!(windows) {
            let p = AbsPath::resolve(r"C:\Windows\Users", None).unwrap();

            let p = p.parent().unwrap();
            assert_eq!(p.inner, Path::new(r"C:\Windows"));

            let p = p.parent().unwrap();
            assert_eq!(p.inner, Path::new(r"C:\"));

            assert_eq!(p.parent(), None);
        } else {
            let p = AbsPath::resolve(r"/home/user", None).unwrap();

            let p = p.parent().unwrap();
            assert_eq!(p.inner, Path::new(r"/home"));

            let p = p.parent().unwrap();
            assert_eq!(p.inner, Path::new(r"/"));

            assert_eq!(p.parent(), None);
        }
    }
}
