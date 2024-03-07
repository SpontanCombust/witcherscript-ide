import * as vscode from 'vscode';
import { client } from "./extension"
import * as requests from './requests';
import * as state from './state';


export function registerCommands(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand("witcherscript-ide.workspace.initProject", commandInitProject()),
        vscode.commands.registerCommand("witcherscript-ide.workspace.createProject", commandCreateProject(context)),
        vscode.commands.registerCommand("witcherscript-ide.debug.showScriptAst", commandShowScriptAst(context))
    );
}

type Cmd = (...args: any[]) => unknown;

function commandInitProject(): Cmd {
    return async () => {
        if (vscode.workspace.workspaceFolders) {
            const projectDirectory = vscode.workspace.workspaceFolders[0].uri;
            const params = new requests.CreateProject.Parameters(projectDirectory);
            client.sendRequest(requests.CreateProject.type, params.intoDto(client.code2ProtocolConverter)).then(
                (dto) => {
                    const response = requests.CreateProject.Response.fromDto(client.protocol2CodeConverter, dto);
    
                    const params: vscode.TextDocumentShowOptions = {
                        selection: response.manifestContentNameRange,
                        preview: false
                    };
                    vscode.window.showTextDocument(response.manifestUri, params).then(
                        _ => {},
                        (err) => client.debug('Manifest could not be shown: ' + err)
                    );
                },
                (error) => {
                    vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
                }
            )
        } else {
            vscode.window.showErrorMessage("No workspace folder is opened in the editor");
        }
    }
}

function commandCreateProject(context: vscode.ExtensionContext): Cmd {
    return async () => {
        vscode.window.showOpenDialog({
            canSelectFiles: false,
            canSelectFolders: true,
            canSelectMany: false,
            title: "Choose the project folder",
        }).then((folders) => {
            if (folders) {
                const projectDirectory = folders[0];
                const params = new requests.CreateProject.Parameters(projectDirectory);
    
                client.sendRequest(requests.CreateProject.type, params.intoDto(client.code2ProtocolConverter)).then(
                    async (dto) => {
                        const response = requests.CreateProject.Response.fromDto(client.protocol2CodeConverter, dto);
    
                        // check if the project directory is contained somewhere inside the workspace
                        // in this case just open the manifest
                        // otherwise, ask the user if they'd like to open the project
                        if (vscode.workspace.workspaceFolders.some(f => projectDirectory.fsPath.startsWith(f.uri.fsPath))) {
                            const params: vscode.TextDocumentShowOptions = {
                                selection: response.manifestContentNameRange,
                                preview: false
                            };
                            vscode.window.showTextDocument(response.manifestUri, params).then(
                                _ => {},
                                (err) => client.debug('Manifest could not be shown: ' + err)
                            );
                        } else {
                            enum Answer {
                                YES_HERE = "Open in this window",
                                YES_IN_NEW = "Open in a new window",
                                NO = "No"
                            }
        
                            const answer = await vscode.window.showInformationMessage("Would you like to open the project?",
                                Answer.YES_HERE, Answer.YES_IN_NEW, Answer.NO);
        
                            if (answer != undefined && answer != Answer.NO) {
                                const memento = new state.OpenManifestOnInit.Memento(
                                    projectDirectory,
                                    response.manifestUri,
                                    response.manifestContentNameRange
                                );
                                await context.globalState.update(state.OpenManifestOnInit.KEY, memento.intoDto());
        
                                const openNewWindow = answer == Answer.YES_IN_NEW;
                                await vscode.commands.executeCommand("vscode.openFolder", projectDirectory, {
                                    forceNewWindow: openNewWindow
                                });
                            }
                        }
                    },
                    (error) => {
                        vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
                    }
                )
            }
        })
    }
}

function commandShowScriptAst(context: vscode.ExtensionContext): Cmd {
    const astSuffix = " - AST";

    const tdcp = new (class implements vscode.TextDocumentContentProvider {
        readonly schema = "witcherscript-ide-ast";

        eventEmitter = new vscode.EventEmitter<vscode.Uri>();

        provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
            // VSCode at the time of writing this does not provide any quick and easy way to display a custom tab label.
            // Its default way of getting the tab name is the file name component of URI passed to openTextDocument.
            // So if I want to display "{file} - AST" I need to do a bit of URI hacking and pass the whole thing to it.
            // Anyways, LS needs name of the actual file, so the decoratory suffix needs to be gone from that URI.
            uri = vscode.Uri.file(uri.fsPath.substring(0, uri.fsPath.length - astSuffix.length));

            const params = new requests.ScriptAst.Parameters(uri);
            return client.sendRequest(requests.ScriptAst.type, params.intoDto(client.code2ProtocolConverter)).then(
                (dto) => {
                    const response = requests.ScriptAst.Response.fromDto(client.protocol2CodeConverter, dto);
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
    })();

    context.subscriptions.push(vscode.workspace.registerTextDocumentContentProvider(tdcp.schema, tdcp));

    return async () => {
        if (!vscode.window.activeTextEditor) {
            return;
        }

        const scriptPath = vscode.window.activeTextEditor.document.uri.fsPath;
        const uri = vscode.Uri.file(scriptPath + astSuffix).with({ scheme: tdcp.schema });
        const options: vscode.TextDocumentShowOptions = {
            viewColumn: vscode.ViewColumn.Two,
            preview: false,
            preserveFocus: true
        };

        const doc = await vscode.workspace.openTextDocument(uri);
        tdcp.eventEmitter.fire(uri);
        return vscode.window.showTextDocument(doc, options);
    };
}