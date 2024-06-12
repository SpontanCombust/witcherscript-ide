use std::path::PathBuf;
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
            code: Some(lsp::NumberOrString::String(self.kind.code().to_string())),
            code_description: Some(lsp::CodeDescription {
                href: self.kind.code_link()
            }),
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticDomain {
    ProjectSystem,
    SyntaxAnalysis,
    ContextualSyntaxAnalysis,
    SymbolAnalysis,
    WorkspaceSymbolAnalysis,
}


/// All diagnostics that will appear in the editor should be collected in this enum.
/// Each domain of diagnostics should be handled by a seperate unit of code to keep the separation of concerns and avoid error duplication.
/// All diagnostics should documented on the diagnostic index page, so if any changes are made here, make sure to reflect them there.
#[derive(Debug, Clone, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum DiagnosticKind {
    // project system
    InvalidProjectManifest(String),
    InvalidProjectName,
    InvalidRedkitProjectManifest(String),
    ProjectDependencyPathNotFound(PathBuf),
    ProjectDependencyNameNotFound(String),
    ProjectDependencyNameNotFoundAtPath(String),
    MultipleMatchingProjectDependencies {
        content_name: String,
        matching_paths: Vec<AbsPath>
    },
    ProjectSelfDependency,

    // syntax analysis
    MissingSyntax(String),
    InvalidSyntax, // for all other syntax cases when it's impossible to determine

    // contextual syntax analysis
    IncompatibleSpecifier {
        spec_name: String,
        sym_name: String
    },
    IncompatibleFunctionFlavour {
        flavour_name: String,
        sym_name: String
    },
    RepeatedSpecifier,
    MultipleAccessModifiers,
    InvalidAnnotation,
    InvalidAnnotationPlacement,
    MissingAnnotationArgument {
        missing: String
    },
    IncompatibleAnnotation {
        expected_sym: String
    },
    GlobalScopeVarDecl,

    // symbol anaysis
    SymbolNameTaken {
        name: String,
        precursor_file_path: Option<AbsPath>,
        precursor_range: Option<lsp::Range>
    },
    MissingTypeArg,
    UnnecessaryTypeArg,
    
    // workspace symbol analysis
    SymbolNameTakenInDependency {
        name: String,
        precursor_file_path: Option<AbsPath>,
        precursor_range: Option<lsp::Range>
    },
}

#[cfg(debug_assertions)]
const DIAGNOSTICS_INDEX_PAGE: &str = "http://127.0.0.1:8000/witcherscript-ide/user-manual/diagnostic-index/";

#[cfg(not(debug_assertions))]
const DIAGNOSTICS_INDEX_PAGE: &str = "https://spontancombust.github.io/witcherscript-ide/user-manual/diagnostic-index/";


impl DiagnosticKind {
    pub fn domain(&self) -> DiagnosticDomain {
        use DiagnosticKind::*;

        match self {
            InvalidProjectManifest(_)
            | InvalidProjectName
            | InvalidRedkitProjectManifest(_)
            | ProjectDependencyPathNotFound(_)
            | ProjectDependencyNameNotFound(_)
            | ProjectDependencyNameNotFoundAtPath(_)
            | MultipleMatchingProjectDependencies { .. } 
            | ProjectSelfDependency => DiagnosticDomain::ProjectSystem,
            MissingSyntax(_)
            | InvalidSyntax => DiagnosticDomain::SyntaxAnalysis,
            IncompatibleSpecifier { .. } 
            | IncompatibleFunctionFlavour { .. } 
            | RepeatedSpecifier
            | MultipleAccessModifiers 
            | InvalidAnnotation 
            | InvalidAnnotationPlacement 
            | MissingAnnotationArgument { .. }
            | IncompatibleAnnotation { .. } 
            | GlobalScopeVarDecl => DiagnosticDomain::ContextualSyntaxAnalysis,
            SymbolNameTaken { .. }
            | MissingTypeArg
            | UnnecessaryTypeArg => DiagnosticDomain::SymbolAnalysis,
            SymbolNameTakenInDependency { .. } => DiagnosticDomain::WorkspaceSymbolAnalysis
        }
    }

    
    fn severity(&self) -> lsp::DiagnosticSeverity {
        use DiagnosticKind::*;

        match self {
            InvalidProjectManifest(_) => lsp::DiagnosticSeverity::ERROR,
            InvalidProjectName => lsp::DiagnosticSeverity::ERROR,
            InvalidRedkitProjectManifest(_) => lsp::DiagnosticSeverity::ERROR,
            ProjectDependencyPathNotFound(_) => lsp::DiagnosticSeverity::ERROR,
            ProjectDependencyNameNotFound(_) => lsp::DiagnosticSeverity::ERROR,
            ProjectDependencyNameNotFoundAtPath(_) => lsp::DiagnosticSeverity::ERROR,
            MultipleMatchingProjectDependencies { .. } => lsp::DiagnosticSeverity::ERROR,
            ProjectSelfDependency => lsp::DiagnosticSeverity::ERROR,

            MissingSyntax(_) => lsp::DiagnosticSeverity::ERROR,
            InvalidSyntax => lsp::DiagnosticSeverity::ERROR,
            
            IncompatibleSpecifier { .. } => lsp::DiagnosticSeverity::ERROR,
            IncompatibleFunctionFlavour { .. } => lsp::DiagnosticSeverity::ERROR,
            RepeatedSpecifier => lsp::DiagnosticSeverity::ERROR,
            MultipleAccessModifiers => lsp::DiagnosticSeverity::ERROR,
            InvalidAnnotation => lsp::DiagnosticSeverity::ERROR,
            InvalidAnnotationPlacement => lsp::DiagnosticSeverity::ERROR,
            MissingAnnotationArgument { .. } => lsp::DiagnosticSeverity::ERROR,
            IncompatibleAnnotation { .. } => lsp::DiagnosticSeverity::ERROR,
            GlobalScopeVarDecl => lsp::DiagnosticSeverity::ERROR,

            SymbolNameTaken { .. } => lsp::DiagnosticSeverity::ERROR,
            MissingTypeArg => lsp::DiagnosticSeverity::ERROR,
            UnnecessaryTypeArg => lsp::DiagnosticSeverity::ERROR,

            SymbolNameTakenInDependency { .. } => lsp::DiagnosticSeverity::ERROR
        }
    }

