use ropey::Rope;
use uuid::Uuid;
use witcherscript::ast::*;
use crate::model::collections::*;
use crate::diagnostics::*;
use crate::model::symbols::*;
use super::commons::SymbolCollectorCommons;


//TODO be able to update existing db and ctx instead of assuming they are new
struct GlobalSymbolCollector<'a> {
    db: &'a mut SymbolDb,
    ctx: &'a mut SymbolContext,
    script_id: Uuid,
    rope: Rope,
    diagnostics: Vec<Diagnostic>,

    current_enum: Option<EnumSymbol>,
}

impl SymbolCollectorCommons for GlobalSymbolCollector<'_> {
    fn db(&mut self) -> &mut SymbolDb {
        &mut self.db
    }

    fn ctx(&mut self) -> &mut SymbolContext {
        &mut self.ctx
    }

    fn db_and_ctx(&mut self) -> (&mut SymbolDb, &mut SymbolContext) {
        (&mut self.db, &mut self.ctx)    
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
        let class_name = n.name()
                        .value(&self.rope)
                        .and_then(|ident| self.check_duplicate(ident.into(), SymbolType::Class, n.span()));

        if let Some(class_name) = class_name {
            let sym = ClassSymbol::new_with_default(&class_name, self.script_id);
            self.ctx.insert(&sym);
            self.db.insert_class(sym);
        }

        false
    }

    fn visit_state_decl(&mut self, n: &StateDeclarationNode) -> bool {
        let state_name = n.name().value(&self.rope);
        let parent_name = n.parent().value(&self.rope);
        if let (Some(state_name), Some(parent_name)) = (state_name, parent_name) {
            let state_class_name = StateSymbol::class_name(&state_name, &parent_name);
            if let Some(state_class_name) = self.check_duplicate(state_class_name, SymbolType::State, n.span()) {
                let sym = StateSymbol::new_with_default(&state_class_name, self.script_id);
                self.ctx.insert(&sym);
                self.db.insert_state(sym);
            }
        }

        false
    }

    fn visit_struct_decl(&mut self, n: &StructDeclarationNode) -> bool {
        let struct_name = n.name()
                          .value(&self.rope)
                          .and_then(|ident| self.check_duplicate(ident.into(), SymbolType::Struct, n.span()));

        if let Some(struct_name) = struct_name {
            let sym = StructSymbol::new_with_default(&struct_name, self.script_id);
            self.ctx.insert(&sym);
            self.db.insert_struct(sym);
        }

        false
    }

    fn visit_enum_decl(&mut self, n: &EnumDeclarationNode) -> bool {
        let enum_name = n.name()
                        .value(&self.rope)
                        .and_then(|ident| self.check_duplicate(ident.into(), SymbolType::Enum, n.span()));

        if let Some(enum_name) = enum_name {
            let sym = EnumSymbol::new_with_default(&enum_name, self.script_id);
            self.current_enum = Some(sym);
            // symbol added to db and ctx during exit
            return true;
        }

        false
    }

    // enum member is WS work just like they do in C - they are global scoped constants
    // enum type doesn't create any sort of scope for them
    fn visit_enum_member_decl(&mut self, n: &EnumMemberDeclarationNode) {
        let member_name = n.name()
                          .value(&self.rope)
                          .and_then(|ident| self.check_duplicate(ident.into(), SymbolType::EnumMember, n.span()));

        if let Some(member_name) = member_name {
            let sym = self.current_enum.as_mut().unwrap().add_member(&member_name);
            self.ctx.insert(&sym);
            self.db.insert_enum_member(sym);
        }
    }

    fn exit_enum_decl(&mut self, _: &EnumDeclarationNode) {
        if let Some(sym) = self.current_enum.take() {
            self.ctx.insert(&sym);
            self.db.insert_enum(sym);
        }
    }

    fn visit_global_func_decl(&mut self, n: &GlobalFunctionDeclarationNode) -> bool {
        let func_name = n.name()
                        .value(&self.rope)
                        .and_then(|ident| self.check_duplicate(ident.into(), SymbolType::GlobalFunction, n.span()));

        if let Some(func_name) = func_name {
            let sym = GlobalFunctionSymbol::new_with_default(&func_name, self.script_id);
            self.ctx.insert(&sym);
            self.db.insert_global_func(sym);
        }

        false
    }
}
