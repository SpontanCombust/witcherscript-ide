use crate::tokens::*;
use crate::ast::*;
use super::policies::*;
use super::contexts::*;


/// Handle visitations to syntax nodes.
/// Visitor functions for nodes that contain visitable children nodes inside them return traversal policy objects that dictate if those children are traversed into.
/// By default all policy fields have `true` value.
/// These nodes also have a corresponding `exit_` function for them, which is run after the node itself and (possibly) its children are visited.
#[allow(unused_variables)]
pub trait SyntaxNodeVisitor {
    fn traversal_policy_default(&self) -> bool {
        true
    }


    /// Called when visiting a parenthesized expression node.
    fn visit_nested_expr(&mut self, n: &NestedExpressionNode, ctx: &TraversalContextStack) -> NestedExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting the nested expression node and possibly also children nodes specified in traversal policy.
    fn exit_nested_expr(&mut self, n: &NestedExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a node representing any literal.
    fn visit_literal_expr(&mut self, n: &LiteralNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a node representing a `this` expression.
    fn visit_this_expr(&mut self, n: &ThisExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a node representing a `super` expression.
    fn visit_super_expr(&mut self, n: &SuperExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a node representing a `parent` expression.
    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a node representing a `virtual_parent` expression.
    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a node representing an identifier in code (not a keyword).
    fn visit_identifier_expr(&mut self, n: &IdentifierNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a function call node.
    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode, ctx: &TraversalContextStack) -> FunctionCallExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting function call expression node and possibly also children nodes specified in traversal policy.
    fn exit_func_call_expr(&mut self, n: &FunctionCallExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a function call argument.
    /// Node may be None due to it referring to an optional function parameter.
    fn visit_func_call_arg(&mut self, n: &FunctionCallArgument, ctx: &TraversalContextStack) -> FunctionCallArgumentTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting function call argument and possibly also children nodes specified in traversal policy.
    fn exit_func_call_arg(&mut self, n: &FunctionCallArgument, ctx: &TraversalContextStack) {}

    /// Called when visiting an indexing expression.
    fn visit_array_expr(&mut self, n: &ArrayExpressionNode, ctx: &TraversalContextStack) -> ArrayExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting an indexing expression and possibly also children nodes specified in traversal policy.
    fn exit_array_expr(&mut self, n: &ArrayExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting an expression of accessing a field in an object.
    fn visit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, ctx: &TraversalContextStack) -> MemberFieldExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting an expression of accessing a field in an object and possibly also children nodes specified in traversal policy.
    fn exit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting an instantiation expression.
    fn visit_new_expr(&mut self, n: &NewExpressionNode, ctx: &TraversalContextStack) -> NewExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting an instantiation expression and possibly also children nodes specified in traversal policy.
    fn exit_new_expr(&mut self, n: &NewExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a type-casting expression.
    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, ctx: &TraversalContextStack) -> TypeCastExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a type-casting expression and possibly also child node specified in traversal policy.
    fn exit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting an unary operation expression.
    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode, ctx: &TraversalContextStack) -> UnaryOperationExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting an unary operation expression and possibly also children nodes specified in traversal policy.
    fn exit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode, ctx: &TraversalContextStack) {}
    
    /// Called when visiting a binary operation expression.
    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, ctx: &TraversalContextStack) -> BinaryOperationExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a binary operation expression and possibly also children nodes specified in traversal policy.
    fn exit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting an assignment operation expression.
    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode, ctx: &TraversalContextStack) -> AssignmentOperationExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting an assignment operation expression and possibly also children nodes specified in traversal policy.
    fn exit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a ternary conditional expression (expr1 ? expr2 : expr3).
    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode, ctx: &TraversalContextStack) -> TernaryConditionalExpressionTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a ternary conditional expression and possibly also children nodes specified in traversal policy.
    fn exit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode, ctx: &TraversalContextStack) {}




    /// Called when visiting the highest node in the hierarchy.
    fn visit_root(&mut self, n: &RootNode) -> RootTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }

    /// Called when visiting a class declaration.
    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a class declaration and possibly also children nodes specified in traversal policy.
    fn exit_class_decl(&mut self, n: &ClassDeclarationNode) {}

    /// Called when visiting a state declaration.
    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a state declaration and possibly also children nodes specified in traversal policy.
    fn exit_state_decl(&mut self, n: &StateDeclarationNode) {}

    /// Called when visiting a struct declaration.
    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a struct declaration and possibly also children nodes specified in traversal policy.
    fn exit_struct_decl(&mut self, n: &StructDeclarationNode) {}

    /// Called when visiting an enum declaration.
    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting an enum declaration and possibly also children nodes specified in traversal policy.
    fn exit_enum_decl(&mut self, n: &EnumDeclarationNode) {}
    /// Called when visiting enum variant's declaration.
    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {}

    /// Called when visiting a variable declaration in the global scope.
    /// THIS IS NOT LEGAL SYNTAX BY ITSELF.
    /// It it allowed here purely to be able to parse @addField variables.
    fn visit_global_var_decl(&mut self, n: &MemberVarDeclarationNode) {}

    /// Called when visiting member variable (i.e. field) declaration.
    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a statement assigning a default value to a field.
    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, ctx: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a statement assigning a default value to a field and possibly also children nodes specified in traversal policy.
    fn exit_member_default_val(&mut self, n: &MemberDefaultValueNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `defaults` block.
    fn visit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode, ctx: &TraversalContextStack) -> MemberDefaultsBlockTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visitng a `defaults` block and possibly also children nodes specified in traversal policy.
    fn exit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a default value assignment inside a `defaults` block.
    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode, ctx: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a default value assignment inside a `defaults` block and possibly also children nodes specified in traversal policy.
    fn exit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a statement noting some information about a perticular type field.
    fn visit_member_hint(&mut self, n: &MemberHintNode, ctx: &TraversalContextStack) {}
    
    /// Called when visiting an autobind variable declaration.
    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, ctx: &TraversalContextStack) {}
    
    /// Called when visiting a group of function parameters. This may mean a single parameter or multiple delimited names with common specifiers and a type.
    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a global function declaration.
    fn visit_global_func_decl(&mut self, n: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting global function declaration and possibly also children nodes specified in traversal policy.
    fn exit_global_func_decl(&mut self, n: &FunctionDeclarationNode) {}

    /// Called when visiting a member function declaration (i.e. a method).
    fn visit_member_func_decl(&mut self, n: &FunctionDeclarationNode, ctx: &TraversalContextStack) -> FunctionDeclarationTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting member function declaration and possibly also children nodes specified in traversal policy.
    fn exit_member_func_decl(&mut self, n: &FunctionDeclarationNode, ctx: &TraversalContextStack) {}

    /// Called when visiting an event function declaration.
    fn visit_event_decl(&mut self, n: &EventDeclarationNode, ctx: &TraversalContextStack) -> EventDeclarationTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting member function declaration and possibly also children nodes specified in traversal policy.
    fn exit_event_decl(&mut self, n: &EventDeclarationNode, ctx: &TraversalContextStack) {}




    /// Called when visiting a local variable declaration inside a function.
    fn visit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, ctx: &TraversalContextStack) -> VarDeclarationTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a local variable declaration and possibly also children nodes specified in traversal policy.
    fn exit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, ctx: &TraversalContextStack) {}

    /// Called when visiting an expression statement inside a function.
    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode, ctx: &TraversalContextStack) -> ExpressionStatementTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting an expression statement and possibly also children nodes specified in traversal policy.
    fn exit_expr_stmt(&mut self, n: &ExpressionStatementNode, ctx: &TraversalContextStack) {}
    
    /// Called when visiting a `for` loop.
    fn visit_for_stmt(&mut self, n: &ForLoopNode, ctx: &TraversalContextStack) -> ForLoopTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a `for` loop and possibly also children nodes specified in traversal policy.
    fn exit_for_stmt(&mut self, n: &ForLoopNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `while` loop.
    fn visit_while_stmt(&mut self, n: &WhileLoopNode, ctx: &TraversalContextStack) -> WhileLoopTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a `while` loop and possibly also children nodes specified in traversal policy.
    fn exit_while_stmt(&mut self, n: &WhileLoopNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `do-while` loop.
    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode, ctx: &TraversalContextStack) -> DoWhileLoopTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a `do-while` loop and possibly also children nodes specified in traversal policy.
    fn exit_do_while_stmt(&mut self, n: &DoWhileLoopNode, ctx: &TraversalContextStack) {}

    /// Called when visiting an `if` condition.
    fn visit_if_stmt(&mut self, n: &IfConditionalNode, ctx: &TraversalContextStack) -> IfConditionalTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting an `if` condition and possibly also children nodes specified in traversal policy.
    fn exit_if_stmt(&mut self, n: &IfConditionalNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `switch` statement.
    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode, ctx: &TraversalContextStack) -> SwitchConditionalTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a `switch` statement and possibly also children nodes specified in traversal policy.
    fn exit_switch_stmt(&mut self, n: &SwitchConditionalNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `case` label inside a `switch` statement.
    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode, ctx: &TraversalContextStack) -> SwitchConditionalCaseLabelTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a `case` label inside a `switch` statement and possibly also children nodes specified in traversal policy.
    fn exit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `default` label inside a `switch` statement.
    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `break` statement.
    fn visit_break_stmt(&mut self, n: &BreakStatementNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `continue` statement.
    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `return` statement.
    fn visit_return_stmt(&mut self, n: &ReturnStatementNode, ctx: &TraversalContextStack) -> ReturnStatementTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a `return` statement and possibly also children nodes specified in traversal policy.
    fn exit_return_stmt(&mut self, n: &ReturnStatementNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a `delete` statement.
    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode, ctx: &TraversalContextStack) -> DeleteStatementTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a `delete` statement and possibly also children nodes specified in traversal policy.
    fn exit_delete_stmt(&mut self, n: &DeleteStatementNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a scope block statement in a function.
    fn visit_compound_stmt(&mut self, n: &CompoundStatementNode, ctx: &TraversalContextStack) -> CompoundStatementTraversalPolicy { TraversalPolicy::default_to(self.traversal_policy_default()) }
    /// Called after visiting a scope block statement in a function and possibly also children nodes specified in traversal policy.
    fn exit_compound_stmt(&mut self, n: &CompoundStatementNode, ctx: &TraversalContextStack) {}

    /// Called when visiting a NOP statement. 
    /// It most notably means:
    /// 1. A trailing "orphan" semicolon somewhere in code
    /// 2. Indicating absence of action, e.g. `while(!AreWeThereYet());`
    fn visit_nop_stmt(&mut self, n: &NopNode, ctx: &TraversalContextStack) {}
}
