use abs_path::AbsPath;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;
use witcherscript::tokens::Keyword;
use witcherscript_analysis::symbol_analysis::symbol_path::SymbolPathBuf;
use witcherscript_analysis::symbol_analysis::symbol_table::iter::*;
use witcherscript_analysis::symbol_analysis::symbol_table::SymbolTable;
use witcherscript_analysis::symbol_analysis::symbols::*;
use crate::Backend;
use super::common::resolve_text_document_position;
use super::common::PositionTargetKind;


pub async fn hover(backend: &Backend, params: lsp::HoverParams) -> Result<Option<lsp::Hover>> {
    let doc_path = AbsPath::try_from(params.text_document_position_params.text_document.uri.clone()).unwrap();

    let content_path;
    if let Some(path) = backend.source_trees.containing_content_path(&doc_path) {
        content_path = path;
    } 
    else {
        // backend.client.send_notification::<notifications::client::show_foreign_script_warning::Type>(()).await;
        return Ok(None);
    }

    let content_dependency_paths = backend.get_content_dependency_paths(&content_path).await;
    let symtabs = backend.symtabs.read().await;
    let symtabs_marcher = symtabs.march(&content_dependency_paths);
    
    let script_state;
    if let Some(ss) = backend.scripts.get(&doc_path) {
        script_state = ss;
    } else {
        return Ok(None);
    }
    
    if let Some(position_target) = resolve_text_document_position(params.text_document_position_params.position, &script_state, symtabs_marcher.clone()) {
        let sympath: Option<SymbolPathBuf> = match position_target.kind {
            PositionTargetKind::ArrayTypeIdentifier => {
                None
            },
            PositionTargetKind::TypeIdentifier(type_name) => {
                Some(BasicTypeSymbolPath::new(&type_name).into())
            },
            PositionTargetKind::StateDeclarationNameIdentifier => {
                Some(position_target.sympath_ctx)
            },
            PositionTargetKind::StateDeclarationBaseIdentifier => {
                let mut state_base_path = None;
    
                if let Some(target_state_sym) = symtabs_marcher.get(&position_target.sympath_ctx).and_then(|v| v.try_as_state_ref()) {
                    let base_state_name = target_state_sym.base_state_name.as_ref().map(|s| s.as_str()).unwrap_or_default();
    
                    'ancestors: for class in symtabs_marcher.class_hierarchy(target_state_sym.parent_class_path()) {
                        for state in symtabs_marcher.class_states(class.path()) {
                            if state.state_name() == base_state_name {
                                state_base_path = Some(state.path().to_owned());
                                break 'ancestors;
                            }
                        }
                    }
                }
                
                state_base_path
            },
            PositionTargetKind::DataDeclarationNameIdentifier(name) => {
                if let Some(ctx_sym) = symtabs_marcher.get(&position_target.sympath_ctx) {
                    if ctx_sym.is_enum() {
                        Some(GlobalDataSymbolPath::new(&name).into())
                    } else {
                        Some(MemberDataSymbolPath::new(&position_target.sympath_ctx, &name).into())
                    }
                } else {
                    None
                }
            },
            PositionTargetKind::CallableDeclarationNameIdentifier => {
                Some(position_target.sympath_ctx)
            },
            PositionTargetKind::ThisKeyword => {
                Some(SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap_or_default(), SpecialVarSymbolKind::This).into())
            },
            PositionTargetKind::SuperKeyword => {
                Some(SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap_or_default(), SpecialVarSymbolKind::Super).into())
            },
            PositionTargetKind::ParentKeyword => {
                Some(SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap_or_default(), SpecialVarSymbolKind::Parent).into())
            },
            PositionTargetKind::VirtualParentKeyword => {
                Some(SpecialVarSymbolPath::new(position_target.sympath_ctx.root().unwrap_or_default(), SpecialVarSymbolKind::VirtualParent).into())
            },
            PositionTargetKind::ExpressionIdentifier(expr_ident) => {
                Some(expr_ident)
            }
        };

        if let Some(sympath) = sympath {
            let category = sympath
                .components().last()
                .map(|c| c.category)
                .unwrap_or(SymbolCategory::Type);

            let mut buf = String::new();
            symtabs_marcher.get_with_containing(&sympath)
                .map(|(symtab, symvar)| symvar.render(&mut buf, symtab))
                .unwrap_or_else(|| buf = SymbolPathBuf::unknown(category).to_string());

            Ok(Some(lsp::Hover {
                contents: lsp::HoverContents::Scalar(lsp::MarkedString::LanguageString(lsp::LanguageString {
                    language: Backend::LANGUAGE_ID.to_string(),
                    value: buf,
                })),
                range: None
            }))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}


