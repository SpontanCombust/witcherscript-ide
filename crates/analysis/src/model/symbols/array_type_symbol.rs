use lsp_types as lsp;
use crate::model::symbol_path::SymbolPath;
use super::*;


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
    pub const TYPE_NAME: &'static str = "array";

    pub fn new(path: ArrayTypeSymbolPath) -> Self {
        Self {
            path
        }
    }

    pub fn data_type_path(&self) -> &TypeSymbolPath {
        &self.path.type_arg_path
    }

    pub fn make_functions(&self, void_path: &TypeSymbolPath, int_path: &TypeSymbolPath, bool_path: &TypeSymbolPath) -> (Vec<MemberFunctionSymbol>, Vec<FunctionParameterSymbol>) {
        let mut funcs = Vec::new();
        let mut params = Vec::new();

        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "operator[]"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = self.data_type_path().clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "index"), lsp::Range::default(), lsp::Range::default());
            p.type_path = int_path.clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Clear"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = void_path.clone();
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Size"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = int_path.clone();
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "PushBack"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = self.data_type_path().clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"), lsp::Range::default(), lsp::Range::default());
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Resize"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = void_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "newSize"), lsp::Range::default(), lsp::Range::default());
            p.type_path = int_path.clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Remove"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = bool_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"), lsp::Range::default(), lsp::Range::default());
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Contains"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = bool_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"), lsp::Range::default(), lsp::Range::default());
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "FindFirst"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = int_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"), lsp::Range::default(), lsp::Range::default());
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "FindLast"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = int_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"), lsp::Range::default(), lsp::Range::default());
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Grow"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = int_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "numElements"), lsp::Range::default(), lsp::Range::default());
            p.type_path = int_path.clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Erase"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = void_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "index"), lsp::Range::default(), lsp::Range::default());
            p.type_path = int_path.clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Insert"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = void_path.clone();
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "index"), lsp::Range::default(), lsp::Range::default());
            p.type_path = int_path.clone();
            params.push(p);
            let mut p = FunctionParameterSymbol::new(DataSymbolPath::new(&f.path(), "element"), lsp::Range::default(), lsp::Range::default());
            p.type_path = self.data_type_path().clone();
            params.push(p);
            funcs.push(f);
        }
        {
            let mut f = MemberFunctionSymbol::new(MemberCallableSymbolPath::new(&self.path, "Last"), lsp::Range::default(), lsp::Range::default());
            f.return_type_path = self.data_type_path().clone();
            funcs.push(f);
        }

        (funcs, params)
    }
}