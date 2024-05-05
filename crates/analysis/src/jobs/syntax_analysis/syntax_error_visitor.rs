use lsp_types::Range;
use witcherscript::{Script, SyntaxError, SyntaxNode};
use witcherscript::tokens::*;
use witcherscript::ast::*;
use crate::diagnostics::{AnalysisDiagnostic, AnalysisError, SyntaxErrorDiagnostic};


pub fn syntax_analysis(script: &Script, diagnostics: &mut Vec<AnalysisDiagnostic>) {
    let mut visitor = SyntaxErrorVisitor {
        diagnostics
    };

    script.visit_nodes(&mut visitor);
}


struct SyntaxErrorVisitor<'a> {
    diagnostics: &'a mut Vec<AnalysisDiagnostic>   
}

impl SyntaxErrorVisitor<'_> {
    fn missing_element(&mut self, range: Range, expected: String) {
        self.diagnostics.push(AnalysisDiagnostic { 
            range, 
            body: AnalysisError::Syntax(SyntaxErrorDiagnostic::MissingElement(expected)).into()
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

        if let Some(type_argn) = n.type_arg() {
            self.check_type_annot(&type_argn);
        }

        self.check_errors(&n);
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
        if self.check_missing(n, "{ or ;") {
            if let FunctionDefinition::Some(block) = n.clone().value() {
                !block.has_errors()
            } else {
                true
            }
        } else {
            false
        }
    }

    fn check_errors<T>(&mut self, n: &SyntaxNode<'_, T>) {
        let errors = n.errors();
        if errors.is_empty() {
            return;
        }

        for err in n.errors() {
            match err {
                SyntaxError::Missing(missing) => {
                    // named nodes are handled seperately
                    if let Ok(missing) = UnnamedNode::try_from(missing.clone()) {
                        match missing.value() {
                            Unnamed::Keyword(kw) => {
                                self.missing_element(missing.range(), format!("keyword {}", kw.as_ref()));
                            },
                            Unnamed::Punctuation(punct) => {
                                self.missing_element(missing.range(), punct.to_string());
                            },
                        }
                    }
                },
                SyntaxError::Invalid(errn) => {
                    self.diagnostics.push(AnalysisDiagnostic { 
                        range: errn.range(), 
                        // for now just create a generic syntax error on the range to know that this thing works
                        body: AnalysisError::Syntax(SyntaxErrorDiagnostic::Other).into()
                    })       
                }
            }
        }
    }
}

impl SyntaxNodeVisitor for SyntaxErrorVisitor<'_> {
    fn traversal_policy_default(&self) -> bool {
        true
    }

    
    fn visit_root(&mut self, n: &RootNode) -> RootTraversalPolicy {
        let traverse = if n.has_errors() {
            self.check_errors(n);
            true
        } else {
            false
        };

        RootTraversalPolicy { 
            traverse 
        }
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        let mut traverse_definition = false;
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            if let Some(base) = n.base() {
                self.check_identifier(&base);
            }
    
    
            self.check_errors(n);
    
            let def = n.definition();
            if def.has_errors() {
                self.check_errors(&def);
                traverse_definition = true;
            }
        }

        ClassDeclarationTraversalPolicy { 
            traverse_definition 
        }
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        let mut traverse_definition = false;
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            self.check_identifier(&n.parent());
    
            if let Some(base) = n.base() {
                self.check_identifier(&base);
            }
    
            self.check_errors(n);
    
            let def = n.definition();
            if def.has_errors() {
                self.check_errors(&def);
                traverse_definition = true;
            }
        }

        StateDeclarationTraversalPolicy { 
            traverse_definition 
        }
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        let mut traverse_definition = false;
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            self.check_errors(n);
            
            let def = n.definition();
            if def.has_errors() {
                self.check_errors(&def);
                traverse_definition = true;
            }
        }

        StructDeclarationTraversalPolicy { 
            traverse_definition
        }
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        let mut traverse_definition = false;
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            self.check_errors(n);
            
            let def = n.definition();
            if def.has_errors() {
                self.check_errors(&def);
                traverse_definition = true;
            }
        }

        EnumDeclarationTraversalPolicy { 
            traverse_definition
        }
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            n.value().map(|v| match v {
                EnumVariantValue::Int(n) => self.check_missing(&n, "variant integer value"),
                EnumVariantValue::Hex(n) => self.check_missing(&n, "variant integer value"),
            });
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: PropertyTraversalContext) {
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(&name); } );
            self.check_type_annot(&n.var_type());
    
            self.check_errors(n);
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: PropertyTraversalContext) -> MemberDefaultValueTraversalPolicy {
        let mut traverse_value = false;
        if n.has_errors() {
            self.check_identifier(&n.member());

            let value = n.value();
            traverse_value = !self.check_expression(&value);
    
            self.check_errors(n);
        }

        MemberDefaultValueTraversalPolicy {
            traverse_value
        }
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: PropertyTraversalContext) {
        if n.has_errors() {
            self.check_identifier(&n.member());
            self.check_missing(&n.value(), "hint string");
    
            self.check_errors(n);
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: PropertyTraversalContext) {
        if n.has_errors() {
            self.check_identifier(&n.name());
            self.check_type_annot(&n.autobind_type());
    
            self.check_errors(n);
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: FunctionTraversalContext) {
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(&name); } );
            self.check_type_annot(&n.param_type());
    
            self.check_errors(n);
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> GlobalFunctionDeclarationTraversalPolicy {
        let mut traverse_params = false;
        let mut traverse_definition = false;
        if n.has_errors() {
            self.check_identifier(&n.name());
            n.return_type().map(|n| self.check_type_annot(&n));
    
            self.check_errors(n);
        
            let params = n.params();
            if params.has_errors() {
                self.check_errors(&params);
                traverse_params = true;
            }

            let def = n.definition();
            if !self.check_function_def(&def) {
                self.check_errors(&def);
                traverse_definition = true;
            }
        }

        GlobalFunctionDeclarationTraversalPolicy { 
            traverse_params,
            traverse_definition
        }
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode, _: PropertyTraversalContext) -> MemberFunctionDeclarationTraversalPolicy {
        let mut traverse_params = false;
        let mut traverse_definition = false;
        if n.has_errors() {
            self.check_identifier(&n.name());
    
            if let Some(ret) = n.return_type() {
                self.check_type_annot(&ret);
            }
    
            self.check_errors(n);
    
            let params = n.params();
            if params.has_errors() {
                self.check_errors(&params);
                traverse_params = true;
            }

            let def = n.definition();
            if !self.check_function_def(&def) {
                self.check_errors(&def);
                traverse_definition = true;
            }
        }
        
        MemberFunctionDeclarationTraversalPolicy { 
            traverse_params,
            traverse_definition
        }
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: PropertyTraversalContext) -> EventDeclarationTraversalPolicy {
        let mut traverse_params = false;
        let mut traverse_definition = false;
        if n.has_errors() {
            self.check_identifier(&n.name());

            if let Some(ret) = n.return_type() {
                self.check_type_annot(&ret);
            }
    
            self.check_errors(n);

            let params = n.params();
            if params.has_errors() {
                self.check_errors(&params);
                traverse_params = true;
            }

            let def = n.definition();
            if !self.check_function_def(&def) {
                self.check_errors(&def);
                traverse_definition = true;
            }
        }
        
        EventDeclarationTraversalPolicy { 
            traverse_params,
            traverse_definition
        }
    }

    fn visit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode, _: PropertyTraversalContext) -> MemberDefaultsBlockTraversalPolicy {
        let traverse = if n.has_errors() {
            self.check_errors(n);
            true
        } else {
            false
        };

        MemberDefaultsBlockTraversalPolicy { 
            traverse 
        }
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode) -> MemberDefaultValueTraversalPolicy {
        let mut traverse_value = false;
        if n.has_errors() {
            self.check_identifier(&n.member());
            traverse_value = !self.check_expression(&n.value());
    
            self.check_errors(n);
        }

        MemberDefaultValueTraversalPolicy {
            traverse_value
        }
    }


    
    
    fn visit_compound_stmt(&mut self, n: &CompoundStatementNode, _: StatementTraversalContext) -> CompoundStatementTraversalPolicy {
         let traverse = if n.has_errors() {
            self.check_errors(n);
            true
        } else {
            false
        };

        CompoundStatementTraversalPolicy { 
            traverse 
        }
    }

    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode, _: StatementTraversalContext) -> VarDeclarationTraversalPolicy {
        let mut traverse_init_value = false;
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(&name); } );
            self.check_type_annot(&n.var_type());
            traverse_init_value = n.init_value().map(|init_value| !self.check_expression(&init_value)).unwrap_or(false);
    
            self.check_errors(n);
        }

        VarDeclarationTraversalPolicy {
            traverse_init_value
        }
    }

    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode, _: StatementTraversalContext) -> ExpressionStatementTraversalPolicy {
        let mut traverse_expr = false;
        if n.has_errors() {
            traverse_expr = !self.check_expression(&n.expr());
    
            self.check_errors(n);
        }

        ExpressionStatementTraversalPolicy {
            traverse_expr
        }
    }

    fn visit_return_stmt(&mut self, n: &ReturnStatementNode, _: StatementTraversalContext) -> ReturnStatementTraversalPolicy {
        let mut traverse_value = false;
        if n.has_errors() {
            traverse_value = n.value().map(|value| !self.check_expression(&value)).unwrap_or(false);
    
            self.check_errors(n);
        }

        ReturnStatementTraversalPolicy {
            traverse_value
        }
    }

    fn visit_break_stmt(&mut self, n: &BreakStatementNode, _: StatementTraversalContext) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }

    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode, _: StatementTraversalContext) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }

    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode, _: StatementTraversalContext) -> DeleteStatementTraversalPolicy {
        let mut traverse_value = false;
        if n.has_errors() {
            traverse_value = !self.check_expression(&n.value());
    
            self.check_errors(n);
        }

        DeleteStatementTraversalPolicy {
            traverse_value
        }
    }

    fn visit_for_stmt(&mut self, n: &ForLoopNode, _: StatementTraversalContext) -> ForLoopTraversalPolicy {
        let mut traverse_init = false;
        let mut traverse_cond = false;
        let mut traverse_iter = false;
        let mut traverse_body = false;
        if n.has_errors() {
            traverse_init = n.init().map(|init| !self.check_expression(&init)).unwrap_or(false);
            traverse_cond = n.cond().map(|cond| !self.check_expression(&cond)).unwrap_or(false);
            traverse_iter = n.iter().map(|iter| !self.check_expression(&iter)).unwrap_or(false);
            traverse_body = !self.check_function_stmt(&n.body());

            self.check_errors(n);
        }

        ForLoopTraversalPolicy { 
            traverse_init,
            traverse_cond,
            traverse_iter,
            traverse_body 
        }
    }

    fn visit_while_stmt(&mut self, n: &WhileLoopNode, _: StatementTraversalContext) -> WhileLoopTraversalPolicy {
        let mut traverse_cond = false;
        let mut traverse_body = false;
        if n.has_errors() {
            traverse_cond = !self.check_expression(&n.cond());
            traverse_body = !self.check_function_stmt(&n.body());

            self.check_errors(n);
        }

        WhileLoopTraversalPolicy { 
            traverse_cond,
            traverse_body 
        }
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode, _: StatementTraversalContext) -> DoWhileLoopTraversalPolicy {
        let mut traverse_cond = false;
        let mut traverse_body = false;
        if n.has_errors() {
            traverse_cond = !self.check_expression(&n.cond());
            traverse_body = !self.check_function_stmt(&n.body());

            self.check_errors(n);
        }

        DoWhileLoopTraversalPolicy { 
            traverse_cond,
            traverse_body 
        }
    }

    fn visit_if_stmt(&mut self, n: &IfConditionalNode, _: StatementTraversalContext) -> IfConditionalTraversalPolicy {
        let mut traverse_cond = false;
        let mut traverse_body = false;
        let mut traverse_else_body = false;
        if n.has_errors() {
            traverse_cond = !self.check_expression(&n.cond());
            traverse_body = !self.check_function_stmt(&n.body());
            traverse_else_body = n.else_body().map(|n| !self.check_function_stmt(&n)).unwrap_or(false);

            self.check_errors(n);
        }

        IfConditionalTraversalPolicy { 
            traverse_cond,
            traverse_body, 
            traverse_else_body
        }
    }

    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode, _: StatementTraversalContext) -> SwitchConditionalTraversalPolicy {
        let mut traverse_cond = false;
        let mut traverse_body = false;
        if n.has_errors() {
            traverse_cond = !self.check_expression(&n.cond());

            let body = n.body();
            if body.has_errors() {
                self.check_errors(&body);
                traverse_body = true;
            }
    
            self.check_errors(n);
        }

        SwitchConditionalTraversalPolicy {
            traverse_cond, 
            traverse_body 
        }
    }

    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode) -> SwitchConditionalCaseLabelTraversalPolicy {
        let mut traverse_value = false;
        if n.has_errors() {
            traverse_value = !self.check_expression(&n.value());
    
            self.check_errors(n);
        }

        SwitchConditionalCaseLabelTraversalPolicy {
            traverse_value
        }
    }

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }




    fn visit_array_expr(&mut self, n: &ArrayExpressionNode, _: ExpressionTraversalContext) -> ArrayExpressionTraversalPolicy {
        let traverse_accessor = !self.check_expression(&n.accessor());
        let traverse_index = !self.check_expression(&n.index());

        self.check_errors(n);

        ArrayExpressionTraversalPolicy {
            traverse_accessor,
            traverse_index
        }
    }

    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode, _: ExpressionTraversalContext) -> AssignmentOperationExpressionTraversalPolicy {
        let traverse_left = !self.check_expression(&n.left());
        let traverse_right = !self.check_expression(&n.right());

        self.check_errors(n);

        AssignmentOperationExpressionTraversalPolicy {
            traverse_left,
            traverse_right
        }
    }

    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, _: ExpressionTraversalContext) -> BinaryOperationExpressionTraversalPolicy {
        let traverse_left = !self.check_expression(&n.left());
        let traverse_right = !self.check_expression(&n.right());

        self.check_errors(n);

        BinaryOperationExpressionTraversalPolicy { 
            traverse_left, 
            traverse_right 
        }
    }

    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode, _: ExpressionTraversalContext) -> UnaryOperationExpressionTraversalPolicy {
        let traverse_right = !self.check_expression(&n.right());

        self.check_errors(n);

        UnaryOperationExpressionTraversalPolicy { 
            traverse_right 
        }
    }

    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode, _: ExpressionTraversalContext) -> FunctionCallExpressionTraversalPolicy {
        let func = n.func();
        let traverse_func = !self.check_missing(&func, "function") || func.has_errors();

        let mut traverse_args = false;
        if let Some(args) = n.args() {
            if args.has_errors() {
                self.check_errors(&args);
                traverse_args = true;
            }
        }

        self.check_errors(n);

        FunctionCallExpressionTraversalPolicy { 
            traverse_func, 
            traverse_args
        }
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: ExpressionTraversalContext) -> NewExpressionTraversalPolicy {
        self.check_identifier(&n.class());

        let mut traverse_lifetime_obj = false;
        if let Some(lifetime_obj) = n.lifetime_obj() {
            traverse_lifetime_obj = lifetime_obj.has_errors();
        }

        self.check_errors(n);

        NewExpressionTraversalPolicy { 
            traverse_lifetime_obj 
        }
    }

    fn visit_member_field_expr(&mut self, n: &MemberFieldExpressionNode, _: ExpressionTraversalContext) -> MemberFieldExpressionTraversalPolicy {
        let traverse_accessor = !self.check_expression(&n.accessor());
        self.check_identifier(&n.member());

        self.check_errors(n);

        MemberFieldExpressionTraversalPolicy { 
            traverse_accessor 
        }
    }

    fn visit_nested_expr(&mut self, n: &NestedExpressionNode, _: ExpressionTraversalContext) -> NestedExpressionTraversalPolicy {
        let traverse_inner = !self.check_expression(&n.inner());

        self.check_errors(n);

        NestedExpressionTraversalPolicy { 
            traverse_inner
        }
    }

    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode, _: ExpressionTraversalContext) -> TernaryConditionalExpressionTraversalPolicy {
        let traverse_cond = !self.check_expression(&n.cond());
        let traverse_conseq = !self.check_expression(&n.conseq());
        let traverse_alt = !self.check_expression(&n.alt());

        self.check_errors(n);

        TernaryConditionalExpressionTraversalPolicy { 
            traverse_cond, 
            traverse_conseq, 
            traverse_alt 
        }
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: ExpressionTraversalContext) -> TypeCastExpressionTraversalPolicy {
        self.check_identifier(&n.target_type());
        let traverse_value = !self.check_expression(&n.value());

        self.check_errors(n);

        TypeCastExpressionTraversalPolicy { 
            traverse_value
        }
    }

    fn visit_func_call_arg(&mut self, _: &FunctionCallArgument) -> FunctionCallArgumentTraversalPolicy {
        FunctionCallArgumentTraversalPolicy { 
            traverse_expr: true 
        }
    }
    
    // No point in checking single token expressions
}