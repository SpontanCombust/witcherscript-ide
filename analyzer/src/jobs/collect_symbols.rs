use ropey::Rope;
use uuid::Uuid;
use witcherscript::tokens::Identifier;
use witcherscript::{SyntaxNode, DocSpan};
use witcherscript::ast::*;
use crate::diagnostics::*;
use crate::model::{collections::*, symbols::*};


struct TypeCollectingVisitor {
    db: SymbolDb,
    symtab: SymbolTable,
    script_id: Uuid,
    rope: Rope,
    diagnostics: Vec<Diagnostic>,
}

impl TypeCollectingVisitor {
    /// If identifier node is missing appends an error and returns None. Otherwise returns identifier text.
    fn check_missing_ident(&mut self, ident_node: &SyntaxNode<'_, Identifier>) -> Option<String> {
        if let Some(ident) = ident_node.value(&self.rope) { 
            Some(ident.into())
        } else {
            self.diagnostics.push(Diagnostic { 
                span: ident_node.span(), 
                severity: DiagnosticSeverity::Error, 
                body: DiagnosticBody::MissingElement { what: "identifier".into() }
            });
            None
        }
    }

    /// If symbol is a duplicate appends an error and returns false, otherwise returns true.
    fn check_duplicate(&mut self, sym_name: &str, sym_type: SymbolType, span: DocSpan) -> bool {
        if let Some(val) = self.symtab.get(sym_name) {
            self.diagnostics.push(Diagnostic { 
                span, 
                severity: DiagnosticSeverity::Error, 
                body: DiagnosticBody::SymbolNameTaken { 
                    name: sym_name.to_string(), 
                    this_type: sym_type, 
                    precursor_type: val.typ 
                }
            });

            false
        } else {
            true
        }
    }
}

impl StatementVisitor for TypeCollectingVisitor {
    fn visit_class_decl(&mut self, n: &SyntaxNode<'_, ClassDeclaration>) -> bool {
        if let Some(c) = self.check_missing_ident(&n.name()) {
            if self.check_duplicate(c.as_str(), SymbolType::Class, n.span()) {
                let sym = ClassSymbol::new(self.script_id, c.as_str());
                self.symtab.insert(c, SymbolTableValue::from_symbol(&sym));
                self.db.classes.insert(sym.symbol_id(), sym);
            }
        }

        false
    }

    fn visit_state_decl(&mut self, n: &SyntaxNode<'_, StateDeclaration>) -> bool {
        if let (Some(state_name), Some(parent_name)) = (self.check_missing_ident(&n.name()), self.check_missing_ident(&n.parent())) {
            let state_class = StateSymbol::class_name(&state_name, &parent_name);
            if self.check_duplicate(state_class.as_str(), SymbolType::State, n.span()) {
                let sym = StateSymbol::new(self.script_id, state_class.as_str());
                self.symtab.insert(state_class, SymbolTableValue::from_symbol(&sym));
                self.db.states.insert(sym.symbol_id(), sym);
            }
        }

        false
    }

    fn visit_struct_decl(&mut self, n: &SyntaxNode<'_, StructDeclaration>) -> bool {
        if let Some(s) = self.check_missing_ident(&n.name()) {
            if self.check_duplicate(s.as_str(), SymbolType::Struct, n.span()) {
                let sym = StructSymbol::new(self.script_id, s.as_str());
                self.symtab.insert(s, SymbolTableValue::from_symbol(&sym));
                self.db.structs.insert(sym.symbol_id(), sym);
            }
        }

        false
    }

    fn visit_enum_decl(&mut self, n: &SyntaxNode<'_, EnumDeclaration>) -> bool {
        if let Some(e) = self.check_missing_ident(&n.name()) {
            if self.check_duplicate(e.as_str(), SymbolType::Enum, n.span()) {
                let sym = EnumSymbol::new(self.script_id, e.as_str());
                self.symtab.insert(e, SymbolTableValue::from_symbol(&sym));
                self.db.enums.insert(sym.symbol_id(), sym);
            }
        }

        false
    }
}