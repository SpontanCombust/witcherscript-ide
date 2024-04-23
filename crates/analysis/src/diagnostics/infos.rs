use std::fmt::Display;


#[derive(Debug, Clone)]
pub enum AnalysisInfo {
    TrailingSemicolon
}

impl Display for AnalysisInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisInfo::TrailingSemicolon => write!(f, "Trailing semicolon, consider removing it"),
        }
    }
}