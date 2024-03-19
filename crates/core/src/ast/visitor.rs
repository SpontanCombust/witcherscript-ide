use crate::tokens::*;
use super::*;


//TODO specify `exit` functions just like in StatementVisitor - this will mean the traversall will no longer be purely right-recursed
pub trait ExpressionVisitor {
    /// Called when visiting a parenthesized expression node.
    /// The call is preceeded by a visit to the [expression][NestedExpressionNode::value] nested inside it.
    fn visit_nested_expr(&mut self, _: &NestedExpressionNode) {}
    /// Called when visiting a node representing any literal.
    fn visit_literal_expr(&mut self, _: &LiteralNode) {}
    /// Called when visiting a node representing a `this` expression.
    fn visit_this_expr(&mut self, _: &ThisExpressionNode) {}
    /// Called when visiting a node representing a `super` expression.
    fn visit_super_expr(&mut self, _: &SuperExpressionNode) {}
    /// Called when visiting a node representing a `parent` expression.
    fn visit_parent_expr(&mut self, _: &ParentExpressionNode) {}
    /// Called when visiting a node representing a `virtual_parent` expression.
    fn visit_virtual_parent_expr(&mut self, _: &VirtualParentExpressionNode) {}
    /// Called when visiting a node representing an identifier in code (not a keyword).
    fn visit_identifier_expr(&mut self, _: &IdentifierNode) {}
    /// Called when visiting a function call node.
    /// The call is preceeded by visits to this node's callee [func][FunctionCallExpressionNode::func] node and [args][FunctionCallExpressionNode::args] in that order.
    fn visit_func_call_expr(&mut self, _: &FunctionCallExpressionNode) {}
    /// Called when visiting a function call argument.
    /// The call is optionally preceeded by a visit to the expression it represents.
    /// Node may be None due to it referring to an optional parameter.
    fn visit_func_call_arg(&mut self, _: &Option<ExpressionNode>) {}
    /// Called when visiting an indexing expression.
    /// The call is preceeded by visits to this node's [accessor][ArrayExpressionNode::accessor] and [index][ArrayExpressionNode::index] expressions in that order.
    fn visit_array_expr(&mut self, _: &ArrayExpressionNode) {}
    /// Called when visiting an expression of accessing a field in an object.
    /// The call is preceeded by visits to this node's [accessor][MemberFieldExpressionNode::accessor] node. 
    /// The [member][MemberFieldExpressionNode::member] identifier is not visited automatically.
    fn visit_member_field_expr(&mut self, _: &MemberFieldExpressionNode) {}
    /// Called when visiting an instantiation expression.
    /// The call is optionally preceeded by a visit to this node's [lifetime_obj][NewExpressionNode::lifetime_obj] node. 
    /// The [class][NewExpressionNode::class] identifier is not visited automatically.
    fn visit_new_expr(&mut self, _: &NewExpressionNode) {}
    /// Called when visiting a type-casting expression.
    /// The call is preceeded by a visit to this node's [value][NewExpressionNode::value] node. 
    /// The [target_type][NewExpressionNode::target_type] identifier is not visited automatically.
    fn visit_type_cast_expr(&mut self, _: &TypeCastExpressionNode) {}
    /// Called when visiting an unary operation expression.
    /// The call is preceeded by a visit to this node's [right][UnaryOperationExpressionNode::right] node. 
    /// The [operator][UnaryOperationExpressionNode::op] node is not visited automatically.
    fn visit_unary_op_expr(&mut self, _: &UnaryOperationExpressionNode) {}
    /// Called when visiting a binary operation expression.
    /// The call is preceeded by visits to this node's [left][BinaryOperationExpressionNode::left] and [right][BinaryOperationExpressionNode::right] nodes in that order. 
    /// The `op` operator node is not visited automatically.
    fn visit_binary_op_expr(&mut self, _: &BinaryOperationExpressionNode) {}
    /// Called when visiting an assignment operation expression.
    /// The call is preceeded by visits to this node's [left][AssignmentOperationExpressionNode::left] and [right][AssignmentOperationExpressionNode::right] nodes in that order. 
    /// The `op` operator node is not visited automatically.
    fn visit_assign_op_expr(&mut self, _: &AssignmentOperationExpressionNode) {}
    /// Called when visiting a ternary conditional expression (expr1 ? expr2 : expr3).
    /// The call is preceeded by visits to this node's [cond][TernaryConditionalExpressionNode::cond], 
    /// [conseq][TernaryConditionalExpressionNode::conseq] and [alt][TernaryConditionalExpressionNode::alt] nodes in that order.
    fn visit_ternary_cond_expr(&mut self, _: &TernaryConditionalExpressionNode) {}
}

