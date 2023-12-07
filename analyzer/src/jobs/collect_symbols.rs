use ropey::Rope;
use uuid::Uuid;
use witcherscript::{SyntaxNode, DocSpan};
use witcherscript::ast::*;
use crate::diagnostics::*;
use crate::model::{collections::*, symbols::*};


//TODO be able to update existing db and symtab instead of assuming they are new
struct GlobalSymbolCollector<'a> {
    db: &'a mut SymbolDb,
    symtab: &'a mut SymbolTable<'a>,
    script_id: Uuid,
    rope: Rope,
    diagnostics: Vec<Diagnostic>,

    current_enum: Option<EnumSymbol>,
}

impl GlobalSymbolCollector<'_> {
    fn check_duplicate(&mut self, sym_name: &str, sym_typ: SymbolType, span: DocSpan) -> bool {
        if let Some(err) = self.symtab.can_insert(sym_name, sym_typ) {
            let precursor_type = match err {
                SymbolTableError::GlobalVarAlreadyExists(_, v) => v.typ,
                SymbolTableError::TypeAlreadyExists(_, v) => v.typ,
                SymbolTableError::DataAlreadyExists(_, v) => v.typ,
                SymbolTableError::CallableAlreadyExists(_, v) => v.typ,
            };
            
            self.diagnostics.push(Diagnostic { 
                span, 
                severity: DiagnosticSeverity::Error, 
                body: DiagnosticBody::SymbolNameTaken { 
                    name: sym_name.to_string(), 
                    this_type: sym_typ, 
                    precursor_type: precursor_type
                }
            });

            false
        } else {
            true
        }
    }
}

impl StatementVisitor for GlobalSymbolCollector<'_> {
    fn visit_class_decl(&mut self, n: &SyntaxNode<'_, ClassDeclaration>) -> bool {
        if let Some(class_name) = n.name().value(&self.rope) {
            let sym_typ = SymbolType::Class;
            if self.check_duplicate(&class_name, sym_typ, n.span()) {
                let sym = ClassSymbol::new_with_default(&class_name, self.script_id);
                self.symtab.insert(&class_name, sym.id(), sym_typ);
                self.db.insert_class(sym);
            }
        }

        false
    }

    fn visit_state_decl(&mut self, n: &SyntaxNode<'_, StateDeclaration>) -> bool {
        let state_name = n.name().value(&self.rope);
        let parent_name = n.parent().value(&self.rope);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let sym_typ = SymbolType::State;
            let state_class_name = StateSymbol::class_name(&state_name, &parent_name);
            if self.check_duplicate(&state_class_name, sym_typ, n.span()) {
                let sym = StateSymbol::new_with_default(&state_class_name, self.script_id);
                self.symtab.insert(&state_class_name, sym.id(), sym_typ);
                self.db.insert_state(sym);
            }
        }

        false
    }

    fn visit_struct_decl(&mut self, n: &SyntaxNode<'_, StructDeclaration>) -> bool {
        if let Some(struct_name) = n.name().value(&self.rope) {
            let sym_typ = SymbolType::Struct;
            if self.check_duplicate(&struct_name, sym_typ, n.span()) {
                let sym = StructSymbol::new_with_default(&struct_name, self.script_id);
                self.symtab.insert(&struct_name, sym.id(), sym_typ);
                self.db.insert_struct(sym);
            }
        }

        false
    }

    fn visit_enum_decl(&mut self, n: &SyntaxNode<'_, EnumDeclaration>) -> bool {
        if let Some(enum_name) = n.name().value(&self.rope) {
            let sym_typ = SymbolType::Enum;
            if self.check_duplicate(&enum_name, sym_typ, n.span()) {
                let sym = EnumSymbol::new_with_default(&enum_name, self.script_id);
                self.current_enum = Some(sym);
                // symbol added to db and symtab during exit
                return true;
            }
        }

        false
    }

    // enum member is WS work just like they do in C - they are global scoped constants
    // enum type doesn't create any sort of scope for them
    fn visit_enum_member_decl(&mut self, n: &SyntaxNode<'_, EnumMemberDeclaration>) {
        if let Some(member_name) = n.name().value(&self.rope) {
            let sym_typ = SymbolType::EnumMember;
            if self.check_duplicate(&member_name, sym_typ, n.span()) {
                let sym = self.current_enum.as_mut().unwrap().add_member(&member_name);
                self.symtab.insert(&member_name, sym.id(), sym_typ);
                self.db.insert_enum_member(sym);
            }
        }
    }

    fn exit_enum_decl(&mut self, _: &SyntaxNode<'_, EnumDeclaration>) {
        if let Some(sym) = self.current_enum.take() {
            self.symtab.insert(sym.name(), sym.id(), sym.typ());
            self.db.insert_enum(sym);
        }
    }

    fn visit_global_func_decl(&mut self, n: &SyntaxNode<'_, GlobalFunctionDeclaration>) -> bool {
        if let Some(func_name) = n.name().value(&self.rope) {
            let sym_typ = SymbolType::GlobalFunction;
            if self.check_duplicate(&func_name, sym_typ, n.span()) {
                let sym = GlobalFunctionSymbol::new_with_default(&func_name, self.script_id);
                self.symtab.insert(&func_name, sym.id(), sym.typ());
                self.db.insert_global_func(sym);
            }
        }

        false
    }
}