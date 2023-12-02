use ropey::Rope;
use uuid::Uuid;
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
        if let Some(c) = n.name().value(&self.rope) {
            if self.check_duplicate(c.as_str(), SymbolType::Class, n.span()) {
                let sym = ClassSymbol::new_with_default(c.as_str(), self.script_id);
                self.symtab.insert(c.into(), SymbolTableValue::from_symbol(&sym));
                self.db.classes.insert(sym.id(), sym);
            }
        }

        false
    }

    fn visit_state_decl(&mut self, n: &SyntaxNode<'_, StateDeclaration>) -> bool {
        let state_name = n.name().value(&self.rope);
        let parent_name = n.parent().value(&self.rope);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let state_class = StateSymbol::class_name(&state_name, &parent_name);
            if self.check_duplicate(state_class.as_str(), SymbolType::State, n.span()) {
                let sym = StateSymbol::new_with_default(state_class.as_str(), self.script_id);
                self.symtab.insert(state_class, SymbolTableValue::from_symbol(&sym));
                self.db.states.insert(sym.id(), sym);
            }
        }

        false
    }

    fn visit_struct_decl(&mut self, n: &SyntaxNode<'_, StructDeclaration>) -> bool {
        if let Some(s) = n.name().value(&self.rope) {
            if self.check_duplicate(s.as_str(), SymbolType::Struct, n.span()) {
                let sym = StructSymbol::new_with_default(s.as_str(), self.script_id);
                self.symtab.insert(s.into(), SymbolTableValue::from_symbol(&sym));
                self.db.structs.insert(sym.id(), sym);
            }
        }

        false
    }

    fn visit_enum_decl(&mut self, n: &SyntaxNode<'_, EnumDeclaration>) -> bool {
        if let Some(e) = n.name().value(&self.rope) {
            if self.check_duplicate(e.as_str(), SymbolType::Enum, n.span()) {
                let sym = EnumSymbol::new_with_default(e.as_str(), self.script_id);
                self.symtab.insert(e.into(), SymbolTableValue::from_symbol(&sym));
                self.db.enums.insert(sym.id(), sym);
                //TODO members are also in global scope!
            }
        }

        false
    }

    //TODO global functions
}