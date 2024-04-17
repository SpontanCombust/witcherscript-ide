use super::symbols::SymbolCategory;


/// Denotes a string that unambiguously identifies a symbol in the global namespace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolPathBuf {
    buff: String
}

const COMPONENT_SEP: char = '/';
const COMPONENT_TAG_SEP: char = ':';
const COMPONENT_TAG_TYPE: char = 'T';
const COMPONENT_TAG_DATA: char = 'D';
const COMPONENT_TAG_CALLABLE: char = 'C';

/// The path is divided into components seperated by a slash '/'.
/// Each component looks like {name}:{tag}, where name is the proper name of the symbol.
/// Tag character denotes the category of the symbol to disambiguate them from each other.
impl SymbolPathBuf {
    /// Can be used to indicate error or default state
    pub fn empty() -> Self {
        Self {
            buff: String::new()
        }
    }

    pub fn new(name: &str, category: SymbolCategory) -> Self {
        let mut p = Self::empty();
        p.push(name, category);
        p
    }


    pub fn as_sympath(&self) -> &SymbolPath {
        SymbolPath::new(&self.buff)
    }


    /// Adds a new component at the end of the path
    pub fn push(&mut self, name: &str, category: SymbolCategory) {
        // allow only alphanumerics and underscore
        debug_assert!(name.chars().filter(|c| !c.is_alphanumeric() && c != &'_').count() == 0);

        if !self.buff.is_empty() {
            self.buff.push(COMPONENT_SEP);
        }

        let tag = match category {
            SymbolCategory::Type => COMPONENT_TAG_TYPE,
            SymbolCategory::Data => COMPONENT_TAG_DATA,
            SymbolCategory::Callable => COMPONENT_TAG_CALLABLE,
        };

        self.buff.push_str(name);
        self.buff.push(COMPONENT_TAG_SEP);
        self.buff.push(tag);
    }

    /// Removes the rightmost component in the path. If there is only one component left, clears the path completely.
    pub fn pop(&mut self) {
        if let Some(i) = self.buff.rfind(COMPONENT_SEP) {
            self.buff.drain(i..);
        } else {
            self.buff.clear();
        }
    }

    /// Removes the leftmost component in the path. If there is only one component left, clears the path completely.
    pub fn pop_root(&mut self) {
        if let Some(i) = self.buff.find(COMPONENT_SEP) {
            self.buff.drain(..i);
        } else {
            self.buff.clear();
        }   
    }
}

impl Default for SymbolPathBuf {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::ops::Deref for SymbolPathBuf {
    type Target = SymbolPath;

    fn deref(&self) -> &Self::Target {
        self.as_sympath()
    }
}

impl std::borrow::Borrow<SymbolPath> for SymbolPathBuf {
    fn borrow(&self) -> &SymbolPath {
        self.as_sympath()
    }
}

impl AsRef<SymbolPathBuf> for SymbolPathBuf {
    fn as_ref(&self) -> &SymbolPathBuf {
        &self
    }
}

impl AsRef<SymbolPath> for SymbolPathBuf {
    fn as_ref(&self) -> &SymbolPath {
        self.as_sympath()
    }
}



#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolPath {
    inner: str
}

impl SymbolPath {
    fn new(path_slice: &str) -> &Self {
        // same as std::path::Path but with &str
        unsafe { &*(path_slice as *const str as *const Self) }
    }

    fn as_str(&self) -> &str {
        &self.inner
    }

    fn slice<R>(&self, range: R) -> &Self 
    where R: std::slice::SliceIndex<str, Output = str> {
        Self::new(&self.inner[range])
    }


    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn starts_with(&self, other: &Self) -> bool {
        self.inner.starts_with(&other.inner)
    }

    /// Returns an iterator over individual parts of the path
    pub fn components(&self) -> Components {
        Components {
            current_sympath: self
        }
    }

    /// Returns the path without the last component if there is any
    pub fn parent(&self) -> Option<&Self> {
        let mut comps = self.components();
        comps.next_back();

        let parent = comps.as_sympath();
        if !parent.is_empty() {
            Some(parent)
        } else {
            None
        }
    }

    /// Returns the first component of this path if there is any
    pub fn root(&self) -> Option<&Self> {
        self.components().next().map(|c| c.as_sympath())
    }

    /// Returns everything after the first path component if there is anything
    pub fn stem(&self) -> Option<&Self> {
        let mut comps = self.components();
        comps.next();

        let stem = comps.as_sympath();
        if !stem.is_empty() {
            Some(stem)
        } else {
            None
        }
    }

    pub fn join(&self, other: &Self) -> SymbolPathBuf {
        let mut joined = self.to_sympath_buf();
        if !joined.is_empty() {
            joined.buff.push(COMPONENT_SEP);
        }
        joined.buff.push_str(&other.inner);

        joined
    }

    /// Make this symbol path owned
    pub fn to_sympath_buf(&self) -> SymbolPathBuf {
        SymbolPathBuf {
            buff: self.inner.to_string()
        }
    }
}

impl ToOwned for SymbolPath {
    type Owned = SymbolPathBuf;

    fn to_owned(&self) -> Self::Owned {
        self.to_sympath_buf()
    }
}

impl std::fmt::Display for SymbolPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.inner.is_empty() {
            let sep = "::";
            f.write_str(
                &self.components()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(sep)
            )?;
        }
        Ok(())
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<'a> {
    sympath: &'a SymbolPath,

    pub name: &'a str,
    pub category: SymbolCategory
}

