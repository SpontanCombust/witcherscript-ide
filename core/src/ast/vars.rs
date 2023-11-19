use crate::{tokens::{Identifier, Keyword}, SyntaxNode, NamedSyntaxNode};
use super::{classes::AccessModifier, expressions::Expression};
use std::str::FromStr;

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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberVarSpecifier {
    AccessModifier(AccessModifier),
    Const,
    Editable,
    Import,
    Inlined,
    Saved,
}

impl NamedSyntaxNode for MemberVarSpecifier {
    const NODE_NAME: &'static str = "member_var_specifier";
}

impl SyntaxNode<'_, MemberVarSpecifier> {
    pub fn value(&self) -> MemberVarSpecifier {
        let s = self.tree_node.kind();
        if let Ok(k) = Keyword::from_str(s)  {
            match k {
                Keyword::Private => return MemberVarSpecifier::AccessModifier(AccessModifier::Private),
                Keyword::Protected => return MemberVarSpecifier::AccessModifier(AccessModifier::Protected),
                Keyword::Public => return MemberVarSpecifier::AccessModifier(AccessModifier::Public),
                Keyword::Const => return MemberVarSpecifier::Const,
                Keyword::Editable => return MemberVarSpecifier::Editable,
                Keyword::Import => return MemberVarSpecifier::Import,
                Keyword::Inlined => return MemberVarSpecifier::Inlined,
                Keyword::Saved => return MemberVarSpecifier::Saved,
                _ => {}
            }
        }

        panic!("Unknown member var specifier: {}", s)
    }
}