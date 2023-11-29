use uuid::Uuid;
use crate::model::{symbols::{PrimitiveTypeSymbol, ArrayTypeSymbol, Symbol, GlobalVarSymbol}, collections::{SymbolTable, SymbolDb}};

/// Should be called at the start, before parsing WS files.
/// 
/// Making assumptions that actual types start with capital letter and aliases are always lower case.
/// If only lower case can be found in vanilla code, then the type name is a guess.
pub fn inject_primitives(db: &mut SymbolDb) {
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
    .for_each(|sym| { db.primitives.insert(sym.symbol_id(), sym); });
}


/// Should be called after injecting primitives.
pub fn inject_misc_native_types(db: &mut SymbolDb, symtab: &SymbolTable) {
    //TODO put the rest that could actually be declared in a script file; include!() that file and parse it to get symbols 
    todo!()
    // "CGUID",
    // "EngineQsTransform",
    // "ISerializable",
    // "EInputKey"
}


/// Should be called after collecting all types from the codebase.
/// 
/// Globally available script variables with "the" prefix.
/// Not all of them are used in scripts, but they're all written down in bin/config/redscripts.ini.
/// Key is global's name, value is variable's type.
pub fn inject_globals(db: &mut SymbolDb, symtab: &SymbolTable) {
    [
        ("theGame", "CR4Game"),
        ("theServer", "CServerInterface"),
        ("thePlayer", "CR4Player"),
        ("theCamera", "CCamera"),
        ("theUI", "CGuiWitcher"),
        ("theSound", "CScriptSoundSystem"),
        ("theDebug", "CDebugAttributesManager"),
        ("theTimer", "CTimerScriptKeyword"),
        ("theInput", "CInputManager")

    ].into_iter()
    .for_each(|(var_name, class_name)| { 
        let gv = GlobalVarSymbol::new(var_name, symtab.get(class_name).unwrap().id);
        db.global_vars.insert(gv.symbol_id(), gv); 
    });
}


/// Called when coming accross an array type that hasn't been inserted into DB yet.
/// Assumes the data type is not some error type.
pub fn inject_array_type(db: &mut SymbolDb, symtab: &SymbolTable, data_type_id: Uuid, data_type_name: &str) {
    let void_id = symtab.get("void").unwrap().id;
    let int_id = symtab.get("int").unwrap().id;
    let bool_id = symtab.get("bool").unwrap().id;

    let arr = ArrayTypeSymbol::new(data_type_id, data_type_name, void_id, int_id, bool_id);
    db.arrays.insert(arr.symbol_id(), arr);
}
