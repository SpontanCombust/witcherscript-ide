use lsp_types::Range;
use witcherscript::ast::*;
use crate::diagnostics::{Diagnostic, ErrorDiagnostic};


pub struct SyntaxErrorVisitor {
    diagnostics: Vec<Diagnostic>   
}

impl SyntaxErrorVisitor {
    fn missing_element(&mut self, span: Range, what: &str) {
        self.diagnostics.push(Diagnostic { 
            span, 
            body: ErrorDiagnostic::MissingElement { what: what.to_string() }.into()
        })
    }
}

//TODO missing punctuation and keywords
impl StatementVisitor for SyntaxErrorVisitor {
    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> bool {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "class name");
        }

        if let Some(base) = n.base() {
            if base.is_missing() {
                self.missing_element(base.span(), "base class name");
            }
        }

        true
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> bool {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "class name");
        }

        if n.parent().is_missing() {
            self.missing_element(n.name().span(), "state parent name");
        }

        if let Some(base) = n.base() {
            if base.is_missing() {
                self.missing_element(base.span(), "base class name");
            }
        }

        true
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> bool {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "struct name");
        }

        true
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> bool {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "enum name");
        }

        true   
    }

    fn visit_enum_member_decl(&mut self, n: &EnumMemberDeclarationNode) {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "enum member name");
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> bool {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "funtion name");
        }

        true
    }

    // TODO the rest of visitors
} 