use std::fmt::Debug;
use crate::{attribs::MemberVarSpecifierNode, tokens::IdentifierNode, AnyNode, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::{SyntaxTraversal, SyntaxVisitor, ExpressionNode};


mod tags {
    pub struct TypeAnnotation;
    pub struct VarDeclaration;
    pub struct MemberVarDeclaration;
}


pub type TypeAnnotationNode<'script> = SyntaxNode<'script, tags::TypeAnnotation>;

impl NamedSyntaxNode for TypeAnnotationNode<'_> {
    const NODE_KIND: &'static str = "type_annot";
}

impl<'script> TypeAnnotationNode<'script> {
    pub fn type_name(&self) -> IdentifierNode<'script> {
        self.field_child("type_name").unwrap().into()
    }

    pub fn type_arg(&self) -> Option<TypeAnnotationNode<'script>> {
        self.field_child("type_arg").map(|n| n.into())
    }
}

impl Debug for TypeAnnotationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("TypeAnnotation {}", self.range().debug()))
            .field("type_name", &self.type_name())
            .field("type_arg", &self.type_arg())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for TypeAnnotationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}



pub type VarDeclarationNode<'script> = SyntaxNode<'script, tags::VarDeclaration>;

impl NamedSyntaxNode for VarDeclarationNode<'_> {
    const NODE_KIND: &'static str = "var_decl_stmt";
}

impl<'script> VarDeclarationNode<'script> {
    pub fn names(&self) -> impl Iterator<Item = IdentifierNode<'script>> {
        self.field_children("names").map(|n| n.into())
    }

    pub fn var_type(&self) -> TypeAnnotationNode<'script> {
        self.field_child("var_type").unwrap().into()
    }

    pub fn init_value(&self) -> Option<ExpressionNode<'script>> {
        self.field_child("init_value").map(|c| c.into())
    }
}

impl Debug for VarDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("VarDeclaration {}", self.range().debug()))
            .field("names", &self.names().collect::<Vec<_>>())
            .field("var_type", &self.var_type())
            .field("init_value", &self.init_value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for VarDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxTraversal for VarDeclarationNode<'_> {
    fn accept<V: SyntaxVisitor>(&self, visitor: &mut V) {
        visitor.visit_local_var_decl_stmt(self);
    }
}



pub type MemberVarDeclarationNode<'script> = SyntaxNode<'script, tags::MemberVarDeclaration>;

impl NamedSyntaxNode for MemberVarDeclarationNode<'_> {
    const NODE_KIND: &'static str = "member_var_decl_stmt";
}

impl<'script> MemberVarDeclarationNode<'script> {
    pub fn specifiers(&self) -> impl Iterator<Item = MemberVarSpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn names(&self) -> impl Iterator<Item = IdentifierNode<'script>> {
        self.field_children("names").map(|n| n.into())
    }

    pub fn var_type(&self) -> TypeAnnotationNode<'script> {
        self.field_child("var_type").unwrap().into()
    }
}

impl Debug for MemberVarDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("MemberVarDeclaration {}", self.range().debug()))
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("names", &self.names().collect::<Vec<_>>())
            .field("var_type", &self.var_type())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberVarDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxTraversal for MemberVarDeclarationNode<'_> {
    fn accept<V: SyntaxVisitor>(&self, visitor: &mut V) {
        visitor.visit_member_var_decl(self);
    }
}