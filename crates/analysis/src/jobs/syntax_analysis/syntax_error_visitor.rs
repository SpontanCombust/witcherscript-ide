use lsp_types::Range;
use witcherscript::{SyntaxNode, SyntaxError};
use witcherscript::tokens::*;
use witcherscript::ast::*;
use crate::diagnostics::{Diagnostic, ErrorDiagnostic, SyntaxErrorDiagnostic};


pub fn syntax_analysis(script_root: RootNode, diagnostics: &mut Vec<Diagnostic>) {
    let mut visitor = SyntaxErrorVisitor {
        diagnostics
    };

    script_root.accept(&mut visitor);
}


struct SyntaxErrorVisitor<'a> {
    diagnostics: &'a mut Vec<Diagnostic>   
}

impl SyntaxErrorVisitor<'_> {
    fn missing_element(&mut self, range: Range, expected: String) {
        self.diagnostics.push(Diagnostic { 
            range, 
            body: ErrorDiagnostic::Syntax(SyntaxErrorDiagnostic::MissingElement(expected)).into()
        })
    }

    /// Returns true if the node is present, false otherwise
    fn check_missing<T>(&mut self, n: &SyntaxNode<'_, T>, expected: &str) -> bool {
        if n.is_missing() {
            self.missing_element(n.range(), expected.to_string());
            false
        } else {
            true
        }
    }

    /// Returns true if the identifier is present, false otherwise
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
    fn check_expression(&mut self, n: &ExpressionNode) -> bool {
        self.check_missing(n, "expression") && !n.has_errors()
    }

    /// Returns true if the statement is present and contains no errors, false otherwise
    fn check_function_stmt(&mut self, n: &FunctionStatementNode) -> bool {
        self.check_missing(n, "statement") && !n.has_errors()
    }

    /// Returns whether the definition contains no errors
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
                    self.diagnostics.push(Diagnostic { 
                        range: errn.range(), 
                        // for now just create a generic syntax error on the range to know that this thing works
                        body: ErrorDiagnostic::Syntax(SyntaxErrorDiagnostic::Other).into()
                    })       
                }
            }
        }
    }
}

impl StatementVisitor for SyntaxErrorVisitor<'_> {
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
            traverse_definition: false 
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

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode) {
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(&name); } );
            self.check_type_annot(&n.var_type());
    
            self.check_errors(n);
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode) {
        if n.has_errors() {
            self.check_identifier(&n.member());

            let value = n.value();
            if !self.check_expression(&value) {
                value.accept(self);
            }
    
            self.check_errors(n);
        }
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode) {
        if n.has_errors() {
            self.check_identifier(&n.member());
            self.check_missing(&n.value(), "hint string");
    
            self.check_errors(n);
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode) {
        if n.has_errors() {
            self.check_identifier(&n.name());
            self.check_type_annot(&n.autobind_type());
    
            self.check_errors(n);
        }
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode) {
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

            traverse_definition = !self.check_function_def(&n.definition());
        }

        GlobalFunctionDeclarationTraversalPolicy { 
            traverse_params, 
            traverse_definition 
        }
    }

    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode) -> MemberFunctionDeclarationTraversalPolicy {
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

            traverse_definition = !self.check_function_def(&n.definition());
        }
        
        return MemberFunctionDeclarationTraversalPolicy { 
            traverse_params, 
            traverse_definition 
        }
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode) -> EventDeclarationTraversalPolicy {
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

            traverse_definition = !self.check_function_def(&n.definition());
        }
        
        EventDeclarationTraversalPolicy { 
            traverse_params, 
            traverse_definition 
        }
    }

    fn visit_block_stmt(&mut self, n: &FunctionBlockNode) -> FunctionBlockTraversalPolicy {
        let mut traverse = false;
        if n.has_errors() {
            self.check_errors(n);
            traverse = true;
        }

        FunctionBlockTraversalPolicy { 
            traverse 
        }
    }

    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode) {
        if n.has_errors() {
            n.names().for_each(|name| { self.check_identifier(&name); } );
            self.check_type_annot(&n.var_type());
    
            self.check_errors(n);
        }
    }

    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode) {
        if n.has_errors() {
            let expr = n.expr();
            if !self.check_expression(&expr) {
                expr.accept(self);
            }
    
            self.check_errors(n);
        }
    }

    fn visit_return_stmt(&mut self, n: &ReturnStatementNode) {
        if n.has_errors() {
            if let Some(value) = n.value() {
                value.accept(self);
            }
    
            self.check_errors(n);
        }
    }

    fn visit_break_stmt(&mut self, n: &BreakStatementNode) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }

    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }

    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode) {
        if n.has_errors() {
            let value = n.value();
            if !self.check_expression(&value) {
                value.accept(self);
            }
    
            self.check_errors(n);
        }
    }

    fn visit_for_stmt(&mut self, n: &ForLoopNode) -> ForLoopTraversalPolicy {
        let mut traverse_body = false;
        if n.has_errors() {
            if let Some(init) = n.init() {
                init.accept(self);
            }
            if let Some(cond) = n.cond() {
                cond.accept(self)
            }
            if let Some(iter) = n.iter() {
                iter.accept(self);
            }

            self.check_errors(n);
    
            traverse_body = !self.check_function_stmt(&n.body());
        }

        ForLoopTraversalPolicy { 
            traverse_body 
        }
    }

    fn visit_while_stmt(&mut self, n: &WhileLoopNode) -> WhileLoopTraversalPolicy {
        let mut traverse_body = false;
        if n.has_errors() {
            let cond = n.cond();
            if !self.check_expression(&cond) {
                cond.accept(self);
            }
    
            self.check_errors(n);
    
            traverse_body = !self.check_function_stmt(&n.body());
        }

        WhileLoopTraversalPolicy { 
            traverse_body 
        }
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode) -> DoWhileLoopTraversalPolicy {
        let mut traverse_body = false;
        if n.has_errors() {
            let cond = n.cond();
            if !self.check_expression(&cond) {
                cond.accept(self);
            }
    
            self.check_errors(n);
    
            traverse_body = !self.check_function_stmt(&n.body());
        }

        DoWhileLoopTraversalPolicy { 
            traverse_body 
        }
    }

    fn visit_if_stmt(&mut self, n: &IfConditionalNode) -> IfConditionalTraversalPolicy {
        let mut traverse_body = false;
        let mut traverse_else_body = false;
        if n.has_errors() {
            let cond = n.cond();
            if !self.check_expression(&cond) {
                cond.accept(self);
            }
    
            self.check_errors(n);
            
            traverse_body = !self.check_function_stmt(&n.body());
            traverse_else_body = n.else_body().map(|n| !self.check_function_stmt(&n)).unwrap_or(false);
        }

        IfConditionalTraversalPolicy { 
            traverse_body, 
            traverse_else_body
        }
    }

    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode) -> SwitchConditionalTraversalPolicy {
        let mut traverse_body = false;
        if n.has_errors() {
            let cond = n.cond();
            if !self.check_expression(&cond) {
                cond.accept(self);
            }
    
            self.check_errors(n);
    
            let body = n.body();
            if body.has_errors() {
                self.check_errors(&body);
                traverse_body = true;
            }
        }

        SwitchConditionalTraversalPolicy { 
            traverse_body 
        }
    }

    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode) {
        if n.has_errors() {
            let value = n.value();
            if !self.check_expression(&value) {
                value.accept(self);
            }
    
            self.check_errors(n);
        }
    }

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode) {
        if n.has_errors() {
            self.check_errors(n);
        }
    }

    fn visit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode) -> MemberDefaultsBlockTraversalPolicy {
        let mut traverse = false;
        if n.has_errors() {
            self.check_errors(n);
            traverse = true;
        }

        MemberDefaultsBlockTraversalPolicy { 
            traverse 
        }
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode) {
        if n.has_errors() {
            self.check_identifier(&n.member());

            let value = n.value();
            if !self.check_expression(&value) {
                value.accept(self);
            }
    
            self.check_errors(n);
        }
    }
}

