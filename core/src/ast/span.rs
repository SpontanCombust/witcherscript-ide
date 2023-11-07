
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Span {
    pub begin: usize,
    pub end: usize
}

impl Span {
    pub fn new(begin: usize, end: usize) -> Span {
        Span { begin, end }
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}, {}]", self.begin, self.end)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    val: T,
    pub span: Span
}

impl<T> Spanned<T> {
    pub fn new(val: T, span: Span) -> Self {
        Self { 
            val, 
            span
        }
    }
}

// Deref makes the span less intrusive into the AST structure
// Even if types are wrapped with Spanned<...> you can still use them as if they're not
impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> DerefMut for Spanned<T> {    
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}