trait RenderTooltip {
    fn render(&self, buf: &mut String, symtab: &SymbolTable);
}

trait RenderShortTooltip {
    fn render_short(&self, buf: &mut String);
}


impl RenderTooltip for SymbolVariant {
    fn render(&self, buf: &mut String, symtab: &SymbolTable) {
        match self {
            SymbolVariant::Class(s) => s.render(buf, symtab),
            SymbolVariant::State(s) => s.render(buf, symtab),
            SymbolVariant::Struct(s) => s.render(buf, symtab),
            SymbolVariant::Enum(s) => s.render(buf, symtab),
            SymbolVariant::Array(s) => s.render(buf, symtab),
            SymbolVariant::GlobalFunc(s) => s.render(buf, symtab),
            SymbolVariant::MemberFunc(s) => s.render(buf, symtab),
            SymbolVariant::Event(s) => s.render(buf, symtab),
            SymbolVariant::Constructor(s) => s.render(buf, symtab),
            SymbolVariant::Primitive(s) => s.render(buf, symtab),
            SymbolVariant::EnumVariant(s) => s.render(buf, symtab),
            SymbolVariant::FuncParam(s) => s.render(buf, symtab),
            SymbolVariant::GlobalVar(s) => s.render(buf, symtab),
            SymbolVariant::MemberVar(s) => s.render(buf, symtab),
            SymbolVariant::Autobind(s) => s.render(buf, symtab),
            SymbolVariant::LocalVar(s) => s.render(buf, symtab),
            SymbolVariant::SpecialVar(s) => s.render(buf, symtab),
        }
    }
}


impl RenderTooltip for ClassSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable) {
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

impl RenderShortTooltip for ClassSymbol {
    fn render_short(&self, buf: &mut String) {
        buf.push_str(Keyword::Class.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
    }
}

impl RenderTooltip for StateSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable) {
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

impl RenderShortTooltip for StateSymbol {
    fn render_short(&self, buf: &mut String) {
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
    fn render(&self, buf: &mut String, _: &SymbolTable) {
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

impl RenderShortTooltip for StructSymbol {
    fn render_short(&self, buf: &mut String) {
        buf.push_str(Keyword::Struct.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
    }
}

impl RenderTooltip for EnumSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable) {
        buf.push_str(Keyword::Enum.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
    }
}

impl RenderShortTooltip for EnumSymbol {
    fn render_short(&self, buf: &mut String) {
        buf.push_str(Keyword::Enum.as_ref());
        buf.push(' ');

        buf.push_str(self.name());
    }
}

impl RenderTooltip for ArrayTypeSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable) {
        buf.push_str(self.name())
    }
}

impl RenderShortTooltip for ArrayTypeSymbol {
    fn render_short(&self, buf: &mut String) {
        buf.push_str(self.name())
    }
}



impl RenderTooltip for GlobalFunctionSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable) {
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
            .get_callable_children(self.path())
            .filter_map(|ch| {
                if let CallableSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        if let Some(param) = params.pop() {
            param.render(buf, symtab);
        }
        for param in params {
            buf.push_str(", ");
            param.render(buf, symtab);
        }
        
        buf.push(')');

        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.return_type_name());
    }
}

impl RenderTooltip for MemberFunctionSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable) {
        let parent_symvar = 
            self.path().parent()
            .and_then(|p| symtab.get(p));

