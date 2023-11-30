use std::fmt::Debug;
use crate::{NamedSyntaxNode, SyntaxNode, tokens::*, attribs::*};
use super::*;


#[derive(Debug, Clone)]
pub struct ClassDeclaration;

impl NamedSyntaxNode for ClassDeclaration {
    const NODE_NAME: &'static str = "class_decl_stmt";
}

impl SyntaxNode<'_, ClassDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, ClassSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn base(&self) -> Option<SyntaxNode<'_, Identifier>> {
        self.field_child("base").map(|n| n.into())
    }

    pub fn definition(&self) -> SyntaxNode<'_, ClassBlock> {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, ClassDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("name", &self.name())
            .field("base", &self.base())
            .field("definition", &self.definition())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, ClassDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_class_decl(self);
        if visitor.should_visit_inner() {
            self.definition().statements().for_each(|s| s.accept(visitor));
        }
    }
}



#[derive(Debug, Clone)]
pub struct ClassBlock;

impl NamedSyntaxNode for ClassBlock {
    const NODE_NAME: &'static str = "class_block";
}

impl SyntaxNode<'_, ClassBlock> {
    pub fn statements(&self) -> impl Iterator<Item = SyntaxNode<'_, ClassStatement>> {
        self.children(true).map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, ClassBlock> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassBlock{:?}", self.statements().collect::<Vec<_>>())
    }
}


#[derive(Debug, Clone)]
pub enum ClassStatement<'script> {
    Var(SyntaxNode<'script, MemberVarDeclaration>),
    Default(SyntaxNode<'script, MemberDefaultValue>),
    Hint(SyntaxNode<'script, MemberHint>),
    Autobind(SyntaxNode<'script, AutobindDeclaration>),
    Method(SyntaxNode<'script, MemberFunctionDeclaration>),
    Event(SyntaxNode<'script, EventDeclaration>),
    Nop
}

impl SyntaxNode<'_, ClassStatement<'_>> {
    pub fn value(&self) -> ClassStatement {
        match self.tree_node.kind() {
            MemberVarDeclaration::NODE_NAME => ClassStatement::Var(self.clone().into()),
            MemberDefaultValue::NODE_NAME => ClassStatement::Default(self.clone().into()),
            MemberHint::NODE_NAME => ClassStatement::Hint(self.clone().into()),
            AutobindDeclaration::NODE_NAME => ClassStatement::Autobind(self.clone().into()),
            MemberFunctionDeclaration::NODE_NAME => ClassStatement::Method(self.clone().into()),
            EventDeclaration::NODE_NAME => ClassStatement::Event(self.clone().into()),
            Nop::NODE_NAME => ClassStatement::Nop,
            _ => panic!("Unknown class statement type: {}", self.tree_node.kind())
        }
    }
}

impl Debug for SyntaxNode<'_, ClassStatement<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}

impl StatementTraversal for SyntaxNode<'_, ClassStatement<'_>> {
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

impl NamedSyntaxNode for AutobindDeclaration {
    const NODE_NAME: &'static str = "class_autobind_stmt";
}

impl SyntaxNode<'_, AutobindDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, AutobindSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn autobind_type(&self) -> SyntaxNode<'_, TypeAnnotation> {
        self.field_child("autobind_type").unwrap().into()
    }

    pub fn value(&self) -> SyntaxNode<'_, AutobindValue> {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, AutobindDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutobindDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("name", &self.name())
            .field("autobind_type", &self.autobind_type())
            .field("value", &self.value())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, AutobindDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_autobind_decl(self);
    }
}



#[derive(Debug, Clone)]
pub enum AutobindValue<'script> {
    Single,
    Concrete(SyntaxNode<'script, LiteralString>)
}

impl SyntaxNode<'_, AutobindValue<'_>> {
    pub fn value(&self) -> AutobindValue {
        let child = self.first_child(false).unwrap();
        let s = child.tree_node.kind();
        if s == LiteralString::NODE_NAME {
            return AutobindValue::Concrete(child.into());
        } else if s == "single" {
            return AutobindValue::Single;
        } else {
            panic!("Unknown autobind value type: {}", s);
        }
    }
} 

impl Debug for SyntaxNode<'_, AutobindValue<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
} 