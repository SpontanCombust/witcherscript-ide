//! Types that allow for disambiguation during node traversal
//! e.g. if a member var declaration is visited it is supplied with a context whether it is visited inside
//! a class, a state or a struct.


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

    NestedExpressionInner,
    FunctionCallExpressionFunc,
    FunctionCallArg,
    ArrayExpressionAccessor,
    ArrayExpressionIndex,
    MemberFieldExpressionAccessor,
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
pub enum PropertyTraversalContext {
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
