use std::{cell::RefCell, rc::Rc};
use lsp_types as lsp;
use witcherscript::{ast::*, tokens::*};


/// Utility node visitor travels only through nodes that span a specified position
/// and it keeps traversing until it stumbles onto a node with no traversable children.
/// Then it sets the `done` flag in the payload it can share with other objects.
/// It is not guaranteed that the `done` flag will be eventually set.
/// This visitor can be used in a visitor chain.
#[derive(Debug, Clone)]
pub struct PositionFilter {
    pos: lsp::Position,
    currently_in_range: bool,
    payload: Rc<RefCell<PositionFilterPayload>>,

    /// Set whether statements should be checked against the position.
    /// 
    /// If false will set and exception for statements inside callables 
    /// and always allow the next link in the chain to see the node even if it doesn't span the position.
    /// 
    /// True by default.
    pub filter_statements: bool,
    currently_in_callable_range: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PositionFilterPayload {
    /// Signals that the given node likely directly contains a node, 
    /// which spans the specified position 
    pub done: bool
}

impl PositionFilter {
    pub fn new(position: lsp::Position) -> (Self, Rc<RefCell<PositionFilterPayload>>) {
        let payload = Rc::new(RefCell::new(PositionFilterPayload {
            done: false
        }));

        let self_ = Self {
            pos: position,
            currently_in_range: false,
            payload: payload.clone(),

            filter_statements: true,
            currently_in_callable_range: false
        };

        (self_, payload)
    }

    pub fn new_rc(position: lsp::Position) -> (Rc<RefCell<Self>>, Rc<RefCell<PositionFilterPayload>>) {
        let (self_, payload) = Self::new(position);
        (Rc::new(RefCell::new(self_)), payload)
    }

    pub fn reset(&mut self, position: lsp::Position) {
        self.pos = position;
        self.currently_in_range = false;
        self.payload.borrow_mut().done = false;
    }
}

impl SyntaxNodeVisitor for PositionFilter {
    fn traversal_policy_default(&self) -> bool {
        false
    }
    

