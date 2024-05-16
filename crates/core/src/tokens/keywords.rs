use strum_macros::{EnumString, Display, AsRefStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, Display, AsRefStr)]
#[strum(serialize_all="snake_case")]
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
    #[strum(serialize="theCamera")]
    TheCamera,
    #[strum(serialize="theDebug")]
    TheDebug,
    #[strum(serialize="theGame")]
    TheGame,
    #[strum(serialize="theInput")]
    TheInput,
    #[strum(serialize="thePlayer")]
    ThePlayer,
    #[strum(serialize="theServer")]
    TheServer,
    #[strum(serialize="theSound")]
    TheSound,
    #[strum(serialize="theTelemetry")]
    TheTelemetry,
    #[strum(serialize="theTimer")]
    TheTimer,
    #[strum(serialize="theUI")]
    TheUI,
    //
    This,
    Timer,
    True,
    Var,
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