/// Do a left-to-right tree traversal using right-recursion.
/// Should first traverse to children and then call the visitor.
/// Used for homogeneously nesting nodes, i.e. expressions.
/// Due to the recursive nature of expressions it is recommended to handle them using a stack.
pub trait ExpressionTraversal {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V);
}


pub trait StatementVisitor {
    /// Called when visiting the highest node in the hierarchy.
    /// Should return whether to traverse into the body of the script afterwards. True by default.
    fn visit_root(&mut self, _: &RootNode) -> bool { true }

    /// Called when visiting a class declaration.
    /// Should return whether to traverse into the [definition][ClassDeclarationNode::definition] of the class afterwards. True by default.
    fn visit_class_decl(&mut self, _: &ClassDeclarationNode) -> bool { true }
    /// Called after visiting a class declaration and possibly its [definition][ClassDeclarationNode::definition].
    fn exit_class_decl(&mut self, _: &ClassDeclarationNode) {}
    /// Called when visiting a state declaration.
    /// Should return whether to traverse into the [definition][StateDeclarationNode::definition] of the state afterwards. True by default.
    fn visit_state_decl(&mut self, _: &StateDeclarationNode) -> bool { true }
    /// Called after visiting a state declaration and possibly its [definition][StateDeclarationNode::definition].
    fn exit_state_decl(&mut self, _: &StateDeclarationNode) {}
    /// Called when visiting a struct declaration.
    /// Should return whether to traverse into the [definition][StructDeclarationNode::definition] of the struct afterwards. True by default.
    fn visit_struct_decl(&mut self, _: &StructDeclarationNode) -> bool { true }
    /// Called after visiting a struct declaration and possibly its [definition][StructDeclarationNode::definition].
    fn exit_struct_decl(&mut self, _: &StructDeclarationNode) {}
    /// Called when visiting an enum declaration.
    /// Should return whether to traverse into the [definition][EnumDeclarationNode::definition] of the enum afterwards. True by default.
    fn visit_enum_decl(&mut self, _: &EnumDeclarationNode) -> bool { true }
    /// Called after visiting an enum declaration and possibly its [definition][EnumDeclarationNode::definition].
    fn exit_enum_decl(&mut self, _: &EnumDeclarationNode) {}

    /// Called when visiting enum variant's declaration.
    fn visit_enum_variant_decl(&mut self, _: &EnumVariantDeclarationNode) {}

    /// Called when visiting member variable (i.e. field) declaration.
    fn visit_member_var_decl(&mut self, _: &MemberVarDeclarationNode) {}
    /// Called when visiting a statement assigning a default value to a field.
    fn visit_member_default_val(&mut self, _: &MemberDefaultValueNode) {}
    /// Called when visiting an default value assignment inside a `defaults` block.
    fn visit_member_defaults_block_assignment(&mut self, _: &MemberDefaultsBlockAssignmentNode) {}
    /// Called when visiting a statement noting some information about a perticular type field.
    fn visit_member_hint(&mut self, _: &MemberHintNode) {}
    /// Called when visiting an autobind variable declaration.
    fn visit_autobind_decl(&mut self, _: &AutobindDeclarationNode) {}
    
