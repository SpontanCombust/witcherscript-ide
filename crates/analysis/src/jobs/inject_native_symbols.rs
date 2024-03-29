use crate::model::symbol_path::SymbolPath;
use crate::model::symbols::*;
use crate::model::collections::symbol_table::SymbolTable;


/// Should be called at the start, before parsing WS files.
/// 
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

    ].into_iter()
    .for_each(|sym| {
        symtab.insert(sym).unwrap();
    });
}


/// Should be called after injecting primitives.
pub fn inject_misc_native_types(symtab: &mut SymbolTable) {
    //TODO put the rest that could actually be declared in a script file; include!() that file and parse it to get symbols 
    todo!()
    // "CGUID",
    // "EngineQsTransform",
    // "ISerializable",
    // "EInputKey"
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
        let gv = GlobalVarSymbol::new(var_name, SymbolPath::new(class_name, SymbolCategory::Type));
        symtab.insert(gv).unwrap(); 
    });
}


/// Should be called when coming accross an array type that hasn't been inserted into symtab yet.
/// Assumes the data type is not some error type and corresponding array type does not yet exist in the symbol table.
/// Use ArrayTypeSymbol::path_for to get the path to array's symbol.
pub fn inject_array_type(symtab: &mut SymbolTable, data_type_path: ArrayTypeSymbolPath) {
    let void_path = TypeSymbolPath::Basic(BasicTypeSymbolPath::new("void"));
    let int_path = TypeSymbolPath::Basic(BasicTypeSymbolPath::new("int"));
    let bool_path = TypeSymbolPath::Basic(BasicTypeSymbolPath::new("bool"));

    let arr = ArrayTypeSymbol::new(data_type_path);
    let (funcs, params) = arr.make_functions(&void_path, &int_path, &bool_path);
    symtab.insert(arr).unwrap();
    funcs.into_iter().for_each(|f| { symtab.insert(f).unwrap(); } );
    params.into_iter().for_each(|p| { symtab.insert(p).unwrap(); } );
}
