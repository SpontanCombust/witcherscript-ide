use std::collections::HashSet;
use ropey::Rope;
use witcherscript::attribs::*;
use witcherscript::ast::*;
use crate::diagnostics::*;
use crate::model::collections::symbol_table::SymbolTable;
use crate::model::symbol_path::SymbolPath;
use crate::model::symbols::*;
use super::commons::SymbolCollectorCommons;

//TODO use this visitor to only collect symbol data
// So have 2 visitors:
// 1. SymbolCollector, 2. SymbolDataCollector
// first two only create symbols
// the last one uses node id for fast lookup
struct ChildSymbolCollector<'a> {
    symtab: &'a mut SymbolTable,
    rope: Rope,
    diagnostics: Vec<Diagnostic>,

    current_path: SymbolPath
}

impl SymbolCollectorCommons for ChildSymbolCollector<'_> {
    fn symtab(&mut self) -> &mut SymbolTable {
        &mut self.symtab
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
            let mut specifiers = HashSet::new();
            for (spec, span) in n.specifiers().map(|specn| (specn.value(), specn.span())) {
                if !specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }

            let base_path = n.base().map(|base| self.check_type_from_identifier(base));


            let path = BasicTypeSymbolPath::new(&class_name);
            if let Some(sym) = self.symtab.get_mut(&path).and_then(|variant| variant.as_class_mut()) {
                sym.specifiers = specifiers;
                sym.base_path = base_path;

                sym.path().clone_into(&mut self.current_path);
                return true;
            }
        }

        false
    }

    fn exit_class_decl(&mut self, _: &ClassDeclarationNode) {
        // even if class' insides were not visited, this will just clear the path 
        self.current_path.pop();
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> bool {
        let state_name = n.name().value(&self.rope);
        let parent_name = n.parent().value(&self.rope);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let path = StateSymbolPath::new(&state_name, BasicTypeSymbolPath::new(&parent_name));
            if let Some(sym) = self.symtab.get_mut(&path).and_then(|variant| variant.as_state_mut()) {
                n.specifiers()
                .map(|specn| (specn.value(), specn.span()))
                .for_each(|(spec, span)| {
                    if !sym.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            span, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                });
    
                sym.base_state_name = n.base().and_then(|base| base.value(&self.rope)).map(|ident| ident.into());
        
                sym.path().clone_into(&mut self.current_path);
                return true;
            }
        }

        false
    }

    fn exit_state_decl(&mut self, _: &StateDeclarationNode) {
        self.current_path.pop();
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> bool {
        if let Some(struct_name) = n.name().value(&self.rope) {
            let path = BasicTypeSymbolPath::new(&struct_name);
            if let Some(sym) = self.symtab.get_mut(&path).and_then(|variant| variant.as_struct_mut()) {
                n.specifiers()
                .map(|specn| (specn.value(), specn.span()))
                .for_each(|(spec, span)| {
                    if !sym.specifiers.insert(spec) {
                        self.diagnostics.push(Diagnostic { 
                            span, 
                            body: ErrorDiagnostic::RepeatedSpecifier.into()
                        });
                    }
                });
    
                sym.path().clone_into(&mut self.current_path);
                return true;
            }
        }

        false
    }

    fn exit_struct_decl(&mut self, _: &StructDeclarationNode) {
        self.current_path.pop();
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode) {
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

        let type_path = self.check_type_from_type_annot(n.var_type());

        
        for name_node in n.names() {
            if let Some(var_name) = name_node.value(&self.rope) {
                let path = DataSymbolPath::new(&self.current_path, &var_name);
                let mut sym = MemberVarSymbol::new(path);
                sym.specifiers = specifiers.clone();
                sym.type_path = type_path.clone();
                self.try_insert_with_duplicate_check(sym, name_node.span());
            }
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode) {
        if let Some(autobind_name) = n.name().value(&self.rope) {
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

            let type_path = self.check_type_from_type_annot(n.autobind_type());
            
            let path = DataSymbolPath::new(&self.current_path, &autobind_name);
            let mut sym = AutobindSymbol::new(path);
            sym.specifiers = specifiers;
            sym.type_path = type_path;
            self.try_insert_with_duplicate_check(sym, n.name().span());
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> bool {
        if let Some(func_name) = n.name().value(&self.rope) {
            let mut specifiers = HashSet::new();
            for (spec, span) in n.specifiers().map(|specn| (specn.value(), specn.span())) {
                if !specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }

            let flavour = n.flavour().map(|flavn| flavn.value());

            let return_type_path = if let Some(ret_typn) = n.return_type() {
                self.check_type_from_type_annot(ret_typn)
            } else {
                TypeSymbolPath::Basic(BasicTypeSymbolPath::new("void"))
            };


            let path = GlobalCallableSymbolPath::new(&func_name);
            if let Some(sym) = self.symtab.get_mut(&path).and_then(|variant| variant.as_global_func_mut()) {
                sym.specifiers = specifiers;
                sym.flavour = flavour;
                sym.return_type_path = return_type_path;

                sym.path().clone_into(&mut self.current_path);
                return true;
            }
        }

        false
    }

    fn exit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) {
        self.current_path.pop();
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode) -> bool {
        if let Some(func_name) = n.name().value(&self.rope) {
            let path = MemberCallableSymbolPath::new(&self.current_path, &func_name);
            let mut sym = MemberFunctionSymbol::new(path);

            n.specifiers()
            .map(|specn| (specn.value(), specn.span()))
            .for_each(|(spec, span)| {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        span, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            });

            sym.flavour = n.flavour().map(|flavn| flavn.value());

            sym.return_type_path = if let Some(ret_typn) = n.return_type() {
                self.check_type_from_type_annot(ret_typn)
            } else {
                TypeSymbolPath::Basic(BasicTypeSymbolPath::new("void"))
            };

            sym.path().clone_into(&mut self.current_path);
            if self.try_insert_with_duplicate_check(sym, n.name().span()) {
                return true;
            }
        }

        false
    }

    fn exit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode) {
        // pop only if visit managed to create the symbol
        if self.current_path.components().last().unwrap().category == SymbolCategory::Callable {
            self.current_path.pop();
        }
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode) -> bool {
        if let Some(event_name) = n.name().value(&self.rope) {
            let path = MemberCallableSymbolPath::new(&self.current_path, &event_name);
            let sym = EventSymbol::new(path);

            sym.path().clone_into(&mut self.current_path);
            if self.try_insert_with_duplicate_check(sym, n.name().span()) {
                return true;
            }
        }

        false
    }

    fn exit_event_decl(&mut self, _: &EventDeclarationNode) {
        if self.current_path.components().last().unwrap().category == SymbolCategory::Callable {
            self.current_path.pop();
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode) {
        let mut specifiers = HashSet::new();
        for (spec, span) in n.specifiers().map(|specn| (specn.value(), specn.span())) {
            if !specifiers.insert(spec) {
                self.diagnostics.push(Diagnostic { 
                    span, 
                    body: ErrorDiagnostic::RepeatedSpecifier.into()
                });
            }
        }

        let type_path = self.check_type_from_type_annot(n.param_type());


        for name_node in n.names() {
            if let Some(param_name) = name_node.value(&self.rope) {
                let path = DataSymbolPath::new(&self.current_path, &param_name);
                let mut sym = FunctionParameterSymbol::new(path);
                sym.specifiers = specifiers.clone();
                sym.type_path = type_path.clone();
                self.try_insert_with_duplicate_check(sym, name_node.span());
            }
        }
    }
}
