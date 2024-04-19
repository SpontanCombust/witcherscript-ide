#[derive(Debug, Clone)]
pub struct NestedExpressionTraversalPolicy {
    pub traverse_inner: bool
}

impl Default for NestedExpressionTraversalPolicy {
    fn default() -> Self {
        Self {
            traverse_inner: true
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCallExpressionTraversalPolicy {
    pub traverse_func: bool,
    pub traverse_args: bool,
}

impl Default for FunctionCallExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_func: true, 
            traverse_args: true
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCallArgumentTraversalPolicy {
    pub traverse_expr: bool
}

impl Default for FunctionCallArgumentTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_expr: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArrayExpressionTraversalPolicy {
    pub traverse_accessor: bool,
    pub traverse_index: bool
}

impl Default for ArrayExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_accessor: true, 
            traverse_index: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberFieldExpressionTraversalPolicy {
    pub traverse_accessor: bool
}

impl Default for MemberFieldExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_accessor: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewExpressionTraversalPolicy {
    pub traverse_lifetime_obj: bool
}

impl Default for NewExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_lifetime_obj: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeCastExpressionTraversalPolicy {
    pub traverse_value: bool
}

impl Default for TypeCastExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_value: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryOperationExpressionTraversalPolicy {
    pub traverse_right: bool
}

impl Default for UnaryOperationExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_right: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryOperationExpressionTraversalPolicy {
    pub traverse_left: bool,
    pub traverse_right: bool
}

impl Default for BinaryOperationExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_left: true, 
            traverse_right: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentOperationExpressionTraversalPolicy {
    pub traverse_left: bool,
    pub traverse_right: bool
}

impl Default for AssignmentOperationExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_left: true, 
            traverse_right: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct TernaryConditionalExpressionTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_conseq: bool,
    pub traverse_alt: bool
}

impl Default for TernaryConditionalExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_cond: true, 
            traverse_conseq: true, 
            traverse_alt: true 
        }
    }
}




#[derive(Debug, Clone)]
pub struct RootTraversalPolicy {
    pub traverse: bool
}

impl Default for RootTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassDeclarationTraversalPolicy {
    pub traverse_definition: bool
}

impl Default for ClassDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_definition: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateDeclarationTraversalPolicy {
    pub traverse_definition: bool
}

impl Default for StateDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_definition: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructDeclarationTraversalPolicy {
    pub traverse_definition: bool
}

impl Default for StructDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_definition: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumDeclarationTraversalPolicy {
    pub traverse_definition: bool
}

impl Default for EnumDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_definition: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberDefaultsBlockTraversalPolicy {
    pub traverse: bool
}

impl Default for MemberDefaultsBlockTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalFunctionDeclarationTraversalPolicy {
    pub traverse_params: bool
}

impl Default for GlobalFunctionDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_params: true
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberFunctionDeclarationTraversalPolicy {
    pub traverse_params: bool
}

impl Default for MemberFunctionDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_params: true
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventDeclarationTraversalPolicy {
    pub traverse_params: bool
}

impl Default for EventDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_params: true
        }
    }
}





#[derive(Debug, Clone)]
pub struct ForLoopTraversalPolicy {
    pub traverse_body: bool
}

impl Default for ForLoopTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_body: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileLoopTraversalPolicy {
    pub traverse_body: bool
}

impl Default for WhileLoopTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_body: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct DoWhileLoopTraversalPolicy {
    pub traverse_body: bool
}

impl Default for DoWhileLoopTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_body: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfConditionalTraversalPolicy {
    pub traverse_body: bool,
    pub traverse_else_body: bool
}

impl Default for IfConditionalTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_body: true, 
            traverse_else_body: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwitchConditionalTraversalPolicy {
    pub traverse_body: bool
}

impl Default for SwitchConditionalTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_body: true 
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionBlockTraversalPolicy {
    pub traverse: bool
}

impl Default for FunctionBlockTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse: true 
        }
    }
}
