//! Types that allow for disambiguation during node traversal
//! e.g. if a member var declaration is visited it is supplied with a context whether it is visited inside
//! a class, a state or a struct.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalContext {
    Global,

    Class,
    State,
    Struct,
    Enum,
    GlobalFunction,

    MemberFunction,
    Event,
    MemberDefaultValue,

    LocalVarDeclarationInitValue,

    ExpressionStatement,
    CompoundStatement,
    ReturnStatement,
    DeleteStatement,

    IfConditionalCond,
    IfConditionalBody,
    IfConditionalElseBody,

    SwitchConditionalCond,
    SwitchConditionalBody,
    SwitchConditionalCaseLabel,

    ForLoopInit,
    ForLoopCond,
    ForLoopIter,
    ForLoopBody,

    WhileLoopCond,
    WhileLoopBody,

    DoWhileLoopCond,
    DoWhileLoopBody, 

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

impl Default for TraversalContext {
    fn default() -> Self {
        Self::Global
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraversalContextStack {
    stack: Vec<TraversalContext>
}

impl TraversalContextStack {
    #[inline]
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(8)
        }
    }

    #[inline]
    pub fn push(&mut self, ctx: TraversalContext) {
        self.stack.push(ctx);
    }

    #[inline]
    pub fn pop(&mut self) {
        self.stack.pop();
    }

    #[inline]
    pub fn top(&self) -> TraversalContext {
        self.stack.last().map(|ctx| *ctx).unwrap_or_default()
    }

    #[inline]
    pub fn contains(&self, ctx: TraversalContext) -> bool {
        self.stack.contains(&ctx)
    }

    /// Iterate the stack top-to-bottom
    #[inline]
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = TraversalContext> + 'a {
        self.stack.iter().rev().cloned()
    }
}

impl Default for TraversalContextStack {
    fn default() -> Self {
        Self::new()
    }
}
