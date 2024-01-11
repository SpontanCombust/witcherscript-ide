use std::fmt::Debug;
use crate::{tokens::{IdentifierNode, LiteralStringNode}, NamedSyntaxNode, SyntaxNode, attribs::StructSpecifierNode, AnyNode};
use super::{StatementTraversal, StatementVisitor, ExpressionNode, MemberVarDeclarationNode, NopNode};


#[derive(Debug, Clone)]
pub struct StructDeclaration;

pub type StructDeclarationNode<'script> = SyntaxNode<'script, StructDeclaration>;

impl NamedSyntaxNode for StructDeclarationNode<'_> {
    const NODE_KIND: &'static str = "struct_decl_stmt";
}

impl StructDeclarationNode<'_> {
    pub fn specifiers(&self) -> impl Iterator<Item = StructSpecifierNode> {
        self.field_children("specifiers").map(|n| n.into())
    }

    pub fn name(&self) -> IdentifierNode {
        self.field_child("name").unwrap().into()
    }

    pub fn definition(&self) -> StructBlockNode {
        self.field_child("definition").unwrap().into()
    }
}

impl Debug for StructDeclarationNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructDeclaration")
            .field("specifiers", &self.specifiers().collect::<Vec<_>>())
            .field("name", &self.name())
            .field("definition", &self.definition())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StructDeclarationNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for StructDeclarationNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_struct_decl(self) {
            self.definition().accept(visitor);
        }
        visitor.exit_struct_decl(self);
    }
}



#[derive(Debug, Clone)]
pub struct StructBlock;

pub type StructBlockNode<'script> = SyntaxNode<'script, StructBlock>;

impl NamedSyntaxNode for StructBlockNode<'_> {
    const NODE_KIND: &'static str = "struct_block";
}

impl StructBlockNode<'_> {
    pub fn statements(&self) -> impl Iterator<Item = StructStatementNode> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for StructBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stmts = self.statements().collect::<Vec<_>>();
        if f.alternate() {
            write!(f, "StructBlock{:#?}", stmts)
        } else {
            write!(f, "StructBlock{:?}", stmts)
        }
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StructBlockNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for StructBlockNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        self.statements().for_each(|s| s.accept(visitor));
    }
}



#[derive(Debug, Clone)]
pub enum StructStatement<'script> {
    Var(MemberVarDeclarationNode<'script>),
    Default(MemberDefaultValueNode<'script>),
    Hint(MemberHintNode<'script>),
    Nop(NopNode<'script>)
}

pub type StructStatementNode<'script> = SyntaxNode<'script, StructStatement<'script>>;

impl StructStatementNode<'_> {
    pub fn value(&self) -> StructStatement {
        match self.tree_node.kind() {
            MemberVarDeclarationNode::NODE_KIND => StructStatement::Var(self.clone().into()),
            MemberDefaultValueNode::NODE_KIND => StructStatement::Default(self.clone().into()),
            MemberHintNode::NODE_KIND => StructStatement::Hint(self.clone().into()),
            NopNode::NODE_KIND => StructStatement::Nop(self.clone().into()),
            _ => panic!("Unknown struct statement type: {}", self.tree_node.kind())
        }
    }
}

impl Debug for StructStatementNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}

impl<'script> TryFrom<AnyNode<'script>> for StructStatementNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        match value.tree_node.kind() {
            MemberVarDeclarationNode::NODE_KIND     |
            MemberDefaultValueNode::NODE_KIND       |
            MemberHintNode::NODE_KIND               |
            NopNode::NODE_KIND                      => Ok(value.into()),
            _ => Err(())
        }
    }
}

impl StatementTraversal for StructStatementNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        match self.value() {
            StructStatement::Var(s) => s.accept(visitor),
            StructStatement::Default(s) => s.accept(visitor),
            StructStatement::Hint(s) => s.accept(visitor),
            StructStatement::Nop(s) => s.accept(visitor),
        }
    }
}



#[derive(Debug, Clone)]
pub struct MemberDefaultValue; 

pub type MemberDefaultValueNode<'script> = SyntaxNode<'script, MemberDefaultValue>;

impl NamedSyntaxNode for MemberDefaultValueNode<'_> {
    const NODE_KIND: &'static str = "member_default_val_stmt";
}

impl MemberDefaultValueNode<'_> {
    pub fn member(&self) -> IdentifierNode {
        self.field_child("member").unwrap().into()
    }

    pub fn value(&self) -> ExpressionNode {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for MemberDefaultValueNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberDefaultValue")
            .field("member", &self.member())
            .field("value", &self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberDefaultValueNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for MemberDefaultValueNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_member_default_val(self);
    }
}



#[derive(Debug, Clone)]
pub struct MemberHint; 

pub type MemberHintNode<'script> = SyntaxNode<'script, MemberHint>;

impl NamedSyntaxNode for MemberHintNode<'_> {
    const NODE_KIND: &'static str = "member_hint_stmt";
}

impl MemberHintNode<'_> {
    pub fn member(&self) -> IdentifierNode {
        self.field_child("member").unwrap().into()
    }

    pub fn value(&self) -> LiteralStringNode {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for MemberHintNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberHint")
            .field("member", &self.member())
            .field("value", &self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for MemberHintNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for MemberHintNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_member_hint(self);
    }
}
