use std::fmt::Display;


#[derive(Debug, Clone)]
pub enum WarningDiagnostic {

}

impl Display for WarningDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}