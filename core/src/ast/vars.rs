use std::fmt::Debug;
use crate::{tokens::Identifier, SyntaxNode, NamedSyntaxNode, attribs::MemberVarSpecifier};
use super::{Expression, StatementTraversal, StatementVisitor};


#[derive(Debug, Clone)]
pub struct TypeAnnotation;

impl NamedSyntaxNode for TypeAnnotation {
    const NODE_NAME: &'static str = "type_annot";
}

impl SyntaxNode<'_, TypeAnnotation> {
    pub fn type_name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("type_name").unwrap().into()
    }

    pub fn generic_arg(&self) -> Option<SyntaxNode<'_, Identifier>> {
        self.field_child("generic_arg").map(|n| n.into())
    }
}

impl Debug for SyntaxNode<'_, TypeAnnotation> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeAnnotation")
            .field("type_name", &self.type_name())
            .field("generic_arg", &self.generic_arg())
            .finish()
    }
}


#[derive(Debug, Clone)]
pub struct VarDeclaration;

impl NamedSyntaxNode for VarDeclaration {
    const NODE_NAME: &'static str = "var_decl_stmt";
}

impl SyntaxNode<'_, VarDeclaration> {
    pub fn names(&self) -> impl Iterator<Item = SyntaxNode<'_, Identifier>> {
        self.field_children("names").map(|n| n.into())
    }

    pub fn var_type(&self) -> SyntaxNode<'_, TypeAnnotation> {
        self.field_child("var_type").unwrap().into()
    }

    pub fn init_value(&self) -> Option<SyntaxNode<'_, Expression>> {
        self.field_child("init_value").map(|c| c.into())
    }
}

impl Debug for SyntaxNode<'_, VarDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VarDeclaration")
            .field("names", &self.names().collect::<Vec<_>>())
            .field("var_type", &self.var_type())
            .field("init_value", &self.init_value())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, VarDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_local_var_decl_stmt(self);
    }
}


#[derive(Debug, Clone)]
pub struct MemberVarDeclaration;

impl NamedSyntaxNode for MemberVarDeclaration {
    const NODE_NAME: &'static str = "member_var_decl_stmt";
}

impl SyntaxNode<'_, MemberVarDeclaration> {
    pub fn specifiers(&self) -> impl Iterator<Item = SyntaxNode<'_, MemberVarSpecifier>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn names(&self) -> impl Iterator<Item = SyntaxNode<'_, Identifier>> {
        self.field_children("names").map(|n| n.into())
    }

    pub fn var_type(&self) -> SyntaxNode<'_, TypeAnnotation> {
        self.field_child("var_type").unwrap().into()
    }
}

impl Debug for SyntaxNode<'_, MemberVarDeclaration> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberVarDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("names", &self.names().collect::<Vec<_>>())
            .field("var_type", &self.var_type())
            .finish()
    }
}

impl StatementTraversal for SyntaxNode<'_, MemberVarDeclaration> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_member_var_decl(self);
    }
}