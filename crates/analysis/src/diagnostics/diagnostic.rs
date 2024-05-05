use std::fmt::Display;
use abs_path::AbsPath;
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

#[derive(Debug, Clone)]
pub struct AnalysisDiagnosticRelatedInfo {
    pub path: AbsPath,
    pub range: Range,
    pub message: String
}


impl AnalysisDiagnosticBody {
    pub fn related_info(&self) -> Option<AnalysisDiagnosticRelatedInfo> {
        match self {
            AnalysisDiagnosticBody::Error(err) => err.related_info(),
            AnalysisDiagnosticBody::Warning(warn) => warn.related_info(),
            AnalysisDiagnosticBody::Info(info) => info.related_info(),
        }
    }
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