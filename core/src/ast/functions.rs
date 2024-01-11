use std::fmt::Debug;
use crate::{SyntaxNode, NamedSyntaxNode, tokens::IdentifierNode, attribs::*, AnyNode};
use super::*;


#[derive(Debug, Clone)]
pub struct EventDeclaration;

pub type EventDeclarationNode<'script> = SyntaxNode<'script, EventDeclaration>;

impl NamedSyntaxNode for EventDeclarationNode<'_> {
    const NODE_KIND: &'static str = "event_decl_stmt";
}

impl EventDeclarationNode<'_> {
    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn params(&self) -> impl Iterator<Item = FunctionParameterGroupNode> {
        self.field_children("params").map(|n| n.into())
    }

    pub fn definition(&self) -> Option<FunctionBlockNode> {
        self.field_child("definition").map(|n| n.into())
    }
}

impl Debug for EventDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventDeclaration")
            .field("name", &self.name())
            .field("params", &self.params().collect::<Vec<_>>())
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

impl StatementTraversal for EventDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_event_decl(self) {
            self.params().for_each(|p| p.accept(visitor));
            self.definition().map(|s| s.accept(visitor));
        }
        visitor.exit_event_decl(self);
    }
}



#[derive(Debug, Clone)]
pub struct GlobalFunctionDeclaration;

pub type GlobalFunctionDeclarationNode<'script> = SyntaxNode<'script, GlobalFunctionDeclaration>;

impl NamedSyntaxNode for GlobalFunctionDeclarationNode<'_> {
    const NODE_KIND: &'static str = "global_func_decl_stmt";
}

impl GlobalFunctionDeclarationNode<'_> {
    pub fn specifiers(&self) -> impl Iterator<Item = GlobalFunctionSpecifierNode> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn flavour(&self) -> Option<GlobalFunctionFlavourNode> {
        self.field_child("flavour").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn params(&self) -> impl Iterator<Item = FunctionParameterGroupNode> {
        self.field_children("params").map(|n| n.into())
    }

    pub fn return_type(&self) -> Option<TypeAnnotationNode> {
        self.field_child("return_type").map(|n| n.into())
    }

    pub fn definition(&self) -> Option<FunctionBlockNode> {
        self.field_child("definition").map(|n| n.into())
    }
}

impl Debug for GlobalFunctionDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalFunctionDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("flavour", &self.flavour())
            .field("name", &self.name())
            .field("params", &self.params().collect::<Vec<_>>())
            .field("return_type", &self.return_type())
            .field("definition", &self.definition())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for GlobalFunctionDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for GlobalFunctionDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_global_func_decl(self) {
            self.params().for_each(|p| p.accept(visitor));
            self.definition().map(|s| s.accept(visitor));
        }
        visitor.exit_global_func_decl(self);
    }
}



#[derive(Debug, Clone)]
pub struct MemberFunctionDeclaration;

pub type MemberFunctionDeclarationNode<'script> = SyntaxNode<'script, MemberFunctionDeclaration>;

impl NamedSyntaxNode for MemberFunctionDeclarationNode<'_> {
    const NODE_KIND: &'static str = "member_func_decl_stmt";
}

impl MemberFunctionDeclarationNode<'_> {
    pub fn specifiers(&self) -> impl Iterator<Item = MemberFunctionSpecifierNode> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn flavour(&self) -> Option<MemberFunctionFlavourNode> {
        self.field_child("flavour").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn params(&self) -> impl Iterator<Item = FunctionParameterGroupNode> {
        self.field_children("params").map(|n| n.into())
    }

    pub fn return_type(&self) -> Option<TypeAnnotationNode> {
        self.field_child("return_type").map(|n| n.into())
    }

    pub fn definition(&self) -> Option<FunctionBlockNode> {
        self.field_child("definition").map(|n| n.into())
    }
}

impl Debug for MemberFunctionDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberFunctionDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("flavour", &self.flavour())
            .field("name", &self.name())
            .field("params", &self.params().collect::<Vec<_>>())
            .field("return_type", &self.return_type())
            .field("definition", &self.definition())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberFunctionDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for MemberFunctionDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_member_func_decl(self) {
            self.params().for_each(|p| p.accept(visitor));
            self.definition().map(|s| s.accept(visitor));
        }
        visitor.exit_member_func_decl(self);
    }
}




#[derive(Debug, Clone)]
pub struct FunctionParameterGroup;

pub type FunctionParameterGroupNode<'script> = SyntaxNode<'script, FunctionParameterGroup>;

impl NamedSyntaxNode for FunctionParameterGroupNode<'_> {
    const NODE_KIND: &'static str = "func_param_group";
}

impl FunctionParameterGroupNode<'_> {
    pub fn specifiers(&self) -> impl Iterator<Item = FunctionParameterSpecifierNode> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn names(&self) -> impl Iterator<Item = IdentifierNode> {
        self.field_children("names").map(|n| n.into())
    }

    pub fn param_type(&self) -> TypeAnnotationNode {
        self.field_child("param_type").unwrap().into()
    }
}

impl Debug for FunctionParameterGroupNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionParameterGroup")
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

impl StatementTraversal for FunctionParameterGroupNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_func_param_group(self);
    }
}



