use witcherscript::{SyntaxNode, ast::*, DocSpan};

use crate::diagnostics::{Diagnostic, DiagnosticSeverity, DiagnosticBody};

pub struct SyntaxErrorVisitor {
    diagnostics: Vec<Diagnostic>   
}

impl SyntaxErrorVisitor {
    fn missing_element(&mut self, span: DocSpan, what: &str) {
        self.diagnostics.push(Diagnostic { 
            span, 
            severity: DiagnosticSeverity::Error, 
            body: DiagnosticBody::MissingElement { what: what.to_string() }
        })
    }
}

//TODO missing punctuation and keywords
impl StatementVisitor for SyntaxErrorVisitor {
    fn visit_class_decl(&mut self, n: &SyntaxNode<'_, ClassDeclaration>) -> bool {
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

    fn visit_state_decl(&mut self, n: &SyntaxNode<'_, StateDeclaration>) -> bool {
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

    fn visit_struct_decl(&mut self, n: &SyntaxNode<'_, StructDeclaration>) -> bool {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "struct name");
        }

        true
    }

    fn visit_enum_decl(&mut self, n: &SyntaxNode<'_, EnumDeclaration>) -> bool {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "enum name");
        }

        true   
    }

    fn visit_enum_decl_value(&mut self, n: &SyntaxNode<'_, EnumDeclarationValue>) {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "enum variant name");
        }
    }

    fn visit_global_func_decl(&mut self, n: &SyntaxNode<'_, GlobalFunctionDeclaration>) -> bool {
        if n.name().is_missing() {
            self.missing_element(n.name().span(), "funtion name");
        }

        true
    }
} 