use std::fmt::Debug;
use crate::{AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::{StatementTraversal, StatementVisitor, ExpressionNode, FunctionStatementNode};


pub struct IfConditional;

pub type IfConditionalNode<'script> = SyntaxNode<'script, IfConditional>;

impl NamedSyntaxNode for IfConditionalNode<'_> {
    const NODE_KIND: &'static str = "if_stmt";
}

impl IfConditionalNode<'_> {
    pub fn cond(&self) -> ExpressionNode {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> FunctionStatementNode {
        self.field_child("body").unwrap().into()
    }

    pub fn else_body(&self) -> Option<FunctionStatementNode> {
        self.field_child("else").map(|n| n.into())
    }
}

impl Debug for IfConditionalNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("IfConditional {}", self.range().debug()))
            .field("cond", &self.cond())
            .field("body", &self.body())
            .field("else", &self.else_body())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for IfConditionalNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for IfConditionalNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_if_stmt(self) {
            self.body().accept(visitor);
            self.else_body().map(|s| { s.accept(visitor) });
        }
    }
}



pub struct SwitchConditional;

pub type SwitchConditionalNode<'script> = SyntaxNode<'script, SwitchConditional>;

impl NamedSyntaxNode for SwitchConditionalNode<'_> {
    const NODE_KIND: &'static str = "switch_stmt";
}

impl SwitchConditionalNode<'_> {
    pub fn cond(&self) -> ExpressionNode {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> SwitchConditionalBlockNode {
        self.field_child("body").unwrap().into()
    }
}

impl Debug for SwitchConditionalNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("SwitchConditional {}", self.range().debug()))
            .field("cond", &self.cond())
            .field("body", &self.body())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SwitchConditionalNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for SwitchConditionalNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        if visitor.visit_switch_stmt(self) {
            self.body().accept(visitor);
        }
    }
}



pub struct SwitchConditionalBlock;

pub type SwitchConditionalBlockNode<'script> = SyntaxNode<'script, SwitchConditionalBlock>;

impl NamedSyntaxNode for SwitchConditionalBlockNode<'_> {
    const NODE_KIND: &'static str = "switch_block";
}

impl SwitchConditionalBlockNode<'_> {
    pub fn sections(&self) -> impl Iterator<Item = SwitchConditionalSectionNode> {
        self.named_children().map(|n| n.into())
    }
}

impl Debug for SwitchConditionalBlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate_named(
            &format!("SwitchConditionalBlock {}", self.range().debug()), 
            &self.sections().collect::<Vec<_>>()
        )
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SwitchConditionalBlockNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for SwitchConditionalBlockNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        self.sections().for_each(|s| s.accept(visitor));
    }
}



#[derive(Clone)]
pub enum SwitchConditionalSection<'script> {
    Statement(FunctionStatementNode<'script>),
    Case(SwitchConditionalCaseLabelNode<'script>),
    Default(SwitchConditionalDefaultLabelNode<'script>)
}

impl Debug for SwitchConditionalSection<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Statement(n) => f.debug_maybe_alternate(n),
            Self::Case(n) => f.debug_maybe_alternate(n),
            Self::Default(n) => f.debug_maybe_alternate(n),
        }
    }
}

pub type SwitchConditionalSectionNode<'script> = SyntaxNode<'script, SwitchConditionalSection<'script>>;

impl<'script> SwitchConditionalSectionNode<'script> {
    pub fn value(self) -> SwitchConditionalSection<'script> {
        let k = self.tree_node.kind();
        // first try if self is a label
        if k == SwitchConditionalCaseLabelNode::NODE_KIND {
            return SwitchConditionalSection::Case(self.into());
        }
        if k == SwitchConditionalDefaultLabelNode::NODE_KIND {
            return SwitchConditionalSection::Default(self.into());
        }

        let range = self.range();
        // if self is not label then it must be a function statement
        if let Ok(stmt) = FunctionStatementNode::try_from(self.into_any()) {
            return SwitchConditionalSection::Statement(stmt);
        }

        panic!("Unexpected switch conditional section node: {} {}", k, range.debug())
    }
}

impl Debug for SwitchConditionalSectionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_maybe_alternate(&self.clone().value())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SwitchConditionalSectionNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        let k = value.tree_node.kind();
        if k == SwitchConditionalCaseLabelNode::NODE_KIND 
        || k == SwitchConditionalDefaultLabelNode::NODE_KIND 
        || FunctionStatementNode::try_from(value.clone()).is_ok() {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for SwitchConditionalSectionNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        match self.clone().value() {
            SwitchConditionalSection::Statement(n) => n.accept(visitor),
            SwitchConditionalSection::Case(n) => n.accept(visitor),
            SwitchConditionalSection::Default(n) => n.accept(visitor),
        }
    }
}



pub struct SwitchConditionalCaseLabel;

pub type SwitchConditionalCaseLabelNode<'script> = SyntaxNode<'script, SwitchConditionalCaseLabel>;

impl NamedSyntaxNode for SwitchConditionalCaseLabelNode<'_> {
    const NODE_KIND: &'static str = "switch_case_label";
}

impl SwitchConditionalCaseLabelNode<'_> {
    pub fn value(&self) -> ExpressionNode {
        self.field_child("value").unwrap().into()
    }
}

impl Debug for SwitchConditionalCaseLabelNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("SwitchConditionalCaseLabel {}", self.range().debug()))
            .field("value", &self.value())
            .finish()
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SwitchConditionalCaseLabelNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for SwitchConditionalCaseLabelNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_switch_stmt_case(self);
    }
}


pub struct SwitchConditionalDefaultLabel;

pub type SwitchConditionalDefaultLabelNode<'script> = SyntaxNode<'script, SwitchConditionalDefaultLabel>;

impl NamedSyntaxNode for SwitchConditionalDefaultLabelNode<'_> {
    const NODE_KIND: &'static str = "switch_default_label";
}

impl Debug for SwitchConditionalDefaultLabelNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SwitchConditionalDefaultLabel {}", self.range().debug())
    }
}

impl<'script> TryFrom<AnyNode<'script>> for SwitchConditionalDefaultLabelNode<'script> {
    type Error = ();

    fn try_from(value: AnyNode<'script>) -> Result<Self, Self::Error> {
        if value.tree_node.kind() == Self::NODE_KIND {
            Ok(value.into())
        } else {
            Err(())
        }
    }
}

impl StatementTraversal for SwitchConditionalDefaultLabelNode<'_> {
    fn accept<V: StatementVisitor>(&self, visitor: &mut V) {
        visitor.visit_switch_stmt_default(self);
    }
}