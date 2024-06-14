use abs_path::AbsPath;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use witcherscript::{ast::AnnotationKind, tokens::Keyword};
use witcherscript_analysis::symbol_analysis::symbol_path::SymbolPathBuf;
use witcherscript_analysis::symbol_analysis::symbol_table::iter::*;
use witcherscript_analysis::symbol_analysis::symbol_table::marcher::SymbolTableMarcher;
use witcherscript_analysis::symbol_analysis::symbol_table::SymbolTable;
use witcherscript_analysis::symbol_analysis::symbols::*;
use crate::Backend;
use super::common::{resolve_text_document_position, PositionTargetKind};


impl Backend {
    pub async fn hover_impl(&self, params: lsp::HoverParams) -> Result<Option<lsp::Hover>> {
        let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();
    
        if doc_path.extension().unwrap_or_default() != "ws" {
            return Ok(None);
        }
    
        let content_path;
        if let Some(path) = self.scripts.get(&doc_path).and_then(|ss| ss.content_info.as_ref().map(|ci| ci.content_path.to_owned())) {
            content_path = path;
        } 
        else {
            // backend.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
            return Ok(None);
        }
    
        let symtabs = self.symtabs.read().await;
        let symtabs_marcher = self.march_symbol_tables(&symtabs, &content_path).await;
        
        let script_state;
        if let Some(ss) = self.scripts.get(&doc_path) {
            script_state = ss;
        } else {
            return Ok(None);
        }
    
        let position_target = resolve_text_document_position(params.text_document_position_params.position, &script_state, symtabs_marcher.clone());
        drop(script_state);
        
        if let Some(position_target) = position_target {
            let mut value = None;
    
            if let Some(sympath) = position_target.target_symbol_path(&symtabs_marcher) {
                let category = sympath
                .components().last()
                .map(|c| c.category)
                .unwrap_or(SymbolCategory::Type);
    
                let mut buf = String::new();
                symtabs_marcher.get_symbol_with_containing_table(&sympath)
                    .map(|(symtab, symvar)| symvar.render(&mut buf, symtab, &symtabs_marcher))
                    .unwrap_or_else(|| buf = SymbolPathBuf::unknown(category).to_string());
    
                value = Some(buf);
            } else if let PositionTargetKind::ArrayTypeIdentifier = position_target.kind {
                value = Some("array<T>".to_string());
            }
    
            Ok(value.map(|value| lsp::Hover {
                contents: lsp::HoverContents::Scalar(lsp::MarkedString::LanguageString(lsp::LanguageString {
                    language: Backend::LANGUAGE_ID.to_string(),
                    value,
                })),
                range: None
            }))
        } else {
            Ok(None)
        }
    }
}


trait RenderTooltip {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, marcher: &SymbolTableMarcher<'_>);
}

/// For when the text is supposed to be rendered as a part of some bigger tooltip
/// e.g. a member var tooltip contains a partial tooltip for containing class at the top 
/// to know where that var comes from
trait RenderPartialTooltip {
    fn render_partial(&self, buf: &mut String);
}


impl RenderTooltip for SymbolVariant {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, marcher: &SymbolTableMarcher<'_>) {
        match self {
            SymbolVariant::Class(s) => s.render(buf, symtab, marcher),
            SymbolVariant::State(s) => s.render(buf, symtab, marcher),
            SymbolVariant::Struct(s) => s.render(buf, symtab, marcher),
            SymbolVariant::Enum(s) => s.render(buf, symtab, marcher),
            SymbolVariant::Array(s) => s.render(buf, symtab, marcher),
            SymbolVariant::ArrayFunc(s) => s.render(buf, symtab, marcher),
            SymbolVariant::ArrayFuncParam(s) => s.render(buf, symtab, marcher),
            SymbolVariant::GlobalFunc(s) => s.render(buf, symtab, marcher),
            SymbolVariant::MemberFunc(s) => s.render(buf, symtab, marcher),
            SymbolVariant::Event(s) => s.render(buf, symtab, marcher),
            SymbolVariant::Constructor(s) => s.render(buf, symtab, marcher),
            SymbolVariant::Primitive(s) => s.render(buf, symtab, marcher),
            SymbolVariant::EnumVariant(s) => s.render(buf, symtab, marcher),
            SymbolVariant::FuncParam(s) => s.render(buf, symtab, marcher),
            SymbolVariant::GlobalVar(s) => s.render(buf, symtab, marcher),
            SymbolVariant::MemberVar(s) => s.render(buf, symtab, marcher),
            SymbolVariant::Autobind(s) => s.render(buf, symtab, marcher),
            SymbolVariant::LocalVar(s) => s.render(buf, symtab, marcher),
            SymbolVariant::ThisVar(s) => s.render(buf, symtab, marcher),
            SymbolVariant::SuperVar(s) => s.render(buf, symtab, marcher),
            SymbolVariant::StateSuperVar(s) => s.render(buf, symtab, marcher),
            SymbolVariant::ParentVar(s) => s.render(buf, symtab, marcher),
            SymbolVariant::VirtualParentVar(s) => s.render(buf, symtab, marcher),
            SymbolVariant::MemberFuncInjector(s) => s.render(buf, symtab, marcher),
            SymbolVariant::MemberFuncReplacer(s) => s.render(buf, symtab, marcher),
            SymbolVariant::GlobalFuncReplacer(s) => s.render(buf, symtab, marcher),
            SymbolVariant::MemberFuncWrapper(s) => s.render(buf, symtab, marcher),
            SymbolVariant::MemberVarInjector(s) => s.render(buf, symtab, marcher),
            SymbolVariant::WrappedMethod(s) => s.render(buf, symtab, marcher),
        }
    }
}


