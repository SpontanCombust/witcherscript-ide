//! Types that allow for disambiguation during node traversal
//! e.g. if a member var declaration is visited it is supplied with a context whether it is visited inside
//! a class, a state or a struct.


//TODO maybe replace separate enums with just one, but pass around a stack of these contexts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionTraversalContext {
    MemberDefaultValue,
    
    ExpressionStatement,
    ReturnStatement,
    DeleteStatement,
    ForLoopInit,
    ForLoopCond,
    ForLoopIter,
    WhileLoopCond,
    DoWhileLoopCond,
    IfConditionalCond,
    SwitchConditionalCond,
    SwitchConditionalCaseLabel,
    LocalVarDeclarationInitValue,

    NestedExpressionInner,
    FunctionCallExpressionFunc,
    FunctionCallArg,
    ArrayExpressionAccessor,
    ArrayExpressionIndex,
    MemberAccessExpressionAccessor,
    NewExpressionLifetimeObj,
    TypeCastExpressionValue,
    UnaryOperationExpressionRight,
    BinaryOperationExpressionLeft,
    BinaryOperationExpressionRight,
    AssignmentOperationExpressionLeft,
    AssignmentOperationExpressionRight,
    TernaryConditionalExpressionCond,
    TernaryConditionalExpressionConseq,
    TernaryConditionalExpressionAlt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationTraversalContext {
    Global,
    ClassDefinition,
    StateDefinition,
    StructDefinition
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionTraversalContext {
    GlobalFunction,
    MemberFunction,
    Event
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatementTraversalContext {
    GlobalFunctionDefinition,
    MemberFunctionDefinition,
    EventDefinition,

    IfConditionalBody,
    IfConditionalElseBody,
    SwitchConditionalBody,
    ForLoopBody,
    WhileLoopBody,
    DoWhileLoopBody,
    InCompoundStatement
}
