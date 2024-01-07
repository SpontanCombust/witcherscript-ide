use ropey::Rope;
use witcherscript::ast::*;
use crate::diagnostics::*;
use crate::model::collections::symbol_table::SymbolTable;
use crate::model::symbol_path::SymbolPath;
use crate::model::symbols::*;
use super::commons::SymbolCollectorCommons;


//TODO be able to update existing symtab instead of assuming data is always new
struct GlobalSymbolCollector<'a> {
    symtab: &'a mut SymbolTable,
    // script_id: Uuid,
    rope: Rope,
    diagnostics: Vec<Diagnostic>,
    
    current_path: SymbolPath
}

impl SymbolCollectorCommons for GlobalSymbolCollector<'_> {
    fn symtab(&mut self) -> &mut SymbolTable {
        &mut self.symtab
    }

    fn diagnostics(&mut self) -> &mut Vec<Diagnostic> {
        &mut self.diagnostics
    }

    fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl StatementVisitor for GlobalSymbolCollector<'_> {
    fn visit_class_decl(&mut self, n: &ClassDeclarationNode) -> bool {
        if let Some(class_name) = n.name().value(&self.rope) {
            let path = BasicTypeSymbolPath::new(&class_name);
            let sym = ClassSymbol::new(path);
            self.try_insert_with_duplicate_check(sym, n.name().span());
        }

        false
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> bool {
        let state_name = n.name().value(&self.rope);
        let parent_name = n.parent().value(&self.rope);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let path = StateSymbolPath::new(&state_name, BasicTypeSymbolPath::new(&parent_name));
            let sym = StateSymbol::new(path);
            self.try_insert_with_duplicate_check(sym, n.name().span());
        }

        false            
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> bool {
        if let Some(struct_name) = n.name().value(&self.rope) {
            let path = BasicTypeSymbolPath::new(&struct_name);
            let sym = StructSymbol::new(path);
            self.try_insert_with_duplicate_check(sym, n.name().span());
        }

        false
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> bool {
        if let Some(enum_name) = n.name().value(&self.rope) {
            let path = BasicTypeSymbolPath::new(&enum_name);
            let sym = EnumSymbol::new(path);

            sym.path().clone_into(&mut self.current_path);
            self.try_insert_with_duplicate_check(sym, n.name().span())
        } else {
            false
        }
    }

    fn visit_enum_member_decl(&mut self, n: &EnumMemberDeclarationNode) {
        if let Some(enum_member_name) = n.name().value(&self.rope) {
            let path = DataSymbolPath::new(&self.current_path, &enum_member_name);
            let sym = EnumMemberSymbol::new(path);
            self.try_insert_with_duplicate_check(sym, n.name().span());
        }
    }

    fn exit_enum_decl(&mut self, _: &EnumDeclarationNode) {
        self.current_path.pop();
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> bool {
        if let Some(func_name) = n.name().value(&self.rope) {
            let path = GlobalCallableSymbolPath::new(&func_name);
            let sym = GlobalFunctionSymbol::new(path);
            self.try_insert_with_duplicate_check(sym, n.name().span());
        }

        false
    }
}
