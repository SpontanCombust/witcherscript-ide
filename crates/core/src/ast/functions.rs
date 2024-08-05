use std::fmt::Debug;
use crate::{attribs::*, tokens::IdentifierNode, AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct EventDeclaration;
    pub struct FunctionDeclaration;
    pub struct FunctionParameters;
    pub struct FunctionParameterGroup;
    pub struct FunctionBlock;
    pub struct BreakStatement;
    pub struct ContinueStatement;
    pub struct ReturnStatement;
    pub struct DeleteStatement;
    pub struct CompoundStatement;
}


pub type EventDeclarationNode<'script> = SyntaxNode<'script, tags::EventDeclaration>;

impl NamedSyntaxNode for EventDeclarationNode<'_> {
    const NODE_KIND: &'static str = "event_decl";
}

impl<'script> EventDeclarationNode<'script> {
    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().into()
    }

    pub fn params(&self) -> FunctionParametersNode<'script> {
        self.field_child("params").unwrap().into()
    }

    pub fn return_type(&self) -> Option<TypeAnnotationNode<'script>> {
        self.field_child("return_type").map(|n| n.into())
    }

    pub fn definition(&self) -> FunctionDefinitionNode<'script> {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for EventDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("EventDeclaration {}", self.range().debug()))
            .field("name", &self.name())
            .field("params", &self.params())
            .field("return_type", &self.return_type())
            .field("definition", &self.definition())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for EventDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for EventDeclarationNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_event_decl(self, ctx);

        ctx.push(TraversalContext::Event);
        if tp.traverse_params {
            self.params().accept(visitor, ctx);
        }
        if tp.traverse_definition {
            self.definition().accept(visitor, ctx);
        }
        ctx.pop();
        
        visitor.exit_event_decl(self, ctx);
    }
}



pub type FunctionDeclarationNode<'script> = SyntaxNode<'script, tags::FunctionDeclaration>;

impl NamedSyntaxNode for FunctionDeclarationNode<'_> {
    const NODE_KIND: &'static str = "func_decl";
}

impl<'script> FunctionDeclarationNode<'script> {
    pub fn annotation(&self) -> Option<AnnotationNode<'script>> {
        self.field_child("annotation").map(|n| n.into())
    }

    pub fn specifiers(&self) -> impl Iterator<Item = SpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn flavour(&self) -> Option<FunctionFlavourNode<'script>> {
        self.field_child("flavour").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().into()
    }

    pub fn params(&self) -> FunctionParametersNode<'script> {
        self.field_child("params").unwrap().into()
    }

    pub fn return_type(&self) -> Option<TypeAnnotationNode<'script>> {
        self.field_child("return_type").map(|n| n.into())
    }

    pub fn definition(&self) -> FunctionDefinitionNode<'script> {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for FunctionDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("FunctionDeclaration {}", self.range().debug()))
            .field("annotation", &self.annotation())
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("flavour", &self.flavour())
            .field("name", &self.name())
            .field("params", &self.params())
            .field("return_type", &self.return_type())
            .field("definition", &self.definition())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for FunctionDeclarationNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        if ctx.top() == TraversalContext::Global {
            let tp = visitor.visit_global_func_decl(self);
    
            ctx.push(TraversalContext::GlobalFunction);
            if tp.traverse_params {
                self.params().accept(visitor, ctx);
            }
            if tp.traverse_definition {
                self.definition().accept(visitor, ctx);
            }
            ctx.pop();

            visitor.exit_global_func_decl(self);
        } else {
            let tp = visitor.visit_member_func_decl(self, ctx);
    
            ctx.push(TraversalContext::MemberFunction);
            if tp.traverse_params {
                self.params().accept(visitor, ctx);
            }
            if tp.traverse_definition {
                self.definition().accept(visitor, ctx);
            }
            ctx.pop();

            visitor.exit_member_func_decl(self, ctx);
        }
    }
}



#[derive(Clone)]
pub enum FunctionDefinition<'script> {
    Some(FunctionBlockNode<'script>),
    None(NopNode<'script>)
}

impl Debug for FunctionDefinition<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(n) => f.debug_tuple("Some").field(n).finish(),
            Self::None(_) => f.debug_tuple("None").finish(),
        }
    }
}

pub type FunctionDefinitionNode<'script> = SyntaxNode<'script, FunctionDefinition<'script>>;

impl<'script> FunctionDefinitionNode<'script> {
    pub fn value(self) -> FunctionDefinition<'script> {
        match self.tree_node.kind() {
            FunctionBlockNode::NODE_KIND => FunctionDefinition::Some(self.into()),
            NopNode::NODE_KIND => FunctionDefinition::None(self.into()),
            _ => panic!("Unknown function definition node: {} {}", self.tree_node.kind(), self.range().debug())
        }
    }
}

impl Debug for FunctionDefinitionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.clone().value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionDefinitionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() {
            return Err(());
        }

        match value.tree_node.kind() {
            FunctionBlockNode::NODE_KIND    |
            NopNode::NODE_KIND              => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl SyntaxNodeTraversal for FunctionDefinitionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        if let FunctionDefinition::Some(block) = self.clone().value() {
            block.accept(visitor, ctx);
        }
    }
}



