use std::fmt::Display;
use lsp_types::Range;
use super::{AnalysisError, AnalysisWarning, AnalysisInfo};


#[derive(Debug, Clone)]
pub struct AnalysisDiagnostic {
    pub range: Range,
    pub body: AnalysisDiagnosticBody
}

#[derive(Debug, Clone)]
pub enum AnalysisDiagnosticBody {
    Error(AnalysisError),
    Warning(AnalysisWarning),
    Info(AnalysisInfo)
}

impl From<AnalysisError> for AnalysisDiagnosticBody {
    fn from(value: AnalysisError) -> Self {
        Self::Error(value)
    }
}

impl From<AnalysisWarning> for AnalysisDiagnosticBody {
    fn from(value: AnalysisWarning) -> Self {
        Self::Warning(value)
    }
}

impl From<AnalysisInfo> for AnalysisDiagnosticBody {
    fn from(value: AnalysisInfo) -> Self {
        Self::Info(value)
    }
}

impl Display for AnalysisDiagnosticBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisDiagnosticBody::Error(err) => write!(f, "{}", err),
            AnalysisDiagnosticBody::Warning(warn) => write!(f, "{}", warn),
            AnalysisDiagnosticBody::Info(info) => write!(f, "{}", info),
        }
    }
}