use crate::{ast::*, tokens::*, NamedSyntaxNode, SyntaxNode};


/// Default opaque node type not possessing any additional capabilities.
pub type AnyNode<'script> = SyntaxNode<'script, ()>;


impl std::fmt::Debug for AnyNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxNode")
            .field("tree_node", &self.tree_node)
            .finish()
    }
}

/// !!! IMPORTANT !!!
/// All visitable node types should be handled here
impl<'script> SyntaxNodeTraversal for AnyNode<'script> {
    fn accept<V: crate::ast::SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        if self.tree_node.is_named() {
            let n = self.to_owned();
    
            match self.tree_node.kind() {
                NestedExpressionNode::NODE_KIND => { let n: NestedExpressionNode = n.into(); n.accept(visitor, ctx); },
    
                LiteralIntNode::NODE_KIND       |
                LiteralHexNode::NODE_KIND       |
                LiteralFloatNode::NODE_KIND     |
                LiteralBoolNode::NODE_KIND      |
                LiteralStringNode::NODE_KIND    |
                LiteralNameNode::NODE_KIND      |
                LiteralNullNode::NODE_KIND      => { let n: LiteralNode = n.into(); n.accept(visitor, ctx); },
    
                ThisExpressionNode::NODE_KIND => { let n: ThisExpressionNode = n.into(); n.accept(visitor, ctx); },
                SuperExpressionNode::NODE_KIND => { let n: SuperExpressionNode = n.into(); n.accept(visitor, ctx); },
                ParentExpressionNode::NODE_KIND => { let n: ParentExpressionNode = n.into(); n.accept(visitor, ctx); },
                VirtualParentExpressionNode::NODE_KIND => { let n: VirtualParentExpressionNode = n.into(); n.accept(visitor, ctx); },
                IdentifierNode::NODE_KIND => { let n: IdentifierNode = n.into(); n.accept(visitor, ctx); },
                
                FunctionCallExpressionNode::NODE_KIND => { let n: FunctionCallExpressionNode = n.into(); n.accept(visitor, ctx); },
                ArrayExpressionNode::NODE_KIND => { let n: ArrayExpressionNode = n.into(); n.accept(visitor, ctx); },
                MemberAccessExpressionNode::NODE_KIND => { let n: MemberAccessExpressionNode = n.into(); n.accept(visitor, ctx); },
                NewExpressionNode::NODE_KIND => { let n: NewExpressionNode = n.into(); n.accept(visitor, ctx); },
                TypeCastExpressionNode::NODE_KIND => { let n: TypeCastExpressionNode = n.into(); n.accept(visitor, ctx); },
                UnaryOperationExpressionNode::NODE_KIND => { let n: UnaryOperationExpressionNode = n.into(); n.accept(visitor, ctx); },
                BinaryOperationExpressionNode::NODE_KIND => { let n: BinaryOperationExpressionNode = n.into(); n.accept(visitor, ctx); },
                AssignmentOperationExpressionNode::NODE_KIND => { let n: AssignmentOperationExpressionNode = n.into(); n.accept(visitor, ctx); },
                TernaryConditionalExpressionNode::NODE_KIND => { let n: TernaryConditionalExpressionNode = n.into(); n.accept(visitor, ctx); },
                
                
                RootNode::NODE_KIND => { let n: RootNode = n.into(); n.accept(visitor, ctx); },
                ClassDeclarationNode::NODE_KIND => { let n: ClassDeclarationNode = n.into(); n.accept(visitor, ctx); },
                StateDeclarationNode::NODE_KIND => { let n: StateDeclarationNode = n.into(); n.accept(visitor, ctx); },
                StructDeclarationNode::NODE_KIND => { let n: StructDeclarationNode = n.into(); n.accept(visitor, ctx); },
                EnumDeclarationNode::NODE_KIND => { let n: EnumDeclarationNode = n.into(); n.accept(visitor, ctx); },
                EnumVariantDeclarationNode::NODE_KIND => { let n: EnumVariantDeclarationNode = n.into(); n.accept(visitor, ctx); },
    
                MemberVarDeclarationNode::NODE_KIND => { let n: MemberVarDeclarationNode = n.into(); n.accept(visitor, ctx); },
                MemberDefaultValueNode::NODE_KIND => { let n: MemberDefaultValueNode = n.into(); n.accept(visitor, ctx); },
                MemberDefaultsBlockNode::NODE_KIND => { let n: MemberDefaultsBlockNode = n.into(); n.accept(visitor, ctx); },
                MemberDefaultsBlockAssignmentNode::NODE_KIND => { let n: MemberDefaultsBlockAssignmentNode = n.into(); n.accept(visitor, ctx); },
                MemberHintNode::NODE_KIND => { let n: MemberHintNode = n.into(); n.accept(visitor, ctx); },
                AutobindDeclarationNode::NODE_KIND => { let n: AutobindDeclarationNode = n.into(); n.accept(visitor, ctx); },
                FunctionParameterGroupNode::NODE_KIND => { let n: FunctionParameterGroupNode = n.into(); n.accept(visitor, ctx); },
                FunctionDeclarationNode::NODE_KIND => { let n: FunctionDeclarationNode = n.into(); n.accept(visitor, ctx); },
                EventDeclarationNode::NODE_KIND => { let n: EventDeclarationNode = n.into(); n.accept(visitor, ctx); },
                
                
                LocalVarDeclarationNode::NODE_KIND => { let n: LocalVarDeclarationNode = n.into(); n.accept(visitor, ctx); },
                ExpressionStatementNode::NODE_KIND => { let n: ExpressionStatementNode = n.into(); n.accept(visitor, ctx); },
                ForLoopNode::NODE_KIND => { let n: ForLoopNode = n.into(); n.accept(visitor, ctx); },
                WhileLoopNode::NODE_KIND => { let n: WhileLoopNode = n.into(); n.accept(visitor, ctx); },
                DoWhileLoopNode::NODE_KIND => { let n: DoWhileLoopNode = n.into(); n.accept(visitor, ctx); },
                IfConditionalNode::NODE_KIND => { let n: IfConditionalNode = n.into(); n.accept(visitor, ctx); },
                SwitchConditionalNode::NODE_KIND => { let n: SwitchConditionalNode = n.into(); n.accept(visitor, ctx); },
                SwitchConditionalCaseLabelNode::NODE_KIND => { let n: SwitchConditionalCaseLabelNode = n.into(); n.accept(visitor, ctx); },
                SwitchConditionalDefaultLabelNode::NODE_KIND => { let n: SwitchConditionalDefaultLabelNode = n.into(); n.accept(visitor, ctx); },
                BreakStatementNode::NODE_KIND => { let n: BreakStatementNode = n.into(); n.accept(visitor, ctx); },
                ContinueStatementNode::NODE_KIND => { let n: ContinueStatementNode = n.into(); n.accept(visitor, ctx); },
                ReturnStatementNode::NODE_KIND => { let n: ReturnStatementNode = n.into(); n.accept(visitor, ctx); },
                DeleteStatementNode::NODE_KIND => { let n: DeleteStatementNode = n.into(); n.accept(visitor, ctx); },
                CompoundStatementNode::NODE_KIND => { let n: CompoundStatementNode = n.into(); n.accept(visitor, ctx); },
                NopNode::NODE_KIND => { let n: NopNode = n.into(); n.accept(visitor, ctx); },
    
                _ => {}
            } 
        }
    }
}