impl ExpressionVisitor for SyntaxErrorVisitor<'_> {
    fn visit_array_expr(&mut self, n: &ArrayExpressionNode) -> ArrayExpressionTraversalPolicy {
        let traverse_accessor = !self.check_expression(&n.accessor());
        let traverse_index = !self.check_expression(&n.index());

        self.check_errors(n);

        ArrayExpressionTraversalPolicy {
            traverse_accessor,
            traverse_index
        }
    }

    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode) -> AssignmentOperationExpressionTraversalPolicy {
        let traverse_left = !self.check_expression(&n.left());
        let traverse_right = !self.check_expression(&n.right());

        self.check_errors(n);

        AssignmentOperationExpressionTraversalPolicy {
            traverse_left,
            traverse_right
        }
    }

    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode) -> BinaryOperationExpressionTraversalPolicy {
        let traverse_left = !self.check_expression(&n.left());
        let traverse_right = !self.check_expression(&n.right());

        self.check_errors(n);

        BinaryOperationExpressionTraversalPolicy { 
            traverse_left, 
            traverse_right 
        }
    }

    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode) -> UnaryOperationExpressionTraversalPolicy {
        let traverse_right = !self.check_expression(&n.right());

        self.check_errors(n);

        UnaryOperationExpressionTraversalPolicy { 
            traverse_right 
        }
    }

    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode) -> FunctionCallExpressionTraversalPolicy {
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

    fn visit_new_expr(&mut self, n: &NewExpressionNode) -> NewExpressionTraversalPolicy {
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

    fn visit_member_field_expr(&mut self, n: &MemberFieldExpressionNode) -> MemberFieldExpressionTraversalPolicy {
        let traverse_accessor = !self.check_expression(&n.accessor());
        self.check_identifier(&n.member());

        self.check_errors(n);

        MemberFieldExpressionTraversalPolicy { 
            traverse_accessor 
        }
    }

    fn visit_nested_expr(&mut self, n: &NestedExpressionNode) -> NestedExpressionTraversalPolicy {
        let traverse_inner = !self.check_expression(&n.inner());

        self.check_errors(n);

        NestedExpressionTraversalPolicy { 
            traverse_inner
        }
    }

    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode) -> TernaryConditionalExpressionTraversalPolicy {
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

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode) -> TypeCastExpressionTraversalPolicy {
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