#[derive(Debug, Clone)]
pub enum FunctionStatement<'script> {
    Var(VarDeclarationNode<'script>),
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
    Block(FunctionBlockNode<'script>),
    Nop(NopNode<'script>),
}

pub type FunctionStatementNode<'script> = SyntaxNode<'script, FunctionStatement<'script>>;

impl FunctionStatementNode<'_> {
    pub fn value(&self) -> FunctionStatement {
        match self.tree_node.kind() {
            VarDeclarationNode::NODE_KIND => FunctionStatement::Var(self.clone().into()),
            ExpressionStatementNode::NODE_KIND => FunctionStatement::Expr(self.clone().into()),
            ForLoopNode::NODE_KIND => FunctionStatement::For(self.clone().into()),
            WhileLoopNode::NODE_KIND => FunctionStatement::While(self.clone().into()),
            DoWhileLoopNode::NODE_KIND => FunctionStatement::DoWhile(self.clone().into()),
            IfConditionalNode::NODE_KIND => FunctionStatement::If(self.clone().into()),
            SwitchConditionalNode::NODE_KIND => FunctionStatement::Switch(self.clone().into()),
            BreakStatementNode::NODE_KIND => FunctionStatement::Break(self.clone().into()),
            ContinueStatementNode::NODE_KIND => FunctionStatement::Continue(self.clone().into()),
            ReturnStatementNode::NODE_KIND => FunctionStatement::Return(self.clone().into()),
            DeleteStatementNode::NODE_KIND => FunctionStatement::Delete(self.clone().into()),
            FunctionBlockNode::NODE_KIND => FunctionStatement::Block(self.clone().into()),
            NopNode::NODE_KIND => FunctionStatement::Nop(self.clone().into()),
            _ => panic!("Unknown function statement type: {}", self.tree_node.kind())
        }
    }
}

impl Debug for FunctionStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}

impl<'script> TryFrom<AnyNode<'script>> for FunctionStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        match value.tree_node.kind() {
            VarDeclarationNode::NODE_KIND       |
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
            FunctionBlockNode::NODE_KIND        |
            NopNode::NODE_KIND                  => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl StatementTraversal for FunctionStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        match self.value() {
            FunctionStatement::Var(s) => s.accept(visitor),
            FunctionStatement::Expr(s) => s.accept(visitor),
            FunctionStatement::For(s) => s.accept(visitor),
            FunctionStatement::While(s) => s.accept(visitor),
            FunctionStatement::DoWhile(s) => s.accept(visitor),
            FunctionStatement::If(s) => s.accept(visitor),
            FunctionStatement::Switch(s) => s.accept(visitor),
            FunctionStatement::Break(s) => s.accept(visitor),
            FunctionStatement::Continue(s) => s.accept(visitor),
            FunctionStatement::Return(s) => s.accept(visitor),
            FunctionStatement::Delete(s) => s.accept(visitor),
            FunctionStatement::Block(s) => s.accept(visitor),
            FunctionStatement::Nop(s) => s.accept(visitor),
        }
    }
}


#[derive(Debug, Clone)]
pub struct FunctionBlock;

pub type FunctionBlockNode<'script> = SyntaxNode<'script, FunctionBlock>;

impl NamedSyntaxNode for FunctionBlockNode<'_> {
    const NODE_KIND: &'static str = "func_block";
}

impl FunctionBlockNode<'_> {
    pub fn statements(&self) -> impl Iterator<Item = FunctionStatementNode> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for FunctionBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stmts = self.statements().collect::<Vec<_>>();
        if f.alternate() {
            write!(f, "FunctionBlock{:#?}", stmts)
        } else {
            write!(f, "FunctionBlock{:?}", stmts)
        }
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

impl StatementTraversal for FunctionBlockNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        self.statements().for_each(|s| s.accept(visitor));
    }
}



#[derive(Debug, Clone)]
pub struct BreakStatement;

pub type BreakStatementNode<'script> = SyntaxNode<'script, BreakStatement>;

impl NamedSyntaxNode for BreakStatementNode<'_> {
    const NODE_KIND: &'static str = "break_stmt";
}

impl BreakStatementNode<'_> {}

impl Debug for BreakStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BreakStatement")
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

impl StatementTraversal for BreakStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_break_stmt(self);
    }
}



#[derive(Debug, Clone)]
pub struct ContinueStatement;

pub type ContinueStatementNode<'script> = SyntaxNode<'script, ContinueStatement>;

impl NamedSyntaxNode for ContinueStatementNode<'_> {
    const NODE_KIND: &'static str = "continue_stmt";
}

impl ContinueStatementNode<'_> {}

impl Debug for ContinueStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContinueStatement")
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

impl StatementTraversal for ContinueStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_continue_stmt(self);
    }
}



#[derive(Debug, Clone)]
pub struct ReturnStatement;

pub type ReturnStatementNode<'script> = SyntaxNode<'script, ReturnStatement>;

impl NamedSyntaxNode for ReturnStatementNode<'_> {
    const NODE_KIND: &'static str = "return_stmt";
}

impl ReturnStatementNode<'_> {
    pub fn value(&self) -> Option<ExpressionNode> {
        self.first_child(true).map(|n| n.into())
    }
}

impl Debug for ReturnStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReturnStatement")
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

impl StatementTraversal for ReturnStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_return_stmt(self);
    }
}



#[derive(Debug, Clone)]
pub struct DeleteStatement;

pub type DeleteStatementNode<'script> = SyntaxNode<'script, DeleteStatement>;

impl NamedSyntaxNode for DeleteStatementNode<'_> {
    const NODE_KIND: &'static str = "delete_stmt";
}

impl DeleteStatementNode<'_> {
    pub fn value(&self) -> ExpressionNode {
        self.first_child(true).unwrap().into()
    }
}

impl Debug for DeleteStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DeleteStatement")
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

impl StatementTraversal for DeleteStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_delete_stmt(self);
    }
}