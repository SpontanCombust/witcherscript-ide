use super::*;


#[derive(Debug, Clone)]
pub enum RootStatement<'script> {
    Function(GlobalFunctionDeclarationNode<'script>),
    Class(ClassDeclarationNode<'script>),
    State(StateDeclarationNode<'script>),
    Struct(StructDeclarationNode<'script>),
    Enum(EnumDeclarationNode<'script>),
    Nop(NopNode<'script>)
}

pub type RootStatementNode<'script> = SyntaxNode<'script, RootStatement<'script>>;

impl<'script> RootStatementNode<'script> {
    pub fn value(self) -> RootStatement<'script> {
        let s = self.tree_node.kind();
        match s {
            GlobalFunctionDeclarationNode::NODE_KIND => RootStatement::Function(self.into()),
            ClassDeclarationNode::NODE_KIND => RootStatement::Class(self.into()),
            StateDeclarationNode::NODE_KIND => RootStatement::State(self.into()),
            StructDeclarationNode::NODE_KIND => RootStatement::Struct(self.into()),
            EnumDeclarationNode::NODE_KIND => RootStatement::Enum(self.into()),
            NopNode::NODE_KIND => RootStatement::Nop(self.into()),
            _ => panic!("Unknown script statement: {}", s)
        }
    }
}

impl Debug for RootStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.clone().value())
        } else {
            write!(f, "{:?}", self.clone().value())
        }
    }
}

impl<'script> TryFrom<AnyNode<'script>> for RootStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        match value.tree_node.kind() {
            GlobalFunctionDeclarationNode::NODE_KIND    |
            ClassDeclarationNode::NODE_KIND             |
            StateDeclarationNode::NODE_KIND             |
            StructDeclarationNode::NODE_KIND            |
            EnumDeclarationNode::NODE_KIND              |
            NopNode::NODE_KIND                          => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl StatementTraversal for RootStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        match self.clone().value() {
            RootStatement::Function(s) => s.accept(visitor),
            RootStatement::Class(s) => s.accept(visitor),
            RootStatement::State(s) => s.accept(visitor),
            RootStatement::Struct(s) => s.accept(visitor),
            RootStatement::Enum(s) => s.accept(visitor),
            RootStatement::Nop(s) => s.accept(visitor),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Root;

pub type RootNode<'script> = SyntaxNode<'script, Root>;

impl NamedSyntaxNode for RootNode<'_> {
    const NODE_KIND: &'static str = "script";
}

impl RootNode<'_> {
    pub fn statements(&self) -> impl Iterator<Item = RootStatementNode> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for RootNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stmts = self.statements().collect::<Vec<_>>();
        if f.alternate() {
            write!(f, "Script{:#?}", stmts)
        } else {
            write!(f, "Script{:?}", stmts)
        }
    }
}

impl<'script> TryFrom<AnyNode<'script>> for RootNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for RootNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_root(self) {
            self.statements().for_each(|s| s.accept(visitor));
        }
    }
}