impl RenderTooltip for ClassSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Class.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
        
        if let Some(base_name) = self.base_name() {
            buf.push(' ');
            buf.push_str(Keyword::Extends.as_ref());
            buf.push(' ');
            buf.push_str(base_name);
        }
    }
}

impl RenderPartialTooltip for ClassSymbol {
    fn render_partial(&self, buf: &mut String) {
        buf.push_str(Keyword::Class.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
    }
}

impl RenderTooltip for StateSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::State.as_ref());
        buf.push(' ');

        buf.push_str(self.state_name());
        buf.push(' ');

        buf.push_str(Keyword::In.as_ref());
        buf.push(' ');

        buf.push_str(self.parent_class_name());

        if let Some(base_name) = &self.base_state_name {
            buf.push(' ');
            buf.push_str(Keyword::Extends.as_ref());
            buf.push(' ');
            buf.push_str(base_name);
        }
    }
}

impl RenderPartialTooltip for StateSymbol {
    fn render_partial(&self, buf: &mut String) {
        buf.push_str(Keyword::State.as_ref());
        buf.push(' ');

        buf.push_str(self.state_name());
        buf.push(' ');

        buf.push_str(Keyword::In.as_ref());
        buf.push(' ');

        buf.push_str(self.parent_class_name());
    }
}

impl RenderTooltip for StructSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Struct.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
    }
}

impl RenderPartialTooltip for StructSymbol {
    fn render_partial(&self, buf: &mut String) {
        buf.push_str(Keyword::Struct.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
    }
}

impl RenderTooltip for EnumSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(Keyword::Enum.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
    }
}

impl RenderPartialTooltip for EnumSymbol {
    fn render_partial(&self, buf: &mut String) {
        buf.push_str(Keyword::Enum.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
    }
}

impl RenderTooltip for ArrayTypeSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(self.name())
    }
}

impl RenderPartialTooltip for ArrayTypeSymbol {
    fn render_partial(&self, buf: &mut String) {
        buf.push_str(self.name())
    }
}

impl RenderTooltip for ArrayTypeFunctionSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(&format!("{}<T>\n", ArrayTypeSymbol::TYPE_NAME));
        
        buf.push_str(Keyword::Function.as_ref());
        buf.push(' ');

        buf.push_str(self.name());

        buf.push('(');

        let mut params = symtab
            .get_symbol_children_filtered(self)
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        let mut params_iter = params.into_iter();
        if let Some(param) = params_iter.next() {
            param.render_partial(buf);
        }
        for param in params_iter {
            buf.push_str(", ");
            param.render_partial(buf);
        }
        
        buf.push(')');

        buf.push(' ');
        buf.push(':');
        buf.push(' ');

        if self.was_return_type_generic {
            buf.push_str("T");
        } else {
            buf.push_str(self.return_type_name());
        }
    }
}

impl RenderTooltip for ArrayTypeFunctionParameterSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        self.render_partial(buf)
    }
}

impl RenderPartialTooltip for ArrayTypeFunctionParameterSymbol {
    fn render_partial(&self, buf: &mut String) {
        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');

        if self.was_type_generic {
            buf.push_str("T");
        } else {
            buf.push_str(self.type_name());
        }
    }
}



impl RenderTooltip for GlobalFunctionSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        if let Some(flavour) = self.flavour.clone() {
            let kw: Keyword = flavour.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Function.as_ref());
        buf.push(' ');

        buf.push_str(self.name());

        buf.push('(');

        let mut params = symtab
            .get_symbol_children_filtered(self)
            .filter_map(|ch| {
                if let CallableSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        let mut params_iter = params.into_iter();
        if let Some(param) = params_iter.next() {
            param.render_partial(buf);
        }
        for param in params_iter {
            buf.push_str(", ");
            param.render_partial(buf);
        }
        
        buf.push(')');

        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.return_type_name());
    }
}

