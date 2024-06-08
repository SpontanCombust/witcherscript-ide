use std::path::Path;
use std::sync::Arc;
use lsp_types::Range;
use witcherscript::attribs::*;
use witcherscript::script_document::ScriptDocument;
use witcherscript::Script;
use witcherscript::tokens::*;
use witcherscript::ast::*;
use witcherscript_diagnostics::*;
use crate::symbol_analysis::symbol_table::SymbolTable;
use crate::symbol_analysis::symbol_path::{SymbolPath, SymbolPathBuf};
use crate::symbol_analysis::symbols::*;


pub fn scan_symbols(
    script: &Script, 
    doc: &ScriptDocument, 
    local_source_path: &Path,
    symtab: &mut SymbolTable,
    diagnostics: &mut Vec<LocatedDiagnostic>
) {
    let mut visitor = SymbolScannerVisitor {
        symtab,
        doc,
        local_source_path: local_source_path.into(),
        diagnostics,
        current_path: SymbolPathBuf::empty(),
        current_constr_path: None,
        current_param_ordinal: 0,
        current_var_ordinal: 0,
        current_enum_variant_value: 0
    };

    script.visit_nodes(&mut visitor);
}


struct SymbolScannerVisitor<'a> {
    symtab: &'a mut SymbolTable,
    doc: &'a ScriptDocument,
    local_source_path: Arc<Path>,
    diagnostics: &'a mut Vec<LocatedDiagnostic>,

    current_path: SymbolPathBuf,
    current_constr_path: Option<SymbolPathBuf>,
    current_param_ordinal: usize,
    current_var_ordinal: usize,
    current_enum_variant_value: i32
}

impl SymbolScannerVisitor<'_> {
    // Returns whether the symbol is not a duplicate
    fn check_contains(&mut self, path: &SymbolPath, range: Range) -> bool {
        if let Err(err) = self.symtab.test_contains_symbol(path) {
            // missing nodes don't get an error, as it's the job of syntax analysis to detect them and inform the user about them
            // these situations are very rare anyways, so doing anything aside from showing a diagnostic is an overkill
            if !path.has_missing() {
                let (precursor_file_path, precursor_range) = err.occupied_location
                    .map(|loc| (Some(self.symtab.script_root().join(loc.local_source_path).unwrap()), Some(loc.label_range)))
                    .unwrap_or((None, None));
    
                self.diagnostics.push(LocatedDiagnostic { 
                    path: self.symtab.script_root().join(&self.local_source_path).unwrap(), 
                    diagnostic: Diagnostic { 
                        range, 
                        kind: DiagnosticKind::SymbolNameTaken { 
                            name: err.occupied_path.components().last().unwrap().name.to_string(),
                            precursor_file_path,
                            precursor_range
                        }
                    }
                });
            }
            
            false
        } else {
            true
        }
    }

    /// Returns type path and type name, if it's invalid returns empty path
    fn check_type_from_identifier(&mut self, n: IdentifierNode) -> BasicTypeSymbolPath {
        let type_name = n.value(&self.doc);
        if type_name == ArrayTypeSymbol::TYPE_NAME {
            self.diagnostics.push(LocatedDiagnostic { 
                path: self.symtab.script_root().join(&self.local_source_path).unwrap(), 
                diagnostic: Diagnostic { 
                    range: n.range(), 
                    kind: DiagnosticKind::MissingTypeArg
                }
            });
        } else {
            return BasicTypeSymbolPath::new(&type_name);
        }

        BasicTypeSymbolPath::unknown()
    }

    /// Returns type path and type name, if it's invalid returns empty path
    fn check_type_from_type_annot(&mut self, n: TypeAnnotationNode) -> TypeSymbolPath {
        let type_name_node = n.type_name();
        if let Some(type_arg_node) = n.type_arg() {
            let type_name = type_name_node.value(&self.doc);
            if type_name == ArrayTypeSymbol::TYPE_NAME {
                let type_arg_path = self.check_type_from_type_annot(type_arg_node);
                if !type_arg_path.is_empty() {
                    let array_path = ArrayTypeSymbolPath::new(type_arg_path);
                    if !self.symtab.contains_symbol(&array_path) {
                        self.inject_array_type(array_path.clone());
                    }
                    return TypeSymbolPath::Array(array_path);
                }   
            } else {
                // since only array type takes type argument, all other uses of type arg are invalid
                self.diagnostics.push(LocatedDiagnostic { 
                    path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                    diagnostic: Diagnostic { 
                        range: type_name_node.range(), 
                        kind: DiagnosticKind::UnnecessaryTypeArg
                    }
                });

                return self.check_type_from_identifier(type_name_node).into();
            }

            TypeSymbolPath::unknown()
        } else {
            self.check_type_from_identifier(type_name_node).into()
        }   
    }

    fn inject_array_type(&mut self, array_sympath: ArrayTypeSymbolPath) {
        let arr = ArrayTypeSymbol::new(array_sympath);
        let (funcs, params) = arr.make_functions();
        self.symtab.insert_array_type_symbol(arr, &self.local_source_path);
        funcs.into_iter().for_each(|f| { self.symtab.insert_symbol(f); } );
        params.into_iter().for_each(|p| { self.symtab.insert_symbol(p); } );
    }
}



