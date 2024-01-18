use std::fmt::Display;


#[derive(Debug, Clone)]
pub enum InfoDiagnostic {
    TrailingSemicolon
}

impl Display for InfoDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfoDiagnostic::TrailingSemicolon => write!(f, "Trailing semicolon, consider removing it"),
        }
    }
}