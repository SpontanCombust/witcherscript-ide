use std::fmt::Debug;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::*, attribs::*};
use super::*;


#[derive(Debug, Clone)]
pub struct ClassDeclaration;

pub type ClassDeclarationNode<'script> = SyntaxNode<'script, ClassDeclaration>;

impl NamedSyntaxNode for ClassDeclarationNode<'_> {
    const NODE_NAME: &'static str = "class_decl_stmt";
}

impl ClassDeclarationNode<'_> {
    pub fn specifiers(&self) -> impl Iterator<Item = ClassSpecifierNode> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn base(&self) -> Option<IdentifierNode> {
        self.field_child("base").map(|n| n.into())
    }

    pub fn definition(&self) -> ClassBlockNode {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for ClassDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("name", &self.name())
            .field("base", &self.base())
            .field("definition", &self.definition())
            .finish()
    }
}

impl StatementTraversal for ClassDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_class_decl(self) {
            self.definition().statements().for_each(|s| s.accept(visitor));
        }
        visitor.exit_class_decl(self);
    }
}



#[derive(Debug, Clone)]
pub struct ClassBlock;

pub type ClassBlockNode<'script> = SyntaxNode<'script, ClassBlock>;

impl NamedSyntaxNode for ClassBlockNode<'_> {
    const NODE_NAME: &'static str = "class_block";
}

impl ClassBlockNode<'_> {
    pub fn statements(&self) -> impl Iterator<Item = ClassStatementNode> {
        self.children(true).map(|n| n.into())
    }
}

impl Debug for ClassBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stmts = self.statements().collect::<Vec<_>>();
        if f.alternate() {
            write!(f, "ClassBlock{:#?}", stmts)
        } else {
            write!(f, "ClassBlock{:?}", stmts)
        }
    }
}


#[derive(Debug, Clone)]
pub enum ClassStatement<'script> {
    Var(MemberVarDeclarationNode<'script>),
    Default(MemberDefaultValueNode<'script>),
    Hint(MemberHintNode<'script>),
    Autobind(AutobindDeclarationNode<'script>),
    Method(MemberFunctionDeclarationNode<'script>),
    Event(EventDeclarationNode<'script>),
    Nop
}

pub type ClassStatementNode<'script> = SyntaxNode<'script, ClassStatement<'script>>;

impl ClassStatementNode<'_> {
    pub fn value(&self) -> ClassStatement {
        match self.tree_node.kind() {
            MemberVarDeclarationNode::NODE_NAME => ClassStatement::Var(self.clone().into()),
            MemberDefaultValueNode::NODE_NAME => ClassStatement::Default(self.clone().into()),
            MemberHintNode::NODE_NAME => ClassStatement::Hint(self.clone().into()),
            AutobindDeclarationNode::NODE_NAME => ClassStatement::Autobind(self.clone().into()),
            MemberFunctionDeclarationNode::NODE_NAME => ClassStatement::Method(self.clone().into()),
            EventDeclarationNode::NODE_NAME => ClassStatement::Event(self.clone().into()),
            NopNode::NODE_NAME => ClassStatement::Nop,
            _ => panic!("Unknown class statement type: {}", self.tree_node.kind())
        }
    }
}

impl Debug for ClassStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}

impl StatementTraversal for ClassStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        match self.value() {
            ClassStatement::Var(s) => s.accept(visitor),
            ClassStatement::Default(s) => s.accept(visitor),
            ClassStatement::Hint(s) => s.accept(visitor),
            ClassStatement::Autobind(s) => s.accept(visitor),
            ClassStatement::Method(s) => s.accept(visitor),
            ClassStatement::Event(s) => s.accept(visitor),
            ClassStatement::Nop => visitor.visit_nop_stmt(),
        }
    }
}



#[derive(Debug, Clone)]
pub struct AutobindDeclaration;

pub type AutobindDeclarationNode<'script> = SyntaxNode<'script, AutobindDeclaration>;

impl NamedSyntaxNode for AutobindDeclarationNode<'_> {
    const NODE_NAME: &'static str = "class_autobind_stmt";
}

impl AutobindDeclarationNode<'_> {
    pub fn specifiers(&self) -> impl Iterator<Item = AutobindSpecifierNode> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn autobind_type(&self) -> TypeAnnotationNode {
        self.field_child("autobind_type").unwrap().into()
    }

    pub fn value(&self) -> AutobindValueNode {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for AutobindDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutobindDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("name", &self.name())
            .field("autobind_type", &self.autobind_type())
            .field("value", &self.value())
            .finish()
    }
}

impl StatementTraversal for AutobindDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_autobind_decl(self);
    }
}



#[derive(Debug, Clone)]
pub enum AutobindValue<'script> {
    Single,
    Concrete(LiteralStringNode<'script>)
}

pub type AutobindValueNode<'script> = SyntaxNode<'script, AutobindValue<'script>>;

impl AutobindValueNode<'_> {
    pub fn value(&self) -> AutobindValue {
        let child = self.first_child(false).unwrap();
        let s = child.tree_node.kind();
        if s == LiteralStringNode::NODE_NAME {
            return AutobindValue::Concrete(child.into());
        } else if s == Keyword::Single.as_ref() {
            return AutobindValue::Single;
        } else {
            panic!("Unknown autobind value type: {}", s);
        }
    }
} 

impl Debug for AutobindValueNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
} 