pub type FunctionBlockNode<'script> = SyntaxNode<'script, tags::FunctionBlock>;

impl NamedSyntaxNode for FunctionBlockNode<'_> {
    const NODE_KIND: &'static str = "func_def";
}

impl<'script> FunctionBlockNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = FunctionStatementNode<'script>> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for FunctionBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("FunctionBlock {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionBlockNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for FunctionBlockNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        self.iter().for_each(|s| s.accept(visitor, ctx));
    }
}



pub type FunctionParametersNode<'script> = SyntaxNode<'script, tags::FunctionParameters>;

impl NamedSyntaxNode for FunctionParametersNode<'_> {
    const NODE_KIND: &'static str = "func_params";
}

impl<'script> FunctionParametersNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = FunctionParameterGroupNode<'script>> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for FunctionParametersNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("FunctionParameters {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionParametersNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for FunctionParametersNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        self.iter().for_each(|s| s.accept(visitor, ctx));
    }
}



pub type FunctionParameterGroupNode<'script> = SyntaxNode<'script, tags::FunctionParameterGroup>;

impl NamedSyntaxNode for FunctionParameterGroupNode<'_> {
    const NODE_KIND: &'static str = "func_param_group";
}

impl<'script> FunctionParameterGroupNode<'script> {
    pub fn specifiers(&self) -> impl Iterator<Item = SpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn names(&self) -> impl Iterator<Item = IdentifierNode<'script>> {
        self.field_children("names").map(|n| n.into())
    }

    pub fn param_type(&self) -> TypeAnnotationNode<'script> {
        self.field_child("param_type").unwrap().into()
    }
}

impl Debug for FunctionParameterGroupNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("FunctionParameterGroup {}", self.range().debug()))
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("names", &self.names().collect::<Vec<_>>())
            .field("param_type", &self.param_type())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionParameterGroupNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for FunctionParameterGroupNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        visitor.visit_func_param_group(self, ctx);
    }
}



