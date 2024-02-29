use std::collections::HashSet;
use lsp_types::Range;
use witcherscript::attribs::AutobindSpecifier;
use witcherscript::attribs::MemberVarSpecifier;
use witcherscript::script_document::ScriptDocument;
use witcherscript::tokens::*;
use witcherscript::ast::*;
use crate::model::collections::symbol_table::SymbolTable;
use crate::model::symbol_path::SymbolPath;
use crate::model::symbol_variant::SymbolVariant;
use crate::model::symbols::*;
use crate::diagnostics::*;


pub fn scan_symbols(script_root: RootNode, doc: &ScriptDocument, symtab: &mut SymbolTable, diagnostics: &mut Vec<Diagnostic>) {
    let mut visitor = SymbolScannerVisitor {
        symtab,
        doc,
        diagnostics,
        current_path: SymbolPath::empty()
    };

    script_root.accept(&mut visitor);
}


struct SymbolScannerVisitor<'a> {
    symtab: &'a mut SymbolTable,
    doc: &'a ScriptDocument,
    diagnostics: &'a mut Vec<Diagnostic>,

    current_path: SymbolPath
}

impl SymbolScannerVisitor<'_> {
    /// Inserts the symbol into symbol table, but only if it is not a duplicate.
    /// Returns true if symbol was inserted successfully, false otherwise.
    fn try_insert_with_duplicate_check<S>(&mut self, sym: S, range: Range) -> bool 
    where S: Symbol + Into<SymbolVariant> {
        let sym_typ = sym.typ();
        if let Err(err) = self.symtab.insert(sym) {
            self.diagnostics.push(Diagnostic { 
                range, 
                body: ErrorDiagnostic::SymbolNameTaken { 
                    name: err.occupied_path.components().last().unwrap().name.to_string(), 
                    this_type: sym_typ, 
                    precursor_type: err.occupyed_type
                }.into()
            });
            
            false
        } else {
            true
        }
    }

    /// Returns type path and type name, if it's invalid returns empty path
    fn check_type_from_identifier(&mut self, n: IdentifierNode) -> TypeSymbolPath {
        if let Some(type_name) = n.value(&self.doc) {
            if type_name.as_str() == ArrayTypeSymbol::TYPE_NAME {
                self.diagnostics.push(Diagnostic { 
                    range: Range::new(n.range().end, n.range().end), 
                    body: ErrorDiagnostic::MissingTypeArg.into()
                });
            } else {
                let path = TypeSymbolPath::Basic(BasicTypeSymbolPath::new(&type_name));
                return path;
            }
        }

        TypeSymbolPath::empty()
    }

    /// Returns type path and type name, if it's invalid returns empty path
    fn check_type_from_type_annot(&mut self, n: TypeAnnotationNode) -> TypeSymbolPath {
        if let Some(type_arg_node) = n.type_arg() {
            if let Some(type_name) = n.type_name().value(&self.doc) {
                if type_name.as_str() == ArrayTypeSymbol::TYPE_NAME {
                    let type_arg_path = self.check_type_from_type_annot(type_arg_node);
                    if !type_arg_path.is_empty() {
                        let path = TypeSymbolPath::Array(ArrayTypeSymbolPath::new(type_arg_path));
                        return path;
                    }   
                } else {
                    // since only array type takes type argument, all other uses of type arg are invalid
                    self.diagnostics.push(Diagnostic { 
                        range: n.type_arg().unwrap().range(), 
                        body: ErrorDiagnostic::UnnecessaryTypeArg.into()
                    });

                    return self.check_type_from_identifier(n.type_name());
                }
            }

            TypeSymbolPath::empty()
        } else {
            self.check_type_from_identifier(n.type_name())
        }   
    }
}