impl RenderTooltip for MemberFunctionSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        let parent_symvar = 
            self.path().parent()
            .and_then(|p| symtab.get_symbol(p));

        if let Some(parent_symvar) = parent_symvar {
            match parent_symvar {
                SymbolVariant::Class(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                SymbolVariant::State(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                SymbolVariant::Array(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                _ => {}
            }
        }


        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        if let Some(flavour) = self.flavour.clone() {
            let kw: Keyword = flavour.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Function.as_ref());
        buf.push(' ');

        buf.push_str(self.name());

        buf.push('(');

        let mut params = symtab
            .get_symbol_children_filtered(self)
            .filter_map(|ch| {
                if let CallableSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        let mut params_iter = params.into_iter();
        if let Some(param) = params_iter.next() {
            param.render_partial(buf);
        }
        for param in params_iter {
            buf.push_str(", ");
            param.render_partial(buf);
        }
        
        buf.push(')');

        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.return_type_name()); 
    }
}

impl RenderTooltip for EventSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        let parent_symvar = 
            self.path().parent()
            .and_then(|p| symtab.get_symbol(p));

        if let Some(parent_symvar) = parent_symvar {
            match parent_symvar {
                SymbolVariant::Class(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                SymbolVariant::State(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                SymbolVariant::Array(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                _ => {}
            }
        }

        buf.push_str(Keyword::Event.as_ref());
        buf.push(' ');

        buf.push_str(self.name());

        buf.push('(');

        let mut params = symtab
            .get_symbol_children_filtered(self)
            .filter_map(|ch| {
                if let CallableSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        let mut params_iter = params.into_iter();
        if let Some(param) = params_iter.next() {
            param.render_partial(buf);
        }
        for param in params_iter {
            buf.push_str(", ");
            param.render_partial(buf);
        }
        
        buf.push(')');
    }
}

impl RenderTooltip for ConstructorSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, marcher: &SymbolTableMarcher<'_>) {
        symtab.get_symbol(&self.parent_type_path)
            .and_then(|s| s.try_as_struct_ref())
            .map(|s| s.render(buf, symtab, marcher));
    }
}



impl RenderTooltip for PrimitiveTypeSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(self.name())
    }
}

impl RenderTooltip for EnumVariantSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        symtab.get_symbol(&self.parent_enum_path)
            .and_then(|s| s.try_as_enum_ref())
            .map(|s| {
                s.render_partial(buf);
                buf.push('\n');
            });

        buf.push_str(self.name());
        buf.push(' ');
        buf.push('=');
        buf.push(' ');
        buf.push_str(&self.value.to_string());
    }
}

