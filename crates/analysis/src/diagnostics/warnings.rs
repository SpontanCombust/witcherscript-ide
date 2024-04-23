use std::fmt::Display;


#[derive(Debug, Clone)]
pub enum AnalysisWarning {

}

impl Display for AnalysisWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}