impl<'a> Component<'a> {
    /// The `sympath` MUST have exactly a single component
    fn new(sympath: &'a SymbolPath) -> Self {
        Self {
            sympath,
            name: &sympath.inner[..sympath.inner.len() - 2],
            category: match sympath.inner.chars().last().unwrap() {
                COMPONENT_TAG_TYPE => SymbolCategory::Type,
                COMPONENT_TAG_DATA => SymbolCategory::Data,
                COMPONENT_TAG_CALLABLE => SymbolCategory::Callable,
                _ => panic!("Invalid symbol path component tag")
            }
        }
    }

    pub fn as_sympath(&self) -> &'a SymbolPath {
        self.sympath
    }
}

impl std::fmt::Display for Component<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)?;
        if self.category == SymbolCategory::Callable {
            f.write_str("()")?;
        }
        Ok(())
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Components<'a> {
    current_sympath: &'a SymbolPath
}

impl<'a> Components<'a> {
    pub fn as_sympath(&self) -> &'a SymbolPath {
        self.current_sympath
    }
}

impl<'a> Iterator for Components<'a> {
    type Item = Component<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sympath.is_empty() {
            None
        } else if let Some(i) = self.current_sympath.as_str().find(COMPONENT_SEP) {
            let comp = Component::new(&self.current_sympath.slice(..i));
            self.current_sympath = &self.current_sympath.slice(i + 1..);
            Some(comp)
        } else {
            let comp = Component::new(self.current_sympath);
            self.current_sympath = SymbolPath::new("");
            Some(comp)
        }
    }
}

impl<'a> DoubleEndedIterator for Components<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_sympath.is_empty() {
            None
        } else if let Some(i) = self.current_sympath.as_str().rfind(COMPONENT_SEP) {
            let comp = Component::new(&self.current_sympath.slice(i + 1..));
            self.current_sympath = &self.current_sympath.slice(..i);
            Some(comp)
        } else {
            let comp = Component::new(self.current_sympath);
            self.current_sympath = SymbolPath::new("");
            Some(comp)
        }
    }
}




#[test]
fn test() {
    {
        let mut p = SymbolPathBuf::empty();
        assert_eq!(p.to_string(), "".to_string());
    
        p.push("Enum1", SymbolCategory::Type);
        assert_eq!(p.to_string(), "Enum1".to_string());
    
        p.push("Member1", SymbolCategory::Data);
        assert_eq!(p.to_string(), "Enum1::Member1".to_string());
        
        p.pop();
        assert_eq!(p.to_string(), "Enum1".to_string());
    
        p.push("Member2", SymbolCategory::Data);
        assert_eq!(p.to_string(), "Enum1::Member2".to_string());
    
        p.pop();
        p.pop();
        p.pop(); // extra
        assert_eq!(p.to_string(), "".to_string());
    }
    {
        let mut p = SymbolPathBuf::empty();
        p.push("theGame", SymbolCategory::Data);
        assert_eq!(p.to_string(), "theGame".to_string());
    }
    {
        let mut p = SymbolPathBuf::empty();
        p.push("UnnecessarilyLongClassNameForSomeReason", SymbolCategory::Type);
        p.push("SomeFunction", SymbolCategory::Callable);
        p.push("functionParam", SymbolCategory::Data);

        assert_eq!(p.to_string(), "UnnecessarilyLongClassNameForSomeReason::SomeFunction()::functionParam".to_string());

        let mut it = p.components();
        let c = it.next().unwrap();
        assert_eq!(c.name, "UnnecessarilyLongClassNameForSomeReason");
        assert_eq!(c.category, SymbolCategory::Type);

        let c = it.next().unwrap();
        assert_eq!(c.name, "SomeFunction");
        assert_eq!(c.category, SymbolCategory::Callable);

        let c = it.next().unwrap();
        assert_eq!(c.name, "functionParam");
        assert_eq!(c.category, SymbolCategory::Data);

        assert!(it.next().is_none());
    }
    {
        let mut p = SymbolPathBuf::empty();

        assert_eq!(p.parent(), None);
        assert_eq!(p.root(), None);
        assert_eq!(p.stem(), None);

        p.push("CClass", SymbolCategory::Type);

        assert_eq!(p.parent(), None);
        assert_eq!(p.root(), Some(SymbolPathBuf::new("CClass", SymbolCategory::Type).as_sympath()));
        assert_eq!(p.stem(), None);

        p.push("SomeFunction", SymbolCategory::Callable);

        assert_eq!(p.parent(), Some(SymbolPathBuf::new("CClass", SymbolCategory::Type).as_sympath()));
        assert_eq!(p.root(), Some(SymbolPathBuf::new("CClass", SymbolCategory::Type).as_sympath()));
        assert_eq!(p.stem(), Some(SymbolPathBuf::new("SomeFunction", SymbolCategory::Callable).as_sympath()));

        p.push("LocalVar", SymbolCategory::Data);

        assert_eq!(p.parent(), Some(
            SymbolPathBuf::new("CClass", SymbolCategory::Type)
                .join(&SymbolPathBuf::new("SomeFunction", SymbolCategory::Callable))
                .as_sympath()
        ));
        assert_eq!(p.root(), Some(SymbolPathBuf::new("CClass", SymbolCategory::Type).as_sympath()));
        assert_eq!(p.stem(), Some(
            SymbolPathBuf::new("SomeFunction", SymbolCategory::Callable)
                .join(&SymbolPathBuf::new("LocalVar", SymbolCategory::Data))
                .as_sympath()
        ));
    }
}