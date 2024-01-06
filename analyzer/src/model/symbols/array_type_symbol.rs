use crate::model::symbol_path::SymbolPath;
use super::{MemberFunctionSymbol, Symbol, SymbolType, FunctionParameterSymbol, ArrayTypeSymbolPath, MemberCallableSymbolPath, DataSymbolPath};


//TODO for later: remember to remove array type symbol when type from type arg is changed 
#[derive(Debug, Clone)]
pub struct ArrayTypeSymbol {
    path: ArrayTypeSymbolPath
}

pub type ArrayTypeSymbolChild = MemberFunctionSymbol;

impl Symbol for ArrayTypeSymbol {
    fn typ(&self) -> SymbolType {
        SymbolType::Array
    }

    fn path(&self) -> &SymbolPath {
        &self.path
    }
}

impl ArrayTypeSymbol {
    pub const TYPE_NAME: &str = "array";

    pub fn new(path: ArrayTypeSymbolPath) -> Self {
        Self {
            path
        }
    }

    pub fn data_type_path(&self) -> &SymbolPath {
        &self.path.type_arg_path
    }

    pub fn make_functions(&self, void_path: &SymbolPath, int_path: &SymbolPath, bool_path: &SymbolPath) -> (Vec<MemberFunctionSymbol>, Vec<FunctionParameterSymbol>) {
        let mut funcs = Vec::new();
        let mut params = Vec::new();

        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "operator[]"));
            f.return_type_path = self.data_type_path().clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "index"));
            p.type_path = int_path.clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Clear"));
            f.return_type_path = void_path.clone();
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Size"));
            f.return_type_path = int_path.clone();
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "PushBack"));
            f.return_type_path = self.data_type_path().clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"));
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Resize"));
            f.return_type_path = void_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "newSize"));
            p.type_path = int_path.clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Remove"));
            f.return_type_path = bool_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"));
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Contains"));
            f.return_type_path = bool_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"));
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "FindFirst"));
            f.return_type_path = int_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"));
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "FindLast"));
            f.return_type_path = int_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"));
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Grow"));
            f.return_type_path = int_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "numElements"));
            p.type_path = int_path.clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Erase"));
            f.return_type_path = void_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "index"));
            p.type_path = int_path.clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Insert"));
            f.return_type_path = void_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "index"));
            p.type_path = int_path.clone();
            params.push(p);
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"));
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Last"));
            f.return_type_path = self.data_type_path().clone();
            funcs.push(f);
        }

        (funcs, params)
    }
}