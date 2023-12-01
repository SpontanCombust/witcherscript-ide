use crate::{SyntaxNode, tokens::{Literal, Identifier}};
use super::*;

pub trait ExpressionVisitor {
    fn visit_nested_expr(&mut self, _: &SyntaxNode<'_, NestedExpression>) {}
    fn visit_literal_expr(&mut self, _: &SyntaxNode<'_, Literal<'_>>) {}
    fn visit_this_expr(&mut self, _: &SyntaxNode<'_, ThisExpression>) {}
    fn visit_super_expr(&mut self, _: &SyntaxNode<'_, SuperExpression>) {}
    fn visit_parent_expr(&mut self, _: &SyntaxNode<'_, ParentExpression>) {}
    fn visit_virtual_parent_expr(&mut self, _: &SyntaxNode<'_, VirtualParentExpression>) {}
    fn visit_identifier_expr(&mut self, _: &SyntaxNode<'_, Identifier>) {}
    fn visit_func_call_expr(&mut self, _: &SyntaxNode<'_, FunctionCallExpression>) {}
    fn visit_func_call_arg(&mut self, _: &Option<SyntaxNode<'_, Expression<'_>>>) {}
    fn visit_array_expr(&mut self, _: &SyntaxNode<'_, ArrayExpression>) {}
    fn visit_member_field_expr(&mut self, _: &SyntaxNode<'_, MemberFieldExpression>) {}
    fn visit_method_call_expr(&mut self, _: &SyntaxNode<'_, MethodCallExpression>) {}
    fn visit_instantiation_expr(&mut self, _: &SyntaxNode<'_, InstantiationExpression>) {}
    fn visit_type_cast_expr(&mut self, _: &SyntaxNode<'_, TypeCastExpression>) {}
    fn visit_unary_op_expr(&mut self, _: &SyntaxNode<'_, UnaryOperationExpression>) {}
    fn visit_binary_op_expr(&mut self, _: &SyntaxNode<'_, BinaryOperationExpression>) {}
    fn visit_assign_op_expr(&mut self, _: &SyntaxNode<'_, AssignmentOperationExpression>) {}
    fn visit_ternary_cond_expr(&mut self, _: &SyntaxNode<'_, TernaryConditionalExpression>) {}
}

/// Do a left-to-right tree traversal using right-recursion.
/// Should first traverse to children and then call the visitor.
/// Used for homogeneously nesting nodes, i.e. expressions.
pub trait ExpressionTraversal {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V);
}


pub trait StatementVisitor {
    /// Should return whether to traverse into the body of the class. True by default.
    fn visit_class_decl(&mut self, _: &SyntaxNode<'_, ClassDeclaration>) -> bool { true }
    /// Should return whether to traverse into the body of the state. True by default.
    fn visit_state_decl(&mut self, _: &SyntaxNode<'_, StateDeclaration>) -> bool { true }
    /// Should return whether to traverse into the body of the struct. True by default.
    fn visit_struct_decl(&mut self, _: &SyntaxNode<'_, StructDeclaration>) -> bool { true }
    /// Should return whether to traverse into the body of the enum. True by default.
    fn visit_enum_decl(&mut self, _: &SyntaxNode<'_, EnumDeclaration>) -> bool { true }

    fn visit_enum_decl_value(&mut self, _: &SyntaxNode<'_, EnumDeclarationValue>) {}

    fn visit_member_var_decl(&mut self, _: &SyntaxNode<'_, MemberVarDeclaration>) {}
    fn visit_member_default_val(&mut self, _: &SyntaxNode<'_, MemberDefaultValue>) {}
    fn visit_member_hint(&mut self, _: &SyntaxNode<'_, MemberHint>) {}
    fn visit_autobind_decl(&mut self, _: &SyntaxNode<'_, AutobindDeclaration>) {}
    
    fn visit_func_param_group(&mut self, _: &SyntaxNode<'_, FunctionParameterGroup>) {}
    /// Should return whether to traverse into params and body of the function. True by default.
    fn visit_global_func_decl(&mut self, _: &SyntaxNode<'_, GlobalFunctionDeclaration>) -> bool { true }
    /// Should return whether to traverse into params and body of the function. True by default.
    fn visit_member_func_decl(&mut self, _: &SyntaxNode<'_, MemberFunctionDeclaration>) -> bool { true }
    /// Should return whether to traverse into params and body of the event. True by default.
    fn visit_event_decl(&mut self, _: &SyntaxNode<'_, EventDeclaration>) -> bool { true }
    
    fn visit_local_var_decl_stmt(&mut self, _: &SyntaxNode<'_, VarDeclaration>) {}
    fn visit_expr_stmt(&mut self, _: &SyntaxNode<'_, ExpressionStatement>) {}
    fn visit_for_stmt(&mut self, _: &SyntaxNode<'_, ForLoop>) {}
    fn visit_while_stmt(&mut self, _: &SyntaxNode<'_, WhileLoop>) {}
    fn visit_do_while_stmt(&mut self, _: &SyntaxNode<'_, DoWhileLoop>) {}
    fn visit_if_stmt(&mut self, _: &SyntaxNode<'_, IfConditional>) {}
    fn visit_switch_stmt(&mut self, _: &SyntaxNode<'_, SwitchConditional>) {}
    fn visit_switch_stmt_case(&mut self, _: &SyntaxNode<'_, SwitchConditionalCase>) {}
    fn visit_switch_stmt_default(&mut self, _: &SyntaxNode<'_, SwitchConditionalDefault>) {}
    fn visit_break_stmt(&mut self, _: &SyntaxNode<'_, BreakStatement>) {}
    fn visit_continue_stmt(&mut self, _: &SyntaxNode<'_, ContinueStatement>) {}
    fn visit_return_stmt(&mut self, _: &SyntaxNode<'_, ReturnStatement>) {}
    fn visit_delete_stmt(&mut self, _: &SyntaxNode<'_, DeleteStatement>) {}
    fn visit_block_stmt(&mut self, _: &SyntaxNode<'_, FunctionBlock>) {}
    fn visit_nop_stmt(&mut self) {}
}

/// Do a left-to-right tree traversal using left-recursion.
/// Should first call the visitor on the node and then traverse to children.
/// Used for sequential nodes, i.e. statements.
pub trait StatementTraversal {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V);
}
