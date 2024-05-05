use std::fmt::Display;
use super::AnalysisDiagnosticRelatedInfo;


#[derive(Debug, Clone)]
pub enum AnalysisWarning {

}

impl AnalysisWarning {
    pub fn related_info(&self) -> Option<AnalysisDiagnosticRelatedInfo> {
        None
    }
}

impl Display for AnalysisWarning {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}