    /// Called when visiting a group of function parameters. This may mean a single parameter or multiple delimited names with common specifiers and a type.
    fn visit_func_param_group(&mut self, _: &FunctionParameterGroupNode) {}
    /// Called when visiting a global function declaration.
    /// Should return whether to traverse into [parameters][GlobalFunctionDeclarationNode::params] 
    /// and [definition][GlobalFunctionDeclarationNode::definition] of the function in that order. True and true by default.
    fn visit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) -> (bool, bool) { (true, true) }
    /// Called after visiting global function declaration 
    /// and possibly also its [parameters][GlobalFunctionDeclarationNode::params] and [definition][GlobalFunctionDeclarationNode::definition].
    fn exit_global_func_decl(&mut self, _: &GlobalFunctionDeclarationNode) {}
    /// Called when visiting a member function declaration (i.e. a method).
    /// Should return whether to traverse into [parameters][MemberFunctionDeclarationNode::params] 
    /// and [definition][MemberFunctionDeclarationNode::definition] of the function in that order. True and true by default.
    fn visit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode) -> (bool, bool) { (true, true) }
    /// Called after visiting member function declaration 
    /// and possibly also its [parameters][MemberFunctionDeclarationNode::params] and [definition][MemberFunctionDeclarationNode::definition].
    fn exit_member_func_decl(&mut self, _: &MemberFunctionDeclarationNode) {}
    /// Called when visiting an event function declaration.
    /// Should return whether to traverse into [parameters][EventDeclarationNode::params] 
    /// and [definition][EventDeclarationNode::definition] of the function in that order. True and true by default.
    fn visit_event_decl(&mut self, _: &EventDeclarationNode) -> (bool, bool) { (true, true) }
    /// Called after visiting member function declaration 
    /// and possibly also its [parameters][EventDeclarationNode::params] and [definition][EventDeclarationNode::definition].
    fn exit_event_decl(&mut self, _: &EventDeclarationNode) {}
    
    /// Called when visiting a local variable declaration inside a function.
    fn visit_local_var_decl_stmt(&mut self, _: &VarDeclarationNode) {}
    /// Called when visiting an expression statement inside a function.
    fn visit_expr_stmt(&mut self, _: &ExpressionStatementNode) {}
    /// Called when visiting a `for` loop.
    /// Should return whether to traverse into the [body][ForLoopNode::body] of the loop. True by default.
    fn visit_for_stmt(&mut self, _: &ForLoopNode) -> bool { true }
    /// Called when visiting a `while` loop.
    /// Should return whether to traverse into the [body][WhileLoopNode::body] of the loop. True by default.
    fn visit_while_stmt(&mut self, _: &WhileLoopNode) -> bool { true }
    /// Called when visiting a `do-while` loop.
    /// Should return whether to traverse into the [body][DoWhileLoopNode::body] of the loop. True by default.
    fn visit_do_while_stmt(&mut self, _: &DoWhileLoopNode) -> bool { true }
    /// Called when visiting an `if` condition.
    /// Should return whether to traverse into the [body][IfConditionalNode::body] of the statement. True by default.
    fn visit_if_stmt(&mut self, _: &IfConditionalNode) -> bool { true }
    /// Called when visiting a `switch` statement.
    /// Should return whether to traverse into [body][SwitchConditionalNode::body] of the statement. True by default.
    fn visit_switch_stmt(&mut self, _: &SwitchConditionalNode) -> bool { true }
    /// Called when visiting a `case` label inside a `switch` statement.
    fn visit_switch_stmt_case(&mut self, _: &SwitchConditionalCaseLabelNode) {}
    /// Called when visiting a `default` label inside a `switch` statement.
    fn visit_switch_stmt_default(&mut self, _: &SwitchConditionalDefaultLabelNode) {}
    /// Called when visiting a `break` statement.
    fn visit_break_stmt(&mut self, _: &BreakStatementNode) {}
    /// Called when visiting a `continue` statement.
    fn visit_continue_stmt(&mut self, _: &ContinueStatementNode) {}
    /// Called when visiting a `return` statement.
    fn visit_return_stmt(&mut self, _: &ReturnStatementNode) {}
    /// Called when visiting a `delete` statement.
    fn visit_delete_stmt(&mut self, _: &DeleteStatementNode) {}
    /// Called when visiting a function block. This may mean a function definition or a scope inside that function.
    /// Should return whether to traverse into [statements][FunctionBlockNode::statements] of the block. True by default.
    fn visit_block_stmt(&mut self, _: &FunctionBlockNode) -> bool { true }
    /// Called when visiting a NOP statement. 
    /// It can mean multiple things, but most notably:
    /// 1. A trailing "orphan" semicolon somewhere in code
    /// 2. Indicating absence of action, e.g. `while(!AreWeThereYet());`
    /// 3. Signaling that a function does not have a definition
    fn visit_nop_stmt(&mut self, _: &NopNode) {}
}

/// Do a left-to-right tree traversal using left-recursion.
/// Should first call the visitor on the node and then traverse to children.
/// Used for sequential nodes, i.e. statements.
pub trait StatementTraversal {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V);
}
