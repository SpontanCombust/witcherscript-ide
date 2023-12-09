use std::collections::HashSet;
use ropey::Rope;
use witcherscript::SyntaxNode;
use witcherscript::attribs::*;
use witcherscript::ast::*;
use crate::model::collections::*;
use crate::diagnostics::*;
use crate::model::symbols::*;
use super::commons::SymbolCollectorCommons;


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

    fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl StatementVisitor for ChildSymbolCollector<'_> {
    fn visit_class_decl(&mut self, n: &SyntaxNode<'_, ClassDeclaration>) -> bool {
        if let Some(class_name) = n.name().value(&self.rope) {
            if let Some(SymbolTableValue { id, typ: SymbolType::Class }) = self.symtab.get(&class_name, SymbolCategory::Type) {
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
                    if let Some(id) = base_node.value(&self.rope).and_then(|base| self.check_type(&base, None, base_node.span())) {
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

    fn visit_state_decl(&mut self, n: &SyntaxNode<'_, StateDeclaration>) -> bool {
        let state_name = n.name().value(&self.rope);
        let parent_name = n.parent().value(&self.rope);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let state_class_name = StateSymbol::class_name(&state_name, &parent_name);

            if let Some(SymbolTableValue { id, typ: SymbolType::State }) = self.symtab.get(&state_class_name, SymbolCategory::Type) {
                let mut sym = self.db.remove_state(*id).expect("state absent in db despite being in symtab");

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
                    if let Some(id) = base_node.value(&self.rope).and_then(|base| self.check_type(&base, None, base_node.span())) {
                        sym.data.base_id = Some(id);
                    }
                }

                if let Some(id) = n.parent().value(&self.rope).and_then(|parent| self.check_type(&parent, None, n.parent().span())) {
                    sym.data.parent_id = id;
                }

                self.current_state = Some(sym);
                self.symtab.push_scope();
                return true;
            }
        }

        false
    }

    fn exit_state_decl(&mut self, _: &SyntaxNode<'_, StateDeclaration>) {
        if let Some(sym) = self.current_state.take() {
            self.symtab.pop_scope();
            self.db.insert_state(sym);
        }
    }

    fn visit_struct_decl(&mut self, n: &SyntaxNode<'_, StructDeclaration>) -> bool {
        if let Some(struct_name) = n.name().value(&self.rope) {
            if let Some(SymbolTableValue { id, typ: SymbolType::Struct }) = self.symtab.get(&struct_name, SymbolCategory::Type) {
                let mut sym = self.db.remove_struct(*id).expect("struct absent in db despite being in symtab");

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

                self.current_struct = Some(sym);
                self.symtab.push_scope();
                return true;
            }
        }

        false
    }

    fn exit_struct_decl(&mut self, _: &SyntaxNode<'_, StructDeclaration>) {
        if let Some(sym) = self.current_struct.take() {
            self.symtab.pop_scope();
            self.db.insert_struct(sym);
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
            if let Some(checked_name) = self.check_duplicate(name.into(), SymbolType::MemberVar, span) {
                checked_names.push(checked_name);
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
            let mut found_access_modif_before = false;
            for (spec, span) in n.specifiers().map(|specn| (specn.value(), specn.span())) {
                if matches!(spec, MemberVarSpecifier::AccessModifier(_)) {
                    if found_access_modif_before {
                        self.diagnostics.push(Diagnostic { 
                            span, 
                            severity: DiagnosticSeverity::Error, 
                            body: DiagnosticBody::MultipleAccessModifiers 
                        })
                    }
                    found_access_modif_before = true;
                }

                if !specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        severity: DiagnosticSeverity::Error, 
                        body: DiagnosticBody::RepeatedSpecifier
                    });
                }
            }


            let type_id = self.get_type_from_node(n.var_type());


            syms.into_iter().for_each(|mut sym| {
                sym.data.specifiers = specifiers.clone();
                sym.data.type_id = type_id;

                self.symtab.insert(&sym);
                self.db.insert_member_var(sym);
            });
        }
    }

    fn visit_autobind_decl(&mut self, n: &SyntaxNode<'_, AutobindDeclaration>) {
        let autobind_name = n.name()
                            .value(&self.rope)
                            .and_then(|ident| self.check_duplicate(ident.into(), SymbolType::Autobind, n.span()));

        if let Some(autobind_name) = autobind_name {
            let mut sym: AutobindSymbol;
            if self.current_class.is_some() {
                sym = self.current_class.as_mut().unwrap().add_autobind(&autobind_name);
            } else if self.current_state.is_some() {
                sym = self.current_state.as_mut().unwrap().add_autobind(&autobind_name);
            } else {
                panic!("No type to create autobind for");
            }


            let mut specifiers = HashSet::new();
            let mut found_access_modif_before = false;
            for (spec, span) in n.specifiers().map(|specn| (specn.value(), specn.span())) {
                if matches!(spec, AutobindSpecifier::AccessModifier(_)) {
                    if found_access_modif_before {
                        self.diagnostics.push(Diagnostic { 
                            span, 
                            severity: DiagnosticSeverity::Error, 
                            body: DiagnosticBody::MultipleAccessModifiers 
                        })
                    }
                    found_access_modif_before = true;
                }

                if !specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        severity: DiagnosticSeverity::Error, 
                        body: DiagnosticBody::RepeatedSpecifier
                    });
                }
            }


            let type_id = self.get_type_from_node(n.autobind_type());


            sym.data.specifiers = specifiers;
            sym.data.type_id = type_id;

            self.symtab.insert(&sym);
            self.db.insert_autobind(sym);
        }
    }
}
