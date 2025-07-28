use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::Result;

use witcherscript_lang::ast::AnnotationKind;
use witcherscript_lang::attribs::{FunctionFlavour, Specifier};
use witcherscript_lang::tokens::Keyword;
use witcherscript_analysis::symbol_analysis::symbol_path::SymbolPathBuf;

use crate::Backend;


impl Backend {
    pub async fn completion_impl(&self, params: lsp::CompletionParams) -> Result<Option<lsp::CompletionResponse>> {
        
        Ok(None)
    }
    
    pub async fn completion_resolve_impl(&self, params: lsp::CompletionItem) -> Result<lsp::CompletionItem> {
        // return the data back for now
        // this will be used for displaying documentation once that is taken care of
        Ok(params)
    }
}


#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum CompletionExpectation {
    /// Default when context cannot be fully resolved or simply nothing can be suggested
    #[default]
    None,
    /// Syntax requires a new identifier to be written
    /// No suggestions should be produced in that case
    NewIdentifier,
    /// Valid syntax in the root of the document
    Root,
    /// Any expression acccessible in a given context
    /// Expected type should not be taken into account at this stage
    AnyExpression,
    /// Cursor is placed after a dot signaling member access expression
    MemberAccess {
        accessor_sympath: SymbolPathBuf 
    },
    /// Syntax expects a known type identifier
    KnownTypeIdentifier,
    /// Any syntax valid inside a function body
    FunctionBody {
        available_local_var_sympaths: Vec<SymbolPathBuf>
    },
    /// Any syntax valid inside class's or state's definition
    ClassBody {
        class_sympath: SymbolPathBuf
    },
    /// Any syntax valid inside struct's definition
    StructBody {
        struct_sympath: SymbolPathBuf
    },
    /// When specifically a field is expected, for example in the `default` assignment
    ClassField {
        class_sympath: SymbolPathBuf
    }
}



trait ToCompletionItem {
    fn to_completion_item(&self, text_edit_range: lsp::Range) -> lsp::CompletionItem;
}

impl ToCompletionItem for Keyword {
    fn to_completion_item(&self, _text_edit_range: lsp::Range) -> lsp::CompletionItem {
        lsp::CompletionItem {
            label: self.as_ref().to_string(),
            kind: Some(lsp::CompletionItemKind::KEYWORD),
            ..Default::default()
        }
    }
}

impl ToCompletionItem for AnnotationKind {
    fn to_completion_item(&self, text_edit_range: lsp::Range) -> lsp::CompletionItem {
        let mut text = self.as_ref().to_string();
        if self.requires_arg() {
            text += "($1)";
        }

        lsp::CompletionItem {
            label: self.as_ref().to_string(),
            kind: Some(lsp::CompletionItemKind::KEYWORD),
            insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
            text_edit: Some(lsp::TextEdit {
                new_text: text,
                range: text_edit_range
            }.into()),
            ..Default::default()
        }
    }
}


lazy_static! {
    static ref ROOT_KEYWORDS: Vec<Keyword> = {
        let mut kws: Vec<Keyword> = Vec::new();

        kws.extend(
            Specifier::iter()
                .map(|s| {
                    let k: Keyword = s.into();
                    k
                })
        );

        kws.extend(
            FunctionFlavour::iter()
                .map(|f| {
                    let k: Keyword = f.into();
                    k
                })
        );

        kws.extend([
            Keyword::Class,
            Keyword::State,
            Keyword::Enum,
            Keyword::Function,
            Keyword::Struct,
            Keyword::Var 
        ]);

        kws
    };
}

fn root_completions(text_edit_range: lsp::Range) -> Vec<lsp::CompletionItem> {
    let annotations = AnnotationKind::iter()
        .map(|ak| ak.to_completion_item(text_edit_range.clone()));

    let keywords = ROOT_KEYWORDS.iter()
        .map(|k| k.to_completion_item(text_edit_range.clone()));

    annotations.chain(keywords).collect()
}