impl RenderTooltip for FunctionParameterSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Var.as_ref());
        buf.push(' ');
        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderPartialTooltip for FunctionParameterSymbol {
    fn render_partial(&self, buf: &mut String) {
        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for GlobalVarSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for MemberVarSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        let parent_symvar = 
            self.path().parent()
            .and_then(|p| symtab.get_symbol(p));

        if let Some(parent_symvar) = parent_symvar {
            match parent_symvar {
                SymbolVariant::Class(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                SymbolVariant::State(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                SymbolVariant::Array(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                SymbolVariant::Struct(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                _ => {}
            }
        }
        

        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Var.as_ref());
        buf.push(' ');
        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for AutobindSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        let parent_symvar = 
            self.path().parent()
            .and_then(|p| symtab.get_symbol(p));

        if let Some(parent_symvar) = parent_symvar {
            match parent_symvar {
                SymbolVariant::Class(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                SymbolVariant::State(s) => {
                    s.render_partial(buf);
                    buf.push('\n');
                },
                _ => {}
            }
        }
        

        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Autobind.as_ref());
        buf.push(' ');
        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for LocalVarSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(Keyword::Var.as_ref());
        buf.push(' ');
        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for ThisVarSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(Keyword::This.as_ref());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for SuperVarSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(Keyword::Super.as_ref());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for StateSuperVarSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, marcher: &SymbolTableMarcher<'_>) {
        buf.push_str(Keyword::Super.as_ref());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');

        if self.base_state_name().is_some() {
            let state_path = self.path().root().unwrap_or_default();
            if let Some(base_state_sym) = marcher.state_hierarchy(state_path).skip(1).next() {
                buf.push_str(base_state_sym.name());
            } else {
                buf.push_str(&SymbolPathBuf::unknown(SymbolCategory::Type).to_string());
            }
        } else {
            buf.push_str(StateSymbol::DEFAULT_STATE_BASE_NAME);
        }
    }
}

impl RenderTooltip for ParentVarSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(Keyword::Parent.as_ref());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for VirtualParentVarSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(Keyword::VirtualParent.as_ref());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for MemberFunctionInjectorSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(AnnotationKind::AddMethod.as_ref());
        buf.push('(');
        buf.push_str(&self.path().parent().unwrap_or_default().to_string());
        buf.push(')');
        buf.push('\n');

        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        if let Some(flavour) = self.flavour.clone() {
            let kw: Keyword = flavour.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Function.as_ref());
        buf.push(' ');

        buf.push_str(self.name());

        buf.push('(');

        let mut params = symtab
            .get_symbol_children_filtered(self)
            .filter_map(|ch| {
                if let CallableSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        let mut params_iter = params.into_iter();
        if let Some(param) = params_iter.next() {
            param.render_partial(buf);
        }
        for param in params_iter {
            buf.push_str(", ");
            param.render_partial(buf);
        }
        
        buf.push(')');

        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.return_type_name()); 
    }
}

impl RenderTooltip for MemberFunctionReplacerSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(AnnotationKind::ReplaceMethod.as_ref());
        buf.push('(');
        buf.push_str(&self.path().parent().unwrap_or_default().to_string());
        buf.push(')');
        buf.push('\n');

        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        if let Some(flavour) = self.flavour.clone() {
            let kw: Keyword = flavour.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Function.as_ref());
        buf.push(' ');

        buf.push_str(self.name());

        buf.push('(');

        let mut params = symtab
            .get_symbol_children_filtered(self)
            .filter_map(|ch| {
                if let CallableSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        let mut params_iter = params.into_iter();
        if let Some(param) = params_iter.next() {
            param.render_partial(buf);
        }
        for param in params_iter {
            buf.push_str(", ");
            param.render_partial(buf);
        }
        
        buf.push(')');

        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.return_type_name()); 
    }
}

impl RenderTooltip for GlobalFunctionReplacerSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(AnnotationKind::ReplaceMethod.as_ref());
        buf.push('\n');

        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        if let Some(flavour) = self.flavour.clone() {
            let kw: Keyword = flavour.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Function.as_ref());
        buf.push(' ');

        buf.push_str(self.name());

        buf.push('(');

        let mut params = symtab
            .get_symbol_children_filtered(self)
            .filter_map(|ch| {
                if let CallableSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        let mut params_iter = params.into_iter();
        if let Some(param) = params_iter.next() {
            param.render_partial(buf);
        }
        for param in params_iter {
            buf.push_str(", ");
            param.render_partial(buf);
        }
        
        buf.push(')');

        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.return_type_name()); 
    }
}

impl RenderTooltip for MemberFunctionWrapperSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(AnnotationKind::WrapMethod.as_ref());
        buf.push('(');
        buf.push_str(&self.path().parent().unwrap_or_default().to_string());
        buf.push(')');
        buf.push('\n');

        // when wrapping functions you don't put any specifiers before `function`

        buf.push_str(Keyword::Function.as_ref());
        buf.push(' ');

        buf.push_str(self.name());

        buf.push('(');

        let mut params = symtab
            .get_symbol_children_filtered(self)
            .filter_map(|ch| {
                if let FunctionWrapperSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        let mut params_iter = params.into_iter();
        if let Some(param) = params_iter.next() {
            param.render_partial(buf);
        }
        for param in params_iter {
            buf.push_str(", ");
            param.render_partial(buf);
        }
        
        buf.push(')');

        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.return_type_name()); 
    }
}

impl RenderTooltip for MemberVarInjectorSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable, _: &SymbolTableMarcher<'_>) {
        buf.push_str(AnnotationKind::AddField.as_ref());
        buf.push('(');
        buf.push_str(&self.path().parent().unwrap_or_default().to_string());
        buf.push(')');
        buf.push('\n');

        for spec in self.specifiers.iter() {
            let kw: Keyword = spec.into();
            buf.push_str(kw.as_ref());
            buf.push(' ');
        }

        buf.push_str(Keyword::Var.as_ref());
        buf.push(' ');
        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for WrappedMethodSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable, marcher: &SymbolTableMarcher<'_>) {
        // skip the wrapper function to get to either another wrapper or the original function
        if let Some(wrapped) = marcher.redefinition_chain(&self.wrapped_path()).skip(1).next() {
            wrapped.render(buf, symtab, marcher)
        }
    }
}