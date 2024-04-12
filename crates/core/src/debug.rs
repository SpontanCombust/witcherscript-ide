/// Purely utility trait to not repeat code when implementing Debug trait for printing regular debug string or pretty debug string
pub trait DebugMaybeAlternate {
    fn debug_maybe_alternate(&mut self, value: &dyn std::fmt::Debug) -> std::fmt::Result;
    fn debug_maybe_alternate_named(&mut self, name: &str, value: &dyn std::fmt::Debug) -> std::fmt::Result;
}

impl DebugMaybeAlternate for std::fmt::Formatter<'_> {
    fn debug_maybe_alternate(&mut self, value: &dyn std::fmt::Debug) -> std::fmt::Result {
        if self.alternate() {
            write!(self, "{value:#?}")
        } else {
            write!(self, "{value:?}")
        }
    }

    fn debug_maybe_alternate_named(&mut self, name: &str, value: &dyn std::fmt::Debug) -> std::fmt::Result {
        if self.alternate() {
            write!(self, "{name} {value:#?}")
        } else {
            write!(self, "{name} {value:?}")
        }
    }
}


pub trait DebugRange {
    fn debug(&self) -> String;
}

impl DebugRange for lsp_types::Position {
    fn debug(&self) -> String {
        // added +1 to make it more intuitive inside an editor
        format!("[{}, {}]", self.line + 1, self.character + 1)
    }
}

impl DebugRange for lsp_types::Range {
    fn debug(&self) -> String {
        format!("{} - {}", self.start.debug(), self.end.debug())
    }
}