        if let Some(parent_symvar) = parent_symvar {
            match parent_symvar {
                SymbolVariant::Class(s) => {
                    s.render_short(buf);
                    buf.push('\n');
                },
                SymbolVariant::State(s) => {
                    s.render_short(buf);
                    buf.push('\n');
                },
                SymbolVariant::Array(s) => {
                    s.render_short(buf);
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
            .get_callable_children(self.path())
            .filter_map(|ch| {
                if let CallableSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        if let Some(param) = params.pop() {
            param.render(buf, symtab);
        }
        for param in params {
            buf.push_str(", ");
            param.render(buf, symtab);
        }
        
        buf.push(')');

        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.return_type_name()); 
    }
}

impl RenderTooltip for EventSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable) {
        let parent_symvar = 
            self.path().parent()
            .and_then(|p| symtab.get(p));

        if let Some(parent_symvar) = parent_symvar {
            match parent_symvar {
                SymbolVariant::Class(s) => {
                    s.render_short(buf);
                    buf.push('\n');
                },
                SymbolVariant::State(s) => {
                    s.render_short(buf);
                    buf.push('\n');
                },
                SymbolVariant::Array(s) => {
                    s.render_short(buf);
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
            .get_callable_children(self.path())
            .filter_map(|ch| {
                if let CallableSymbolChild::Param(param) = ch {
                    Some(param)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        params.sort_by(|param1, param2| param1.ordinal.cmp(&param2.ordinal));

        if let Some(param) = params.pop() {
            param.render(buf, symtab);
        }
        for param in params {
            buf.push_str(", ");
            param.render(buf, symtab);
        }
        
        buf.push(')');
    }
}

impl RenderTooltip for ConstructorSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable) {
        symtab.get(&self.parent_type_path)
            .and_then(|s| s.try_as_struct_ref())
            .map(|s| s.render(buf, symtab));
    }
}



impl RenderTooltip for PrimitiveTypeSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable) {
        buf.push_str(self.alias_name().unwrap_or(self.name()))
    }
}

impl RenderTooltip for EnumVariantSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable) {
        symtab.get(&self.parent_enum_path)
            .and_then(|s| s.try_as_enum_ref())
            .map(|s| {
                s.render_short(buf);
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
    fn render(&self, buf: &mut String, _: &SymbolTable) {
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
    fn render(&self, buf: &mut String, _: &SymbolTable) {
        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for MemberVarSymbol {
    fn render(&self, buf: &mut String, symtab: &SymbolTable) {
        let parent_symvar = 
            self.path().parent()
            .and_then(|p| symtab.get(p));

        if let Some(parent_symvar) = parent_symvar {
            match parent_symvar {
                SymbolVariant::Class(s) => {
                    s.render_short(buf);
                    buf.push('\n');
                },
                SymbolVariant::State(s) => {
                    s.render_short(buf);
                    buf.push('\n');
                },
                SymbolVariant::Array(s) => {
                    s.render_short(buf);
                    buf.push('\n');
                },
                SymbolVariant::Struct(s) => {
                    s.render_short(buf);
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
    fn render(&self, buf: &mut String, symtab: &SymbolTable) {
        let parent_symvar = 
            self.path().parent()
            .and_then(|p| symtab.get(p));

        if let Some(parent_symvar) = parent_symvar {
            match parent_symvar {
                SymbolVariant::Class(s) => {
                    s.render_short(buf);
                    buf.push('\n');
                },
                SymbolVariant::State(s) => {
                    s.render_short(buf);
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
    fn render(&self, buf: &mut String, _: &SymbolTable) {
        buf.push_str(Keyword::Var.as_ref());
        buf.push(' ');
        buf.push_str(self.name());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}

impl RenderTooltip for SpecialVarSymbol {
    fn render(&self, buf: &mut String, _: &SymbolTable) {
        let kw: Keyword = self.kind().into();
        buf.push_str(kw.as_ref());
        buf.push(' ');
        buf.push(':');
        buf.push(' ');
        buf.push_str(self.type_name());
    }
}
