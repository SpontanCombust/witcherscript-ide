use std::fmt::Debug;
use crate::{tokens::{Identifier, LiteralString}, NamedSyntaxNode, SyntaxNode, attribs::StructSpecifier};
use super::{MemberVarDeclaration, Expression, Nop, StatementTraversal, StatementVisitor};


#[derive(Debug, Clone)]
pub struct StructDeclaration;

impl NamedSyntaxNode for StructDeclaration {
    const NODE_NAME: &'static str = "struct_decl_stmt";
}

impl SyntaxNode<'_, StructDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, StructSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("name").unwrap().into()
    }

    pub fn definition(&self) -> SyntaxNode<'_, StructBlock> {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, StructDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("name", &self.name())
            .field("definition", &self.definition())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, StructDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_struct_decl(self) {
            self.definition().statements().for_each(|s| s.accept(visitor));
        }
        visitor.exit_struct_decl(self);
    }
}



#[derive(Debug, Clone)]
pub struct StructBlock;

impl NamedSyntaxNode for StructBlock {
    const NODE_NAME: &'static str = "struct_block";
}

impl SyntaxNode<'_, StructBlock> {
    pub fn statements(&self) -> impl Iterator<Item = SyntaxNode<'_, StructStatement>> {
        self.children(true).map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, StructBlock> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StructBlock{:?}", self.statements().collect::<Vec<_>>())
    }
}



#[derive(Debug, Clone)]
pub enum StructStatement<'script> {
    Var(SyntaxNode<'script, MemberVarDeclaration>),
    Default(SyntaxNode<'script, MemberDefaultValue>),
    Hint(SyntaxNode<'script, MemberHint>),
    Nop
}

impl SyntaxNode<'_, StructStatement<'_>> {
    pub fn value(&self) -> StructStatement {
        match self.tree_node.kind() {
            MemberVarDeclaration::NODE_NAME => StructStatement::Var(self.clone().into()),
            MemberDefaultValue::NODE_NAME => StructStatement::Default(self.clone().into()),
            MemberHint::NODE_NAME => StructStatement::Hint(self.clone().into()),
            Nop::NODE_NAME => StructStatement::Nop,
            _ => panic!("Unknown struct statement type: {}", self.tree_node.kind())
        }
    }
}

impl Debug for SyntaxNode<'_, StructStatement<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}

impl StatementTraversal for SyntaxNode<'_, StructStatement<'_>> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        match self.value() {
            StructStatement::Var(s) => s.accept(visitor),
            StructStatement::Default(s) => s.accept(visitor),
            StructStatement::Hint(s) => s.accept(visitor),
            StructStatement::Nop => visitor.visit_nop_stmt(),
        }
    }
}



#[derive(Debug, Clone)]
pub struct MemberDefaultValue; 

impl NamedSyntaxNode for MemberDefaultValue {
    const NODE_NAME: &'static str = "member_default_val_stmt";
}

impl SyntaxNode<'_, MemberDefaultValue> {
    pub fn member(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("member").unwrap().into()
    }

    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, MemberDefaultValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberDefaultValue")
            .field("member", &self.member())
            .field("value", &self.value())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, MemberDefaultValue> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_member_default_val(self);
    }
}



#[derive(Debug, Clone)]
pub struct MemberHint; 

impl NamedSyntaxNode for MemberHint {
    const NODE_NAME: &'static str = "member_hint_stmt";
}

impl SyntaxNode<'_, MemberHint> {
    pub fn member(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("member").unwrap().into()
    }

    pub fn value(&self) -> SyntaxNode<'_, LiteralString> {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, MemberHint> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberHint")
            .field("member", &self.member())
            .field("value", &self.value())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, MemberHint> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_member_hint(self);
    }
}
