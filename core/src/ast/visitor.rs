use crate::{SyntaxNode, tokens::{Literal, Identifier}};
use super::*;

pub trait ExpressionVisitor {
    fn visit_nested_expr(&mut self, _node: &SyntaxNode<'_, NestedExpression>) {}
    fn visit_literal_expr(&mut self, _node: &SyntaxNode<'_, Literal<'_>>) {}
    fn visit_this_expr(&mut self, _node: &SyntaxNode<'_, ThisExpression>) {}
    fn visit_super_expr(&mut self, _node: &SyntaxNode<'_, SuperExpression>) {}
    fn visit_parent_expr(&mut self, _node: &SyntaxNode<'_, ParentExpression>) {}
    fn visit_virtual_parent_expr(&mut self, _node: &SyntaxNode<'_, VirtualParentExpression>) {}
    fn visit_identifier_expr(&mut self, _node: &SyntaxNode<'_, Identifier>) {}
    fn visit_func_call_expr(&mut self, _node: &SyntaxNode<'_, FunctionCallExpression>) {}
    fn visit_func_call_arg(&mut self, _node: &Option<SyntaxNode<'_, Expression<'_>>>) {}
    fn visit_array_expr(&mut self, _node: &SyntaxNode<'_, ArrayExpression>) {}
    fn visit_member_field_expr(&mut self, _node: &SyntaxNode<'_, MemberFieldExpression>) {}
    fn visit_method_call_expr(&mut self, _node: &SyntaxNode<'_, MethodCallExpression>) {}
    fn visit_instantiation_expr(&mut self, _node: &SyntaxNode<'_, InstantiationExpression>) {}
    fn visit_type_cast_expr(&mut self, _node: &SyntaxNode<'_, TypeCastExpression>) {}
    fn visit_unary_op_expr(&mut self, _node: &SyntaxNode<'_, UnaryOperationExpression>) {}
    fn visit_binary_op_expr(&mut self, _node: &SyntaxNode<'_, BinaryOperationExpression>) {}
    fn visit_assign_op_expr(&mut self, _node: &SyntaxNode<'_, AssignmentOperationExpression>) {}
    fn visit_ternary_cond_expr(&mut self, _node: &SyntaxNode<'_, TernaryConditionalExpression>) {}
}

/// Do a left-to-right tree traversal using right-recursion.
/// Should first traverse to children and then call the visitor.
/// Used for homogeneously nesting nodes, i.e. expressions.
pub trait ExpressionTraversal {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V);
}


pub trait StatementVisitor {
    fn visit_class_decl(&mut self, _node: &SyntaxNode<'_, ClassDeclaration>) {}
    fn visit_state_decl(&mut self, _node: &SyntaxNode<'_, StateDeclaration>) {}
    fn visit_struct_decl(&mut self, _node: &SyntaxNode<'_, StructDeclaration>) {}
    fn visit_enum_decl(&mut self, _node: &SyntaxNode<'_, EnumDeclaration>) {}

    fn visit_enum_decl_value(&mut self, _node: &SyntaxNode<'_, EnumDeclarationValue>) {}

    fn visit_member_var_decl(&mut self, _node: &SyntaxNode<'_, MemberVarDeclaration>) {}
    fn visit_member_default_val(&mut self, _node: &SyntaxNode<'_, MemberDefaultValue>) {}
    fn visit_member_hint(&mut self, _node: &SyntaxNode<'_, MemberHint>) {}
    fn visit_autobind_decl(&mut self, _node: &SyntaxNode<'_, AutobindDeclaration>) {}
    
    fn visit_func_param_group(&mut self, _node: &SyntaxNode<'_, FunctionParameterGroup>) {}
    fn visit_global_func_decl(&mut self, _node: &SyntaxNode<'_, GlobalFunctionDeclaration>) {}
    fn visit_member_func_decl(&mut self, _node: &SyntaxNode<'_, MemberFunctionDeclaration>) {}
    fn visit_event_decl(&mut self, _node: &SyntaxNode<'_, EventDeclaration>) {}
    
    fn visit_local_var_decl_stmt(&mut self, _node: &SyntaxNode<'_, VarDeclaration>) {}
    fn visit_expr_stmt(&mut self, _node: &SyntaxNode<'_, ExpressionStatement>) {}
    fn visit_for_stmt(&mut self, _node: &SyntaxNode<'_, ForLoop>) {}
    fn visit_while_stmt(&mut self, _node: &SyntaxNode<'_, WhileLoop>) {}
    fn visit_do_while_stmt(&mut self, _node: &SyntaxNode<'_, DoWhileLoop>) {}
    fn visit_if_stmt(&mut self, _node: &SyntaxNode<'_, IfConditional>) {}
    fn visit_switch_stmt(&mut self, _node: &SyntaxNode<'_, SwitchConditional>) {}
    fn visit_switch_stmt_case(&mut self, _node: &SyntaxNode<'_, SwitchConditionalCase>) {}
    fn visit_switch_stmt_default(&mut self, _node: &SyntaxNode<'_, SwitchConditionalDefault>) {}
    fn visit_break_stmt(&mut self, _node: &SyntaxNode<'_, BreakStatement>) {}
    fn visit_continue_stmt(&mut self, _node: &SyntaxNode<'_, ContinueStatement>) {}
    fn visit_return_stmt(&mut self, _node: &SyntaxNode<'_, ReturnStatement>) {}
    fn visit_delete_stmt(&mut self, _node: &SyntaxNode<'_, DeleteStatement>) {}
    fn visit_block_stmt(&mut self, _node: &SyntaxNode<'_, FunctionBlock>) {}
    fn visit_nop_stmt(&mut self) {}


    /// Specify whether traversal should be done on statements that may be nested in a statement
    /// Can be used to, for example, ignore what's inside of the function 
    fn should_visit_inner(&self) -> bool;
}

/// Do a left-to-right tree traversal using left-recursion.
/// Should first call the visitor on the node and then traverse to children.
/// Used for sequential nodes, i.e. statements.
pub trait StatementTraversal {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V);
}
