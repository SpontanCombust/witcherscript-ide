use std::collections::HashSet;
use ropey::Rope;
use witcherscript::attribs::*;
use witcherscript::ast::*;
use crate::model::collections::*;
use crate::diagnostics::*;
use crate::model::symbols::*;
use super::commons::SymbolCollectorCommons;

//TODO use this visitor to only collect symbol data
// So have 3 visitors:
// 1. GlobalSymbolCollector, 2. ChildSymbolCollector, 3. SymbolDataCollector
// first two only create symbols
// the last one uses node id for fast lookup
struct ChildSymbolCollector<'a> {
    db: &'a mut SymbolDb,
    ctx: &'a mut SymbolContext,
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

    fn ctx(&mut self) -> &mut SymbolContext {
        &mut self.ctx
    }

    fn db_and_ctx(&mut self) -> (&mut SymbolDb, &mut SymbolContext) {
        (&mut self.db, &mut self.ctx)    
    }

    fn diagnostics(&mut self) -> &mut Vec<Diagnostic> {
        &mut self.diagnostics
    }

    fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl StatementVisitor for ChildSymbolCollector<'_> {
    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> bool {
        if let Some(class_name) = n.name().value(&self.rope) {
            if let Some(SymbolPointer { id, typ: SymbolType::Class }) = self.ctx.get(&class_name, SymbolCategory::Type) {
                let mut sym = self.db.remove_class(*id).expect("class absent in db despite being found in context");

                n.specifiers()
                .map(|specn| (specn.value(), specn.span()))
                .for_each(|(spec, span)| {
                    if !sym.data.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            span, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                });

                if let Some(base_node) = n.base() {
                    if let Some(id) = base_node.value(&self.rope).and_then(|base| self.check_type(&base, None, base_node.span())) {
                        sym.data.base_id = Some(id);
                    }
                }

                self.current_class = Some(sym);
                self.ctx.push_scope();
                return true;
            }
        }

        false
    }

    fn exit_class_decl(&mut self, _: &ClassDeclarationNode) {
        if let Some(sym) = self.current_class.take() {
            self.ctx.pop_scope();
            self.db.insert_class(sym);
        }
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> bool {
        let state_name = n.name().value(&self.rope);
        let parent_name = n.parent().value(&self.rope);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let state_class_name = StateSymbol::class_name(&state_name, &parent_name);

            if let Some(SymbolPointer { id, typ: SymbolType::State }) = self.ctx.get(&state_class_name, SymbolCategory::Type) {
                let mut sym = self.db.remove_state(*id).expect("state absent in db despite being found in context");

                n.specifiers()
                .map(|specn| (specn.value(), specn.span()))
                .for_each(|(spec, span)| {
                    if !sym.data.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            span, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
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
                self.ctx.push_scope();
                return true;
            }
        }

        false
    }

    fn exit_state_decl(&mut self, _: &StateDeclarationNode) {
        if let Some(sym) = self.current_state.take() {
            self.ctx.pop_scope();
            self.db.insert_state(sym);
        }
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> bool {
        if let Some(struct_name) = n.name().value(&self.rope) {
            if let Some(SymbolPointer { id, typ: SymbolType::Struct }) = self.ctx.get(&struct_name, SymbolCategory::Type) {
                let mut sym = self.db.remove_struct(*id).expect("struct absent in db despite being found in context");

                n.specifiers()
                .map(|specn| (specn.value(), specn.span()))
                .for_each(|(spec, span)| {
                    if !sym.data.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            span, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                });

                self.current_struct = Some(sym);
                self.ctx.push_scope();
                return true;
            }
        }

        false
    }

    fn exit_struct_decl(&mut self, _: &StructDeclarationNode) {
        if let Some(sym) = self.current_struct.take() {
            self.ctx.pop_scope();
            self.db.insert_struct(sym);
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode) {
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
                            body: ErrorDiagnostic::MultipleAccessModifiers.into()
                        })
                    }
                    found_access_modif_before = true;
                }

                if !specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }


            let type_id = self.get_type_from_node(n.var_type());


            syms.into_iter().for_each(|mut sym| {
                sym.data.specifiers = specifiers.clone();
                sym.data.type_id = type_id;

                self.ctx.insert(&sym);
                self.db.insert_member_var(sym);
            });
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode) {
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
                            body: ErrorDiagnostic::MultipleAccessModifiers.into()
                        })
                    }
                    found_access_modif_before = true;
                }

                if !specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }


            let type_id = self.get_type_from_node(n.autobind_type());


            sym.data.specifiers = specifiers;
            sym.data.type_id = type_id;

            self.ctx.insert(&sym);
            self.db.insert_autobind(sym);
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> bool {
        if let Some(func_name) = n.name().value(&self.rope) {
            if let Some(SymbolPointer { id, typ: SymbolType::GlobalFunction }) = self.ctx.get(&func_name, SymbolCategory::Callable) {
                let mut sym = self.db.remove_global_func(*id).expect("global function absent from db despite being found in context");

                n.specifiers()
                .map(|specn| (specn.value(), specn.span()))
                .for_each(|(spec, span)| {
                    if !sym.data.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            span, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                });


                if let Some(flavour) = n.flavour().map(|flavn| flavn.value()) {
                    sym.data.flavour = Some(flavour);
                } else {
                    sym.data.flavour = None;
                }


                if let Some(ret_typn) = n.return_type() {
                    sym.data.return_type_id = self.get_type_from_node(ret_typn);
                } else {
                    sym.data.return_type_id = self.ctx.get("void", SymbolCategory::Type).unwrap().id;
                }


                self.current_global_func = Some(sym);
                self.ctx.push_scope();
                return true;
            }
        }

        false
    }

    fn exit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) {
        if let Some(sym) = self.current_global_func.take() {
            self.ctx.pop_scope();
            self.db.insert_global_func(sym);
        }
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode) -> bool {
        let func_name = n.name()
                        .value(&self.rope)
                        .and_then(|ident| self.check_duplicate(ident.into(), SymbolType::MemberFunction, n.span()));

        if let Some(func_name) = func_name {
            let mut sym: MemberFunctionSymbol;
            if self.current_class.is_some() {
                sym = self.current_class.as_mut().unwrap().add_member_func(&func_name);
            } else if self.current_state.is_some() {
                sym = self.current_state.as_mut().unwrap().add_member_func(&func_name);
            } else {
                panic!("No type to create member function for");
            }


            n.specifiers()
            .map(|specn| (specn.value(), specn.span()))
            .for_each(|(spec, span)| {
                if !sym.data.specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            });


            if let Some(flavour) = n.flavour().map(|flavn| flavn.value()) {
                sym.data.flavour = Some(flavour);
            } else {
                sym.data.flavour = None;
            }


            if let Some(ret_typn) = n.return_type() {
                sym.data.return_type_id = self.get_type_from_node(ret_typn);
            } else {
                sym.data.return_type_id = self.ctx.get("void", SymbolCategory::Type).unwrap().id;
            }


            self.ctx.insert(&sym);
            self.ctx.push_scope();
            self.current_member_func = Some(sym);
            return true;
        }

        false
    }

    fn exit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode) {
        if let Some(sym) = self.current_member_func.take() {
            self.ctx.pop_scope();
            self.db.insert_member_func(sym);
        }
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode) -> bool {
        let event_name = n.name()
                        .value(&self.rope)
                        .and_then(|ident| self.check_duplicate(ident.into(), SymbolType::Event, n.span()));

        if let Some(event_name) = event_name {
            let sym: EventSymbol;
            if self.current_class.is_some() {
                sym = self.current_class.as_mut().unwrap().add_event(&event_name);
            } else if self.current_state.is_some() {
                sym = self.current_state.as_mut().unwrap().add_event(&event_name);
            } else {
                panic!("No type to create event for");
            }

            self.ctx.insert(&sym);
            self.ctx.push_scope();
            self.current_event = Some(sym);
            return true;
        }

        false
    }

    fn exit_event_decl(&mut self, _: &EventDeclarationNode) {
        if let Some(sym) = self.current_event.take() {
            self.ctx.pop_scope();
            self.db.insert_event(sym);
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode) {
        let names = n.names()
                    .map(|identn| (identn.span(), identn))
                    .filter_map(|(span, identn)| {
                        if let Some(ident) = identn.value(&self.rope) {
                            Some((span, ident))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

        let mut checked_names: Vec<String> = Vec::new();
        for (span, name) in names.into_iter() {
            if let Some(checked_name) = self.check_duplicate(name.into(), SymbolType::Parameter, span) {
                checked_names.push(checked_name);
            }
        }
                
        if !checked_names.is_empty() {
            let syms: Vec<FunctionParameterSymbol>;
            if self.current_global_func.is_some() {
                syms = checked_names.into_iter().map(|n| self.current_global_func.as_mut().unwrap().add_param(&n)).collect();
            } else if self.current_member_func.is_some() {
                syms = checked_names.into_iter().map(|n| self.current_member_func.as_mut().unwrap().add_param(&n)).collect();
            } else if self.current_event.is_some() {
                syms = checked_names.into_iter().map(|n| self.current_event.as_mut().unwrap().add_param(&n)).collect();
            } else {
                panic!("No function to create parameter for");
            }


            let mut specifiers = HashSet::new();
            for (spec, span) in n.specifiers().map(|specn| (specn.value(), specn.span())) {
                if !specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }


            let type_id = self.get_type_from_node(n.param_type());


            syms.into_iter().for_each(|mut sym| {
                sym.data.specifiers = specifiers.clone();
                sym.data.type_id = type_id;

                self.ctx.insert(&sym);
                self.db.insert_func_param(sym);
            });
        }
    }
}
