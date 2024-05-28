use crate::symbol_analysis::symbol_path::SymbolPath;
use super::*;


#[derive(Debug, Clone)]
pub struct ArrayTypeSymbol {
    path: ArrayTypeSymbolPath
}

impl Symbol for ArrayTypeSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Array
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl ArrayTypeSymbol {
    pub const TYPE_NAME: &'static str = "array";
    pub const INDEX_OPERATOR_NAME: &'static str = "operator[]";

    pub fn new(path: ArrayTypeSymbolPath) -> Self {
        Self {
            path
        }
    }

    pub fn data_type_path(&self) -> &TypeSymbolPath {
        &self.path.type_arg_path
    }

    pub fn make_functions(&self) -> (Vec<ArrayTypeFunctionSymbol>, Vec<ArrayTypeFunctionParameterSymbol>) {
        let void_path = TypeSymbolPath::BasicOrState(BasicTypeSymbolPath::new("void"));
        let int_path = TypeSymbolPath::BasicOrState(BasicTypeSymbolPath::new("int"));
        let bool_path = TypeSymbolPath::BasicOrState(BasicTypeSymbolPath::new("bool"));

        let mut funcs = Vec::new();
        let mut params = Vec::new();

        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, Self::INDEX_OPERATOR_NAME),
                return_type_path: self.data_type_path().clone(),
                was_return_type_generic: true 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "index"),
                type_path: int_path.clone(),
                was_type_generic: false,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "Clear"),
                return_type_path: void_path.clone(),
                was_return_type_generic: false 
            };
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "Size"),
                return_type_path: int_path.clone(),
                was_return_type_generic: false 
            };
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "PushBack"),
                return_type_path: self.data_type_path().clone(),
                was_return_type_generic: true 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "element"),
                type_path: self.data_type_path().clone(),
                was_type_generic: true,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "Resize"),
                return_type_path: void_path.clone(),
                was_return_type_generic: false 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "newSize"),
                type_path: int_path.clone(),
                was_type_generic: false,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "Remove"),
                return_type_path: bool_path.clone(),
                was_return_type_generic: false 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "element"),
                type_path: self.data_type_path().clone(),
                was_type_generic: true,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "Contains"),
                return_type_path: bool_path.clone(),
                was_return_type_generic: false 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "element"),
                type_path: self.data_type_path().clone(),
                was_type_generic: true,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "FindFirst"),
                return_type_path: int_path.clone(),
                was_return_type_generic: false 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "element"),
                type_path: self.data_type_path().clone(),
                was_type_generic: true,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "FindLast"),
                return_type_path: int_path.clone(),
                was_return_type_generic: false 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "element"),
                type_path: self.data_type_path().clone(),
                was_type_generic: true,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "Grow"),
                return_type_path: int_path.clone(),
                was_return_type_generic: false 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "numElements"),
                type_path: int_path.clone(),
                was_type_generic: false,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "Erase"),
                return_type_path: void_path.clone(),
                was_return_type_generic: false 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "index"),
                type_path: int_path.clone(),
                was_type_generic: false,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "EraseFast"),
                return_type_path: void_path.clone(),
                was_return_type_generic: false 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "index"),
                type_path: int_path.clone(),
                was_type_generic: false,
                ordinal: 0 
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "Insert"),
                return_type_path: void_path.clone(),
                was_return_type_generic: false 
            };
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "index"),
                type_path: int_path.clone(),
                was_type_generic: false,
                ordinal: 0 
            };
            params.push(p);
            let p = ArrayTypeFunctionParameterSymbol {
                path: MemberDataSymbolPath::new(&f.path(), "element"),
                type_path: self.data_type_path().clone(),
                was_type_generic: true,
                ordinal: 1
            };
            params.push(p);
            funcs.push(f);
        }
        {
            let f = ArrayTypeFunctionSymbol {
                path: MemberCallableSymbolPath::new(&self.path, "Last"),
                return_type_path: self.data_type_path().clone(),
                was_return_type_generic: true
            };
            funcs.push(f);
        }

        (funcs, params)
    }
}


#[derive(Debug, Clone)]
pub struct ArrayTypeFunctionSymbol {
    path: MemberCallableSymbolPath,
    pub return_type_path: TypeSymbolPath,
    pub was_return_type_generic: bool
}

impl Symbol for ArrayTypeFunctionSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::MemberFunction
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl ArrayTypeFunctionSymbol {
    pub fn return_type_name(&self) -> &str {
        self.return_type_path.components().next().map(|c| c.name).unwrap_or_default()
    }
}


#[derive(Debug, Clone)]
pub struct ArrayTypeFunctionParameterSymbol {
    path: MemberDataSymbolPath,
    pub type_path: TypeSymbolPath,
    pub was_type_generic: bool,
    pub ordinal: usize
}

impl Symbol for ArrayTypeFunctionParameterSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Parameter
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl ArrayTypeFunctionParameterSymbol {
    pub fn type_name(&self) -> &str {
        self.type_path.components().next().map(|c| c.name).unwrap_or_default()
    }
}