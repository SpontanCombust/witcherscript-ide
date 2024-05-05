use std::fmt::Display;
use super::AnalysisDiagnosticRelatedInfo;


#[derive(Debug, Clone)]
pub enum AnalysisInfo {
    TrailingSemicolon
}

impl AnalysisInfo {
    pub fn related_info(&self) -> Option<AnalysisDiagnosticRelatedInfo> {
        None
    }
}

impl Display for AnalysisInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisInfo::TrailingSemicolon => write!(f, "Trailing semicolon, consider removing it"),
        }
    }
}