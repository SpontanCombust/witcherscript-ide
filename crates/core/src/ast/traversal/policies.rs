use std::ops::BitAnd;


pub trait TraversalPolicy: Sized + std::ops::BitAnd<Output = Self> {
    fn default_to(value: bool) -> Self;
    fn any(&self) -> bool;
}


#[derive(Debug, Clone)]
pub struct NestedExpressionTraversalPolicy {
    pub traverse_inner: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for NestedExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_inner: value,
            traverse_errors: value
        }
    }
    
    #[inline]
    fn any(&self) -> bool {
        self.traverse_inner || 
        self.traverse_errors
    }
}

impl BitAnd for NestedExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_inner: self.traverse_inner && rhs.traverse_inner,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct FunctionCallExpressionTraversalPolicy {
    pub traverse_func: bool,
    pub traverse_args: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for FunctionCallExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_func: value,
            traverse_args: value,
            traverse_errors: value
        }
    }
    
    #[inline]
    fn any(&self) -> bool {
        self.traverse_func || 
        self.traverse_args || 
        self.traverse_errors
    }
}

impl BitAnd for FunctionCallExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_args: self.traverse_args && rhs.traverse_args,
            traverse_func: self.traverse_func && rhs.traverse_func,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
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

    #[inline]
    fn any(&self) -> bool {
        self.traverse_expr
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
    pub traverse_index: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for ArrayExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_accessor: value,
            traverse_index: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_accessor || 
        self.traverse_index || 
        self.traverse_errors
    }
}

impl BitAnd for ArrayExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_accessor: self.traverse_accessor && rhs.traverse_accessor,
            traverse_index: self.traverse_index && rhs.traverse_index,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct MemberFieldExpressionTraversalPolicy {
    pub traverse_accessor: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for MemberFieldExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_accessor: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_accessor || 
        self.traverse_errors
    }
}

impl BitAnd for MemberFieldExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_accessor: self.traverse_accessor && rhs.traverse_accessor,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct NewExpressionTraversalPolicy {
    pub traverse_lifetime_obj: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for NewExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_lifetime_obj: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_lifetime_obj || 
        self.traverse_errors
    }
}

impl BitAnd for NewExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_lifetime_obj: self.traverse_lifetime_obj && rhs.traverse_lifetime_obj,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct TypeCastExpressionTraversalPolicy {
    pub traverse_value: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for TypeCastExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_value || 
        self.traverse_errors
    }
}

impl BitAnd for TypeCastExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct UnaryOperationExpressionTraversalPolicy {
    pub traverse_right: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for UnaryOperationExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_right: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_right || 
        self.traverse_errors
    }
}

impl BitAnd for UnaryOperationExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_right: self.traverse_right && rhs.traverse_right,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct BinaryOperationExpressionTraversalPolicy {
    pub traverse_left: bool,
    pub traverse_right: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for BinaryOperationExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_right: value,
            traverse_left: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_left || 
        self.traverse_right || 
        self.traverse_errors
    }
}

impl BitAnd for BinaryOperationExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_left: self.traverse_left && rhs.traverse_left,
            traverse_right: self.traverse_right && rhs.traverse_right,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct AssignmentOperationExpressionTraversalPolicy {
    pub traverse_left: bool,
    pub traverse_right: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for AssignmentOperationExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_right: value,
            traverse_left: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_left || 
        self.traverse_right || 
        self.traverse_errors
    }
}

impl BitAnd for AssignmentOperationExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_left: self.traverse_left && rhs.traverse_left,
            traverse_right: self.traverse_right && rhs.traverse_right,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct TernaryConditionalExpressionTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_conseq: bool,
    pub traverse_alt: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for TernaryConditionalExpressionTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_conseq: value,
            traverse_alt: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_cond || 
        self.traverse_conseq || 
        self.traverse_alt || 
        self.traverse_errors
    }
}

impl BitAnd for TernaryConditionalExpressionTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_conseq: self.traverse_conseq && rhs.traverse_conseq,
            traverse_alt: self.traverse_alt && rhs.traverse_alt,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}




