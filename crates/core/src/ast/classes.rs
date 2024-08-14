use std::fmt::Debug;
use crate::{attribs::*, tokens::*, AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct ClassDeclaration;
    pub struct ClassBlock;
}


pub type ClassDeclarationNode<'script> = SyntaxNode<'script, tags::ClassDeclaration>;

impl NamedSyntaxNode for ClassDeclarationNode<'_> {
    const NODE_KIND: &'static str = "class_decl";
}

impl<'script> ClassDeclarationNode<'script> {
    pub fn specifiers(&self) -> impl Iterator<Item = SpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.unsafe_into())
    }

    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().unsafe_into()
    }

    pub fn base(&self) -> Option<IdentifierNode<'script>> {
        self.field_child("base").map(|n| n.unsafe_into())
    }

    pub fn definition(&self) -> ClassBlockNode<'script> {
        self.field_child("definition").unwrap().unsafe_into()
    }
}

impl Debug for ClassDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("ClassDeclaration {}", self.range().debug()))
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
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ClassDeclarationNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_class_decl(self);

        if tp.any() {
            ctx.push(TraversalContext::Class);
    
            for ch in self.children_detailed().must_be_named(true) {
                match ch {
                    Ok((definition, Some("definition"))) if tp.traverse_definition => {
                        let definition: ClassBlockNode = definition.unsafe_into();

                        definition.accept_with_policy(visitor, ctx, tp.clone());
                    }
                    Err(e) if tp.traverse_errors => {
                        e.accept(visitor, ctx)
                    },
                    _ => {}
                }
            }
    
            ctx.pop();
        }

        visitor.exit_class_decl(self);
    }
}



pub type ClassBlockNode<'script> = SyntaxNode<'script, tags::ClassBlock>;

impl NamedSyntaxNode for ClassBlockNode<'_> {
    const NODE_KIND: &'static str = "class_def";
}

impl<'script> ClassBlockNode<'script> {
    pub fn iter(&self) -> impl Iterator<Item = ClassPropertyNode<'script>> {
        self.named_children().map(|n| n.unsafe_into())
    }


    fn accept_with_policy<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack, tp: ClassDeclarationTraversalPolicy) {
        for ch in self.children_detailed().must_be_named(true) {
            match ch {
                Ok((prop, _)) => {
                    let prop: ClassPropertyNode = prop.unsafe_into();

                    prop.accept(visitor, ctx);
                },
                Err(e) if tp.traverse_errors => {
                    e.accept(visitor, ctx);
                },
                _ => {}
            }
        }
    }
}

impl Debug for ClassBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("ClassBlock {}", self.range().debug()), 
            &self.iter().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ClassBlockNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.unsafe_into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ClassBlockNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        // UNUSED
        self.iter().for_each(|s| s.accept(visitor, ctx));
    }
}


#[derive(Clone)]
pub enum ClassProperty<'script> {
    Var(MemberVarDeclarationNode<'script>),
    Default(MemberDefaultValueNode<'script>),
    DefaultsBlock(MemberDefaultsBlockNode<'script>),
    Hint(MemberHintNode<'script>),
    Autobind(AutobindDeclarationNode<'script>),
    Method(FunctionDeclarationNode<'script>),
    Event(EventDeclarationNode<'script>),
    Nop(NopNode<'script>)
}

impl Debug for ClassProperty<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(n) => f.debug_maybe_alternate(n),
            Self::Default(n) => f.debug_maybe_alternate(n),
            Self::DefaultsBlock(n) => f.debug_maybe_alternate(n),
            Self::Hint(n) => f.debug_maybe_alternate(n),
            Self::Autobind(n) => f.debug_maybe_alternate(n),
            Self::Method(n) => f.debug_maybe_alternate(n),
            Self::Event(n) => f.debug_maybe_alternate(n),
            Self::Nop(n) => f.debug_maybe_alternate(n),
        }
    }
} 

pub type ClassPropertyNode<'script> = SyntaxNode<'script, ClassProperty<'script>>;

impl<'script> ClassPropertyNode<'script> {
    pub fn value(self) -> ClassProperty<'script> {
        match self.tree_node.kind() {
            MemberVarDeclarationNode::NODE_KIND => ClassProperty::Var(self.unsafe_into()),
            MemberDefaultValueNode::NODE_KIND => ClassProperty::Default(self.unsafe_into()),
            MemberDefaultsBlockNode::NODE_KIND => ClassProperty::DefaultsBlock(self.unsafe_into()),
            MemberHintNode::NODE_KIND => ClassProperty::Hint(self.unsafe_into()),
            AutobindDeclarationNode::NODE_KIND => ClassProperty::Autobind(self.unsafe_into()),
            FunctionDeclarationNode::NODE_KIND => ClassProperty::Method(self.unsafe_into()),
            EventDeclarationNode::NODE_KIND => ClassProperty::Event(self.unsafe_into()),
            NopNode::NODE_KIND => ClassProperty::Nop(self.unsafe_into()),
            _ => panic!("Unknown class property type: {} {}", self.tree_node.kind(), self.range().debug())
        }
    }
}

impl Debug for ClassPropertyNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.clone().value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for ClassPropertyNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        match value.tree_node.kind() {
            MemberVarDeclarationNode::NODE_KIND         |
            MemberDefaultValueNode::NODE_KIND           |
            MemberDefaultsBlockNode::NODE_KIND          |
            MemberHintNode::NODE_KIND                   |
            AutobindDeclarationNode::NODE_KIND          |
            FunctionDeclarationNode::NODE_KIND          |
            EventDeclarationNode::NODE_KIND             |
            NopNode::NODE_KIND                          => Ok(value.unsafe_into()),
            _ => Err(())
        }
    }
}

impl SyntaxNodeTraversal for ClassPropertyNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        match self.clone().value() {
            ClassProperty::Var(s) => s.accept(visitor, ctx),
            ClassProperty::Default(s) => s.accept(visitor, ctx),
            ClassProperty::DefaultsBlock(s) => s.accept(visitor, ctx),
            ClassProperty::Hint(s) => s.accept(visitor, ctx),
            ClassProperty::Autobind(s) => s.accept(visitor, ctx),
            ClassProperty::Method(s) => s.accept(visitor, ctx),
            ClassProperty::Event(s) => s.accept(visitor, ctx),
            ClassProperty::Nop(_) => {},
        }
    }
}