    fn visit_root(&mut self, n: &RootNode) -> RootTraversalPolicy {
        self.currently_in_range = n.spans_position(self.pos);

        RootTraversalPolicy { 
            traverse: self.currently_in_range
        }
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        let mut tp = ClassDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
            } 
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        let mut tp = StateDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
            } 
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        let mut tp = StructDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
            } 
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        let mut tp = EnumDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
            } 
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_global_func_decl(&mut self, n: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy {
        let mut tp = FunctionDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.currently_in_callable_range = true;
            if n.params().spans_position(self.pos) {
                tp.traverse_params = true;
            }
            else if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
            } 
            else {
                self.payload.borrow_mut().done = true;
            }
        }

        tp
    }

    fn exit_global_func_decl(&mut self, _: &FunctionDeclarationNode) {
        self.currently_in_callable_range = false;
    }




    fn visit_member_func_decl(&mut self, n: &FunctionDeclarationNode, _: PropertyTraversalContext) -> FunctionDeclarationTraversalPolicy {
        let mut tp = FunctionDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.currently_in_callable_range = true;
            if n.params().spans_position(self.pos) {
                tp.traverse_params = true;
            }
            else if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
            } 
            else {
                self.payload.borrow_mut().done = true;
            }
        }

        tp
    }

    fn exit_member_func_decl(&mut self, _: &FunctionDeclarationNode, _: PropertyTraversalContext) {
        self.currently_in_callable_range = false;
    }
    
    fn visit_event_decl(&mut self, n: &EventDeclarationNode, _: PropertyTraversalContext) -> EventDeclarationTraversalPolicy {
        let mut tp = EventDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.currently_in_callable_range = true;
            if n.params().spans_position(self.pos) {
                tp.traverse_params = true;
            }
            else if n.definition().spans_position(self.pos) {
                tp.traverse_definition = true;
            } 
            else {
                self.payload.borrow_mut().done = true;
            }
        }

        tp
    }

    fn exit_event_decl(&mut self, _: &EventDeclarationNode, _: PropertyTraversalContext) {
        self.currently_in_callable_range = false;
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, _: FunctionTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, _: PropertyTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, _: PropertyTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, _: PropertyTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, _: PropertyTraversalContext) -> MemberDefaultValueTraversalPolicy {
        let mut tp = MemberDefaultValueTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else {
                self.payload.borrow_mut().done = true;
            }
        }

        tp
    }

    fn visit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode, _: PropertyTraversalContext) -> MemberDefaultsBlockTraversalPolicy {
        let mut tp = MemberDefaultsBlockTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            tp.traverse = true;
        }

        tp
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode) -> MemberDefaultValueTraversalPolicy {
        let mut tp = MemberDefaultValueTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else {
                self.payload.borrow_mut().done = true;
            }
        }

        tp
    }




    fn visit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, _: StatementTraversalContext) -> VarDeclarationTraversalPolicy {
        let mut tp = VarDeclarationTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.init_value().map(|init_value| init_value.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_init_value = true;
            } 
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_compound_stmt(&mut self, n: &CompoundStatementNode, _: StatementTraversalContext) -> CompoundStatementTraversalPolicy {
        let mut tp = CompoundStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            tp.traverse = true;
        }
      
        tp
    }

    fn visit_for_stmt(&mut self, n: &ForLoopNode, _: StatementTraversalContext) -> ForLoopTraversalPolicy {
        let mut tp = ForLoopTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.init().map(|init| init.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_init = true;
            }
            else if n.cond().map(|cond| cond.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_cond = true;
            }
            else if n.iter().map(|iter| iter.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_iter = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_while_stmt(&mut self, n: &WhileLoopNode, _: StatementTraversalContext) -> WhileLoopTraversalPolicy {
        let mut tp = WhileLoopTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode, _: StatementTraversalContext) -> DoWhileLoopTraversalPolicy {
        let mut tp = DoWhileLoopTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_if_stmt(&mut self, n: &IfConditionalNode, _: StatementTraversalContext) -> IfConditionalTraversalPolicy {
        let mut tp = IfConditionalTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
            }
            else if n.else_body().map(|else_body| else_body.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_else_body = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode, _: StatementTraversalContext) -> SwitchConditionalTraversalPolicy {
        let mut tp = SwitchConditionalTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.body().spans_position(self.pos) {
                tp.traverse_body = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode) -> SwitchConditionalCaseLabelTraversalPolicy {
        let mut tp = SwitchConditionalCaseLabelTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_break_stmt(&mut self, n: &BreakStatementNode, _: StatementTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode, _: StatementTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode, _: StatementTraversalContext) -> DeleteStatementTraversalPolicy {
        let mut tp = DeleteStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_return_stmt(&mut self, n: &ReturnStatementNode, _: StatementTraversalContext) -> ReturnStatementTraversalPolicy {
        let mut tp = ReturnStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().map(|value| value.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_value = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode, _: StatementTraversalContext) -> ExpressionStatementTraversalPolicy {
        let mut tp = ExpressionStatementTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.expr().spans_position(self.pos) {
                tp.traverse_expr = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_nop_stmt(&mut self, n: &NopNode, _: StatementTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }




    fn visit_nested_expr(&mut self, n: &NestedExpressionNode, _: ExpressionTraversalContext) -> NestedExpressionTraversalPolicy {
        let mut tp = NestedExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.inner().spans_position(self.pos) {
                tp.traverse_inner = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode, _: ExpressionTraversalContext) -> AssignmentOperationExpressionTraversalPolicy {
        let mut tp = AssignmentOperationExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.left().spans_position(self.pos) {
                tp.traverse_left = true;
            }
            else if n.right().spans_position(self.pos) {
                tp.traverse_right = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, _: ExpressionTraversalContext) -> BinaryOperationExpressionTraversalPolicy {
        let mut tp = BinaryOperationExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.left().spans_position(self.pos) {
                tp.traverse_left = true;
            }
            else if n.right().spans_position(self.pos) {
                tp.traverse_right = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode, _: ExpressionTraversalContext) -> UnaryOperationExpressionTraversalPolicy {
        let mut tp = UnaryOperationExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.right().spans_position(self.pos) {
                tp.traverse_right = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, _: ExpressionTraversalContext) -> NewExpressionTraversalPolicy {
        let mut tp = NewExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.lifetime_obj().map(|lifetime_obj| lifetime_obj.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_lifetime_obj = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, _: ExpressionTraversalContext) -> TypeCastExpressionTraversalPolicy {
        let mut tp = TypeCastExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.value().spans_position(self.pos) {
                tp.traverse_value = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode, _: ExpressionTraversalContext) -> TernaryConditionalExpressionTraversalPolicy {
        let mut tp = TernaryConditionalExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.cond().spans_position(self.pos) {
                tp.traverse_cond = true;
            }
            else if n.conseq().spans_position(self.pos) {
                tp.traverse_conseq = true;
            }
            else if n.alt().spans_position(self.pos) {
                tp.traverse_alt = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, _: ExpressionTraversalContext) -> MemberFieldExpressionTraversalPolicy {
        let mut tp = MemberFieldExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.accessor().spans_position(self.pos) {
                tp.traverse_accessor = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_array_expr(&mut self, n: &ArrayExpressionNode, _: ExpressionTraversalContext) -> ArrayExpressionTraversalPolicy {
        let mut tp = ArrayExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.accessor().spans_position(self.pos) {
                tp.traverse_accessor = true;
            }
            else if n.index().spans_position(self.pos) {
                tp.traverse_index = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode, _: ExpressionTraversalContext) -> FunctionCallExpressionTraversalPolicy {
        let mut tp = FunctionCallExpressionTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            if n.func().spans_position(self.pos) {
                tp.traverse_func = true;
            }
            else if n.args().map(|args| args.spans_position(self.pos)).unwrap_or(false) {
                tp.traverse_args = true;
            }
            else { 
                self.payload.borrow_mut().done = true;
            }
        }
      
        tp
    }

    fn visit_func_call_arg(&mut self, n: &FunctionCallArgument) -> FunctionCallArgumentTraversalPolicy {
        let mut tp = FunctionCallArgumentTraversalPolicy::default_to(false);

        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            match n {
                FunctionCallArgument::Some(_) => {
                    tp.traverse_expr = true;
                }
                FunctionCallArgument::Omitted(_) => {
                    self.payload.borrow_mut().done = true;
                }
            }
        }
        
        tp
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, _: ExpressionTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_literal_expr(&mut self, n: &LiteralNode, _: ExpressionTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_this_expr(&mut self, n: &ThisExpressionNode, _: ExpressionTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_super_expr(&mut self, n: &SuperExpressionNode, _: ExpressionTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, _: ExpressionTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }

    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, _: ExpressionTraversalContext) {
        self.currently_in_range = n.spans_position(self.pos);
        if self.currently_in_range {
            self.payload.borrow_mut().done = true;
        }
    }
}


impl SyntaxNodeVisitorChainLink for PositionFilter {
    fn pass_onto_next_link(&self) -> bool { 
        self.currently_in_range || (self.currently_in_callable_range && !self.filter_statements)
    }
}