#[derive(Debug, Clone)]
pub struct RootTraversalPolicy {
    pub traverse: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for RootTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse || 
        self.traverse_errors
    }
}

impl BitAnd for RootTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse: self.traverse && rhs.traverse,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct ClassDeclarationTraversalPolicy {
    pub traverse_definition: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for ClassDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_definition: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_definition || 
        self.traverse_errors
    }
}

impl BitAnd for ClassDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_definition: self.traverse_definition && rhs.traverse_definition,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct StateDeclarationTraversalPolicy {
    pub traverse_definition: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for StateDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_definition: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_definition || 
        self.traverse_errors
    }
}

impl BitAnd for StateDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_definition: self.traverse_definition && rhs.traverse_definition,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct StructDeclarationTraversalPolicy {
    pub traverse_definition: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for StructDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_definition: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_definition || 
        self.traverse_errors
    }
}

impl BitAnd for StructDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_definition: self.traverse_definition && rhs.traverse_definition,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct EnumDeclarationTraversalPolicy {
    pub traverse_definition: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for EnumDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_definition: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_definition || 
        self.traverse_errors
    }
}

impl BitAnd for EnumDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_definition: self.traverse_definition && rhs.traverse_definition,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct EnumVariantDeclarationTraversalPolicy {
    pub traverse_errors: bool
}

impl TraversalPolicy for EnumVariantDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_errors
    }
}

impl BitAnd for EnumVariantDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct MemberVarDeclarationTraversalPolicy {
    pub traverse_errors: bool
}

impl TraversalPolicy for MemberVarDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_errors
    }
}

impl BitAnd for MemberVarDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct MemberDefaultValueTraversalPolicy {
    pub traverse_value: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for MemberDefaultValueTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_value || 
        self.traverse_errors
    }
}

impl BitAnd for MemberDefaultValueTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct MemberDefaultsBlockTraversalPolicy {
    pub traverse: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for MemberDefaultsBlockTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse || 
        self.traverse_errors
    }
}

impl BitAnd for MemberDefaultsBlockTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse: self.traverse && rhs.traverse,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct MemberHintTraversalPolicy {
    pub traverse_errors: bool
}

impl TraversalPolicy for MemberHintTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_errors
    }
}

impl BitAnd for MemberHintTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct AutobindDeclarationTraversalPolicy {
    pub traverse_errors: bool
}

impl TraversalPolicy for AutobindDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_errors
    }
}

impl BitAnd for AutobindDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct FunctionParameterGroupTraversalPolicy {
    pub traverse_errors: bool
}

impl TraversalPolicy for FunctionParameterGroupTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_errors
    }
}

impl BitAnd for FunctionParameterGroupTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct FunctionDeclarationTraversalPolicy {
    pub traverse_params: bool,
    pub traverse_definition: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for FunctionDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_params: value,
            traverse_definition: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_params || 
        self.traverse_definition || 
        self.traverse_errors
    }
}

impl BitAnd for FunctionDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_params: self.traverse_params && rhs.traverse_params,
            traverse_definition: self.traverse_definition && rhs.traverse_definition,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct EventDeclarationTraversalPolicy {
    pub traverse_params: bool,
    pub traverse_definition: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for EventDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_params: value,
            traverse_definition: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_params || 
        self.traverse_definition || 
        self.traverse_errors
    }
}

impl BitAnd for EventDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_params: self.traverse_params && rhs.traverse_params,
            traverse_definition: self.traverse_definition && rhs.traverse_definition,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}




#[derive(Debug, Clone)]
pub struct ForLoopTraversalPolicy {
    pub traverse_init: bool,
    pub traverse_cond: bool,
    pub traverse_iter: bool,
    pub traverse_body: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for ForLoopTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_init: value,
            traverse_cond: value,
            traverse_iter: value,
            traverse_body: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_init || 
        self.traverse_cond || 
        self.traverse_iter || 
        self.traverse_body || 
        self.traverse_errors
    }
}

impl BitAnd for ForLoopTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_init: self.traverse_init && rhs.traverse_init,
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_iter: self.traverse_iter && rhs.traverse_iter,
            traverse_body: self.traverse_body && rhs.traverse_body,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct WhileLoopTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_body: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for WhileLoopTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_body: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_cond || 
        self.traverse_body || 
        self.traverse_errors
    }
}