#[derive(Clone)]
pub enum FunctionStatement<'script> {
    Var(LocalVarDeclarationNode<'script>),
    Expr(ExpressionStatementNode<'script>),
    For(ForLoopNode<'script>),
    While(WhileLoopNode<'script>),
    DoWhile(DoWhileLoopNode<'script>),
    If(IfConditionalNode<'script>),
    Switch(SwitchConditionalNode<'script>),
    Break(BreakStatementNode<'script>),
    Continue(ContinueStatementNode<'script>),
    Return(ReturnStatementNode<'script>),
    Delete(DeleteStatementNode<'script>),
    Compound(CompoundStatementNode<'script>),
    Nop(NopNode<'script>),
}

impl Debug for FunctionStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(n) => f.debug_maybe_alternate(n),
            Self::Expr(n) => f.debug_maybe_alternate(n),
            Self::For(n) => f.debug_maybe_alternate(n),
            Self::While(n) => f.debug_maybe_alternate(n),
            Self::DoWhile(n) => f.debug_maybe_alternate(n),
            Self::If(n) => f.debug_maybe_alternate(n),
            Self::Switch(n) => f.debug_maybe_alternate(n),
            Self::Break(n) => f.debug_maybe_alternate(n),
            Self::Continue(n) => f.debug_maybe_alternate(n),
            Self::Return(n) => f.debug_maybe_alternate(n),
            Self::Delete(n) => f.debug_maybe_alternate(n),
            Self::Compound(n) => f.debug_maybe_alternate(n),
            Self::Nop(n) => f.debug_maybe_alternate(n),
        }
    }
}

pub type FunctionStatementNode<'script> = SyntaxNode<'script, FunctionStatement<'script>>;

impl<'script> FunctionStatementNode<'script> {
    pub fn value(self) -> FunctionStatement<'script> {
        match self.tree_node.kind() {
            LocalVarDeclarationNode::NODE_KIND => FunctionStatement::Var(self.into()),
            ExpressionStatementNode::NODE_KIND => FunctionStatement::Expr(self.into()),
            ForLoopNode::NODE_KIND => FunctionStatement::For(self.into()),
            WhileLoopNode::NODE_KIND => FunctionStatement::While(self.into()),
            DoWhileLoopNode::NODE_KIND => FunctionStatement::DoWhile(self.into()),
            IfConditionalNode::NODE_KIND => FunctionStatement::If(self.into()),
            SwitchConditionalNode::NODE_KIND => FunctionStatement::Switch(self.into()),
            BreakStatementNode::NODE_KIND => FunctionStatement::Break(self.into()),
            ContinueStatementNode::NODE_KIND => FunctionStatement::Continue(self.into()),
            ReturnStatementNode::NODE_KIND => FunctionStatement::Return(self.into()),
            DeleteStatementNode::NODE_KIND => FunctionStatement::Delete(self.into()),
            CompoundStatementNode::NODE_KIND => FunctionStatement::Compound(self.into()),
            NopNode::NODE_KIND => FunctionStatement::Nop(self.into()),
            _ => panic!("Unknown function statement type: {} {}", self.tree_node.kind(), self.range().debug())
        }
    }
}

impl Debug for FunctionStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.clone().value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if !value.tree_node.is_named() {
            return Err(());
        }

        match value.tree_node.kind() {
            LocalVarDeclarationNode::NODE_KIND       |
            ExpressionStatementNode::NODE_KIND  |
            ForLoopNode::NODE_KIND              |
            WhileLoopNode::NODE_KIND            |
            DoWhileLoopNode::NODE_KIND          |
            IfConditionalNode::NODE_KIND        |
            SwitchConditionalNode::NODE_KIND    |
            BreakStatementNode::NODE_KIND       |
            ContinueStatementNode::NODE_KIND    |
            ReturnStatementNode::NODE_KIND      |
            DeleteStatementNode::NODE_KIND      |
            CompoundStatementNode::NODE_KIND    |
            NopNode::NODE_KIND                  => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl SyntaxNodeTraversal for FunctionStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        match self.clone().value() {
            FunctionStatement::Var(s) => s.accept(visitor, ctx),
            FunctionStatement::Expr(s) => s.accept(visitor, ctx),
            FunctionStatement::For(s) => s.accept(visitor, ctx),
            FunctionStatement::While(s) => s.accept(visitor, ctx),
            FunctionStatement::DoWhile(s) => s.accept(visitor, ctx),
            FunctionStatement::If(s) => s.accept(visitor, ctx),
            FunctionStatement::Switch(s) => s.accept(visitor, ctx),
            FunctionStatement::Break(s) => s.accept(visitor, ctx),
            FunctionStatement::Continue(s) => s.accept(visitor, ctx),
            FunctionStatement::Return(s) => s.accept(visitor, ctx),
            FunctionStatement::Delete(s) => s.accept(visitor, ctx),
            FunctionStatement::Compound(s) => s.accept(visitor, ctx),
            FunctionStatement::Nop(s) => s.accept(visitor, ctx),
        }
    }
}



pub type BreakStatementNode<'script> = SyntaxNode<'script, tags::BreakStatement>;

impl NamedSyntaxNode for BreakStatementNode<'_> {
    const NODE_KIND: &'static str = "break_stmt";
}

impl BreakStatementNode<'_> {}

impl Debug for BreakStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BreakStatement {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for BreakStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for BreakStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        visitor.visit_break_stmt(self, ctx);
    }
}



pub type ContinueStatementNode<'script> = SyntaxNode<'script, tags::ContinueStatement>;

impl NamedSyntaxNode for ContinueStatementNode<'_> {
    const NODE_KIND: &'static str = "continue_stmt";
}

impl ContinueStatementNode<'_> {}

impl Debug for ContinueStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContinueStatement {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ContinueStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ContinueStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        visitor.visit_continue_stmt(self, ctx);
    }
}



pub type ReturnStatementNode<'script> = SyntaxNode<'script, tags::ReturnStatement>;

impl NamedSyntaxNode for ReturnStatementNode<'_> {
    const NODE_KIND: &'static str = "return_stmt";
}

impl<'script> ReturnStatementNode<'script> {
    pub fn value(&self) -> Option<ExpressionNode<'script>> {
        self.first_child(true).map(|n| n.into())
    }
}

impl Debug for ReturnStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("ReturnStatement {}", self.range().debug()))
            .field(&self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ReturnStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ReturnStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_return_stmt(self, ctx);
        if tp.traverse_value {
            ctx.push(TraversalContext::ReturnStatement);
            self.value().map(|expr| expr.accept(visitor, ctx));
            ctx.pop();
        }
        visitor.exit_return_stmt(self, ctx);
    }
}



pub type DeleteStatementNode<'script> = SyntaxNode<'script, tags::DeleteStatement>;

impl NamedSyntaxNode for DeleteStatementNode<'_> {
    const NODE_KIND: &'static str = "delete_stmt";
}

impl<'script> DeleteStatementNode<'script> {
    pub fn value(&self) -> ExpressionNode<'script> {
        self.first_child(true).unwrap().into()
    }
}

impl Debug for DeleteStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("DeleteStatement {}", self.range().debug()))
            .field(&self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for DeleteStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for DeleteStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_delete_stmt(self, ctx);
        if tp.traverse_value {
            ctx.push(TraversalContext::DeleteStatement);
            self.value().accept(visitor, ctx);
            ctx.pop();
        }
        visitor.exit_delete_stmt(self, ctx);
    }
}



pub type CompoundStatementNode<'script> = SyntaxNode<'script, tags::CompoundStatement>;

impl NamedSyntaxNode for CompoundStatementNode<'_> {
    const NODE_KIND: &'static str = "compound_stmt";
}

impl<'script> CompoundStatementNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = FunctionStatementNode<'script>> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for CompoundStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("CompoundStatement {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for CompoundStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for CompoundStatementNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_compound_stmt(self, ctx);
        if tp.traverse {
            ctx.push(TraversalContext::CompoundStatement);
            self.iter().for_each(|s| s.accept(visitor, ctx));
            ctx.pop();
        }
        visitor.exit_compound_stmt(self, ctx);
    }
}