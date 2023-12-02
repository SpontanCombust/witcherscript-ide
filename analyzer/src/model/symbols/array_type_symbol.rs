use uuid::Uuid;
use super::{MemberFunctionSymbol, Symbol, SymbolType, NATIVE_SYMBOL_SCRIPT_ID, FunctionParameterSymbol, SymbolData};


#[derive(Debug, Clone, Default)]
pub struct ArrayTypeSymbolData {
    data_type_id: Uuid,
    func_ids: Vec<Uuid>
}

impl ArrayTypeSymbolData {
    pub fn data_type_id(&self) -> Uuid {
        self.data_type_id
    }

    pub fn func_ids(&self) -> &[Uuid] {
        self.func_ids.as_ref()
    }
}

impl SymbolData for ArrayTypeSymbolData {
    const SYMBOL_TYPE: SymbolType = SymbolType::Array;
}

pub type ArrayTypeSymbol = Symbol<ArrayTypeSymbolData>;

impl ArrayTypeSymbol {
    pub fn new_with_type(data_type_id: Uuid, data_type_name: &str, void_id: Uuid, int_id: Uuid, bool_id: Uuid) -> (Self, Vec<MemberFunctionSymbol>, Vec<FunctionParameterSymbol>) {
        let mut arr = Self::new_with_data(
            &format!("array<{}>", data_type_name), 
            NATIVE_SYMBOL_SCRIPT_ID, 
            ArrayTypeSymbolData { 
                data_type_id, 
                func_ids: Vec::new() 
            }
        );
        let (funcs, params) = arr.add_functions(void_id, int_id, bool_id);
        (arr, funcs, params)
    }

    fn new_func(&mut self, name: &str) -> MemberFunctionSymbol {
        let f = MemberFunctionSymbol::new(name, self.id);
        self.data.func_ids.push(f.id);
        f
    }

    fn add_functions(&mut self, void_id: Uuid, int_id: Uuid, bool_id: Uuid) -> (Vec<MemberFunctionSymbol>, Vec<FunctionParameterSymbol>) {
        let mut funcs = Vec::new();
        let mut params = Vec::new();

        {
            let mut f = self.new_func("operator[]");
            f.data.return_type_id = self.data.data_type_id;
            let mut p = f.add_param("index");
            p.data.type_id = int_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Clear");
            f.data.return_type_id = void_id;
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Size");
            f.data.return_type_id = int_id;
            funcs.push(f);
        }
        {
            let mut f = self.new_func("PushBack");
            f.data.return_type_id = self.data.data_type_id;
            let mut p = f.add_param("element");
            p.data.type_id = self.data.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Resize");
            f.data.return_type_id = void_id;
            let mut p = f.add_param("newSize");
            p.data.type_id = int_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Remove");
            f.data.return_type_id = bool_id;
            let mut p = f.add_param("element");
            p.data.type_id = self.data.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Contains");
            f.data.return_type_id = bool_id;
            let mut p = f.add_param("element");
            p.data.type_id = self.data.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("FindFirst");
            f.data.return_type_id = int_id;
            let mut p = f.add_param("element");
            p.data.type_id = self.data.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("FindLast");
            f.data.return_type_id = int_id;
            let mut p = f.add_param("element");
            p.data.type_id = self.data.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Grow");
            f.data.return_type_id = int_id;
            let mut p = f.add_param("numElements");
            p.data.type_id = int_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Erase");
            f.data.return_type_id = void_id;
            let mut p = f.add_param("index");
            p.data.type_id = int_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Insert");
            f.data.return_type_id = void_id;
            let mut p = f.add_param("index");
            p.data.type_id = int_id;
            params.push(p);
            let mut p = f.add_param("element");
            p.data.type_id = self.data.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Last");
            f.data.return_type_id = self.data.data_type_id;
            funcs.push(f);
        }

        (funcs, params)
    }
}