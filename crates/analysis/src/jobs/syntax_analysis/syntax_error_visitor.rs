use lsp_types::Range;
use witcherscript::{ErrorNode, Script, SyntaxNode};
use witcherscript::tokens::*;
use witcherscript::ast::*;
use witcherscript_diagnostics::*;


pub fn syntax_analysis(script: &Script, diagnostics: &mut Vec<Diagnostic>) {
    let mut visitor = SyntaxErrorVisitor {
        diagnostics
    };

    script.visit_nodes(&mut visitor);
}


struct SyntaxErrorVisitor<'a> {
    diagnostics: &'a mut Vec<Diagnostic>   
}

impl SyntaxErrorVisitor<'_> {
    fn missing_element(&mut self, range: Range, expected: String) {
        self.diagnostics.push(Diagnostic { 
            range, 
            kind: DiagnosticKind::MissingSyntax(expected)
        })
    }

    /// Returns true if the node is present, false otherwise
    #[inline]
    fn check_missing<T>(&mut self, n: &SyntaxNode<'_, T>, expected: &str) -> bool {
        if n.is_missing() {
            self.missing_element(n.range(), expected.to_string());
            false
        } else {
            true
        }
    }

    /// Returns true if the identifier is present, false otherwise
    #[inline]
    fn check_identifier(&mut self, n: &IdentifierNode) -> bool {
        self.check_missing(n, "identifier")
    }

    fn check_type_annot(&mut self, n: &TypeAnnotationNode) {
        self.check_missing(&n.type_name(), "type");
    }

    /// Returns true if the expression is present and contains no errors, false otherwise
    #[inline]
    fn check_expression(&mut self, n: &ExpressionNode) -> bool {
        self.check_missing(n, "expression") && !n.has_errors()
    }

    /// Returns true if the statement is present and contains no errors, false otherwise
    #[inline]
    fn check_function_stmt(&mut self, n: &FunctionStatementNode) -> bool {
        self.check_missing(n, "statement") && !n.has_errors()
    }

    /// Returns whether the definition contains no errors
    #[inline]
    fn check_function_def(&mut self, n: &FunctionDefinitionNode) -> bool {
        if self.check_missing(n, "block or ;") {
            if let FunctionDefinition::Some(block) = n.clone().value() {
                !block.has_errors()
            } else {
                true
            }
        } else {
            false
        }
    }
}