impl StatementVisitor for SymbolScannerVisitor<'_> {
    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> bool {
        if let Some(class_name) = n.name().value(&self.doc) {
            let path = BasicTypeSymbolPath::new(&class_name);
            let mut sym = ClassSymbol::new(path);

            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }

            sym.base_path = n.base().map(|base| self.check_type_from_identifier(base));

            sym.path().clone_into(&mut self.current_path);
            if self.try_insert_with_duplicate_check(sym, n.name().range()) {
                return true;
            }
        }

        false
    }

    fn exit_class_decl(&mut self, _: &ClassDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
        }
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> bool {
        let state_name = n.name().value(&self.doc);
        let parent_name = n.parent().value(&self.doc);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let path = StateSymbolPath::new(&state_name, BasicTypeSymbolPath::new(&parent_name));
            let mut sym = StateSymbol::new(path);

            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }

            sym.base_state_name = n.base().and_then(|base| base.value(&self.doc)).map(|ident| ident.into());

            sym.path().clone_into(&mut self.current_path);
            if self.try_insert_with_duplicate_check(sym, n.name().range()) {
                return true;
            }
        }

        false            
    }

    fn exit_state_decl(&mut self, _: &StateDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
        }
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> bool {
        if let Some(struct_name) = n.name().value(&self.doc) {
            let path = BasicTypeSymbolPath::new(&struct_name);
            let mut sym = StructSymbol::new(path);

            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }

            sym.path().clone_into(&mut self.current_path);
            if self.try_insert_with_duplicate_check(sym, n.name().range()) {
                return true;
            }
        }

        false
    }

    fn exit_struct_decl(&mut self, _: &StructDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
        }
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> bool {
        if let Some(enum_name) = n.name().value(&self.doc) {
            let path = BasicTypeSymbolPath::new(&enum_name);
            let sym = EnumSymbol::new(path);

            sym.path().clone_into(&mut self.current_path);
            if self.try_insert_with_duplicate_check(sym, n.name().range()) {
                return true;
            }
        }

        false
    }

    fn exit_enum_decl(&mut self, _: &EnumDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
        }
    }

    fn visit_enum_member_decl(&mut self, n: &EnumMemberDeclarationNode) {
        if let Some(enum_member_name) = n.name().value(&self.doc) {
            let path = DataSymbolPath::new(&self.current_path, &enum_member_name);
            let sym = EnumMemberSymbol::new(path);

            self.try_insert_with_duplicate_check(sym, n.name().range());
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> (bool, bool) {
        if let Some(func_name) = n.name().value(&self.doc) {
            let path = GlobalCallableSymbolPath::new(&func_name);
            let mut sym = GlobalFunctionSymbol::new(path);

            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }

            sym.flavour = n.flavour().map(|flavn| flavn.value());

            sym.return_type_path = if let Some(ret_typn) = n.return_type() {
                self.check_type_from_type_annot(ret_typn)
            } else {
                TypeSymbolPath::Basic(BasicTypeSymbolPath::new("void"))
            };

            sym.path().clone_into(&mut self.current_path);
            if self.try_insert_with_duplicate_check(sym, n.name().range()) {
                return (true, true);
            }
        }

        (false, false)
    }

    fn exit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Callable).unwrap_or(false)  {
            self.current_path.pop();
        }
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode) -> (bool, bool) {
        if let Some(func_name) = n.name().value(&self.doc) {
            let path = MemberCallableSymbolPath::new(&self.current_path, &func_name);
            let mut sym = MemberFunctionSymbol::new(path);

            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }

            sym.flavour = n.flavour().map(|flavn| flavn.value());

            sym.return_type_path = if let Some(ret_typn) = n.return_type() {
                self.check_type_from_type_annot(ret_typn)
            } else {
                TypeSymbolPath::Basic(BasicTypeSymbolPath::new("void"))
            };

            sym.path().clone_into(&mut self.current_path);
            if self.try_insert_with_duplicate_check(sym, n.name().range()) {
                return (true, true);
            }
        }

        (false, false)
    }

    fn exit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode) {
        // pop only if visit managed to create the symbol
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Callable).unwrap_or(false)  {
            self.current_path.pop();
        }
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode) -> (bool, bool) {
        if let Some(event_name) = n.name().value(&self.doc) {
            let path = MemberCallableSymbolPath::new(&self.current_path, &event_name);
            let sym = EventSymbol::new(path);

            sym.path().clone_into(&mut self.current_path);
            if self.try_insert_with_duplicate_check(sym, n.name().range()) {
                return (true, true);
            }
        }

        (false, false)
    }

    fn exit_event_decl(&mut self, _: &EventDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Callable).unwrap_or(false)  {
            self.current_path.pop();
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode) {
        let mut specifiers = HashSet::new();
        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if !specifiers.insert(spec) {
                self.diagnostics.push(Diagnostic { 
                    range, 
                    body: ErrorDiagnostic::RepeatedSpecifier.into()
                });
            }
        }

        let type_path = self.check_type_from_type_annot(n.param_type());


        for name_node in n.names() {
            if let Some(param_name) = name_node.value(&self.doc) {
                let path = DataSymbolPath::new(&self.current_path, &param_name);
                let mut sym = FunctionParameterSymbol::new(path);
                sym.specifiers = specifiers.clone();
                sym.type_path = type_path.clone();
                self.try_insert_with_duplicate_check(sym, name_node.range());
            }
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode) {
        let mut specifiers = HashSet::new();
        let mut found_access_modif_before = false;
        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if matches!(spec, MemberVarSpecifier::AccessModifier(_)) {
                if found_access_modif_before {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        body: ErrorDiagnostic::MultipleAccessModifiers.into()
                    })
                }
                found_access_modif_before = true;
            }

            if !specifiers.insert(spec) {
                self.diagnostics.push(Diagnostic { 
                    range, 
                    body: ErrorDiagnostic::RepeatedSpecifier.into()
                });
            }
        }

        let type_path = self.check_type_from_type_annot(n.var_type());

        
        for name_node in n.names() {
            if let Some(var_name) = name_node.value(&self.doc) {
                let path = DataSymbolPath::new(&self.current_path, &var_name);
                let mut sym = MemberVarSymbol::new(path);
                sym.specifiers = specifiers.clone();
                sym.type_path = type_path.clone();
                self.try_insert_with_duplicate_check(sym, name_node.range());
            }
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode) {
        if let Some(autobind_name) = n.name().value(&self.doc) {
            let path = DataSymbolPath::new(&self.current_path, &autobind_name);
            let mut sym = AutobindSymbol::new(path);

            let mut found_access_modif_before = false;
            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if matches!(spec, AutobindSpecifier::AccessModifier(_)) {
                    if found_access_modif_before {
                        self.diagnostics.push(Diagnostic { 
                            range, 
                            body: ErrorDiagnostic::MultipleAccessModifiers.into()
                        })
                    }
                    found_access_modif_before = true;
                }

                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(Diagnostic { 
                        range, 
                        body: ErrorDiagnostic::RepeatedSpecifier.into()
                    });
                }
            }

            sym.type_path = self.check_type_from_type_annot(n.autobind_type());

            self.try_insert_with_duplicate_check(sym, n.name().range());
        }
    }

    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode) {
        let type_path = self.check_type_from_type_annot(n.var_type());

        for name_node in n.names() {
            if let Some(var_name) = name_node.value(&self.doc) {
                let path = DataSymbolPath::new(&self.current_path, &var_name);
                let mut sym = LocalVarSymbol::new(path);
                sym.type_path = type_path.clone();
                self.try_insert_with_duplicate_check(sym, name_node.range());
            }
        }
    }
}