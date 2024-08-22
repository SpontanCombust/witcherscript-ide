use std::ops::BitAnd;


pub trait TraversalPolicy: Sized + std::ops::BitAnd<Output = Self> {
    fn default_to(value: bool) -> Self;
    fn any(&self) -> bool;
}


macro_rules! traversal_policy {
    ($Type: ident, $field0: ident $(, $fields: ident)*) => {
        #[derive(Debug, Clone)]
        pub struct $Type {
            pub $field0: bool,
            $(pub $fields: bool,)*
        }

        impl TraversalPolicy for $Type {
            #[inline]
            fn default_to(value: bool) -> Self {
                Self {
                    $field0: value,
                    $($fields: value,)*
                }
            }
            
            #[inline]
            fn any(&self) -> bool {
                self.$field0
                $(|| self.$fields)*
            }
        }

        impl BitAnd for $Type {
            type Output = Self;
        
            fn bitand(self, rhs: Self) -> Self::Output {
                Self {
                    $field0: self.$field0 && rhs.$field0,
                    $($fields: self.$fields && rhs.$fields,)*
                }
            }
        }
    };
}



traversal_policy!(NestedExpressionTraversalPolicy,
    traverse_inner,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(FunctionCallExpressionTraversalPolicy,
    traverse_func,
    traverse_args,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(FunctionCallArgumentTraversalPolicy,
    traverse_expr
);

traversal_policy!(ArrayExpressionTraversalPolicy,
    traverse_accessor,
    traverse_index,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(MemberFieldExpressionTraversalPolicy,
    traverse_accessor,
    traverse_errors
);

traversal_policy!(NewExpressionTraversalPolicy,
    traverse_lifetime_obj,
    traverse_errors
);

traversal_policy!(TypeCastExpressionTraversalPolicy,
    traverse_value,
    traverse_errors
);

traversal_policy!(UnaryOperationExpressionTraversalPolicy,
    traverse_right,
    traverse_errors
);

traversal_policy!(BinaryOperationExpressionTraversalPolicy,
    traverse_left,
    traverse_right,
    traverse_errors
);

traversal_policy!(AssignmentOperationExpressionTraversalPolicy,
    traverse_left,
    traverse_right,
    traverse_errors
);

traversal_policy!(TernaryConditionalExpressionTraversalPolicy,
    traverse_cond,
    traverse_conseq,
    traverse_alt,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(ArrayInitializerExpressionTraversalPolicy,
    traverse_items,
    traverse_unnamed,
    traverse_errors
);



traversal_policy!(RootTraversalPolicy,
    traverse_statements,
    traverse_errors
);

traversal_policy!(ClassDeclarationTraversalPolicy,
    traverse_definition,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(StateDeclarationTraversalPolicy,
    traverse_definition,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(StructDeclarationTraversalPolicy,
    traverse_definition,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(EnumDeclarationTraversalPolicy,
    traverse_definition,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(EnumVariantDeclarationTraversalPolicy,
    traverse_unnamed,
    traverse_errors
);



traversal_policy!(MemberVarDeclarationTraversalPolicy,
    traverse_annotation,
    traverse_type,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(MemberDefaultValueTraversalPolicy,
    traverse_value,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(MemberDefaultsBlockTraversalPolicy,
    traverse_assignments,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(MemberHintTraversalPolicy,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(AutobindDeclarationTraversalPolicy,
    traverse_type,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(FunctionParameterGroupTraversalPolicy,
    traverse_type,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(FunctionDeclarationTraversalPolicy,
    traverse_annotation,
    traverse_params,
    traverse_return_type,
    traverse_definition,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(EventDeclarationTraversalPolicy,
    traverse_params,
    traverse_return_type,
    traverse_definition,
    traverse_unnamed,
    traverse_errors
);



traversal_policy!(ForLoopTraversalPolicy,
    traverse_init,
    traverse_cond,
    traverse_iter,
    traverse_body,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(WhileLoopTraversalPolicy,
    traverse_cond,
    traverse_body,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(DoWhileLoopTraversalPolicy,
    traverse_cond,
    traverse_body,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(IfConditionalTraversalPolicy,
    traverse_cond,
    traverse_body,
    traverse_else_body,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(SwitchConditionalTraversalPolicy,
    traverse_cond,
    traverse_body,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(SwitchConditionalCaseLabelTraversalPolicy,
    traverse_value,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(SwitchConditionalDefaultLabelTraversalPolicy,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(BreakStatementTraversalPolicy,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(ContinueStatementTraversalPolicy,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(CompoundStatementTraversalPolicy,
    traverse_statements,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(VarDeclarationTraversalPolicy,
    traverse_type,
    traverse_init_value,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(ExpressionStatementTraversalPolicy,
    traverse_expr,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(ReturnStatementTraversalPolicy,
    traverse_value,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(DeleteStatementTraversalPolicy,
    traverse_value,
    traverse_unnamed,
    traverse_errors
);



traversal_policy!(TypeAnnotationTraversalPolicy,
    traverse_type_arg,
    traverse_unnamed,
    traverse_errors
);

traversal_policy!(AnnotationTraversalPolicy,
    traverse_unnamed,
    traverse_errors
);



traversal_policy!(ErrorTraversalPolicy,
    traverse
    // traverse_errors is implicitly true
);
