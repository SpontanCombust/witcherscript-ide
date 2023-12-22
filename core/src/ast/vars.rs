use std::fmt::Debug;
use crate::{tokens::IdentifierNode, SyntaxNode, NamedSyntaxNode, attribs::MemberVarSpecifierNode};
use super::{StatementTraversal, StatementVisitor, ExpressionNode};


#[derive(Debug, Clone)]
pub struct TypeAnnotation;

pub type TypeAnnotationNode<'script> = SyntaxNode<'script, TypeAnnotation>;

impl NamedSyntaxNode for TypeAnnotationNode<'_> {
    const NODE_NAME: &'static str = "type_annot";
}

impl TypeAnnotationNode<'_> {
    pub fn type_name(&self) -> IdentifierNode {
        self.field_child("type_name").unwrap().into()
    }

    pub fn type_arg(&self) -> Option<TypeAnnotationNode> {
        self.field_child("type_arg").map(|n| n.into())
    }
}

impl Debug for TypeAnnotationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeAnnotation")
            .field("type_name", &self.type_name())
            .field("type_arg", &self.type_arg())
            .finish()
    }
}


#[derive(Debug, Clone)]
pub struct VarDeclaration;

pub type VarDeclarationNode<'script> = SyntaxNode<'script, VarDeclaration>;

impl NamedSyntaxNode for VarDeclarationNode<'_> {
    const NODE_NAME: &'static str = "var_decl_stmt";
}

impl VarDeclarationNode<'_> {
    pub fn names(&self) -> impl Iterator<Item = IdentifierNode> {
        self.field_children("names").map(|n| n.into())
    }

    pub fn var_type(&self) -> TypeAnnotationNode {
        self.field_child("var_type").unwrap().into()
    }

    pub fn init_value(&self) -> Option<ExpressionNode> {
        self.field_child("init_value").map(|c| c.into())
    }
}

impl Debug for VarDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VarDeclaration")
            .field("names", &self.names().collect::<Vec<_>>())
            .field("var_type", &self.var_type())
            .field("init_value", &self.init_value())
            .finish()
    }
}

impl StatementTraversal for VarDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_local_var_decl_stmt(self);
    }
}


#[derive(Debug, Clone)]
pub struct MemberVarDeclaration;

pub type MemberVarDeclarationNode<'script> = SyntaxNode<'script, MemberVarDeclaration>;

impl NamedSyntaxNode for MemberVarDeclarationNode<'_> {
    const NODE_NAME: &'static str = "member_var_decl_stmt";
}

impl MemberVarDeclarationNode<'_> {
    pub fn specifiers(&self) -> impl Iterator<Item = MemberVarSpecifierNode> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn names(&self) -> impl Iterator<Item = IdentifierNode> {
        self.field_children("names").map(|n| n.into())
    }

    pub fn var_type(&self) -> TypeAnnotationNode {
        self.field_child("var_type").unwrap().into()
    }
}

impl Debug for MemberVarDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberVarDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("names", &self.names().collect::<Vec<_>>())
            .field("var_type", &self.var_type())
            .finish()
    }
}

impl StatementTraversal for MemberVarDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_member_var_decl(self);
    }
}