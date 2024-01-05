use std::{fmt::Display, ops::Add};
use super::symbols::SymbolCategory;


/// Denotes a string that unambiguously identifies a symbol in the global namespace.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolPath {
    buff: String
}

/// The path is divided into components seperated by a slash '/'.
/// Each component looks like {name}:{tag}, where name is the proper name of the symbol.
/// Tag character denotes the category of the symbol to disambiguate them from each other.
impl SymbolPath {
    const COMPONENT_SEP: char = '/';
    const COMPONENT_TAG_SEP: char = ':';
    const COMPONENT_TAG_TYPE: char = 'T';
    const COMPONENT_TAG_DATA: char = 'D';
    const COMPONENT_TAG_CALLABLE: char = 'C';

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


    pub fn push(&mut self, name: &str, category: SymbolCategory) {
        // allow only alphanumerics and underscore
        debug_assert!(name.chars().filter(|c| !c.is_alphanumeric() && c != &'_').count() == 0);

        if !self.buff.is_empty() {
            self.buff.push(Self::COMPONENT_SEP);
        }

        let tag = match category {
            SymbolCategory::Type => Self::COMPONENT_TAG_TYPE,
            SymbolCategory::Data => Self::COMPONENT_TAG_DATA,
            SymbolCategory::Callable => Self::COMPONENT_TAG_CALLABLE,
        };

        self.buff.push_str(name);
        self.buff.push(Self::COMPONENT_TAG_SEP);
        self.buff.push(tag);
    }

    pub fn pop(&mut self) {
        if let Some(i) = self.buff.rfind(Self::COMPONENT_SEP) {
            self.buff.drain(i..);
        } else {
            self.buff.clear();
        }
    }

    pub fn pop_root(&mut self) {
        if let Some(i) = self.buff.find(Self::COMPONENT_SEP) {
            self.buff.drain(..i);
        } else {
            self.buff.clear();
        }   
    }


    pub fn is_empty(&self) -> bool {
        self.buff.is_empty()
    }

    /// Returns an iterator over individual parts of the path
    pub fn components(&self) -> impl DoubleEndedIterator<Item = SymbolPathComponent> {
        self.buff
            .split_terminator(Self::COMPONENT_SEP)
            .map(|c| SymbolPathComponent::new(c))
            .into_iter()
    }


    // /// Returns the path without the last component if there is any
    // pub fn parent(&self) -> Option<Self> {
    //     if let Some(i) = self.buff.rfind(Self::COMPONENT_SEP) {
    //         Some(Self {
    //             buff: self.buff[..i].to_string()
    //         })
    //     } else {
    //         None
    //     }
    // }

    // /// Returns the first component of this path if there is any 
    // pub fn root(&self) -> Option<Self> {
    //     if let Some(i) = self.buff.find(Self::COMPONENT_SEP) {
    //         Some(Self {
    //             buff: self.buff[..i].to_string()
    //         })
    //     } else if !self.is_empty() {
    //         Some(self.clone())
    //     } else {
    //         None
    //     }
    // }

    // /// Returns everything after the first path component if there is anything
    // pub fn stem(&self) -> Option<Self> {
    //     if let Some(i) = self.buff.find(Self::COMPONENT_SEP) {
    //         Some(Self {
    //             buff: self.buff[i..].to_string()
    //         })
    //     } else {
    //         None
    //     }
    // }
}


impl Default for SymbolPath {
    fn default() -> Self {
        Self::empty()
    }
}

impl Display for SymbolPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.buff.is_empty() {
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

impl Add<&SymbolPath> for SymbolPath {
    type Output = SymbolPath;

    fn add(self, rhs: &SymbolPath) -> Self::Output {
        Self {
            buff: self.buff + &rhs.buff
        }
    }
}



pub struct SymbolPathComponent<'a> {
    pub name: &'a str,
    pub category: SymbolCategory
}

impl<'a> SymbolPathComponent<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            name: &s[..s.len() - 2],
            category: match s.chars().last().unwrap() {
                SymbolPath::COMPONENT_TAG_TYPE => SymbolCategory::Type,
                SymbolPath::COMPONENT_TAG_DATA => SymbolCategory::Data,
                SymbolPath::COMPONENT_TAG_CALLABLE => SymbolCategory::Callable,
                _ => panic!("Invalid symbol path component tag")
            }
        }
    }
}

impl Display for SymbolPathComponent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)?;
        if self.category == SymbolCategory::Callable {
            f.write_str("()")?;
        }
        Ok(())
    }
}

impl<'a> From<SymbolPathComponent<'a>> for SymbolPath {
    fn from(value: SymbolPathComponent<'a>) -> Self {
        SymbolPath::new(value.name, value.category)
    }
}




#[test]
fn test() {
    {
        let mut p = SymbolPath::empty();
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
        let mut p = SymbolPath::empty();
        p.push("theGame", SymbolCategory::Data);
        assert_eq!(p.to_string(), "theGame".to_string());
    }
    {
        let mut p = SymbolPath::empty();
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
}