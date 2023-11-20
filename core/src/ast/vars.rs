use crate::{tokens::Identifier, SyntaxNode, NamedSyntaxNode, attribs::MemberVarSpecifier};
use super::Expression;


#[derive(Debug, Clone)]
pub struct TypeAnnotation;

impl NamedSyntaxNode for TypeAnnotation {
    const NODE_NAME: &'static str = "type_annot";
}

impl SyntaxNode<'_, TypeAnnotation> {
    pub fn type_name(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("type_name").unwrap().into()
    }

    pub fn generic_arg(&self) -> SyntaxNode<'_, Identifier> {
        self.field_child("generic_arg").unwrap().into()
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
