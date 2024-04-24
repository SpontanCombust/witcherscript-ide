use std::ops::BitAnd;

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

impl Default for FunctionCallExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_func: true, 
            traverse_args: true
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

impl Default for FunctionCallArgumentTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_expr: true 
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

impl Default for ArrayExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_accessor: true, 
            traverse_index: true 
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

impl Default for MemberFieldExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_accessor: true 
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

impl Default for NewExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_lifetime_obj: true 
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

impl Default for TypeCastExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_value: true 
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

impl Default for UnaryOperationExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_right: true 
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

impl Default for BinaryOperationExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_left: true, 
            traverse_right: true 
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

impl Default for AssignmentOperationExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_left: true, 
            traverse_right: true 
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

impl Default for TernaryConditionalExpressionTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_cond: true, 
            traverse_conseq: true, 
            traverse_alt: true 
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

impl Default for RootTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse: true 
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
    //TODO? continue_traversing_root: bool and similar
}

impl Default for ClassDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_definition: true 
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

impl Default for StateDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_definition: true 
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

impl Default for StructDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_definition: true 
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

impl Default for EnumDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_definition: true 
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
    pub traverse_params: bool
}

impl Default for GlobalFunctionDeclarationTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse_params: true
        }
    }
}

impl BitAnd for GlobalFunctionDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_params: self.traverse_params && rhs.traverse_params
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

impl BitAnd for MemberFunctionDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_params: self.traverse_params && rhs.traverse_params
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

impl BitAnd for EventDeclarationTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_params: self.traverse_params && rhs.traverse_params
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

impl BitAnd for ForLoopTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_body: self.traverse_body && rhs.traverse_body
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

impl BitAnd for WhileLoopTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_body: self.traverse_body && rhs.traverse_body
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

impl BitAnd for DoWhileLoopTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_body: self.traverse_body && rhs.traverse_body
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

impl BitAnd for IfConditionalTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_body: self.traverse_body && rhs.traverse_body,
            traverse_else_body: self.traverse_else_body && rhs.traverse_else_body
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

impl BitAnd for SwitchConditionalTraversalPolicy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            traverse_body: self.traverse_body && rhs.traverse_body
        }
    }
}


#[derive(Debug, Clone)]
pub struct CompoundStatementTraversalPolicy {
    pub traverse: bool
}

impl Default for CompoundStatementTraversalPolicy {
    fn default() -> Self {
        Self { 
            traverse: true 
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
