use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    // globally available script variables with "the" prefix
    // not all of them are used in scripts, but they're all written down in bin/config/redscripts.ini
    // key is global's name, value is variable's type
    pub static ref GLOBALS: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("theGame", "CR4Game"),
            ("theServer", "CServerInterface"),
            ("thePlayer", "CR4Player"),
            ("theCamera", "CCamera"),
            ("theUI", "CGuiWitcher"),
            ("theSound", "CScriptSoundSystem"),
            ("theDebug", "CDebugAttributesManager"),
            ("theTimer", "CTimerScriptKeyword"),
            ("theInput", "CInputManager"),
        ])
    };
}