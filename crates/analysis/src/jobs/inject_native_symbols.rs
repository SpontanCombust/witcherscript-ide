use crate::symbol_analysis::symbols::*;
use crate::symbol_analysis::symbol_table::SymbolTable;


/// Making assumptions that actual types start with capital letter and aliases are always lower case.
/// If only lower case can be found in vanilla code, then the type name is a guess.
pub fn inject_primitives(symtab: &mut SymbolTable) {
    [
        PrimitiveTypeSymbol::new("void", None),
        PrimitiveTypeSymbol::new("Byte", None),
        PrimitiveTypeSymbol::new("byte", Some("Byte")),
        PrimitiveTypeSymbol::new("Int8", None),
        PrimitiveTypeSymbol::new("Int32", None),
        PrimitiveTypeSymbol::new("int", Some("Int32")),
        PrimitiveTypeSymbol::new("Uint64", None),
        PrimitiveTypeSymbol::new("Float", None),
        PrimitiveTypeSymbol::new("float", Some("Float")),
        PrimitiveTypeSymbol::new("Bool", None),
        PrimitiveTypeSymbol::new("bool", Some("Bool")),
        PrimitiveTypeSymbol::new("String", None),
        PrimitiveTypeSymbol::new("string", Some("String")),
        PrimitiveTypeSymbol::new("CName", None),
        PrimitiveTypeSymbol::new("name", Some("CName")),
        PrimitiveTypeSymbol::new("NULL", None),

    ].into_iter()
    .for_each(|sym| {
        symtab.insert_symbol(sym);
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
        symtab.insert_symbol(gv); 
    });
}