    fn message(&self) -> String {
        use DiagnosticKind::*;

        match self {
            InvalidProjectManifest(err) => err.clone(),
            InvalidProjectName => "This project name is not valid".into(),
            InvalidRedkitProjectManifest(err) => err.clone(),
            ProjectDependencyPathNotFound(p) => format!("Dependency could not be found at path \"{}\"", p.display()),
            ProjectDependencyNameNotFound(n) => format!("Dependency could not be found with name \"{n}\""),
            ProjectDependencyNameNotFoundAtPath(n) => format!("Dependency with name \"{n}\" could not be found at specified path"),
            MultipleMatchingProjectDependencies { content_name: project_name, matching_paths } => format!("Multiple matching contents for dependency with name \"{project_name}\": {:?}", matching_paths.into_iter().map(|p| p.to_string()).collect::<Vec<_>>()),
            ProjectSelfDependency => "Content may not specify itself as its own dependency".into(),

            MissingSyntax(s) => format!("Syntax error: expected {}", s),
            InvalidSyntax => "Syntax error: unexpected syntax".into(),

            IncompatibleSpecifier { spec_name, sym_name } => format!("\"{}\" cannot be used for {}", spec_name, sym_name),
            IncompatibleFunctionFlavour { flavour_name, sym_name } => format!("\"{}\" cannot be used for {}", flavour_name, sym_name),
            RepeatedSpecifier => "Specifiers can not be repeating".into(),
            MultipleAccessModifiers => "Only one access modifier is allowed".into(),
            InvalidAnnotation => "Unsupported annotation".into(),
            InvalidAnnotationPlacement => "Annotations can only be used at the global scope".into(),
            MissingAnnotationArgument { missing } => format!("This annotation requires {missing} argument"),
            IncompatibleAnnotation { expected_sym } => format!("The annotation expects {}", expected_sym),
            GlobalScopeVarDecl => "Syntax error: variable declarations in the global scope are not allowed unless you use the @addField annotation.".into(),

            SymbolNameTaken { name, .. } => format!("The name \"{}\" is defined multiple times", name),
            MissingTypeArg => "Missing type argument".into(),
            UnnecessaryTypeArg => "This type does not take any type arguments".into(),

            SymbolNameTakenInDependency { name, .. } => format!("The name \"{}\" is already defined in another content", name),
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
            SymbolNameTakenInDependency { precursor_file_path, precursor_range, .. } if precursor_file_path.is_some() => Some(DiagnosticRelatedInfo {
                path: precursor_file_path.clone().unwrap(),
                range: precursor_range.unwrap_or_default(),
                message: "Name originally defined here".into()
            }),
            _ => None
        }
    }

    fn code(&self) -> &str {
        // using strum's IntoStaticStr
        let code: &str = self.into();
        code
    }

    fn code_link(&self) -> lsp::Url {
        let s = format!("{}#{}", DIAGNOSTICS_INDEX_PAGE, self.code());
        lsp::Url::parse(&s).unwrap()
    }
}

struct DiagnosticRelatedInfo {
    path: AbsPath,
    range: lsp::Range,
    message: String
}


#[derive(Debug, Clone)]
pub struct LocatedDiagnostic {
    pub path: AbsPath,
    pub diagnostic: Diagnostic
}