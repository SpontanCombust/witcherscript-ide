use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    // Types that can be used in the code, but are not explicitly imported
    pub static ref NATIVE_TYPES: HashSet<&'static str> = {
        HashSet::from([
            // primitives
            "void",
            "byte", "int", "Int8", "Int32", "Uint64",
            "float", "Float", 
            "bool", "Bool", 
            // string-like types
            "string", "String", 
            "name", "CName",
            // other (class/struct/enum)
            //TODO complex types need to be inserted together with their properties
            // "array", 
            // "CGUID",
            // "EngineQsTransform",
            // "ISerializable",
            // "EInputKey"
        ])  
    };
}
