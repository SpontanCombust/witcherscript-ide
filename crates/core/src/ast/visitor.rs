use crate::tokens::*;
use super::*;

/// Handle visitations to expression nodes.
/// Nodes that contain visitable children nodes inside them have a corresponding `exit_` function,
/// which is run after the node itself and its children are visited.
#[allow(unused_variables)]
pub trait ExpressionVisitor {
    /// Called when visiting a parenthesized expression node.
    /// Should return whether to traverse into the expression [nested][NestedExpressionNode::inner] inside it. True by default.
    fn visit_nested_expr(&mut self, n: &NestedExpressionNode) -> bool { true }
    /// Called after visiting the nested expression node and possibly its [inner][NestedExpressionNode::inner] node.
    fn exit_nested_expr(&mut self, n: &NestedExpressionNode) {}

    /// Called when visiting a node representing any literal.
    fn visit_literal_expr(&mut self, n: &LiteralNode) {}
    /// Called when visiting a node representing a `this` expression.
    fn visit_this_expr(&mut self, n: &ThisExpressionNode) {}
    /// Called when visiting a node representing a `super` expression.
    fn visit_super_expr(&mut self, n: &SuperExpressionNode) {}
    /// Called when visiting a node representing a `parent` expression.
    fn visit_parent_expr(&mut self, n: &ParentExpressionNode) {}
    /// Called when visiting a node representing a `virtual_parent` expression.
    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode) {}
    /// Called when visiting a node representing an identifier in code (not a keyword).
    fn visit_identifier_expr(&mut self, n: &IdentifierNode) {}

    /// Called when visiting a function call node.
    /// Should return whether to traverse into this node's callee [func][FunctionCallExpressionNode::func] node 
    /// and [args][FunctionCallExpressionNode::args] in that order. Both true by default.
    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode) -> (bool, bool) { (true, true) }
    /// Called after visiting function call expression node and possibly its [func][FunctionCallExpressionNode::func] and [args][FunctionCallExpressionNode::args] nodes.
    fn exit_func_call_expr(&mut self, n: &FunctionCallExpressionNode) {}

    /// Called when visiting a function call argument.
    /// Node may be None due to it referring to an optional function parameter.
    /// Should return whether to traverse into the expression that this argument contains unless said argument was omitted. True by default.
    fn visit_func_call_arg(&mut self, n: &FunctionCallArgument) -> bool { true }
    /// Called after visiting function call argument and possibly also the expression that it represents.
    fn exit_func_call_arg(&mut self, n: &FunctionCallArgument) {}

    /// Called when visiting an indexing expression.
    /// Should return whether to traverse into this node's [accessor][ArrayExpressionNode::accessor] 
    /// and [index][ArrayExpressionNode::index] expressions in that order. Both true by default.
    fn visit_array_expr(&mut self, n: &ArrayExpressionNode) -> (bool, bool) { (true, true) }
    /// Called after visiting an indexing expression and possibly its [accessor][ArrayExpressionNode::accessor] 
    /// and [index][ArrayExpressionNode::index] expressions.
    fn exit_array_expr(&mut self, n: &ArrayExpressionNode) {}

    /// Called when visiting an expression of accessing a field in an object.
    /// Should return whether to traverse into this node's [accessor][MemberFieldExpressionNode::accessor] node. True by default.
    /// The [member][MemberFieldExpressionNode::member] identifier node is not visited automatically.
    fn visit_member_field_expr(&mut self, n: &MemberFieldExpressionNode) -> bool { true }
    /// Called after visiting an expression of accessing a field in an object and possibly its
    /// [accessor][MemberFieldExpressionNode::accessor]  and [member][MemberFieldExpressionNode::member] nodes.
    fn exit_member_field_expr(&mut self, n: &MemberFieldExpressionNode) {}

    /// Called when visiting an instantiation expression.
    /// Should return whether to traverse into this node's [lifetime_obj][NewExpressionNode::lifetime_obj] node if there is any. True by default.
    /// The [class][NewExpressionNode::class] identifier is not visited automatically.
    fn visit_new_expr(&mut self, n: &NewExpressionNode) -> bool { true }
    /// Called after visiting an instantiation expression and possibly its [lifetime_obj][NewExpressionNode::lifetime_obj] node.
    fn exit_new_expr(&mut self, n: &NewExpressionNode) {}

    /// Called when visiting a type-casting expression.
    /// Should return whether to traverse into this node's [value][NewExpressionNode::class] node. True by default.
    /// The [target_type][NewExpressionNode::target_type] identifier is not visited automatically.
    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode) -> bool { true }
    /// Called after visiting a type-casting expression and possibly also its [value][NewExpressionNode::class] node.
    fn exit_type_cast_expr(&mut self, n: &TypeCastExpressionNode) {}

    /// Called when visiting an unary operation expression.
    /// Should return whether to traverse into this node's [right][UnaryOperationExpressionNode::right] node. True by default.
    /// The [operator][UnaryOperationExpressionNode::op] node is not visited automatically.
    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode) -> bool { true }
    /// Called after visiting an unary operation expression and possibly also its [right][UnaryOperationExpressionNode::right] node.
    fn exit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode) {}
    
    /// Called when visiting a binary operation expression.
    /// Should return whether to traverse into to this node's [left][BinaryOperationExpressionNode::left] 
    /// and [right][BinaryOperationExpressionNode::right] nodes in that order. Both true by default.
    /// The [operator][BinaryOperationExpressionNode::op] node is not visited automatically.
    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode) -> (bool, bool) { (true, true) }
    /// Called after visiting a binary operation expression and possibly also its [left][BinaryOperationExpressionNode::left] 
    /// and [right][BinaryOperationExpressionNode::right] nodes.
    fn exit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode) {}

    /// Called when visiting an assignment operation expression.
    /// Should return whether to traverse into this node's [left][AssignmentOperationExpressionNode::left] 
    /// and [right][AssignmentOperationExpressionNode::right] nodes in that order. Both true by default.
    /// The [operator][AssignmentOperationExpressionNode::op] node is not visited automatically.
    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode) -> (bool, bool) { (true, true) }
    /// Called after visiting an assignment operation expression and possibly also 
    /// its [left][AssignmentOperationExpressionNode::left] and [right][AssignmentOperationExpressionNode::right] nodes.
    fn exit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode) {}

    /// Called when visiting a ternary conditional expression (expr1 ? expr2 : expr3).
    /// Should return whether to traverse into this node's [cond][TernaryConditionalExpressionNode::cond], 
    /// [conseq][TernaryConditionalExpressionNode::conseq] and [alt][TernaryConditionalExpressionNode::alt] nodes in that order.
    /// All true by default.
    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode) -> (bool, bool, bool) { (true, true, true) }
    /// Called after visiting a ternary conditional expression and possiblt also its 
    /// [cond][TernaryConditionalExpressionNode::cond], [conseq][TernaryConditionalExpressionNode::conseq] 
    /// and [alt][TernaryConditionalExpressionNode::alt] nodes.
    fn exit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode) {}
}

