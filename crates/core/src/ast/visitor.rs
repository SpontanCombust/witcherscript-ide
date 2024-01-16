use crate::tokens::*;
use super::*;

//TODO describe how are nested expressions traversed into (e.g. in FunctionCallExpressionNode it is traversed into arguments)
pub trait ExpressionVisitor {
    fn visit_nested_expr(&mut self, _: &NestedExpressionNode) {}
    fn visit_literal_expr(&mut self, _: &LiteralNode) {}
    fn visit_this_expr(&mut self, _: &ThisExpressionNode) {}
    fn visit_super_expr(&mut self, _: &SuperExpressionNode) {}
    fn visit_parent_expr(&mut self, _: &ParentExpressionNode) {}
    fn visit_virtual_parent_expr(&mut self, _: &VirtualParentExpressionNode) {}
    fn visit_identifier_expr(&mut self, _: &IdentifierNode) {}
    fn visit_func_call_expr(&mut self, _: &FunctionCallExpressionNode) {}
    fn visit_func_call_arg(&mut self, _: &Option<ExpressionNode>) {}
    fn visit_array_expr(&mut self, _: &ArrayExpressionNode) {}
    fn visit_member_field_expr(&mut self, _: &MemberFieldExpressionNode) {}
    fn visit_method_call_expr(&mut self, _: &MethodCallExpressionNode) {}
    fn visit_instantiation_expr(&mut self, _: &InstantiationExpressionNode) {}
    fn visit_type_cast_expr(&mut self, _: &TypeCastExpressionNode) {}
    fn visit_unary_op_expr(&mut self, _: &UnaryOperationExpressionNode) {}
    fn visit_binary_op_expr(&mut self, _: &BinaryOperationExpressionNode) {}
    fn visit_assign_op_expr(&mut self, _: &AssignmentOperationExpressionNode) {}
    fn visit_ternary_cond_expr(&mut self, _: &TernaryConditionalExpressionNode) {}
}

/// Do a left-to-right tree traversal using right-recursion.
/// Should first traverse to children and then call the visitor.
/// Used for homogeneously nesting nodes, i.e. expressions.
pub trait ExpressionTraversal {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V);
}


//TODO describe how are nested statements traversed into
pub trait StatementVisitor {
    /// Should return whether to traverse into the body of the script. True by default.
    fn visit_root(&mut self, _: &RootNode) -> bool { true }

    /// Should return whether to traverse into the body of the class. True by default.
    fn visit_class_decl(&mut self, _: &ClassDeclarationNode) -> bool { true }
    fn exit_class_decl(&mut self, _: &ClassDeclarationNode) {}
    /// Should return whether to traverse into the body of the state. True by default.
    fn visit_state_decl(&mut self, _: &StateDeclarationNode) -> bool { true }
    fn exit_state_decl(&mut self, _: &StateDeclarationNode) {}
    /// Should return whether to traverse into the body of the struct. True by default.
    fn visit_struct_decl(&mut self, _: &StructDeclarationNode) -> bool { true }
    fn exit_struct_decl(&mut self, _: &StructDeclarationNode) {}
    /// Should return whether to traverse into the body of the enum. True by default.
    fn visit_enum_decl(&mut self, _: &EnumDeclarationNode) -> bool { true }
    fn exit_enum_decl(&mut self, _: &EnumDeclarationNode) {}

    fn visit_enum_member_decl(&mut self, _: &EnumMemberDeclarationNode) {}

    fn visit_member_var_decl(&mut self, _: &MemberVarDeclarationNode) {}
    fn visit_member_default_val(&mut self, _: &MemberDefaultValueNode) {}
    fn visit_member_hint(&mut self, _: &MemberHintNode) {}
    fn visit_autobind_decl(&mut self, _: &AutobindDeclarationNode) {}
    
    fn visit_func_param_group(&mut self, _: &FunctionParameterGroupNode) {}
    /// Should return whether to traverse into params and body of the function. True by default.
    fn visit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) -> bool { true }
    fn exit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) {}
    /// Should return whether to traverse into params and body of the function. True by default.
    fn visit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode) -> bool { true }
    fn exit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode) {}
    /// Should return whether to traverse into params and body of the event. True by default.
    fn visit_event_decl(&mut self, _: &EventDeclarationNode) -> bool { true }
    fn exit_event_decl(&mut self, _: &EventDeclarationNode) {}
    
    fn visit_local_var_decl_stmt(&mut self, _: &VarDeclarationNode) {}
    fn visit_expr_stmt(&mut self, _: &ExpressionStatementNode) {}
    /// Should return whether to traverse into body of the loop. True by default.
    fn visit_for_stmt(&mut self, _: &ForLoopNode) -> bool { true }
    /// Should return whether to traverse into body of the loop. True by default.
    fn visit_while_stmt(&mut self, _: &WhileLoopNode) -> bool { true }
    /// Should return whether to traverse into body of the loop. True by default.
    fn visit_do_while_stmt(&mut self, _: &DoWhileLoopNode) -> bool { true }
    /// Should return whether to traverse into body of the conditional. True by default.
    fn visit_if_stmt(&mut self, _: &IfConditionalNode) -> bool { true }
    /// Should return whether to traverse into body of the conditional. True by default.
    fn visit_switch_stmt(&mut self, _: &SwitchConditionalNode) -> bool { true }
    fn visit_switch_stmt_case(&mut self, _: &SwitchConditionalCaseNode) {}
    fn visit_switch_stmt_default(&mut self, _: &SwitchConditionalDefaultNode) {}
    fn visit_break_stmt(&mut self, _: &BreakStatementNode) {}
    fn visit_continue_stmt(&mut self, _: &ContinueStatementNode) {}
    fn visit_return_stmt(&mut self, _: &ReturnStatementNode) {}
    fn visit_delete_stmt(&mut self, _: &DeleteStatementNode) {}
    /// Should return whether to traverse into statements of the block. True by default.
    fn visit_block_stmt(&mut self, _: &FunctionBlockNode) -> bool { true }
    fn visit_nop_stmt(&mut self, _: &NopNode) {}
}

/// Do a left-to-right tree traversal using left-recursion.
/// Should first call the visitor on the node and then traverse to children.
/// Used for sequential nodes, i.e. statements.
pub trait StatementTraversal {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V);
}
