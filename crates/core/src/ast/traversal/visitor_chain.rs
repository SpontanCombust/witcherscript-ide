use std::{cell::{RefCell, RefMut}, rc::Rc};
use crate::ast::*;
use crate::tokens::*;


pub trait SyntaxNodeVisitorChainLink: SyntaxNodeVisitor {
    fn pass_onto_next_link(&self) -> bool { true }
}


/// A chain that can combine multiple node visitors (here links) and then call visitation methods on them in sequence on each node.
/// A link can decide to stop the propagation to links after it. If it does that for a node with traversable children,
/// that choice is remembered for the corresponding `exit_` method call and the chain stops on the same link.
pub struct SyntaxNodeVisitorChain<'a> {
    /// Visitors whose methods will be called in a sequence
    links: Vec<Rc<RefCell<dyn SyntaxNodeVisitorChainLink + 'a>>>,
    /// Numbers of links called during a visit to a node with traversable children
    /// The number on the top is remembered for the exit_ function
    link_passes_stack: Vec<usize>
}

impl<'a> SyntaxNodeVisitorChain<'a> {
    pub fn new() -> Self {
        Self {
            links: Vec::with_capacity(2),
            link_passes_stack: Vec::with_capacity(32)
        }
    }

    pub fn link<L>(mut self, link: L) -> Self
    where L: SyntaxNodeVisitorChainLink + 'a {
        self.links.push(Rc::new(RefCell::new(link)));
        self
    }

    pub fn link_rc<L>(mut self, link: Rc<RefCell<L>>) -> Self
    where L: SyntaxNodeVisitorChainLink + 'a {
        self.links.push(link);
        self
    }



    fn chain_visit<F>(&mut self, f: F)
    where
        F: Fn(&mut RefMut<'_, dyn SyntaxNodeVisitorChainLink + 'a>)
    {
        for link in self.links.iter_mut() {
            let mut link_ref = link.borrow_mut();
            f(&mut link_ref);
            if !link_ref.pass_onto_next_link() {
                break;
            }
        }
    }

    fn chain_visit_traversable<TP, F>(&mut self, f: F) -> TP 
    where 
        TP: TraversalPolicy + std::ops::BitAnd<Output = TP>,
        F: Fn(&mut RefMut<'_, dyn SyntaxNodeVisitorChainLink + 'a>) -> TP
    {
        let mut tp = TraversalPolicy::default_to(true);
        let mut visited_link_count = 0;
        for link in self.links.iter_mut() {
            let mut link_ref = link.borrow_mut();
            tp = tp & f(&mut link_ref);
            visited_link_count += 1;
            if !link_ref.pass_onto_next_link() {
                break;
            }
        }

        self.link_passes_stack.push(visited_link_count);

        tp
    }

    fn chain_exit<F>(&mut self, f: F)
    where
        F: Fn(&mut RefMut<'_, dyn SyntaxNodeVisitorChainLink + 'a>)
    {
        for i in 0..self.link_passes_stack.pop().unwrap_or(0) {
            let mut link_ref = self.links[i].borrow_mut();
            f(&mut link_ref);
        }
    }
}

impl<'a> SyntaxNodeVisitor for SyntaxNodeVisitorChain<'a> {
    fn traversal_policy_default(&self) -> bool {
        true
    }