/// Traverse an expression node using left-recursion.
pub trait ExpressionTraversal {
    fn accept<V: ExpressionVisitor>(&self, visitor: &mut V);
}


/// Handle visitations to statement nodes.
/// Nodes that contain visitable children nodes inside them have a corresponding `exit_` function,
/// which is run after the node itself and its children are visited.
#[allow(unused_variables)]
pub trait StatementVisitor {
    /// Called when visiting the highest node in the hierarchy.
    /// Should return whether to traverse into the body of the script afterwards. True by default.
    fn visit_root(&mut self, n: &RootNode) -> bool { true }

    /// Called when visiting a class declaration.
    /// Should return whether to traverse into the [definition][ClassDeclarationNode::definition] of the class afterwards. True by default.
    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> bool { true }
    /// Called after visiting a class declaration and possibly its [definition][ClassDeclarationNode::definition].
    fn exit_class_decl(&mut self, n: &ClassDeclarationNode) {}
    /// Called when visiting a state declaration.
    /// Should return whether to traverse into the [definition][StateDeclarationNode::definition] of the state afterwards. True by default.
    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> bool { true }
    /// Called after visiting a state declaration and possibly its [definition][StateDeclarationNode::definition].
    fn exit_state_decl(&mut self, n: &StateDeclarationNode) {}
    /// Called when visiting a struct declaration.
    /// Should return whether to traverse into the [definition][StructDeclarationNode::definition] of the struct afterwards. True by default.
    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> bool { true }
    /// Called after visiting a struct declaration and possibly its [definition][StructDeclarationNode::definition].
    fn exit_struct_decl(&mut self, n: &StructDeclarationNode) {}
    /// Called when visiting an enum declaration.
    /// Should return whether to traverse into the [definition][EnumDeclarationNode::definition] of the enum afterwards. True by default.
    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> bool { true }
    /// Called after visiting an enum declaration and possibly its [definition][EnumDeclarationNode::definition].
    fn exit_enum_decl(&mut self, n: &EnumDeclarationNode) {}

    /// Called when visiting enum variant's declaration.
    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {}

