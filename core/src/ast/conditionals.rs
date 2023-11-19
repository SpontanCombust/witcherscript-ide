use crate::{NamedSyntaxNode, SyntaxNode};
use super::{expressions::Expression, functions::FunctionStatement};


#[derive(Debug, Clone)]
pub struct IfConditional;

impl NamedSyntaxNode for IfConditional {
    const NODE_NAME: &'static str = "if_stmt";
}

impl SyntaxNode<'_, IfConditional> {
    pub fn cond(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("cond").unwrap().into()
    }

    pub fn body(&self) -> SyntaxNode<'_, FunctionStatement> {
        self.field_child("body").unwrap().into()
    }

    pub fn else_body(&self) -> Option<SyntaxNode<'_, FunctionStatement>> {
        self.field_child("else").map(|n| n.into())
    }
}



#[derive(Debug, Clone)]
pub struct SwitchConditional;

impl NamedSyntaxNode for SwitchConditional {
    const NODE_NAME: &'static str = "switch_stmt";
}

impl SyntaxNode<'_, SwitchConditional> {

}


#[derive(Debug, Clone)]
pub struct SwitchConditionalCase;

impl NamedSyntaxNode for SwitchConditionalCase {
    const NODE_NAME: &'static str = "switch_case";
}

impl SyntaxNode<'_, SwitchConditionalCase> {
    pub fn value(&self) -> SyntaxNode<'_, Expression> {
        self.field_child("value").unwrap().into()
    }

    pub fn body(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionStatement>> {
        self.field_children("body").map(|n| n.into())
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalDefault;

impl NamedSyntaxNode for SwitchConditionalDefault {
    const NODE_NAME: &'static str = "switch_default";
}

impl SyntaxNode<'_, SwitchConditionalDefault> {
    pub fn body(&self) -> impl Iterator<Item = SyntaxNode<'_, FunctionStatement>> {
        self.field_children("body").map(|n| n.into())
    }
}