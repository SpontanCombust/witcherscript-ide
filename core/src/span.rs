use std::fmt::{self, Debug};

use tree_sitter::{Point, Range};

/// Zero-based document position
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocPos {
    pub line: usize,
    pub col: usize
}

impl Debug for DocPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.line, self.col)
    }
}

impl From<Point> for DocPos {
    fn from(value: Point) -> Self {
        Self {
            line: value.row,
            col: value.column
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocSpan {
    pub start: DocPos,
    pub end: DocPos
}

impl Debug for DocSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?} - {:?}]", self.start, self.end)
    }
}

impl From<Range> for DocSpan {
    fn from(value: Range) -> Self {
        Self {
            start: value.start_point.into(),
            end: value.end_point.into(),
        }
    }
}