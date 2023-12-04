use ropey::Rope;
use uuid::Uuid;
use witcherscript::{SyntaxNode, DocSpan};
use witcherscript::ast::*;
use crate::diagnostics::*;
use crate::model::{collections::*, symbols::*};


struct GlobalSymbolCollector {
    db: SymbolDb,
    symtab: SymbolTable,
    script_id: Uuid,
    rope: Rope,
    diagnostics: Vec<Diagnostic>,
}

impl GlobalSymbolCollector {
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

impl StatementVisitor for GlobalSymbolCollector {
    fn visit_class_decl(&mut self, n: &SyntaxNode<'_, ClassDeclaration>) -> bool {
        if let Some(class_name) = n.name().value(&self.rope) {
            if self.check_duplicate(class_name.as_str(), SymbolType::Class, n.span()) {
                let sym = ClassSymbol::new_with_default(class_name.as_str(), self.script_id);
                self.symtab.insert(class_name.into(), SymbolTableValue::from_symbol(&sym));
                self.db.classes.insert(sym.id(), sym);
            }
        }

        false
    }

    fn visit_state_decl(&mut self, n: &SyntaxNode<'_, StateDeclaration>) -> bool {
        let state_name = n.name().value(&self.rope);
        let parent_name = n.parent().value(&self.rope);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let state_class_name = StateSymbol::class_name(&state_name, &parent_name);
            if self.check_duplicate(state_class_name.as_str(), SymbolType::State, n.span()) {
                let sym = StateSymbol::new_with_default(state_class_name.as_str(), self.script_id);
                self.symtab.insert(state_class_name, SymbolTableValue::from_symbol(&sym));
                self.db.states.insert(sym.id(), sym);
            }
        }

        false
    }

    fn visit_struct_decl(&mut self, n: &SyntaxNode<'_, StructDeclaration>) -> bool {
        if let Some(struct_name) = n.name().value(&self.rope) {
            if self.check_duplicate(struct_name.as_str(), SymbolType::Struct, n.span()) {
                let sym = StructSymbol::new_with_default(struct_name.as_str(), self.script_id);
                self.symtab.insert(struct_name.into(), SymbolTableValue::from_symbol(&sym));
                self.db.structs.insert(sym.id(), sym);
            }
        }

        false
    }

    fn visit_enum_decl(&mut self, n: &SyntaxNode<'_, EnumDeclaration>) -> bool {
        if let Some(enum_name) = n.name().value(&self.rope) {
            if self.check_duplicate(enum_name.as_str(), SymbolType::Enum, n.span()) {
                let mut sym = EnumSymbol::new_with_default(enum_name.as_str(), self.script_id);
                
                // enum member is WS work just like they do in C - they are global scoped constants
                // enum type doesn't create any sort of scope for them
                for member in n.definition().values() {
                    if let Some(member_name) = member.name().value(&self.rope) {
                        let memsym = sym.add_member(&member_name);
                        self.symtab.insert(member_name.to_string(), SymbolTableValue::from_symbol(&memsym));
                        self.db.enum_members.insert(memsym.id(), memsym);
                    }
                }

                self.symtab.insert(enum_name.into(), SymbolTableValue::from_symbol(&sym));
                self.db.enums.insert(sym.id(), sym);
            }
        }

        false
    }

    fn visit_global_func_decl(&mut self, n: &SyntaxNode<'_, GlobalFunctionDeclaration>) -> bool {
        if let Some(func_name) = n.name().value(&self.rope) {
            if self.check_duplicate(func_name.as_str(), SymbolType::GlobalFunction, n.span()) {
                let sym = GlobalFunctionSymbol::new_with_default(func_name.as_str(), self.script_id);
                self.symtab.insert(func_name.to_string(), SymbolTableValue::from_symbol(&sym));
                self.db.global_funcs.insert(sym.id(), sym);
            }
        }

        false
    }
}