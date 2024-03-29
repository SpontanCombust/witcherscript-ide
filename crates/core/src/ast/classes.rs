use std::fmt::Debug;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::*, attribs::*, AnyNode, DebugMaybeAlternate};
use super::*;


#[derive(Debug, Clone)]
pub struct ClassDeclaration;

pub type ClassDeclarationNode<'script> = SyntaxNode<'script, ClassDeclaration>;

impl NamedSyntaxNode for ClassDeclarationNode<'_> {
    const NODE_KIND: &'static str = "class_decl_stmt";
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

impl<'script> TryFrom<AnyNode<'script>> for ClassDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for ClassDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_class_decl(self) {
            self.definition().accept(visitor);
        }
        visitor.exit_class_decl(self);
    }
}



#[derive(Debug, Clone)]
pub struct ClassBlock;

pub type ClassBlockNode<'script> = SyntaxNode<'script, ClassBlock>;

impl NamedSyntaxNode for ClassBlockNode<'_> {
    const NODE_KIND: &'static str = "class_block";
}

impl ClassBlockNode<'_> {
    pub fn statements(&self) -> impl Iterator<Item = ClassStatementNode> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for ClassBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named("ClassBlock", &self.statements().collect::<Vec<_>>())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ClassBlockNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for ClassBlockNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        self.statements().for_each(|s| s.accept(visitor));
    }
}


#[derive(Clone)]
pub enum ClassStatement<'script> {
    Var(MemberVarDeclarationNode<'script>),
    Default(MemberDefaultValueNode<'script>),
    Hint(MemberHintNode<'script>),
    Autobind(AutobindDeclarationNode<'script>),
    Method(MemberFunctionDeclarationNode<'script>),
    Event(EventDeclarationNode<'script>),
    Nop(NopNode<'script>)
}

impl Debug for ClassStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(n) => f.debug_maybe_alternate(n),
            Self::Default(n) => f.debug_maybe_alternate(n),
            Self::Hint(n) => f.debug_maybe_alternate(n),
            Self::Autobind(n) => f.debug_maybe_alternate(n),
            Self::Method(n) => f.debug_maybe_alternate(n),
            Self::Event(n) => f.debug_maybe_alternate(n),
            Self::Nop(n) => f.debug_maybe_alternate(n),
        }
    }
} 

pub type ClassStatementNode<'script> = SyntaxNode<'script, ClassStatement<'script>>;

impl<'script> ClassStatementNode<'script> {
    pub fn value(self) -> ClassStatement<'script> {
        match self.tree_node.kind() {
            MemberVarDeclarationNode::NODE_KIND => ClassStatement::Var(self.into()),
            MemberDefaultValueNode::NODE_KIND => ClassStatement::Default(self.into()),
            MemberHintNode::NODE_KIND => ClassStatement::Hint(self.into()),
            AutobindDeclarationNode::NODE_KIND => ClassStatement::Autobind(self.into()),
            MemberFunctionDeclarationNode::NODE_KIND => ClassStatement::Method(self.into()),
            EventDeclarationNode::NODE_KIND => ClassStatement::Event(self.into()),
            NopNode::NODE_KIND => ClassStatement::Nop(self.into()),
            _ => panic!("Unknown class statement type: {}", self.tree_node.kind())
        }
    }
}

impl Debug for ClassStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.clone().value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ClassStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        match value.tree_node.kind() {
            MemberVarDeclarationNode::NODE_KIND         |
            MemberDefaultValueNode::NODE_KIND           |
            MemberHintNode::NODE_KIND                   |
            AutobindDeclarationNode::NODE_KIND          |
            MemberFunctionDeclarationNode::NODE_KIND    |
            EventDeclarationNode::NODE_KIND             |
            NopNode::NODE_KIND                          => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl StatementTraversal for ClassStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        match self.clone().value() {
            ClassStatement::Var(s) => s.accept(visitor),
            ClassStatement::Default(s) => s.accept(visitor),
            ClassStatement::Hint(s) => s.accept(visitor),
            ClassStatement::Autobind(s) => s.accept(visitor),
            ClassStatement::Method(s) => s.accept(visitor),
            ClassStatement::Event(s) => s.accept(visitor),
            ClassStatement::Nop(s) => s.accept(visitor),
        }
    }
}



#[derive(Debug, Clone)]
pub struct AutobindDeclaration;

pub type AutobindDeclarationNode<'script> = SyntaxNode<'script, AutobindDeclaration>;

impl NamedSyntaxNode for AutobindDeclarationNode<'_> {
    const NODE_KIND: &'static str = "class_autobind_stmt";
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

impl<'script> TryFrom<AnyNode<'script>> for AutobindDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for AutobindDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_autobind_decl(self);
    }
}



#[derive(Clone)]
pub enum AutobindValue<'script> {
    Single,
    Concrete(LiteralStringNode<'script>)
}

impl Debug for AutobindValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single => write!(f, "Single"),
            Self::Concrete(n) => f.debug_tuple("Concrete").field(n).finish(),
        }
    }
}

pub type AutobindValueNode<'script> = SyntaxNode<'script, AutobindValue<'script>>;

impl AutobindValueNode<'_> {
    pub fn value(&self) -> AutobindValue {
        let child = self.first_child(false).unwrap();
        let s = child.tree_node.kind();
        if s == LiteralStringNode::NODE_KIND {
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
        f.debug_maybe_alternate(&self.value())
    }
} 