impl SyntaxNodeVisitor for SymbolScannerVisitor<'_> {
    fn traversal_policy_default(&self) -> bool {
        false
    }


    fn visit_root(&mut self, _: &RootNode) -> RootTraversalPolicy {
        RootTraversalPolicy { traverse: true }
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        let mut traverse_definition = false;

        let name_node = n.name();
        let class_name = name_node.value(&self.doc);
        let path = BasicTypeSymbolPath::new(&class_name);
        if self.check_contains(&path, name_node.range()) {
            let mut sym = ClassSymbol::new(path.clone(), SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: name_node.range()
            });
            
            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(LocatedDiagnostic { 
                        path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                        diagnostic: Diagnostic { 
                            range, 
                            kind: DiagnosticKind::RepeatedSpecifier
                        }
                    });
                }
            }
    
            sym.base_path = n.base().map(|base| self.check_type_from_identifier(base));


            let this_path = ThisVarSymbolPath::new(&path);
            let this_sym = ThisVarSymbol::new(this_path, path.clone().into());
            self.symtab.insert_symbol(this_sym);

            if let Some(base_path) = &sym.base_path {
                let super_path = SuperVarSymbolPath::new(&path);
                let super_sym = SuperVarSymbol::new(super_path, base_path.clone().into());
                self.symtab.insert_symbol(super_sym);
            }


            path.as_ref().clone_into(&mut self.current_path);
            self.symtab.insert_primary_symbol(sym);
            
            traverse_definition = true;
        }

        ClassDeclarationTraversalPolicy { 
            traverse_definition 
        }
    }

    fn exit_class_decl(&mut self, _: &ClassDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
            self.current_var_ordinal = 0;
        }
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        let mut traverse_definition = false;

        let state_name_node = n.name();
        let state_name = state_name_node.value(&self.doc);
        let parent_name = n.parent().value(&self.doc);
        let path = StateSymbolPath::new(&state_name, &parent_name);
        if self.check_contains(&path, state_name_node.range()) {
            let mut sym = StateSymbol::new(path.clone(), SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: state_name_node.range()
            });

            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(LocatedDiagnostic { 
                        path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                        diagnostic: Diagnostic { 
                            range, 
                            kind: DiagnosticKind::RepeatedSpecifier
                        }
                    });
                }
            }

            sym.base_state_name = n.base().map(|base| base.value(&self.doc).to_string());


            let this_path = ThisVarSymbolPath::new(&path);
            let this_sym = ThisVarSymbol::new(this_path, path.clone().into());
            self.symtab.insert_symbol(this_sym);

            let super_path = SuperVarSymbolPath::new(&path);
            let super_sym = StateSuperVarSymbol::new(super_path, sym.base_state_name.clone());
            self.symtab.insert_symbol(super_sym);
            
            let parent_path = ParentVarSymbolPath::new(&path);
            let parent_sym = ParentVarSymbol::new(parent_path, path.parent_class_path.clone().into());
            self.symtab.insert_symbol(parent_sym);

            let virtual_parent_path = VirtualParentVarSymbolPath::new(&path);
            let virtual_parent_sym = VirtualParentVarSymbol::new(virtual_parent_path, path.parent_class_path.clone().into());
            self.symtab.insert_symbol(virtual_parent_sym);


            path.as_ref().clone_into(&mut self.current_path);
            self.symtab.insert_primary_symbol(sym);

            traverse_definition = true;
        }

        StateDeclarationTraversalPolicy { 
            traverse_definition 
        }       
    }

    fn exit_state_decl(&mut self, _: &StateDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
            self.current_var_ordinal = 0;
        }
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        let mut traverse_definition = false;

        let name_node = n.name();
        let struct_name = name_node.value(&self.doc);
        let path = BasicTypeSymbolPath::new(&struct_name);
        if self.check_contains(&path, name_node.range()) {
            let mut sym = StructSymbol::new(path.clone(), SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: name_node.range()
            });

            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(LocatedDiagnostic { 
                        path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                        diagnostic: Diagnostic { 
                            range, 
                            kind: DiagnosticKind::RepeatedSpecifier
                        }
                    });
                }
            }

            sym.path().clone_into(&mut self.current_path);
            self.symtab.insert_primary_symbol(sym);

            let constr_path = GlobalCallableSymbolPath::new(&struct_name);
            let mut constr_sym = ConstructorSymbol::new(constr_path, SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: name_node.range()
            });
            constr_sym.parent_type_path = path;

            self.current_constr_path = Some(constr_sym.path().to_owned());
            self.symtab.insert_primary_symbol(constr_sym);

            traverse_definition = true;
        }

        StructDeclarationTraversalPolicy { 
            traverse_definition 
        }
    }

    fn exit_struct_decl(&mut self, _: &StructDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_constr_path = None;
            self.current_path.pop();
            self.current_var_ordinal = 0;
        }
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        let mut traverse_definition = false;

        let name_node = n.name();
        let enum_name = name_node.value(&self.doc);
        let path = BasicTypeSymbolPath::new(&enum_name);
        if self.check_contains(&path, name_node.range()) {
            let sym = EnumSymbol::new(path, SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: name_node.range()
            });

            sym.path().clone_into(&mut self.current_path);
            self.symtab.insert_primary_symbol(sym);

            traverse_definition = true;
        }

        EnumDeclarationTraversalPolicy { 
            traverse_definition 
        }
    }

    fn exit_enum_decl(&mut self, _: &EnumDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Type).unwrap_or(false)  {
            self.current_path.pop();
            self.current_enum_variant_value = 0;
        }
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        let name_node = n.name();
        let enum_variant_name = name_node.value(&self.doc);
        let path = GlobalDataSymbolPath::new(&enum_variant_name);
        if self.check_contains(&path, name_node.range()) {
            let mut sym = EnumVariantSymbol::new(path, SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: name_node.range()
            });
            sym.parent_enum_path = BasicTypeSymbolPath::new(self.current_path.components().next().unwrap().name);

            let value = n.value()
                .and_then(|v| match v {
                    EnumVariantValue::Int(i) => {
                        i.value(self.doc).ok().map(|i| *i)
                    },
                    EnumVariantValue::Hex(h) => {
                        h.value(self.doc).ok().map(|h| *h).map(|h| i32::from_le_bytes(h.to_le_bytes()))
                    }
                })
                .unwrap_or(self.current_enum_variant_value);

            sym.value = value;
            self.current_enum_variant_value = value + 1;

            self.symtab.insert_primary_symbol(sym);
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> GlobalFunctionDeclarationTraversalPolicy {
        let mut traverse = false;

        let name_node = n.name();
        let func_name = name_node.value(&self.doc);
        let path = GlobalCallableSymbolPath::new(&func_name);
        if self.check_contains(&path, name_node.range()) {
            let mut sym = GlobalFunctionSymbol::new(path, SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: name_node.range()
            });

            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(LocatedDiagnostic { 
                        path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                        diagnostic: Diagnostic { 
                            range, 
                            kind: DiagnosticKind::RepeatedSpecifier
                        }
                    });
                }
            }

            sym.flavour = n.flavour().map(|flavn| flavn.value());

            sym.return_type_path = if let Some(ret_typn) = n.return_type() {
                self.check_type_from_type_annot(ret_typn)
            } else {
                TypeSymbolPath::BasicOrState(BasicTypeSymbolPath::new(GlobalFunctionSymbol::DEFAULT_RETURN_TYPE_NAME))
            };

            sym.path().clone_into(&mut self.current_path);
            self.symtab.insert_primary_symbol(sym);

            traverse = true;
        }

        GlobalFunctionDeclarationTraversalPolicy { 
            traverse_params: traverse,
            traverse_definition: traverse
        }
    }

    fn exit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Callable).unwrap_or(false)  {
            self.current_path.pop();
            self.current_param_ordinal = 0;
        }
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode, _: PropertyTraversalContext) -> MemberFunctionDeclarationTraversalPolicy {
        let mut traverse = false;

        let name_node = n.name();
        let func_name = name_node.value(&self.doc);
        let path = MemberCallableSymbolPath::new(&self.current_path, &func_name);
        if self.check_contains(&path, name_node.range()) {
            let mut sym = MemberFunctionSymbol::new(path, SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: name_node.range()
            });

            let mut found_access_modif_before = false;
            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if matches!(spec, MemberFunctionSpecifier::AccessModifier(_)) {
                    if found_access_modif_before {
                        self.diagnostics.push(LocatedDiagnostic { 
                            path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                            diagnostic: Diagnostic { 
                                range, 
                                kind: DiagnosticKind::MultipleAccessModifiers
                            }
                        });
                    }
                    found_access_modif_before = true;
                }

                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(LocatedDiagnostic { 
                        path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                        diagnostic: Diagnostic { 
                            range, 
                            kind: DiagnosticKind::RepeatedSpecifier
                        }
                    });
                }
            }

            sym.flavour = n.flavour().map(|flavn| flavn.value());

            sym.return_type_path = if let Some(ret_typn) = n.return_type() {
                self.check_type_from_type_annot(ret_typn)
            } else {
                TypeSymbolPath::BasicOrState(BasicTypeSymbolPath::new(MemberFunctionSymbol::DEFAULT_RETURN_TYPE_NAME))
            };

            sym.path().clone_into(&mut self.current_path);
            self.symtab.insert_symbol(sym);

            traverse = true;
        }

        MemberFunctionDeclarationTraversalPolicy {
            traverse_params: traverse,
            traverse_definition: traverse
        }
    }

    fn exit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode, _: PropertyTraversalContext) {
        // pop only if visit managed to create the symbol
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Callable).unwrap_or(false)  {
            self.current_path.pop();
            self.current_param_ordinal = 0;
        }
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: PropertyTraversalContext) -> EventDeclarationTraversalPolicy {
        let mut traverse = false;

        let name_node = n.name();
        let event_name = name_node.value(&self.doc);
        let path = MemberCallableSymbolPath::new(&self.current_path, &event_name);
        if self.check_contains(&path, name_node.range()) {
            let sym = EventSymbol::new(path, SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: name_node.range()
            });

            sym.path().clone_into(&mut self.current_path);
            self.symtab.insert_symbol(sym);

            traverse = true;
        }

        EventDeclarationTraversalPolicy { 
            traverse_params: traverse,
            traverse_definition: traverse
        }
    }

    fn exit_event_decl(&mut self, _: &EventDeclarationNode, _: PropertyTraversalContext) {
        if self.current_path.components().last().map(|comp| comp.category == SymbolCategory::Callable).unwrap_or(false)  {
            self.current_path.pop();
            self.current_param_ordinal = 0;
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: FunctionTraversalContext) {
        let mut specifiers = SymbolSpecifiers::new();
        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if !specifiers.insert(spec) {
                self.diagnostics.push(LocatedDiagnostic { 
                    path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                    diagnostic: Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    }
                });
            }
        }

        let type_path = self.check_type_from_type_annot(n.param_type());


        for name_node in n.names() {
            let param_name = name_node.value(&self.doc);
            let path = MemberDataSymbolPath::new(&self.current_path, &param_name);
            if self.check_contains(&path, name_node.range()) {
                let mut sym = FunctionParameterSymbol::new(path, SymbolLocation { 
                    scripts_root: self.symtab.script_root_arc(), 
                    local_source_path: self.local_source_path.clone(), 
                    range: n.range(), 
                    label_range: name_node.range()
                });
                sym.specifiers = specifiers.clone();
                sym.type_path = type_path.clone();
                sym.ordinal = self.current_param_ordinal;

                self.symtab.insert_symbol(sym);
            }

            self.current_param_ordinal += 1;
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: PropertyTraversalContext) {
        let mut specifiers = SymbolSpecifiers::new();
        let mut found_access_modif_before = false;
        for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
            if matches!(spec, MemberVarSpecifier::AccessModifier(_)) {
                if found_access_modif_before {
                    self.diagnostics.push(LocatedDiagnostic { 
                        path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                        diagnostic: Diagnostic { 
                            range, 
                            kind: DiagnosticKind::MultipleAccessModifiers
                        }
                    });
                }
                found_access_modif_before = true;
            }

            if !specifiers.insert(spec) {
                self.diagnostics.push(LocatedDiagnostic { 
                    path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                    diagnostic: Diagnostic { 
                        range, 
                        kind: DiagnosticKind::RepeatedSpecifier
                    }
                });
            }
        }

        let type_path = self.check_type_from_type_annot(n.var_type());

        
        for name_node in n.names() {
            let var_name = name_node.value(&self.doc);
            let path = MemberDataSymbolPath::new(&self.current_path, &var_name);
            let constr_param_path = self.current_constr_path.as_ref().map(|constr_path| MemberDataSymbolPath::new(constr_path, &var_name));
            if self.check_contains(&path, name_node.range()) {
                let mut sym = MemberVarSymbol::new(path, SymbolLocation { 
                    scripts_root: self.symtab.script_root_arc(), 
                    local_source_path: self.local_source_path.clone(), 
                    range: n.range(), 
                    label_range: name_node.range()
                });
                sym.specifiers = specifiers.clone();
                sym.type_path = type_path.clone();
                sym.ordinal = self.current_var_ordinal;

                self.symtab.insert_symbol(sym);

                if let Some(constr_param_path) = constr_param_path {
                    let mut constr_param_sym = FunctionParameterSymbol::new(constr_param_path, SymbolLocation { 
                        scripts_root: self.symtab.script_root_arc(), 
                        local_source_path: self.local_source_path.clone(), 
                        range: n.range(), 
                        label_range: name_node.range()
                    });
                    constr_param_sym.type_path = type_path.clone();
                    constr_param_sym.ordinal = self.current_var_ordinal;

                    self.symtab.insert_symbol(constr_param_sym);
                }
            }

            self.current_var_ordinal += 1;
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: PropertyTraversalContext) {
        let name_node = n.name();
        let autobind_name = name_node.value(&self.doc);
        let path = MemberDataSymbolPath::new(&self.current_path, &autobind_name);
        if self.check_contains(&path, name_node.range()) {
            let mut sym = AutobindSymbol::new(path, SymbolLocation { 
                scripts_root: self.symtab.script_root_arc(), 
                local_source_path: self.local_source_path.clone(), 
                range: n.range(), 
                label_range: name_node.range()
            });

            let mut found_access_modif_before = false;
            for (spec, range) in n.specifiers().map(|specn| (specn.value(), specn.range())) {
                if matches!(spec, AutobindSpecifier::AccessModifier(_)) {
                    if found_access_modif_before {
                        self.diagnostics.push(LocatedDiagnostic { 
                            path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                            diagnostic: Diagnostic { 
                                range, 
                                kind: DiagnosticKind::MultipleAccessModifiers
                            }
                        });
                    }
                    found_access_modif_before = true;
                }

                if !sym.specifiers.insert(spec) {
                    self.diagnostics.push(LocatedDiagnostic { 
                        path: self.symtab.script_root().join(&self.local_source_path).unwrap(),  
                        diagnostic: Diagnostic { 
                            range, 
                            kind: DiagnosticKind::RepeatedSpecifier
                        }
                    });
                }
            }

            sym.type_path = self.check_type_from_type_annot(n.autobind_type());

            self.symtab.insert_symbol(sym);
        }
    }



    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode, _: StatementTraversalContext) -> VarDeclarationTraversalPolicy {
        let type_path = self.check_type_from_type_annot(n.var_type());
    
        for name_node in n.names() {
            let var_name = name_node.value(&self.doc);
            let path = MemberDataSymbolPath::new(&self.current_path, &var_name);
            if self.check_contains(&path, name_node.range()) {
                let mut sym = LocalVarSymbol::new(path, SymbolLocation { 
                    scripts_root: self.symtab.script_root_arc(), 
                    local_source_path: self.local_source_path.clone(), 
                    range: n.range(), 
                    label_range: name_node.range()
                });
                sym.type_path = type_path.clone();

                self.symtab.insert_symbol(sym);
            }
        }

        VarDeclarationTraversalPolicy {
            traverse_init_value: false
        }
    }

    fn visit_compound_stmt(&mut self, _: &CompoundStatementNode, _: StatementTraversalContext) -> CompoundStatementTraversalPolicy {
        CompoundStatementTraversalPolicy { traverse: true }
    }
    
    fn visit_while_stmt(&mut self, _: &WhileLoopNode, _: StatementTraversalContext) -> WhileLoopTraversalPolicy {
        WhileLoopTraversalPolicy { traverse_cond: false, traverse_body: true }
    }

    fn visit_do_while_stmt(&mut self, _: &DoWhileLoopNode, _: StatementTraversalContext) -> DoWhileLoopTraversalPolicy {
        DoWhileLoopTraversalPolicy { traverse_cond: false, traverse_body: true }
    }

    fn visit_for_stmt(&mut self, _: &ForLoopNode, _: StatementTraversalContext) -> ForLoopTraversalPolicy {
        ForLoopTraversalPolicy { traverse_init: false, traverse_cond: false, traverse_iter: false, traverse_body: true }
    }

    fn visit_if_stmt(&mut self, _: &IfConditionalNode, _: StatementTraversalContext) -> IfConditionalTraversalPolicy {
        IfConditionalTraversalPolicy { traverse_cond: false, traverse_body: true, traverse_else_body: true }
    }

    fn visit_switch_stmt(&mut self, _: &SwitchConditionalNode, _: StatementTraversalContext) -> SwitchConditionalTraversalPolicy {
        SwitchConditionalTraversalPolicy { traverse_cond: false, traverse_body: true }
    }
}