    fn visit_root(&mut self, n: &RootNode) -> RootTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_root(n))
    }

    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> ClassDeclarationTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_class_decl(n))
    }

    fn exit_class_decl(&mut self, n: &ClassDeclarationNode) {
        self.chain_exit(move |link| link.exit_class_decl(n))
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> StructDeclarationTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_struct_decl(n))
    }

    fn exit_struct_decl(&mut self, n: &StructDeclarationNode) {
        self.chain_exit(move |link| link.exit_struct_decl(n))
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> StateDeclarationTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_state_decl(n))
    }

    fn exit_state_decl(&mut self, n: &StateDeclarationNode) {
        self.chain_exit(move |link| link.exit_state_decl(n))
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> EnumDeclarationTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_enum_decl(n))
    }

    fn exit_enum_decl(&mut self, n: &EnumDeclarationNode) {
        self.chain_exit(move |link| link.exit_enum_decl(n))
    }

    fn visit_enum_variant_decl(&mut self, n: &EnumVariantDeclarationNode) {
        self.chain_visit(move |link| link.visit_enum_variant_decl(n))
    }

    fn visit_global_func_decl(&mut self, n: &FunctionDeclarationNode) -> FunctionDeclarationTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_global_func_decl(n))        
    }

    fn exit_global_func_decl(&mut self, n: &FunctionDeclarationNode) {
        self.chain_exit(move |link| link.exit_global_func_decl(n))
    }

    fn visit_global_var_decl(&mut self, n: &MemberVarDeclarationNode) {
        self.chain_visit(move |link| link.visit_global_var_decl(n))
    }



    fn visit_member_func_decl(&mut self, n: &FunctionDeclarationNode, ctx: &TraversalContextStack) -> FunctionDeclarationTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_member_func_decl(n, ctx))
    }

    fn exit_member_func_decl(&mut self, n: &FunctionDeclarationNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_member_func_decl(n, ctx))
    }

    fn visit_event_decl(&mut self, n: &EventDeclarationNode, ctx: &TraversalContextStack) -> EventDeclarationTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_event_decl(n, ctx))
    }

    fn exit_event_decl(&mut self, n: &EventDeclarationNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_event_decl(n, ctx))
    }

    fn visit_func_param_group(&mut self, n: &FunctionParameterGroupNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_func_param_group(n, ctx))
    }

    fn visit_member_var_decl(&mut self, n: &MemberVarDeclarationNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_member_var_decl(n, ctx))
    }

    fn visit_autobind_decl(&mut self, n: &AutobindDeclarationNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_autobind_decl(n, ctx))
    }

    fn visit_member_default_val(&mut self, n: &MemberDefaultValueNode, ctx: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_member_default_val(n, ctx))
    }

    fn exit_member_default_val(&mut self, n: &MemberDefaultValueNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_member_default_val(n, ctx))
    }

    fn visit_member_hint(&mut self, n: &MemberHintNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_member_hint(n, ctx))
    }

    fn visit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode, ctx: &TraversalContextStack) -> MemberDefaultsBlockTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_member_defaults_block(n, ctx))
    }

    fn exit_member_defaults_block(&mut self, n: &MemberDefaultsBlockNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_member_defaults_block(n, ctx))
    }

    fn visit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode, ctx: &TraversalContextStack) -> MemberDefaultValueTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_member_defaults_block_assignment(n, ctx))
    }

    fn exit_member_defaults_block_assignment(&mut self, n: &MemberDefaultsBlockAssignmentNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_member_defaults_block_assignment(n, ctx))
    }



    fn visit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, ctx: &TraversalContextStack) -> VarDeclarationTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_local_var_decl_stmt(n, ctx))
    }

    fn exit_local_var_decl_stmt(&mut self, n: &LocalVarDeclarationNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_local_var_decl_stmt(n, ctx))
    }

    fn visit_compound_stmt(&mut self, n: &CompoundStatementNode, ctx: &TraversalContextStack) -> CompoundStatementTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_compound_stmt(n, ctx))
    }

    fn exit_compound_stmt(&mut self, n: &CompoundStatementNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_compound_stmt(n, ctx))
    }

    fn visit_while_stmt(&mut self, n: &WhileLoopNode, ctx: &TraversalContextStack) -> WhileLoopTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_while_stmt(n, ctx))
    }

    fn exit_while_stmt(&mut self, n: &WhileLoopNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_while_stmt(n, ctx))
    }

    fn visit_do_while_stmt(&mut self, n: &DoWhileLoopNode, ctx: &TraversalContextStack) -> DoWhileLoopTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_do_while_stmt(n, ctx))
    }

    fn exit_do_while_stmt(&mut self, n: &DoWhileLoopNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_do_while_stmt(n, ctx))
    }

    fn visit_for_stmt(&mut self, n: &ForLoopNode, ctx: &TraversalContextStack) -> ForLoopTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_for_stmt(n, ctx))
    }

    fn exit_for_stmt(&mut self, n: &ForLoopNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_for_stmt(n, ctx))
    }

    fn visit_if_stmt(&mut self, n: &IfConditionalNode, ctx: &TraversalContextStack) -> IfConditionalTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_if_stmt(n, ctx))
    }

    fn exit_if_stmt(&mut self, n: &IfConditionalNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_if_stmt(n, ctx))
    }

    fn visit_switch_stmt(&mut self, n: &SwitchConditionalNode, ctx: &TraversalContextStack) -> SwitchConditionalTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_switch_stmt(n, ctx))
    }

    fn exit_switch_stmt(&mut self, n: &SwitchConditionalNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_switch_stmt(n, ctx))
    }

    fn visit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode, ctx: &TraversalContextStack) -> SwitchConditionalCaseLabelTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_switch_stmt_case(n, ctx))
    }

    fn exit_switch_stmt_case(&mut self, n: &SwitchConditionalCaseLabelNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_switch_stmt_case(n, ctx))
    }

    fn visit_switch_stmt_default(&mut self, n: &SwitchConditionalDefaultLabelNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_switch_stmt_default(n, ctx))
    }

    fn visit_expr_stmt(&mut self, n: &ExpressionStatementNode, ctx: &TraversalContextStack) -> ExpressionStatementTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_expr_stmt(n, ctx))
    }

    fn exit_expr_stmt(&mut self, n: &ExpressionStatementNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_expr_stmt(n, ctx))
    }

    fn visit_return_stmt(&mut self, n: &ReturnStatementNode, ctx: &TraversalContextStack) -> ReturnStatementTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_return_stmt(n, ctx))
    }

    fn exit_return_stmt(&mut self, n: &ReturnStatementNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_return_stmt(n, ctx))
    }

    fn visit_delete_stmt(&mut self, n: &DeleteStatementNode, ctx: &TraversalContextStack) -> DeleteStatementTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_delete_stmt(n, ctx))
    }

    fn exit_delete_stmt(&mut self, n: &DeleteStatementNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_delete_stmt(n, ctx))
    }

    fn visit_break_stmt(&mut self, n: &BreakStatementNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_break_stmt(n, ctx))
    }

    fn visit_continue_stmt(&mut self, n: &ContinueStatementNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_continue_stmt(n, ctx))
    }

    fn visit_nop_stmt(&mut self, n: &NopNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_nop_stmt(n, ctx))
    }



    fn visit_nested_expr(&mut self, n: &NestedExpressionNode, ctx: &TraversalContextStack) -> NestedExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_nested_expr(n, ctx))
    }

    fn exit_nested_expr(&mut self, n: &NestedExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_nested_expr(n, ctx))
    }

    fn visit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode, ctx: &TraversalContextStack) -> AssignmentOperationExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_assign_op_expr(n, ctx))
    }

    fn exit_assign_op_expr(&mut self, n: &AssignmentOperationExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_assign_op_expr(n, ctx))
    }

    fn visit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, ctx: &TraversalContextStack) -> BinaryOperationExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_binary_op_expr(n, ctx))
    }

    fn exit_binary_op_expr(&mut self, n: &BinaryOperationExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_binary_op_expr(n, ctx))
    }

    fn visit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode, ctx: &TraversalContextStack) -> UnaryOperationExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_unary_op_expr(n, ctx))
    }

    fn exit_unary_op_expr(&mut self, n: &UnaryOperationExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_unary_op_expr(n, ctx))
    }

    fn visit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, ctx: &TraversalContextStack) -> MemberFieldExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_member_access_expr(n, ctx))
    }

    fn exit_member_access_expr(&mut self, n: &MemberAccessExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_member_access_expr(n, ctx))
    }

    fn visit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, ctx: &TraversalContextStack) -> TypeCastExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_type_cast_expr(n, ctx))
    }

    fn exit_type_cast_expr(&mut self, n: &TypeCastExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_type_cast_expr(n, ctx))
    }

    fn visit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode, ctx: &TraversalContextStack) -> TernaryConditionalExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_ternary_cond_expr(n, ctx))
    }

    fn exit_ternary_cond_expr(&mut self, n: &TernaryConditionalExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_ternary_cond_expr(n, ctx))
    }

    fn visit_new_expr(&mut self, n: &NewExpressionNode, ctx: &TraversalContextStack) -> NewExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_new_expr(n, ctx))
    }

    fn exit_new_expr(&mut self, n: &NewExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_new_expr(n, ctx))
    }

    fn visit_array_expr(&mut self, n: &ArrayExpressionNode, ctx: &TraversalContextStack) -> ArrayExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_array_expr(n, ctx))
    }

    fn exit_array_expr(&mut self, n: &ArrayExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_array_expr(n, ctx))
    }

    fn visit_func_call_expr(&mut self, n: &FunctionCallExpressionNode, ctx: &TraversalContextStack) -> FunctionCallExpressionTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_func_call_expr(n, ctx))
    }

    fn exit_func_call_expr(&mut self, n: &FunctionCallExpressionNode, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_func_call_expr(n, ctx))
    }

    fn visit_func_call_arg(&mut self, n: &FunctionCallArgument, ctx: &TraversalContextStack) -> FunctionCallArgumentTraversalPolicy {
        self.chain_visit_traversable(move |link| link.visit_func_call_arg(n, ctx))
    }

    fn exit_func_call_arg(&mut self, n: &FunctionCallArgument, ctx: &TraversalContextStack) {
        self.chain_exit(move |link| link.exit_func_call_arg(n, ctx))
    }

    fn visit_identifier_expr(&mut self, n: &IdentifierNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_identifier_expr(n, ctx))
    }

    fn visit_literal_expr(&mut self, n: &LiteralNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_literal_expr(n, ctx))
    }

    fn visit_this_expr(&mut self, n: &ThisExpressionNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_this_expr(n, ctx))
    }

    fn visit_super_expr(&mut self, n: &SuperExpressionNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_super_expr(n, ctx))
    }

    fn visit_parent_expr(&mut self, n: &ParentExpressionNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_parent_expr(n, ctx))
    }

    fn visit_virtual_parent_expr(&mut self, n: &VirtualParentExpressionNode, ctx: &TraversalContextStack) {
        self.chain_visit(move |link| link.visit_virtual_parent_expr(n, ctx))
    }
}