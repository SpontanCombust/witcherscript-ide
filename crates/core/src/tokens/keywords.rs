use strum_macros::{EnumString, Display, AsRefStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, Display, AsRefStr)]
#[strum(serialize_all="camelCase")]
pub enum Keyword {
    Abstract,
    Autobind,
    Break,
    Case,
    Class,
    Cleanup,
    Const,
    Continue,
    Default,
    Defaults,
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
    Reward,
    Saved,
    Single,
    State,
    Statemachine,
    Storyscene,
    Struct,
    Super,
    Switch,
    // global var keywords
    TheCamera,
    TheDebug,
    TheGame,
    TheInput,
    ThePlayer,
    TheServer,
    TheSound,
    TheTelemetry,
    TheTimer,
    TheUI,
    //
    This,
    Timer,
    True,
    Var,
    #[strum(serialize="virtual_parent")]
	VirtualParent,
    While,
}

impl Keyword {
    pub fn is_global_var(&self) -> bool {
        use Keyword::*;
        match self {
            TheCamera 
            | TheDebug
            | TheGame
            | TheInput
            | ThePlayer
            | TheServer
            | TheSound
            | TheTelemetry
            | TheTimer
            | TheUI => true,
            _ => false
        }
    }
}