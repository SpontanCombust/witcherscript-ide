use std::fmt::Debug;
use crate::{attribs::*, tokens::*, AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct ClassDeclaration;
    pub struct ClassBlock;
    pub struct AutobindDeclaration;
    pub struct AutobindValueSingle;
}


pub type ClassDeclarationNode<'script> = SyntaxNode<'script, tags::ClassDeclaration>;

impl NamedSyntaxNode for ClassDeclarationNode<'_> {
    const NODE_KIND: &'static str = "class_decl";
}

impl<'script> ClassDeclarationNode<'script> {
    pub fn specifiers(&self) -> impl Iterator<Item = SpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().into()
    }

    pub fn base(&self) -> Option<IdentifierNode<'script>> {
        self.field_child("base").map(|n| n.into())
    }

    pub fn definition(&self) -> ClassBlockNode<'script> {
        self.field_child("definition").unwrap().into()
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
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ClassDeclarationNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        let tp = visitor.visit_class_decl(self);
        if tp.traverse_definition {
            self.definition().accept(visitor, ());
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
        self.named_children().map(|n| n.into())
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
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for ClassBlockNode<'_> {
    type TraversalCtx = ();

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, _: Self::TraversalCtx) {
        self.iter().for_each(|s| s.accept(visitor, PropertyTraversalContext::ClassDefinition));
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
            MemberVarDeclarationNode::NODE_KIND => ClassProperty::Var(self.into()),
            MemberDefaultValueNode::NODE_KIND => ClassProperty::Default(self.into()),
            MemberDefaultsBlockNode::NODE_KIND => ClassProperty::DefaultsBlock(self.into()),
            MemberHintNode::NODE_KIND => ClassProperty::Hint(self.into()),
            AutobindDeclarationNode::NODE_KIND => ClassProperty::Autobind(self.into()),
            FunctionDeclarationNode::NODE_KIND => ClassProperty::Method(self.into()),
            EventDeclarationNode::NODE_KIND => ClassProperty::Event(self.into()),
            NopNode::NODE_KIND => ClassProperty::Nop(self.into()),
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
            NopNode::NODE_KIND                          => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl SyntaxNodeTraversal for ClassPropertyNode<'_> {
    type TraversalCtx = PropertyTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        match self.clone().value() {
            ClassProperty::Var(s) => s.accept(visitor, ctx),
            ClassProperty::Default(s) => s.accept(visitor, ctx),
            ClassProperty::DefaultsBlock(s) => s.accept(visitor, ctx),
            ClassProperty::Hint(s) => s.accept(visitor, ctx),
            ClassProperty::Autobind(s) => s.accept(visitor, ctx),
            ClassProperty::Method(s) => s.accept(visitor, if ctx == PropertyTraversalContext::ClassDefinition {
                FunctionDeclarationTraversalContext::ClassDefinition
            } else {
                FunctionDeclarationTraversalContext::StateDefinition
            }),
            ClassProperty::Event(s) => s.accept(visitor, ctx),
            ClassProperty::Nop(_) => {},
        }
    }
}



pub type AutobindDeclarationNode<'script> = SyntaxNode<'script, tags::AutobindDeclaration>;

impl NamedSyntaxNode for AutobindDeclarationNode<'_> {
    const NODE_KIND: &'static str = "autobind_decl";
}

impl<'script> AutobindDeclarationNode<'script> {
    pub fn specifiers(&self) -> impl Iterator<Item = SpecifierNode<'script>> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode<'script> {
        self.field_child("name").unwrap().into()
    }

    pub fn autobind_type(&self) -> TypeAnnotationNode<'script> {
        self.field_child("autobind_type").unwrap().into()
    }

    pub fn value(&self) -> AutobindValue<'script> {
        let n = self.field_child("value").unwrap();
        let kind = n.tree_node.kind();
        match kind {
            AutobindValueSingleNode::NODE_KIND => AutobindValue::Single(n.into()),
            LiteralStringNode::NODE_KIND => AutobindValue::Concrete(n.into()),
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
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl SyntaxNodeTraversal for AutobindDeclarationNode<'_> {
    type TraversalCtx = PropertyTraversalContext;

    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: Self::TraversalCtx) {
        visitor.visit_autobind_decl(self, ctx);
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
            Ok(value.into())
        } else {
            Err(())
        }
    }
}