    /// Called when visiting member variable (i.e. field) declaration.
    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode) {}
    /// Called when visiting a statement assigning a default value to a field.
    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode) {}
    /// Called when visiting a `defaults` block.
    /// Should return whether to traverse into its [assignment][MemberDefaultsBlockNode::iter] nodes.
    fn visit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode) -> bool { true }
    /// Called after visitng a `defaults` block and possibly also its assignment nodes.
    fn exit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode) {}
    /// Called when visiting a default value assignment inside a `defaults` block.
    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode) {}
    /// Called when visiting a statement noting some information about a perticular type field.
    fn visit_member_hint(&mut self, n: &MemberHintNode) {}
    /// Called when visiting an autobind variable declaration.
    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode) {}
    
    /// Called when visiting a group of function parameters. This may mean a single parameter or multiple delimited names with common specifiers and a type.
    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode) {}
    /// Called when visiting a global function declaration.
    /// Should return whether to traverse into [parameters][GlobalFunctionDeclarationNode::params] 
    /// and [definition][GlobalFunctionDeclarationNode::definition] of the function in that order. True and true by default.
    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> (bool, bool) { (true, true) }
    /// Called after visiting global function declaration 
    /// and possibly also its [parameters][GlobalFunctionDeclarationNode::params] and [definition][GlobalFunctionDeclarationNode::definition].
    fn exit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) {}
    /// Called when visiting a member function declaration (i.e. a method).
    /// Should return whether to traverse into [parameters][MemberFunctionDeclarationNode::params] 
    /// and [definition][MemberFunctionDeclarationNode::definition] of the function in that order. True and true by default.
    fn visit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode) -> (bool, bool) { (true, true) }
    /// Called after visiting member function declaration 
    /// and possibly also its [parameters][MemberFunctionDeclarationNode::params] and [definition][MemberFunctionDeclarationNode::definition].
    fn exit_member_func_decl(&mut self, n: &MemberFunctionDeclarationNode) {}
    /// Called when visiting an event function declaration.
    /// Should return whether to traverse into [parameters][EventDeclarationNode::params] 
    /// and [definition][EventDeclarationNode::definition] of the function in that order. True and true by default.
    fn visit_event_decl(&mut self, n: &EventDeclarationNode) -> (bool, bool) { (true, true) }
    /// Called after visiting member function declaration 
    /// and possibly also its [parameters][EventDeclarationNode::params] and [definition][EventDeclarationNode::definition].
    fn exit_event_decl(&mut self, n: &EventDeclarationNode) {}
    
    /// Called when visiting a local variable declaration inside a function.
    fn visit_local_var_decl_stmt(&mut self, n: &VarDeclarationNode) {}
    /// Called when visiting an expression statement inside a function.
    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode) {}
    /// Called when visiting a `for` loop.
    /// Should return whether to traverse into the [body][ForLoopNode::body] of the loop. True by default.
    fn visit_for_stmt(&mut self, n: &ForLoopNode) -> bool { true }
    /// Called when visiting a `while` loop.
    /// Should return whether to traverse into the [body][WhileLoopNode::body] of the loop. True by default.
    fn visit_while_stmt(&mut self, n: &WhileLoopNode) -> bool { true }
    /// Called when visiting a `do-while` loop.
    /// Should return whether to traverse into the [body][DoWhileLoopNode::body] of the loop. True by default.
    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode) -> bool { true }
    /// Called when visiting an `if` condition.
    /// Should return whether to traverse into the [body][IfConditionalNode::body] of the statement. True by default.
    fn visit_if_stmt(&mut self, n: &IfConditionalNode) -> bool { true }
    /// Called when visiting a `switch` statement.
    /// Should return whether to traverse into [body][SwitchConditionalNode::body] of the statement. True by default.
    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode) -> bool { true }
    /// Called when visiting a `case` label inside a `switch` statement.
    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode) {}
    /// Called when visiting a `default` label inside a `switch` statement.
    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode) {}
    /// Called when visiting a `break` statement.
    fn visit_break_stmt(&mut self, n: &BreakStatementNode) {}
    /// Called when visiting a `continue` statement.
    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode) {}
    /// Called when visiting a `return` statement.
    fn visit_return_stmt(&mut self, n: &ReturnStatementNode) {}
    /// Called when visiting a `delete` statement.
    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode) {}
    /// Called when visiting a function block. This may mean a function definition or a scope inside that function.
    /// Should return whether to traverse into [statements][FunctionBlockNode::iter] of the block. True by default.
    fn visit_block_stmt(&mut self, n: &FunctionBlockNode) -> bool { true }
    /// Called when visiting a NOP statement. 
    /// It can mean multiple things, but most notably:
    /// 1. A trailing "orphan" semicolon somewhere in code
    /// 2. Indicating absence of action, e.g. `while(!AreWeThereYet());`
    /// 3. Signaling that a function does not have a definition
    fn visit_nop_stmt(&mut self, n: &NopNode) {}
}

/// Traverse a statement node using left-recursion.
pub trait StatementTraversal {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V);
}
