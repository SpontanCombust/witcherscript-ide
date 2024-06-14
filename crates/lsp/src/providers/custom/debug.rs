use abs_path::AbsPath;
use tower_lsp::lsp_types as lsp;
use tower_lsp::jsonrpc::{self, Result};
use crate::{requests, Backend};


pub trait LangaugeServerCustomDebug {
    async fn script_ast(&self, params: requests::debug::script_ast::Parameters) -> Result<requests::debug::script_ast::Response>;

    async fn content_graph_dot(&self, params: requests::debug::content_graph_dot::Parameters) -> Result<requests::debug::content_graph_dot::Response>;

    async fn script_symbols(&self, params: requests::debug::script_symbols::Parameters) -> Result<requests::debug::script_symbols::Response>;

    async fn script_cst(&self, params: requests::debug::script_cst::Parameters) -> Result<requests::debug::script_cst::Response>;
}


impl LangaugeServerCustomDebug for Backend {
    async fn script_ast(&self, params: requests::debug::script_ast::Parameters) -> Result<requests::debug::script_ast::Response> {
        let script_path: AbsPath;
        if let Ok(path) = AbsPath::try_from(params.script_uri) {
            script_path = path;
        } else {
            return Err(jsonrpc::Error::invalid_params("script_uri parameter is not a valid file URI"));
        }

        let script_entry = self.scripts.get(&script_path).ok_or(jsonrpc::Error {
            code: jsonrpc::ErrorCode::ServerError(-1010),
            message: "Script file not found".into(),
            data: None
        })?;

        let ast = format!("{:#?}", script_entry.value().script.root_node());
        drop(script_entry);

        Ok(requests::debug::script_ast::Response { 
            ast
        })
    }

    async fn content_graph_dot(&self, _: requests::debug::content_graph_dot::Parameters) -> Result<requests::debug::content_graph_dot::Response> {
        let graph = self.content_graph.read().await;

        let mut dot_graph = String::new();
        dot_graph += "digraph {\n";
        dot_graph += "\tcomment=\"Edge direction is: dependant ---> dependency. Edge label denotes dependency priority.\"\n";
        dot_graph += "\trankdir=\"BT\"\n";
        dot_graph += "\n";

        for n in graph.nodes() {
            let content_name = n.content.content_name();
            let content_uri = lsp::Url::from_file_path(n.content.path()).unwrap().to_string();
            dot_graph += &format!("\t{content_name} [URL=\"{content_uri}\"]\n");
        }

        dot_graph += "\n";

        for n in graph.nodes() {
            let content_name = n.content.content_name();
            for dep in graph.direct_dependencies(n.content.path()) {
                let dep_name = dep.content.content_name();
                let prio = graph.dependency_priority(n.content.path(), dep.content.path()).unwrap_or(-1);
                dot_graph += &format!("\t{content_name} -> {dep_name} [label={prio}]\n");
            }
        }

        dot_graph += "}";

        Ok(requests::debug::content_graph_dot::Response {
            dot_graph
        })
    }

    async fn script_symbols(&self, params: requests::debug::script_symbols::Parameters) -> Result<requests::debug::script_symbols::Response> {
        let script_path: AbsPath;
        if let Ok(path) = AbsPath::try_from(params.script_uri) {
            script_path = path;
        } else {
            return Err(jsonrpc::Error::invalid_params("script_uri parameter is not a valid file URI"));
        }

        let content_info;
        if let Some(ci) = self.scripts.get(&script_path).and_then(|ss| ss.content_info.clone()) {
            content_info = ci;
        } else {
            return Err(jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(-1060),
                message: "Script file does not belong to any known content".into(),
                data: None
            });
        }

        let symtabs = self.symtabs.read().await;
        let symtab_ref;
        if let Some(symtab) = symtabs.get(&content_info.content_path) {
            symtab_ref = symtab;
        } else {
            return Err(jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(-1061),
                message: "Symbol table for the content could not be found".into(),
                data: None
            });
        }

        let sym_iter = symtab_ref.get_symbols_for_source(&content_info.source_tree_path.local());
        let script_symbols = format!("{:#?}", sym_iter.collect::<Vec<_>>());

        Ok(requests::debug::script_symbols::Response {
            symbols: script_symbols
        })
    }

    async fn script_cst(&self, params: requests::debug::script_cst::Parameters) -> Result<requests::debug::script_cst::Response> {
        let script_path: AbsPath;
        if let Ok(path) = AbsPath::try_from(params.script_uri) {
            script_path = path;
        } else {
            return Err(jsonrpc::Error::invalid_params("script_uri parameter is not a valid file URI"));
        }

        let script_entry = self.scripts.get(&script_path).ok_or(jsonrpc::Error {
            code: jsonrpc::ErrorCode::ServerError(-1080),
            message: "Script file not found".into(),
            data: None
        })?;

        let script = &script_entry.script;
        let cst = script.root_node().cst_to_string();
        drop(script_entry);

        Ok(requests::debug::script_cst::Response { 
            cst
        })
    }
}