use std::ops::BitAnd;


pub trait TraversalPolicy: Sized + std::ops::BitAnd<Output = Self> {
    fn default_to(value: bool) -> Self;
}


#[derive(Debug, Clone)]
pub struct NestedExpressionTraversalPolicy {
    pub traverse_inner: bool
}

impl TraversalPolicy for NestedExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_inner: value
        }
    }
}

impl BitAnd for NestedExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_inner: self.traverse_inner && rhs.traverse_inner
        }
    }
}


#[derive(Debug, Clone)]
pub struct FunctionCallExpressionTraversalPolicy {
    pub traverse_func: bool,
    pub traverse_args: bool,
}

impl TraversalPolicy for FunctionCallExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_func: value,
            traverse_args: value
        }
    }
}

impl BitAnd for FunctionCallExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_args: self.traverse_args && rhs.traverse_args,
            traverse_func: self.traverse_func && rhs.traverse_func
        }
    }
}


#[derive(Debug, Clone)]
pub struct FunctionCallArgumentTraversalPolicy {
    pub traverse_expr: bool
}

impl TraversalPolicy for FunctionCallArgumentTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_expr: value,
        }
    }
}

impl BitAnd for FunctionCallArgumentTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_expr: self.traverse_expr && rhs.traverse_expr
        }
    }
}


#[derive(Debug, Clone)]
pub struct ArrayExpressionTraversalPolicy {
    pub traverse_accessor: bool,
    pub traverse_index: bool
}

impl TraversalPolicy for ArrayExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_accessor: value,
            traverse_index: value,
        }
    }
}

impl BitAnd for ArrayExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_accessor: self.traverse_accessor && rhs.traverse_accessor,
            traverse_index: self.traverse_index && rhs.traverse_index
        }
    }
}


#[derive(Debug, Clone)]
pub struct MemberFieldExpressionTraversalPolicy {
    pub traverse_accessor: bool
}

impl TraversalPolicy for MemberFieldExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_accessor: value,
        }
    }
}

impl BitAnd for MemberFieldExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_accessor: self.traverse_accessor && rhs.traverse_accessor
        }
    }
}


#[derive(Debug, Clone)]
pub struct NewExpressionTraversalPolicy {
    pub traverse_lifetime_obj: bool
}

impl TraversalPolicy for NewExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_lifetime_obj: value,
        }
    }
}

impl BitAnd for NewExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_lifetime_obj: self.traverse_lifetime_obj && rhs.traverse_lifetime_obj
        }
    }
}


#[derive(Debug, Clone)]
pub struct TypeCastExpressionTraversalPolicy {
    pub traverse_value: bool
}

impl TraversalPolicy for TypeCastExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
        }
    }
}

impl BitAnd for TypeCastExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value
        }
    }
}


#[derive(Debug, Clone)]
pub struct UnaryOperationExpressionTraversalPolicy {
    pub traverse_right: bool
}

impl TraversalPolicy for UnaryOperationExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_right: value,
        }
    }
}

impl BitAnd for UnaryOperationExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_right: self.traverse_right && rhs.traverse_right
        }
    }
}


#[derive(Debug, Clone)]
pub struct BinaryOperationExpressionTraversalPolicy {
    pub traverse_left: bool,
    pub traverse_right: bool
}

impl TraversalPolicy for BinaryOperationExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_right: value,
            traverse_left: value
        }
    }
}

impl BitAnd for BinaryOperationExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_left: self.traverse_left && rhs.traverse_left,
            traverse_right: self.traverse_right && rhs.traverse_right
        }
    }
}


#[derive(Debug, Clone)]
pub struct AssignmentOperationExpressionTraversalPolicy {
    pub traverse_left: bool,
    pub traverse_right: bool
}

impl TraversalPolicy for AssignmentOperationExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_right: value,
            traverse_left: value
        }
    }
}

impl BitAnd for AssignmentOperationExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_left: self.traverse_left && rhs.traverse_left,
            traverse_right: self.traverse_right && rhs.traverse_right
        }
    }
}


#[derive(Debug, Clone)]
pub struct TernaryConditionalExpressionTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_conseq: bool,
    pub traverse_alt: bool
}

impl TraversalPolicy for TernaryConditionalExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_conseq: value,
            traverse_alt: value
        }
    }
}

impl BitAnd for TernaryConditionalExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_conseq: self.traverse_conseq && rhs.traverse_conseq,
            traverse_alt: self.traverse_alt && rhs.traverse_alt
        }
    }
}




#[derive(Debug, Clone)]
pub struct RootTraversalPolicy {
    pub traverse: bool
}

impl TraversalPolicy for RootTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse: value,
        }
    }
}

impl BitAnd for RootTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse: self.traverse && rhs.traverse
        }
    }
}


#[derive(Debug, Clone)]
pub struct ClassDeclarationTraversalPolicy {
    pub traverse_definition: bool
}

impl TraversalPolicy for ClassDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_definition: value,
        }
    }
}

impl BitAnd for ClassDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_definition: self.traverse_definition && rhs.traverse_definition
        }
    }
}


#[derive(Debug, Clone)]
pub struct StateDeclarationTraversalPolicy {
    pub traverse_definition: bool
}

impl TraversalPolicy for StateDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_definition: value,
        }
    }
}

impl BitAnd for StateDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_definition: self.traverse_definition && rhs.traverse_definition
        }
    }
}


#[derive(Debug, Clone)]
pub struct StructDeclarationTraversalPolicy {
    pub traverse_definition: bool
}

impl TraversalPolicy for StructDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_definition: value,
        }
    }
}

impl BitAnd for StructDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_definition: self.traverse_definition && rhs.traverse_definition
        }
    }
}


#[derive(Debug, Clone)]
pub struct EnumDeclarationTraversalPolicy {
    pub traverse_definition: bool
}

