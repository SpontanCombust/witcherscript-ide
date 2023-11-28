use uuid::Uuid;
use super::{MemberFunctionSymbol, Symbol, SymbolType, NATIVE_SYMBOL_SCRIPT_ID};


#[derive(Debug, Clone)]
pub struct ArrayTypeSymbol {
    symbol_id: Uuid,
    name: String,
    data_type_id: Uuid,
    funcs: Vec<MemberFunctionSymbol>
}

impl ArrayTypeSymbol {
    pub fn new(data_type_id: Uuid, data_type_name: &str, void_id: Uuid, int_id: Uuid, bool_id: Uuid) -> Self {
        let symbol_id = Uuid::new_v4();
        let mut arr = Self {
            symbol_id,
            name: format!("array<{}>", data_type_name),
            data_type_id,
            funcs: Vec::new()
        };

        arr.add_functions(void_id, int_id, bool_id);
        arr
    }

    fn new_func(&self, name: &str) -> MemberFunctionSymbol {
        MemberFunctionSymbol::new(self.symbol_id, name)
    }

    fn add_functions(&mut self, void_id: Uuid, int_id: Uuid, bool_id: Uuid) {
        let mut f = self.new_func("operator[]");
        f.return_type_id = self.data_type_id;
        f.add_param("index").type_id = int_id;
        self.funcs.push(f);

        let mut f = self.new_func("Clear");
        f.return_type_id = void_id;
        self.funcs.push(f);

        let mut f = self.new_func("Size");
        f.return_type_id = int_id;
        self.funcs.push(f);

        let mut f = self.new_func("PushBack");
        f.return_type_id = self.data_type_id;
        f.add_param("element").type_id = self.data_type_id;
        self.funcs.push(f);

        let mut f = self.new_func("Resize");
        f.return_type_id = void_id;
        f.add_param("newSize").type_id = int_id;
        self.funcs.push(f);

        let mut f = self.new_func("Remove");
        f.return_type_id = bool_id;
        f.add_param("element").type_id = self.data_type_id;
        self.funcs.push(f);

        let mut f = self.new_func("Contains");
        f.return_type_id = bool_id;
        f.add_param("element").type_id = self.data_type_id;
        self.funcs.push(f);

        let mut f = self.new_func("FindFirst");
        f.return_type_id = int_id;
        f.add_param("element").type_id = self.data_type_id;
        self.funcs.push(f);

        let mut f = self.new_func("FindLast");
        f.return_type_id = int_id;
        f.add_param("element").type_id = self.data_type_id;
        self.funcs.push(f);

        let mut f = self.new_func("Grow");
        f.return_type_id = int_id;
        f.add_param("numElements").type_id = int_id;
        self.funcs.push(f);

        let mut f = self.new_func("Erase");
        f.return_type_id = void_id;
        f.add_param("index").type_id = int_id;
        self.funcs.push(f);

        let mut f = self.new_func("Insert");
        f.return_type_id = void_id;
        f.add_param("index").type_id = int_id;
        f.add_param("element").type_id = self.data_type_id;
        self.funcs.push(f);

        let mut f = self.new_func("Last");
        f.return_type_id = self.data_type_id;
        self.funcs.push(f);
    }



    pub fn data_type_id(&self) -> Uuid {
        self.data_type_id
    }

    pub fn funcs(&self) -> &[MemberFunctionSymbol] {
        self.funcs.as_ref()
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