use uuid::Uuid;
use crate::model::{symbols::{PrimitiveTypeSymbol, GlobalVarSymbol, ArrayTypeSymbol}, collections::{SymbolTable, SymbolDb}};

/// Should be called at the start, before parsing WS files.
/// 
/// Making assumptions that actual types start with capital letter and aliases are always lower case.
/// If only lower case can be found in vanilla code, then the type name is a guess.
pub fn inject_primitives(db: &mut SymbolDb) {
    [
        PrimitiveTypeSymbol::new_with_alias("Void", Some("void")),
        PrimitiveTypeSymbol::new_with_alias("Byte", Some("byte")),
        PrimitiveTypeSymbol::new_with_alias("Int8", None),
        PrimitiveTypeSymbol::new_with_alias("Int32", Some("int")),
        PrimitiveTypeSymbol::new_with_alias("UInt64", None),
        PrimitiveTypeSymbol::new_with_alias("Float", Some("float")),
        PrimitiveTypeSymbol::new_with_alias("Bool", Some("bool")),
        PrimitiveTypeSymbol::new_with_alias("String", Some("string")),
        PrimitiveTypeSymbol::new_with_alias("CName", Some("name")),

    ].into_iter()
    .for_each(|sym| { db.primitives.insert(sym.id(), sym); });
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
        let gv = GlobalVarSymbol::new_with_type(var_name, symtab.get(class_name).unwrap().id);
        db.global_vars.insert(gv.id(), gv); 
    });
}


/// Called when coming accross an array type that hasn't been inserted into DB yet.
/// Assumes the data type is not some error type.
pub fn inject_array_type(db: &mut SymbolDb, symtab: &SymbolTable, data_type_id: Uuid, data_type_name: &str) {
    let void_id = symtab.get("void").unwrap().id;
    let int_id = symtab.get("int").unwrap().id;
    let bool_id = symtab.get("bool").unwrap().id;

    let (arr, funcs, params) = ArrayTypeSymbol::new_with_type(data_type_id, data_type_name, void_id, int_id, bool_id);
    db.arrays.insert(arr.id(), arr);
    funcs.into_iter().for_each(|f| { db.member_funcs.insert(f.id(), f); } );
    params.into_iter().for_each(|p| { db.params.insert(p.id(), p); } );
}