impl TraversalPolicy for EnumDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_definition: value,
        }
    }
}

impl BitAnd for EnumDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_definition: self.traverse_definition && rhs.traverse_definition
        }
    }
}


#[derive(Debug, Clone)]
pub struct MemberDefaultValueTraversalPolicy {
    pub traverse_value: bool
}

impl TraversalPolicy for MemberDefaultValueTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
        }
    }
}

impl BitAnd for MemberDefaultValueTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value
        }
    }
}


#[derive(Debug, Clone)]
pub struct MemberDefaultsBlockTraversalPolicy {
    pub traverse: bool
}

impl TraversalPolicy for MemberDefaultsBlockTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse: value,
        }
    }
}

impl BitAnd for MemberDefaultsBlockTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse: self.traverse && rhs.traverse
        }
    }
}


#[derive(Debug, Clone)]
pub struct GlobalFunctionDeclarationTraversalPolicy {
    pub traverse_params: bool,
    pub traverse_definition: bool
}

impl TraversalPolicy for GlobalFunctionDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_params: value,
            traverse_definition: value
        }
    }
}

impl BitAnd for GlobalFunctionDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_params: self.traverse_params && rhs.traverse_params,
            traverse_definition: self.traverse_definition && rhs.traverse_definition
        }
    }
}


#[derive(Debug, Clone)]
pub struct MemberFunctionDeclarationTraversalPolicy {
    pub traverse_params: bool,
    pub traverse_definition: bool
}

impl TraversalPolicy for MemberFunctionDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_params: value,
            traverse_definition: value
        }
    }
}

impl BitAnd for MemberFunctionDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_params: self.traverse_params && rhs.traverse_params,
            traverse_definition: self.traverse_definition && rhs.traverse_definition
        }
    }
}


#[derive(Debug, Clone)]
pub struct EventDeclarationTraversalPolicy {
    pub traverse_params: bool,
    pub traverse_definition: bool
}

impl TraversalPolicy for EventDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_params: value,
            traverse_definition: value
        }
    }
}

impl BitAnd for EventDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_params: self.traverse_params && rhs.traverse_params,
            traverse_definition: self.traverse_definition && rhs.traverse_definition
        }
    }
}




#[derive(Debug, Clone)]
pub struct ForLoopTraversalPolicy {
    pub traverse_init: bool,
    pub traverse_cond: bool,
    pub traverse_iter: bool,
    pub traverse_body: bool
}

impl TraversalPolicy for ForLoopTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_init: value,
            traverse_cond: value,
            traverse_iter: value,
            traverse_body: value,
        }
    }
}

impl BitAnd for ForLoopTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_init: self.traverse_init && rhs.traverse_init,
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_iter: self.traverse_iter && rhs.traverse_iter,
            traverse_body: self.traverse_body && rhs.traverse_body
        }
    }
}


#[derive(Debug, Clone)]
pub struct WhileLoopTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_body: bool
}

impl TraversalPolicy for WhileLoopTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_body: value,
        }
    }
}

impl BitAnd for WhileLoopTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_body: self.traverse_body && rhs.traverse_body
        }
    }
}


#[derive(Debug, Clone)]
pub struct DoWhileLoopTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_body: bool
}

impl TraversalPolicy for DoWhileLoopTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_body: value,
        }
    }
}

impl BitAnd for DoWhileLoopTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_body: self.traverse_body && rhs.traverse_body
        }
    }
}


#[derive(Debug, Clone)]
pub struct IfConditionalTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_body: bool,
    pub traverse_else_body: bool
}

impl TraversalPolicy for IfConditionalTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_body: value,
            traverse_else_body: value
        }
    }
}

impl BitAnd for IfConditionalTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_body: self.traverse_body && rhs.traverse_body,
            traverse_else_body: self.traverse_else_body && rhs.traverse_else_body
        }
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_body: bool
}

impl TraversalPolicy for SwitchConditionalTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_body: value,
        }
    }
}

impl BitAnd for SwitchConditionalTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_body: self.traverse_body && rhs.traverse_body
        }
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalCaseLabelTraversalPolicy {
    pub traverse_value: bool
}

impl TraversalPolicy for SwitchConditionalCaseLabelTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
        }
    }
}

impl BitAnd for SwitchConditionalCaseLabelTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value
        }
    }
}


#[derive(Debug, Clone)]
pub struct CompoundStatementTraversalPolicy {
    pub traverse: bool
}

impl TraversalPolicy for CompoundStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse: value,
        }
    }
}

impl BitAnd for CompoundStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse: self.traverse && rhs.traverse
        }
    }
}


#[derive(Debug, Clone)]
pub struct VarDeclarationTraversalPolicy {
    pub traverse_init_value: bool
}

impl TraversalPolicy for VarDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_init_value: value,
        }
    }
}

impl BitAnd for VarDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_init_value: self.traverse_init_value && rhs.traverse_init_value
        }
    }
}


#[derive(Debug, Clone)]
pub struct ExpressionStatementTraversalPolicy {
    pub traverse_expr: bool
}

impl TraversalPolicy for ExpressionStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_expr: value,
        }
    }
}

impl BitAnd for ExpressionStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_expr: self.traverse_expr && rhs.traverse_expr
        }
    }
}


#[derive(Debug, Clone)]
pub struct ReturnStatementTraversalPolicy {
    pub traverse_value: bool
}

impl TraversalPolicy for ReturnStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
        }
    }
}

impl BitAnd for ReturnStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value
        }
    }
}


#[derive(Debug, Clone)]
pub struct DeleteStatementTraversalPolicy {
    pub traverse_value: bool
}

impl TraversalPolicy for DeleteStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
        }
    }
}

impl BitAnd for DeleteStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value
        }
    }
}