use crate::symbol_analysis::symbols::*;
use crate::symbol_analysis::symbol_table::SymbolTable;


/// Making assumptions that actual types start with capital letter and aliases are always lower case.
/// If only lower case can be found in vanilla code, then the type name is a guess.
pub fn inject_primitives(symtab: &mut SymbolTable) {
    [
        PrimitiveTypeSymbol::new("Void", Some("void")),
        PrimitiveTypeSymbol::new("Byte", Some("byte")),
        PrimitiveTypeSymbol::new("Int8", None),
        PrimitiveTypeSymbol::new("Int32", Some("int")),
        PrimitiveTypeSymbol::new("UInt64", None),
        PrimitiveTypeSymbol::new("Float", Some("float")),
        PrimitiveTypeSymbol::new("Bool", Some("bool")),
        PrimitiveTypeSymbol::new("String", Some("string")),
        PrimitiveTypeSymbol::new("CName", Some("name")),
        PrimitiveTypeSymbol::new("NULL", None),

    ].into_iter()
    .for_each(|sym| {
        symtab.insert_primitive(sym);
    });
}


/// Globally available script variables with "the" prefix.
/// Not all of them are used in scripts, but they're all written down in bin/config/redscripts.ini.
pub fn inject_globals(symtab: &mut SymbolTable) {
    // Key is global's name, value is variable's type.
    [
        ("theGame", "CR4Game"),
        ("theServer", "CServerInterface"),
        ("thePlayer", "CR4Player"),
        ("theCamera", "CCamera"),
        ("theUI", "CGuiWitcher"),
        ("theSound", "CScriptSoundSystem"),
        ("theDebug", "CDebugAttributesManager"),
        ("theTimer", "CTimerScriptKeyword"),
        ("theInput", "CInputManager"),
        ("theTelemetry", "CR4TelemetryScriptProxy")

    ].into_iter()
    .for_each(|(var_name, class_name)| { 
        let gv = GlobalVarSymbol::new(var_name, BasicTypeSymbolPath::new(class_name));
        symtab.insert(gv); 
    });
}
