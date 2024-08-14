use std::fmt::Debug;
use crate::{attribs::*, tokens::*, debug::*, AnyNode, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct TypeAnnotation;
    pub struct LocalVarDeclaration;
    pub struct MemberVarDeclaration;
    pub struct AutobindDeclaration;
    pub struct AutobindValueSingle;
}


pub type TypeAnnotationNode<'script> = SyntaxNode<'script, tags::TypeAnnotation>;

impl NamedSyntaxNode for TypeAnnotationNode<'_> {
    const NODE_KIND: &'static str = "type_annot";
}

impl<'script> TypeAnnotationNode<'script> {
    pub fn type_name(&self) -> IdentifierNode<'script> {
        self.field_child("type_name").unwrap().unsafe_into()
    }

    pub fn type_arg(&self) -> Option<TypeAnnotationNode<'script>> {
        self.field_child("type_arg").map(|n| n.unsafe_into())
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}



pub type LocalVarDeclarationNode<'script> = SyntaxNode<'script, tags::LocalVarDeclaration>;

impl NamedSyntaxNode for LocalVarDeclarationNode<'_> {
    const NODE_KIND: &'static str = "local_var_decl_stmt";
}

impl<'script> LocalVarDeclarationNode<'script> {
    pub fn names(&self) -> impl Iterator<Item = IdentifierNode<'script>> {
        self.field_children("names").map(|n| n.unsafe_into())
    }

    pub fn var_type(&self) -> TypeAnnotationNode<'script> {
        self.field_child("var_type").unwrap().unsafe_into()
    }

    pub fn init_value(&self) -> Option<ExpressionNode<'script>> {
        self.field_child("init_value").map(|c| c.unsafe_into())
    }
}

impl Debug for LocalVarDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("LocalVarDeclaration {}", self.range().debug()))
            .field("names", &self.names().collect::<Vec<_>>())
            .field("var_type", &self.var_type())
            .field("init_value", &self.init_value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for LocalVarDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for LocalVarDeclarationNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_local_var_decl_stmt(self, ctx);

        if tp.any() {
            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((init_value, Some("init_value"))) if tp.traverse_init_value => {
                        let init_value: ExpressionNode = init_value.unsafe_into();

                        ctx.push(TraversalContext::LocalVarDeclarationInitValue);
                        init_value.accept(visitor, ctx);
                        ctx.pop();
                    },
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }
        }

        visitor.exit_local_var_decl_stmt(self, ctx);
    }
}



pub type MemberVarDeclarationNode<'script> = SyntaxNode<'script, tags::MemberVarDeclaration>;

impl NamedSyntaxNode for MemberVarDeclarationNode<'_> {
    const NODE_KIND: &'static str = "member_var_decl";
}

impl<'script> MemberVarDeclarationNode<'script> {
    pub fn annotation(&self) -> Option<AnnotationNode<'script>> {
        self.field_child("annotation").map(|n| n.unsafe_into())
    }

    pub fn specifiers(&self) -> impl Iterator<Item = SpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.unsafe_into())
    }

    pub fn names(&self) -> impl Iterator<Item = IdentifierNode<'script>> {
        self.field_children("names").map(|n| n.unsafe_into())
    }

    pub fn var_type(&self) -> TypeAnnotationNode<'script> {
        self.field_child("var_type").unwrap().unsafe_into()
    }
}

impl Debug for MemberVarDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("MemberVarDeclaration {}", self.range().debug()))
            .field("annotation", &self.annotation())
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for MemberVarDeclarationNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        // closure to avoid code repetition below
        let accept_proper = |self_: &Self, visitor: &mut V, ctx: &mut TraversalContextStack, tp: MemberVarDeclarationTraversalPolicy| {
            for ch in self_.children_detailed().must_be_named(true) {
                match ch {
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx);
                    },
                    _ => {}
                }
            }
        };

        if ctx.top() == TraversalContext::Global {
            let tp = visitor.visit_global_var_decl(self);

            accept_proper(self, visitor, ctx, tp);

            visitor.exit_global_var_decl(self);
        } else {
            let tp = visitor.visit_member_var_decl(self, ctx);

            accept_proper(self, visitor, ctx, tp);

            visitor.exit_member_var_decl(self, ctx);
        }
    }
}



pub type AutobindDeclarationNode<'script> = SyntaxNode<'script, tags::AutobindDeclaration>;

impl NamedSyntaxNode for AutobindDeclarationNode<'_> {
    const NODE_KIND: &'static str = "autobind_decl";
}

impl<'script> AutobindDeclarationNode<'script> {
    pub fn specifiers(&self) -> impl Iterator<Item = SpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.unsafe_into())
    }

    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().unsafe_into()
    }

    pub fn autobind_type(&self) -> TypeAnnotationNode<'script> {
        self.field_child("autobind_type").unwrap().unsafe_into()
    }

    pub fn value(&self) -> AutobindValue<'script> {
        let n = self.field_child("value").unwrap();
        let kind = n.tree_node.kind();
        match kind {
            AutobindValueSingleNode::NODE_KIND => AutobindValue::Single(n.unsafe_into()),
            LiteralStringNode::NODE_KIND => AutobindValue::Concrete(n.unsafe_into()),
            _ => panic!("Unknown autobind value kind: {} {}", kind, self.range().debug())
        }
    }
}

impl Debug for AutobindDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("AutobindDeclaration {}", self.range().debug()))
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for AutobindDeclarationNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_autobind_decl(self, ctx);

        if tp.any() {
            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Err(e) => {
                        e.accept(visitor, ctx)
                    },
                    _ => {}
                }
            }
        }

        visitor.exit_autobind_decl(self, ctx);
    }
}


#[derive(Clone)]
pub enum AutobindValue<'script> {
    Single(AutobindValueSingleNode<'script>),
    Concrete(LiteralStringNode<'script>)
}

impl Debug for AutobindValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(n) => f.debug_maybe_alternate(n),
            Self::Concrete(n) => f.debug_maybe_alternate(n)
        }
    }
}


pub type AutobindValueSingleNode<'script> = SyntaxNode<'script, tags::AutobindValueSingle>;

impl NamedSyntaxNode for AutobindValueSingleNode<'_> {
    const NODE_KIND: &'static str = "autobind_single";
}

impl AutobindValueSingleNode<'_> {}

impl Debug for AutobindValueSingleNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Single {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for AutobindValueSingleNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}