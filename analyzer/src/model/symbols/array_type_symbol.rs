use uuid::Uuid;
use super::{MemberFunctionSymbol, Symbol, SymbolType, NATIVE_SYMBOL_SCRIPT_ID, FunctionParameterSymbol};


#[derive(Debug, Clone)]
pub struct ArrayTypeSymbol {
    symbol_id: Uuid,
    name: String,
    data_type_id: Uuid,
    func_ids: Vec<Uuid>
}

impl ArrayTypeSymbol {
    pub fn new(data_type_id: Uuid, data_type_name: &str, void_id: Uuid, int_id: Uuid, bool_id: Uuid) -> (Self, Vec<MemberFunctionSymbol>, Vec<FunctionParameterSymbol>) {
        let symbol_id = Uuid::new_v4();
        let mut arr = Self {
            symbol_id,
            name: format!("array<{}>", data_type_name),
            data_type_id,
            func_ids: Vec::new()
        };

        let (funcs, params) = arr.add_functions(void_id, int_id, bool_id);
        (arr, funcs, params)
    }

    fn new_func(&mut self, name: &str) -> MemberFunctionSymbol {
        let f = MemberFunctionSymbol::new(self.symbol_id, name);
        self.func_ids.push(f.symbol_id());
        f
    }

    fn add_functions(&mut self, void_id: Uuid, int_id: Uuid, bool_id: Uuid) -> (Vec<MemberFunctionSymbol>, Vec<FunctionParameterSymbol>) {
        let mut funcs = Vec::new();
        let mut params = Vec::new();

        {
            let mut f = self.new_func("operator[]");
            f.return_type_id = self.data_type_id;
            let mut p = f.add_param("index");
            p.type_id = int_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Clear");
            f.return_type_id = void_id;
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Size");
            f.return_type_id = int_id;
            funcs.push(f);
        }
        {
            let mut f = self.new_func("PushBack");
            f.return_type_id = self.data_type_id;
            let mut p = f.add_param("element");
            p.type_id = self.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Resize");
            f.return_type_id = void_id;
            let mut p = f.add_param("newSize");
            p.type_id = int_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Remove");
            f.return_type_id = bool_id;
            let mut p = f.add_param("element");
            p.type_id = self.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Contains");
            f.return_type_id = bool_id;
            let mut p = f.add_param("element");
            p.type_id = self.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("FindFirst");
            f.return_type_id = int_id;
            let mut p = f.add_param("element");
            p.type_id = self.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("FindLast");
            f.return_type_id = int_id;
            let mut p = f.add_param("element");
            p.type_id = self.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Grow");
            f.return_type_id = int_id;
            let mut p = f.add_param("numElements");
            p.type_id = int_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Erase");
            f.return_type_id = void_id;
            let mut p = f.add_param("index");
            p.type_id = int_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Insert");
            f.return_type_id = void_id;
            let mut p = f.add_param("index");
            p.type_id = int_id;
            params.push(p);
            let mut p = f.add_param("element");
            p.type_id = self.data_type_id;
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = self.new_func("Last");
            f.return_type_id = self.data_type_id;
            funcs.push(f);
        }

        (funcs, params)
    }



    pub fn data_type_id(&self) -> Uuid {
        self.data_type_id
    }

    pub fn funcs(&self) -> &[Uuid] {
        self.func_ids.as_ref()
    }
}

impl Symbol for ArrayTypeSymbol {
    const TYPE: SymbolType = SymbolType::Array;

    fn symbol_id(&self) -> Uuid {
        self.symbol_id
    }

    fn symbol_name(&self) -> &str {
        self.name.as_str()
    }

    fn parent_symbol_id(&self) -> Uuid {
        NATIVE_SYMBOL_SCRIPT_ID
    }
}