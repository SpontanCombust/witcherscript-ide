import * as vscode from 'vscode';

import { getLanguageClient } from "../lsp/lang_client"
import * as requests from '../lsp/requests';


export class ScriptAstProvider implements vscode.TextDocumentContentProvider {
    private static instance: ScriptAstProvider;

    private constructor() {}
    public static getInstance(): ScriptAstProvider {
        if (!ScriptAstProvider.instance) {
            ScriptAstProvider.instance = new ScriptAstProvider();
        }

        return ScriptAstProvider.instance;
    }


    public static readonly scheme = "witcherscript-ide-ast";
    public static readonly pathSuffix = " - AST";

    public eventEmitter = new vscode.EventEmitter<vscode.Uri>();


    provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        const client = getLanguageClient();
        if (client == undefined) {
            vscode.window.showErrorMessage("Language Server is not active!");
            return;
        }

        // VSCode at the time of writing this does not provide any quick and easy way to display a custom tab label.
        // Its default way of getting the tab name is the file name component of URI passed to openTextDocument.
        // So if I want to display "{file} - AST" I need to do a bit of URI hacking and pass the whole thing to it.
        // Anyways, LS needs name of the actual file, so the decoratory suffix needs to be gone from that URI.
        uri = vscode.Uri.file(uri.fsPath.substring(0, uri.fsPath.length - ScriptAstProvider.pathSuffix.length));

        const params: requests.debug.scriptAst.Parameters = {
            scriptUri: client.code2ProtocolConverter.asUri(uri)
        }
        return client.sendRequest(requests.debug.scriptAst.type, params).then(
            (response) => {
                return response.ast;
            },
            (error) => {
                vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
                return ""
            }
        )
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event;
    }
}


export class ScriptCstProvider implements vscode.TextDocumentContentProvider {
    private static instance: ScriptCstProvider;

    private constructor() {}
    public static getInstance(): ScriptCstProvider {
        if (!ScriptCstProvider.instance) {
            ScriptCstProvider.instance = new ScriptCstProvider();
        }

        return ScriptCstProvider.instance;
    }


    public static readonly scheme = "witcherscript-ide-cst";
    public static readonly pathSuffix = " - CST";

    public eventEmitter = new vscode.EventEmitter<vscode.Uri>();


    provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        const client = getLanguageClient();
        if (client == undefined) {
            vscode.window.showErrorMessage("Language Server is not active!");
            return;
        }

        uri = vscode.Uri.file(uri.fsPath.substring(0, uri.fsPath.length - ScriptCstProvider.pathSuffix.length));

        const params: requests.debug.scriptCst.Parameters = {
            scriptUri: client.code2ProtocolConverter.asUri(uri)
        }
        return client.sendRequest(requests.debug.scriptCst.type, params).then(
            (response) => {
                return response.cst;
            },
            (error) => {
                vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
                return ""
            }
        )
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event;
    }
}


export class ContentGraphDotProvider implements vscode.TextDocumentContentProvider {
    private static instance: ContentGraphDotProvider;

    private constructor() {}
    public static getInstance(): ContentGraphDotProvider {
        if (!ContentGraphDotProvider.instance) {
            ContentGraphDotProvider.instance = new ContentGraphDotProvider();
        }

        return ContentGraphDotProvider.instance;
    }


    public static readonly scheme = "witcherscript-ide-graph-dot";

    public eventEmitter = new vscode.EventEmitter<vscode.Uri>();

    provideTextDocumentContent(_: vscode.Uri): vscode.ProviderResult<string> {
        const client = getLanguageClient();
        if (client == undefined) {
            vscode.window.showErrorMessage("Language Server is not active!");
            return;
        }

        const params: requests.debug.contentGraphDot.Parameters = {};
        return client.sendRequest(requests.debug.contentGraphDot.type, params).then(
            (response) => {
                return response.dotGraph;
            },
            (error) => {
                vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
                return ""
            }
        )
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event;
    }
}


export class ScriptSymbolsProvider implements vscode.TextDocumentContentProvider {
    private static instance: ScriptSymbolsProvider;

    private constructor() {}
    public static getInstance(): ScriptSymbolsProvider {
        if (!ScriptSymbolsProvider.instance) {
            ScriptSymbolsProvider.instance = new ScriptSymbolsProvider();
        }

        return ScriptSymbolsProvider.instance;
    }


    public static readonly scheme = "witcherscript-ide-symbols";
    public static readonly pathSuffix = " - symbols";

    public eventEmitter = new vscode.EventEmitter<vscode.Uri>();

    provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        const client = getLanguageClient();
        if (client == undefined) {
            vscode.window.showErrorMessage("Language Server is not active!");
            return;
        }
        
        uri = vscode.Uri.file(uri.fsPath.substring(0, uri.fsPath.length - ScriptSymbolsProvider.pathSuffix.length));

        const params: requests.debug.scriptSymbols.Parameters = {
            scriptUri: client.code2ProtocolConverter.asUri(uri)
        }
        return client.sendRequest(requests.debug.scriptSymbols.type, params).then(
            (response) => {
                return response.symbols;
            },
            (error) => {
                vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
                return ""
            }
        )
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event;
    }
}


export class ReadOnlyContentProvider implements vscode.TextDocumentContentProvider {
    private static instance: ReadOnlyContentProvider;

    private constructor() {}
    public static getInstance(): ReadOnlyContentProvider {
        if (!ReadOnlyContentProvider.instance) {
            ReadOnlyContentProvider.instance = new ReadOnlyContentProvider();
        }

        return ReadOnlyContentProvider.instance;
    }


    public static readonly scheme = "witcherscript-ide-file-readonly";

    public eventEmitter = new vscode.EventEmitter<vscode.Uri>();

    provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        return vscode.workspace.openTextDocument(uri.with({ scheme: 'file' }))
            .then(td => td.getText())
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event;
    }
}