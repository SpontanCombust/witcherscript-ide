use std::collections::HashSet;
use ropey::Rope;
use uuid::Uuid;
use witcherscript::{SyntaxNode, DocSpan};
use witcherscript::ast::*;
use crate::diagnostics::*;
use crate::model::{collections::*, symbols::*};
use super::inject_native_symbols::inject_array_type;



trait SymbolCollectorCommons {
    fn db(&mut self) -> &mut SymbolDb;
    fn symtab(&mut self) -> &mut SymbolTable;
    fn db_and_symtab(&mut self) -> (&mut SymbolDb, &mut SymbolTable);
    fn diagnostics(&mut self) -> &mut Vec<Diagnostic>;


    fn check_duplicate(&mut self, sym_name: &str, sym_typ: SymbolType, span: DocSpan) -> bool {
        if let Some(err) = self.symtab().can_insert(sym_name, sym_typ) {
            let precursor_type = match err {
                SymbolTableError::GlobalVarAlreadyExists(_, v) => v.typ,
                SymbolTableError::TypeAlreadyExists(_, v) => v.typ,
                SymbolTableError::DataAlreadyExists(_, v) => v.typ,
                SymbolTableError::CallableAlreadyExists(_, v) => v.typ,
            };
            
            self.diagnostics().push(Diagnostic { 
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

    fn check_array_type(&mut self, generic_arg: Option<&str>, span: DocSpan) -> Option<Uuid> {
        if let Some(t) = generic_arg {
            if let Some(t_id) = self.check_type_missing(t, None, span) {
                let final_typ = ArrayTypeSymbol::final_type_name(t);
                if let Some(SymbolTableValue { id, .. }) = self.symtab().get(&final_typ, SymbolCategory::Type) {
                    Some(*id)
                } else {
                    let (db, symtab) = self.db_and_symtab();
                    Some(inject_array_type(db, symtab, t_id, t))
                }
            } else {
                None
            }

        } else {
            self.diagnostics().push(Diagnostic { 
                span, 
                severity: DiagnosticSeverity::Error, 
                body: DiagnosticBody::MissingGenericArg 
            });
    
            None
        }
    }

    fn check_type_missing(&mut self, typ: &str, generic_arg: Option<&str>, span: DocSpan) -> Option<Uuid> {
        if typ == ArrayTypeSymbol::TYPE_NAME {
            self.check_array_type(generic_arg, span)
        } else {
            if let Some(SymbolTableValue { id, .. }) = self.symtab().get(typ, SymbolCategory::Type) {
                Some(*id)
            } else {
                self.diagnostics().push(Diagnostic { 
                    span, 
                    severity: DiagnosticSeverity::Error, 
                    body: DiagnosticBody::TypeNotFound 
                });
                None
            }
        }

    }
}

//TODO be able to update existing db and symtab instead of assuming they are new
struct GlobalSymbolCollector<'a> {
    db: &'a mut SymbolDb,
    symtab: &'a mut SymbolTable,
    script_id: Uuid,
    rope: Rope,
    diagnostics: Vec<Diagnostic>,

    current_enum: Option<EnumSymbol>,
}

impl SymbolCollectorCommons for GlobalSymbolCollector<'_> {
    fn db(&mut self) -> &mut SymbolDb {
        &mut self.db
    }

    fn symtab(&mut self) -> &mut SymbolTable {
        &mut self.symtab
    }
    /// So they can both be borrowed at the same time
    fn db_and_symtab(&mut self) -> (&mut SymbolDb, &mut SymbolTable) {
        (&mut self.db, &mut self.symtab)    
    }

    fn diagnostics(&mut self) -> &mut Vec<Diagnostic> {
        &mut self.diagnostics
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




struct ChildSymbolCollector<'a> {
    db: &'a mut SymbolDb,
    symtab: &'a mut SymbolTable,
    rope: Rope,
    diagnostics: Vec<Diagnostic>,

    current_class: Option<ClassSymbol>,
    current_state: Option<StateSymbol>,
    current_struct: Option<StructSymbol>,
    current_member_func: Option<MemberFunctionSymbol>,
    current_event: Option<EventSymbol>,
    current_global_func: Option<GlobalFunctionSymbol>,
}

impl SymbolCollectorCommons for ChildSymbolCollector<'_> {
    fn db(&mut self) -> &mut SymbolDb {
        &mut self.db
    }

    fn symtab(&mut self) -> &mut SymbolTable {
        &mut self.symtab
    }

    fn db_and_symtab(&mut self) -> (&mut SymbolDb, &mut SymbolTable) {
        (&mut self.db, &mut self.symtab)    
    }

    fn diagnostics(&mut self) -> &mut Vec<Diagnostic> {
        &mut self.diagnostics
    }
}

impl StatementVisitor for ChildSymbolCollector<'_> {
    fn visit_class_decl(&mut self, n: &SyntaxNode<'_, ClassDeclaration>) -> bool {
        if let Some(class_name) = n.name().value(&self.rope) {
            if let Some(SymbolTableValue { id, .. }) = self.symtab.get(&class_name, SymbolCategory::Type) {
                let mut sym = self.db.remove_class(*id).expect("class absent in db despite being in symtab");

                n.specifiers()
                .map(|specn| (specn.value(), specn.span()))
                .for_each(|(spec, span)| {
                    if !sym.data.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            span, 
                            severity: DiagnosticSeverity::Error, 
                            body: DiagnosticBody::RepeatedSpecifier
                        });
                    }
                });

                if let Some(base_node) = n.base() {
                    if let Some(id) = base_node.value(&self.rope).and_then(|base| self.check_type_missing(&base, None, base_node.span())) {
                        sym.data.base_id = Some(id);
                    }
                }

                self.current_class = Some(sym);
                self.symtab.push_scope();
                return true;
            }
        }

        false
    }

    fn exit_class_decl(&mut self, _: &SyntaxNode<'_, ClassDeclaration>) {
        if let Some(sym) = self.current_class.take() {
            self.symtab.pop_scope();
            self.db.insert_class(sym);
        }
    }

    fn visit_member_var_decl(&mut self, n: &SyntaxNode<'_, MemberVarDeclaration>) {
        let names = n.names()
                    .map(|identn| (identn.span(), identn))
                    .filter_map(|(span, identn)| {
                        if let Some(ident) = identn.value(&self.rope) {
                            Some((span, ident))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(); // need to collect because of later mut borrow

        let mut checked_names: Vec<String> = Vec::new();
        for (span, name) in names.into_iter() {
            if self.check_duplicate(&name, SymbolType::MemberVar, span) {
                checked_names.push(name.into());
            }
        }
                
        if !checked_names.is_empty() {
            let syms: Vec<MemberVarSymbol>;
            if self.current_class.is_some() {
                syms = checked_names.into_iter().map(|n| self.current_class.as_mut().unwrap().add_member_var(&n)).collect();
            } else if self.current_state.is_some() {
                syms = checked_names.into_iter().map(|n| self.current_state.as_mut().unwrap().add_member_var(&n)).collect();
            } else if self.current_struct.is_some() {
                syms = checked_names.into_iter().map(|n| self.current_struct.as_mut().unwrap().add_member_var(&n)).collect();
            } else {
                panic!("No type to create member var symbol for");
            }

            
            let mut specifiers = HashSet::new();

            n.specifiers()
            .map(|specn| (specn.value(), specn.span()))
            .for_each(|(spec, span)| {
                if !specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        severity: DiagnosticSeverity::Error, 
                        body: DiagnosticBody::RepeatedSpecifier
                    });
                }
            });

            let mut type_id: Uuid = ERROR_SYMBOL_ID;
            let var_typen = n.var_type();
            if let Some(primary_type) = var_typen.type_name().value(&self.rope) {
                let generic_arg = var_typen.generic_arg().and_then(|g| g.value(&self.rope));
                let generic_arg_ref = generic_arg.as_ref().map(|s| s.as_str());

                if let Some(id) = self.check_type_missing(&primary_type, generic_arg_ref, var_typen.span()) {
                    type_id = id;
                }
            }

            syms.into_iter().for_each(|mut sym| {
                sym.data.specifiers = specifiers.clone();
                sym.data.type_id = type_id;

                self.symtab.insert(sym.name(), sym.id(), sym.typ());
                self.db.insert_member_var(sym);
            });
        }
    }
}