impl SyntaxNodeVisitor for SyntaxErrorVisitor<'_> {
    fn traversal_policy_default(&self) -> bool {
        true
    }


    fn visit_error(&mut self, n: &ErrorNode, _: &TraversalContextStack) -> ErrorTraversalPolicy {
        self.diagnostics.push(Diagnostic { 
            range: n.range(), 
            // for now just create a generic syntax error on the range to know that this thing works
            kind: DiagnosticKind::InvalidSyntax
        });

        ErrorTraversalPolicy {
            traverse: false
        }
    }

    
    fn visit_root(&mut self, n: &RootNode) -> RootTraversalPolicy {
        let any_error = n.has_errors();

        RootTraversalPolicy { 
            traverse_statements: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        let mut any_error = false;
        let mut traverse_definition = false;

        if n.has_errors() {
            self.check_identifier(&n.name());
    
            if let Some(base) = n.base() {
                self.check_identifier(&base);
            }
    
            any_error = true;
            traverse_definition = n.definition().has_errors();
        }

        ClassDeclarationTraversalPolicy { 
            traverse_definition,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        let mut any_error = false;
        let mut traverse_definition = false;

        if n.has_errors() {
            self.check_identifier(&n.name());
    
            self.check_identifier(&n.parent());
    
            if let Some(base) = n.base() {
                self.check_identifier(&base);
            }
    
            any_error = true;
            traverse_definition = n.definition().has_errors();
        }

        StateDeclarationTraversalPolicy { 
            traverse_definition,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        let mut any_error = false;
        let mut traverse_definition = false;

        if n.has_errors() {
            self.check_identifier(&n.name());
    
            any_error = true;
            traverse_definition = n.definition().has_errors();
        }

        StructDeclarationTraversalPolicy { 
            traverse_definition,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        let mut any_error = false;
        let mut traverse_definition = false;

        if n.has_errors() {
            self.check_identifier(&n.name());
    
            any_error = true;
            traverse_definition = n.definition().has_errors();
        }

        EnumDeclarationTraversalPolicy { 
            traverse_definition,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) -> EnumVariantDeclarationTraversalPolicy {
        let mut any_error = false;

        if n.has_errors() {
            self.check_identifier(&n.name());
    
            n.value().map(|v| match v {
                EnumVariantValue::Int(n) => self.check_missing(&n, "variant integer value"),
                EnumVariantValue::Hex(n) => self.check_missing(&n, "variant integer value"),
            });

            any_error = true;
        }

        EnumVariantDeclarationTraversalPolicy {
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }



    fn visit_type_annotation(&mut self, n: &TypeAnnotationNode, _: &TraversalContextStack) -> TypeAnnotationTraversalPolicy {
        let mut any_error = false;

        if n.has_errors() {
            self.check_missing(&n.type_name(), "type");

            any_error = true;
        }

        TypeAnnotationTraversalPolicy {
            traverse_type_arg: any_error,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_global_var_decl(&mut self, n: &MemberVarDeclarationNode) -> MemberVarDeclarationTraversalPolicy {
        let mut any_error = false;

        if n.has_errors() {
            n.names().for_each(|name| { 
                self.check_identifier(&name); 
            });

            any_error = true;
        }

        MemberVarDeclarationTraversalPolicy {
            traverse_annotation: any_error,
            traverse_type: any_error,
            traverse_unnamed: any_error,
            traverse_errors: any_error,
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: &TraversalContextStack) -> MemberVarDeclarationTraversalPolicy {
        let mut any_error = false;

        if n.has_errors() {
            n.names().for_each(|name| { 
                self.check_identifier(&name); 
            });

            any_error = true;
        }

        MemberVarDeclarationTraversalPolicy {
            traverse_annotation: any_error,
            traverse_type: any_error,
            traverse_unnamed: any_error,
            traverse_errors: any_error,
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        let mut any_error = false;
        let mut traverse_value = false;
        
        if n.has_errors() {
            self.check_identifier(&n.member());

            any_error = true;
            traverse_value = !self.check_expression(&n.value());
        }

        MemberDefaultValueTraversalPolicy {
            traverse_value,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: &TraversalContextStack) -> MemberHintTraversalPolicy {
        let mut any_error = false;

        if n.has_errors() {
            self.check_identifier(&n.member());
            self.check_missing(&n.value(), "hint string");

            any_error = true;
        }

        MemberHintTraversalPolicy {
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: &TraversalContextStack) -> AutobindDeclarationTraversalPolicy {
        let mut any_error = false;

        if n.has_errors() {
            self.check_identifier(&n.name());
            self.check_type_annot(&n.autobind_type());
    
            any_error = true;
        }

        AutobindDeclarationTraversalPolicy {
            traverse_type: any_error,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: &TraversalContextStack) -> FunctionParameterGroupTraversalPolicy {
        let mut any_error = false;

        if n.has_errors() {
            n.names().for_each(|name| { 
                self.check_identifier(&name); 
            });

            any_error = true;
        }

        FunctionParameterGroupTraversalPolicy {
            traverse_type: any_error,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_global_func_decl(&mut self, n: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy {
        let mut any_error = false;
        let mut traverse_params = false;
        let mut traverse_definition = false;

        if n.has_errors() {
            self.check_identifier(&n.name());
        
            any_error = true;
            traverse_params = n.params().has_errors();
            traverse_definition = !self.check_function_def(&n.definition());
        }

        FunctionDeclarationTraversalPolicy { 
            traverse_annotation: any_error,
            traverse_return_type: any_error,
            traverse_params,
            traverse_definition,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_member_func_decl(&mut self, n: &FunctionDeclarationNode, _: &TraversalContextStack) -> FunctionDeclarationTraversalPolicy {
        let mut any_error = false;
        let mut traverse_params = false;
        let mut traverse_definition = false;

        if n.has_errors() {
            self.check_identifier(&n.name());
        
            any_error = true;
            traverse_params = n.params().has_errors();
            traverse_definition = !self.check_function_def(&n.definition());
        }

        FunctionDeclarationTraversalPolicy { 
            traverse_annotation: any_error,
            traverse_return_type: any_error,
            traverse_params,
            traverse_definition,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: &TraversalContextStack) -> EventDeclarationTraversalPolicy {
        let mut any_error = false;
        let mut traverse_params = false;
        let mut traverse_definition = false;

        if n.has_errors() {
            self.check_identifier(&n.name());
        
            any_error = true;
            traverse_params = n.params().has_errors();
            traverse_definition = !self.check_function_def(&n.definition());
        }

        EventDeclarationTraversalPolicy { 
            traverse_return_type: any_error,
            traverse_params,
            traverse_definition,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode, _: &TraversalContextStack) -> MemberDefaultsBlockTraversalPolicy {
        let any_error = n.has_errors();

        MemberDefaultsBlockTraversalPolicy { 
            traverse_assignments: any_error,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode, _: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        let mut any_error = false;
        let mut traverse_value = false;

        if n.has_errors() {
            self.check_identifier(&n.member());
            
            any_error = true;
            traverse_value = !self.check_expression(&n.value());
        }

        MemberDefaultValueTraversalPolicy {
            traverse_value,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }


    
    
    fn visit_compound_stmt(&mut self, n: &CompoundStatementNode, _: &TraversalContextStack) -> CompoundStatementTraversalPolicy {
        let any_error = n.has_errors();

        CompoundStatementTraversalPolicy { 
            traverse_statements: any_error,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, _: &TraversalContextStack) -> VarDeclarationTraversalPolicy {
        let mut any_error = false;
        let mut traverse_init_value = false;

        if n.has_errors() {
            n.names().for_each(|name| { 
                self.check_identifier(&name); 
            });
            self.check_type_annot(&n.var_type());

            any_error = true;
            traverse_init_value = n.init_value().map(|init_value| !self.check_expression(&init_value)).unwrap_or(false);
        }

        VarDeclarationTraversalPolicy {
            traverse_type: any_error,
            traverse_init_value,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode, _: &TraversalContextStack) -> ExpressionStatementTraversalPolicy {
        let mut any_error = false;
        let mut traverse_expr = false;

        if n.has_errors() {
            any_error = true;
            traverse_expr = !self.check_expression(&n.expr());
        }

        ExpressionStatementTraversalPolicy {
            traverse_expr,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_return_stmt(&mut self, n: &ReturnStatementNode, _: &TraversalContextStack) -> ReturnStatementTraversalPolicy {
        let mut any_error = true;
        let mut traverse_value = false;

        if n.has_errors() {
            any_error = true;
            traverse_value = n.value().map(|value| !self.check_expression(&value)).unwrap_or(false);
        }

        ReturnStatementTraversalPolicy {
            traverse_value,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_break_stmt(&mut self, n: &BreakStatementNode, _: &TraversalContextStack) -> BreakStatementTraversalPolicy {
        let any_error = n.has_errors();

        BreakStatementTraversalPolicy {
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode, _: &TraversalContextStack) -> ContinueStatementTraversalPolicy {
        let any_error = n.has_errors();

        ContinueStatementTraversalPolicy {
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode, _: &TraversalContextStack) -> DeleteStatementTraversalPolicy {
        let mut any_error = false;
        let mut traverse_value = false;

        if n.has_errors() {
            any_error = true;
            traverse_value = !self.check_expression(&n.value());
        }

        DeleteStatementTraversalPolicy {
            traverse_value,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_for_stmt(&mut self, n: &ForLoopNode, _: &TraversalContextStack) -> ForLoopTraversalPolicy {
        let mut any_error = false;
        let mut traverse_init = false;
        let mut traverse_cond = false;
        let mut traverse_iter = false;
        let mut traverse_body = false;

        if n.has_errors() {
            any_error = true;
            traverse_init = n.init().map(|init| !self.check_expression(&init)).unwrap_or(false);
            traverse_cond = n.cond().map(|cond| !self.check_expression(&cond)).unwrap_or(false);
            traverse_iter = n.iter().map(|iter| !self.check_expression(&iter)).unwrap_or(false);
            traverse_body = !self.check_function_stmt(&n.body());
        }

        ForLoopTraversalPolicy { 
            traverse_init,
            traverse_cond,
            traverse_iter,
            traverse_body,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_while_stmt(&mut self, n: &WhileLoopNode, _: &TraversalContextStack) -> WhileLoopTraversalPolicy {
        let mut any_error = false;
        let mut traverse_cond = false;
        let mut traverse_body = false;

        if n.has_errors() {
            any_error = true;
            traverse_cond = !self.check_expression(&n.cond());
            traverse_body = !self.check_function_stmt(&n.body());
        }

        WhileLoopTraversalPolicy { 
            traverse_cond,
            traverse_body,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode, _: &TraversalContextStack) -> DoWhileLoopTraversalPolicy {
        let mut any_error = false;
        let mut traverse_cond = false;
        let mut traverse_body = false;

        if n.has_errors() {
            any_error = true;
            traverse_cond = !self.check_expression(&n.cond());
            traverse_body = !self.check_function_stmt(&n.body());
        }

        DoWhileLoopTraversalPolicy { 
            traverse_cond,
            traverse_body,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_if_stmt(&mut self, n: &IfConditionalNode, _: &TraversalContextStack) -> IfConditionalTraversalPolicy {
        let mut any_error = false;
        let mut traverse_cond = false;
        let mut traverse_body = false;
        let mut traverse_else_body = false;

        if n.has_errors() {
            any_error = true;
            traverse_cond = !self.check_expression(&n.cond());
            traverse_body = !self.check_function_stmt(&n.body());
            traverse_else_body = n.else_body().map(|n| !self.check_function_stmt(&n)).unwrap_or(false);
        }

        IfConditionalTraversalPolicy { 
            traverse_cond,
            traverse_body, 
            traverse_else_body,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode, _: &TraversalContextStack) -> SwitchConditionalTraversalPolicy {
        let mut any_error = false;
        let mut traverse_cond = false;
        let mut traverse_body = false;

        if n.has_errors() {
            any_error = true;
            traverse_cond = !self.check_expression(&n.cond());
            traverse_body = n.body().has_errors();
        }

        SwitchConditionalTraversalPolicy {
            traverse_cond, 
            traverse_body,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode, _: &TraversalContextStack) -> SwitchConditionalCaseLabelTraversalPolicy {
        let mut any_error = false;
        let mut traverse_value = false;

        if n.has_errors() {
            any_error = true;
            traverse_value = !self.check_expression(&n.value());
        }

        SwitchConditionalCaseLabelTraversalPolicy {
            traverse_value,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode, _: &TraversalContextStack) -> SwitchConditionalDefaultLabelTraversalPolicy {
        let any_error = n.has_errors();

        SwitchConditionalDefaultLabelTraversalPolicy {
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }




    fn visit_array_expr(&mut self, n: &ArrayExpressionNode, _: &TraversalContextStack) -> ArrayExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_accessor = false;
        let mut traverse_index = false;

        if n.has_errors() {
            any_error = true;
            traverse_accessor = !self.check_expression(&n.accessor());
            traverse_index = !self.check_expression(&n.index());
        }

        ArrayExpressionTraversalPolicy {
            traverse_accessor,
            traverse_index,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode, _: &TraversalContextStack) -> AssignmentOperationExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_left = false;
        let mut traverse_right = false;

        if n.has_errors() {
            any_error = true;
            traverse_left = !self.check_expression(&n.left());
            traverse_right = !self.check_expression(&n.right());
        }

        AssignmentOperationExpressionTraversalPolicy {
            traverse_left,
            traverse_right,
            traverse_errors: any_error
        }
    }

    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, _: &TraversalContextStack) -> BinaryOperationExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_left = false;
        let mut traverse_right = false;

        if n.has_errors() {
            any_error = true;
            traverse_left = !self.check_expression(&n.left());
            traverse_right = !self.check_expression(&n.right());
        }

        BinaryOperationExpressionTraversalPolicy { 
            traverse_left, 
            traverse_right,
            traverse_errors: any_error
        }
    }

    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode, _: &TraversalContextStack) -> UnaryOperationExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_right = false;

        if n.has_errors() {
            any_error = true;
            traverse_right = !self.check_expression(&n.right());
        }

        UnaryOperationExpressionTraversalPolicy {
            traverse_right,
            traverse_errors: any_error
        }
    }

    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode, _: &TraversalContextStack) -> FunctionCallExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_func = false;
        let mut traverse_args = false;

        if n.has_errors() {
            any_error = true;
            let func = n.func();
            traverse_func = !self.check_missing(&func, "function") || func.has_errors();
            traverse_args = n.args().map(|args| args.has_errors()).unwrap_or(false);
        }

        FunctionCallExpressionTraversalPolicy { 
            traverse_func, 
            traverse_args,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: &TraversalContextStack) -> NewExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_lifetime_obj = false;

        if n.has_errors() {
            self.check_identifier(&n.class());

            any_error = true;
            traverse_lifetime_obj = n.lifetime_obj().map(|lo| lo.has_errors()).unwrap_or(false);
        }

        NewExpressionTraversalPolicy { 
            traverse_lifetime_obj,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, _: &TraversalContextStack) -> MemberFieldExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_accessor = false;

        if n.has_errors() {
            self.check_identifier(&n.member());

            any_error = true;
            traverse_accessor = !self.check_expression(&n.accessor());
        }

        MemberFieldExpressionTraversalPolicy { 
            traverse_accessor,
            traverse_errors: any_error
        }
    }

    fn visit_nested_expr(&mut self, n: &NestedExpressionNode, _: &TraversalContextStack) -> NestedExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_inner = false;

        if n.has_errors() {
            any_error = true;
            traverse_inner = !self.check_expression(&n.inner());
        }

        NestedExpressionTraversalPolicy { 
            traverse_inner,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode, _: &TraversalContextStack) -> TernaryConditionalExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_cond = false;
        let mut traverse_conseq = false;
        let mut traverse_alt = false;

        if n.has_errors() {
            any_error = true;
            traverse_cond = !self.check_expression(&n.cond());
            traverse_conseq = !self.check_expression(&n.conseq());
            traverse_alt = !self.check_expression(&n.alt());
        }

        TernaryConditionalExpressionTraversalPolicy { 
            traverse_cond, 
            traverse_conseq, 
            traverse_alt,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: &TraversalContextStack) -> TypeCastExpressionTraversalPolicy {
        let mut any_error = false;
        let mut traverse_value = false;

        if n.has_errors() {
            self.check_identifier(&n.target_type());

            any_error = true;
            traverse_value = !self.check_expression(&n.value());
        }

        TypeCastExpressionTraversalPolicy { 
            traverse_value,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }

    fn visit_func_call_arg(&mut self, _: &FunctionCallArgument, _: &TraversalContextStack) -> FunctionCallArgumentTraversalPolicy {
        FunctionCallArgumentTraversalPolicy { 
            traverse_expr: true
        }
    }
    
    fn visit_array_initializer_expr(&mut self, n: &ArrayInitializerExpressionNode, _: &TraversalContextStack) -> ArrayInitializerExpressionTraversalPolicy {
        let any_error = n.has_errors();

        ArrayInitializerExpressionTraversalPolicy {
            traverse_items: any_error,
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }


    fn visit_annotation(&mut self, n: &AnnotationNode, _: &TraversalContextStack) -> AnnotationTraversalPolicy {
        let any_error = n.has_errors();

        AnnotationTraversalPolicy {
            traverse_unnamed: any_error,
            traverse_errors: any_error
        }
    }


    fn visit_unnamed(&mut self, n: &UnnamedNode, _: &TraversalContextStack) {
        if n.is_missing() {
            match n.value() {
                Unnamed::Keyword(kw) => {
                    self.missing_element(n.range(), format!("keyword {}", kw.as_ref()));
                },
                Unnamed::Punctuation(punct) => {
                    self.missing_element(n.range(), punct.to_string());
                },
            }
        }
    }
}