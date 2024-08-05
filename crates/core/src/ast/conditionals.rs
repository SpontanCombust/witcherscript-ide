use std::fmt::Debug;
use crate::{AnyNode, DebugMaybeAlternate, DebugRange, NamedSyntaxNode, SyntaxNode};
use super::*;


mod tags {
    pub struct IfConditional;
    pub struct SwitchConditional;
    pub struct SwitchConditionalBlock;
    pub struct SwitchConditionalCaseLabel;
    pub struct SwitchConditionalDefaultLabel;
}


pub type IfConditionalNode<'script> = SyntaxNode<'script, tags::IfConditional>;

impl NamedSyntaxNode for IfConditionalNode<'_> {
    const NODE_KIND: &'static str = "if_stmt";
}

impl<'script> IfConditionalNode<'script> {
    pub fn cond(&self) -> ExpressionNode<'script> {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> FunctionStatementNode<'script> {
        self.field_child("body").unwrap().into()
    }

    pub fn else_body(&self) -> Option<FunctionStatementNode<'script>> {
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

impl SyntaxNodeTraversal for IfConditionalNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_if_stmt(self, ctx);
        if tp.traverse_cond {
            ctx.push(TraversalContext::IfConditionalCond);
            self.cond().accept(visitor, ctx);
            ctx.pop();
        }
        if tp.traverse_body {
            ctx.push(TraversalContext::IfConditionalBody);
            self.body().accept(visitor, ctx);
            ctx.pop();
        }
        if tp.traverse_else_body {
            ctx.push(TraversalContext::IfConditionalElseBody);
            self.else_body().map(|s| s.accept(visitor, ctx));
            ctx.pop();
        }
        visitor.exit_if_stmt(self, ctx);
    }
}



pub type SwitchConditionalNode<'script> = SyntaxNode<'script, tags::SwitchConditional>;

impl NamedSyntaxNode for SwitchConditionalNode<'_> {
    const NODE_KIND: &'static str = "switch_stmt";
}

impl<'script> SwitchConditionalNode<'script> {
    pub fn cond(&self) -> ExpressionNode<'script> {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> SwitchConditionalBlockNode<'script> {
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

impl SyntaxNodeTraversal for SwitchConditionalNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_switch_stmt(self, ctx);
        if tp.traverse_cond {
            ctx.push(TraversalContext::SwitchConditionalCond);
            self.cond().accept(visitor, ctx);
            ctx.pop();
        }
        if tp.traverse_body {
            ctx.push(TraversalContext::SwitchConditionalBody);
            self.body().accept(visitor, ctx);
            ctx.pop();
        }
        visitor.exit_switch_stmt(self, ctx);
    }
}



pub type SwitchConditionalBlockNode<'script> = SyntaxNode<'script, tags::SwitchConditionalBlock>;

impl NamedSyntaxNode for SwitchConditionalBlockNode<'_> {
    const NODE_KIND: &'static str = "switch_block";
}

impl<'script> SwitchConditionalBlockNode<'script> {
    pub fn sections(&self) -> impl Iterator<Item = SwitchConditionalSectionNode<'script>> {
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

impl SyntaxNodeTraversal for SwitchConditionalBlockNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        self.sections().for_each(|s| s.accept(visitor, ctx));
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

impl SyntaxNodeTraversal for SwitchConditionalSectionNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        match self.clone().value() {
            SwitchConditionalSection::Statement(n) => n.accept(visitor, ctx),
            SwitchConditionalSection::Case(n) => n.accept(visitor, ctx),
            SwitchConditionalSection::Default(n) => n.accept(visitor, ctx),
        }
    }
}



pub type SwitchConditionalCaseLabelNode<'script> = SyntaxNode<'script, tags::SwitchConditionalCaseLabel>;

impl NamedSyntaxNode for SwitchConditionalCaseLabelNode<'_> {
    const NODE_KIND: &'static str = "switch_case_label";
}

impl<'script> SwitchConditionalCaseLabelNode<'script> {
    pub fn value(&self) -> ExpressionNode<'script> {
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

impl SyntaxNodeTraversal for SwitchConditionalCaseLabelNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        let tp = visitor.visit_switch_stmt_case(self, ctx);
        if tp.traverse_value {
            ctx.push(TraversalContext::SwitchConditionalCaseLabel);
            self.value().accept(visitor, ctx);
            ctx.pop();
        }
        visitor.exit_switch_stmt_case(self, ctx);
    }
}



pub type SwitchConditionalDefaultLabelNode<'script> = SyntaxNode<'script, tags::SwitchConditionalDefaultLabel>;

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

impl SyntaxNodeTraversal for SwitchConditionalDefaultLabelNode<'_> {
    fn accept<V: SyntaxNodeVisitor>(&self, visitor: &mut V, ctx: &mut TraversalContextStack) {
        visitor.visit_switch_stmt_default(self, ctx);
    }
}