impl BitAnd for WhileLoopTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_body: self.traverse_body && rhs.traverse_body,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct DoWhileLoopTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_body: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for DoWhileLoopTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_body: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_cond || 
        self.traverse_body || 
        self.traverse_errors
    }
}

impl BitAnd for DoWhileLoopTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_body: self.traverse_body && rhs.traverse_body,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct IfConditionalTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_body: bool,
    pub traverse_else_body: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for IfConditionalTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_body: value,
            traverse_else_body: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_cond || 
        self.traverse_body || 
        self.traverse_else_body || 
        self.traverse_errors
    }
}

impl BitAnd for IfConditionalTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_body: self.traverse_body && rhs.traverse_body,
            traverse_else_body: self.traverse_else_body && rhs.traverse_else_body,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalTraversalPolicy {
    pub traverse_cond: bool,
    pub traverse_body: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for SwitchConditionalTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_cond: value,
            traverse_body: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_cond || 
        self.traverse_body || 
        self.traverse_errors
    }
}

impl BitAnd for SwitchConditionalTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_cond: self.traverse_cond && rhs.traverse_cond,
            traverse_body: self.traverse_body && rhs.traverse_body,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalCaseLabelTraversalPolicy {
    pub traverse_value: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for SwitchConditionalCaseLabelTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_value || 
        self.traverse_errors
    }
}

impl BitAnd for SwitchConditionalCaseLabelTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct SwitchConditionalDefaultLabelTraversalPolicy {
    pub traverse_errors: bool
}

impl TraversalPolicy for SwitchConditionalDefaultLabelTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_errors
    }
}

impl BitAnd for SwitchConditionalDefaultLabelTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct BreakStatementTraversalPolicy {
    pub traverse_errors: bool
}

impl TraversalPolicy for BreakStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_errors
    }
}

impl BitAnd for BreakStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct ContinueStatementTraversalPolicy {
    pub traverse_errors: bool
}

impl TraversalPolicy for ContinueStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_errors
    }
}

impl BitAnd for ContinueStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct CompoundStatementTraversalPolicy {
    pub traverse: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for CompoundStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse || 
        self.traverse_errors
    }
}

impl BitAnd for CompoundStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse: self.traverse && rhs.traverse,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct VarDeclarationTraversalPolicy {
    pub traverse_init_value: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for VarDeclarationTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_init_value: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_init_value || 
        self.traverse_errors
    }
}

impl BitAnd for VarDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_init_value: self.traverse_init_value && rhs.traverse_init_value,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct ExpressionStatementTraversalPolicy {
    pub traverse_expr: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for ExpressionStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_expr: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_expr || 
        self.traverse_errors
    }
}

impl BitAnd for ExpressionStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_expr: self.traverse_expr && rhs.traverse_expr,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct ReturnStatementTraversalPolicy {
    pub traverse_value: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for ReturnStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_value || 
        self.traverse_errors
    }
}

impl BitAnd for ReturnStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}


#[derive(Debug, Clone)]
pub struct DeleteStatementTraversalPolicy {
    pub traverse_value: bool,
    pub traverse_errors: bool
}

impl TraversalPolicy for DeleteStatementTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse_value: value,
            traverse_errors: value
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse_value || 
        self.traverse_errors
    }
}

impl BitAnd for DeleteStatementTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_value: self.traverse_value && rhs.traverse_value,
            traverse_errors: self.traverse_errors && rhs.traverse_errors
        }
    }
}




#[derive(Debug, Clone)]
pub struct ErrorTraversalPolicy {
    pub traverse: bool
    // traverse_errors is implicitly true
}

impl TraversalPolicy for ErrorTraversalPolicy {
    #[inline(always)]
    fn default_to(value: bool) -> Self {
        Self {
            traverse: value,
        }
    }

    #[inline]
    fn any(&self) -> bool {
        self.traverse
    }
}

impl BitAnd for ErrorTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse: self.traverse && rhs.traverse
        }
    }
}