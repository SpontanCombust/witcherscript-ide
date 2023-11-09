use strum_macros::{EnumString, Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, Display)]
#[strum(serialize_all="snake_case")]
pub enum Keyword {
    Abstract,
    Autobind,
    Break,
    Case,
    Class,
    Const,
    Continue,
    Default,
    Delete,
    Do,
    Editable,
    Else,
    Entry,
    Enum,
    Event,
    Exec,
    Extends,
    False,
    Final,
    For,
    Function,
    Hint,
    If,
    In,
    Inlined,
    Import,
    Latent,
    New,
    #[strum(serialize="NULL")]
    Null,
    Optional,
    Out,
    Parent,
    Private,
    Protected,
    Public,
    Quest,
    Return,
    Saved,
    Single,
    State,
    Statemachine,
    Storyscene,
    Struct,
    Super,
    Switch,
    This,
    Timer,
    True,
    Var,
    While,
    VirtualParent,
}