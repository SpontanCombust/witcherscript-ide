use lsp_types as lsp;
use abs_path::AbsPath;
use strum_macros::IntoStaticStr;


#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range: lsp::Range,
    pub kind: DiagnosticKind
}

impl Into<lsp::Diagnostic> for Diagnostic {
    fn into(self) -> lsp::Diagnostic {
        lsp::Diagnostic {
            range: self.range,
            severity: Some(self.kind.severity()),
            code: Some(lsp::NumberOrString::String(
                // using strum's IntoStaticStr
                Into::<&'static str>::into(&self.kind).to_string()
            )),
            code_description: None,
            source: Some("witcherscript-ide".into()),
            message: self.kind.message(),
            related_information: self.kind.related_info().map(|ri| {
                vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location { 
                        uri: ri.path.to_uri(), 
                        range: ri.range
                    },
                    message: ri.message
                }]
            }),
            tags: None,
            data: None
        }
    }
}

//TODO DiagnosticDomain here instead of DiagnosticGroup in lsp
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum DiagnosticDomain {

// }


/// All diagnostics that will appear in the editor should be collected in this enum.
/// Each domain of diagnostics should be handled by a seperate unit of code to keep the separation of concerns and avoid error duplication.
#[derive(Debug, Clone, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum DiagnosticKind {
    // syntax analysis
    MissingSyntax(String),
    InvalidSyntax, // for all other syntax cases when it's impossible to determine

    // symbol scanning
    SymbolNameTaken {
        name: String,
        precursor_file_path: Option<AbsPath>,
        precursor_range: Option<lsp::Range>
    },
    MissingTypeArg,
    UnnecessaryTypeArg,
    RepeatedSpecifier,
    MultipleAccessModifiers
}

impl DiagnosticKind {
    fn severity(&self) -> lsp::DiagnosticSeverity {
        use DiagnosticKind::*;

        match self {
            MissingSyntax(_) => lsp::DiagnosticSeverity::ERROR,
            InvalidSyntax => lsp::DiagnosticSeverity::ERROR,

            SymbolNameTaken { .. } => lsp::DiagnosticSeverity::ERROR,
            MissingTypeArg => lsp::DiagnosticSeverity::ERROR,
            UnnecessaryTypeArg => lsp::DiagnosticSeverity::ERROR,
            RepeatedSpecifier => lsp::DiagnosticSeverity::ERROR,
            MultipleAccessModifiers => lsp::DiagnosticSeverity::ERROR,
        }
    }

    fn message(&self) -> String {
        use DiagnosticKind::*;

        match self {
            MissingSyntax(s) => format!("Syntax error: expected {}", s),
            InvalidSyntax => "Syntax error: unexpected syntax".into(),

            SymbolNameTaken { name, .. } => format!("The name {} is defined multiple times", name),
            MissingTypeArg => "Missing type argument".into(),
            UnnecessaryTypeArg => "This type does not take any type arguments".into(),
            RepeatedSpecifier => "Specifiers can not be repeating".into(),
            MultipleAccessModifiers => "Only one access modifier is allowed".into(),
        }
    }

    fn related_info(&self) -> Option<DiagnosticRelatedInfo> {
        use DiagnosticKind::*;

        match self {
            SymbolNameTaken { precursor_file_path, precursor_range, .. } if precursor_file_path.is_some() => Some(DiagnosticRelatedInfo {
                path: precursor_file_path.clone().unwrap(),
                range: precursor_range.unwrap_or_default(),
                message: "Name originally defined here".into()
            }),
            _ => None
        }
    }
}

struct DiagnosticRelatedInfo {
    path: AbsPath,
    range: lsp::Range,
    message: String
}


#[test]
fn test() {
    let s: &str = DiagnosticKind::MissingTypeArg.into();
    println!("{}", s);
}