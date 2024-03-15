import * as vscode from 'vscode';
import { client } from "./extension"
import * as requests from './requests';
import * as state from './state';


export function registerCommands(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand("witcherscript-ide.projects.init", commandInitProject()),
        vscode.commands.registerCommand("witcherscript-ide.projects.create", commandCreateProject(context)),
        vscode.commands.registerCommand("witcherscript-ide.debug.showScriptAst", commandShowScriptAst(context))
    );
}

type Cmd = (...args: any[]) => unknown;

function commandInitProject(): Cmd {
    return async () => {
        if (vscode.workspace.workspaceFolders) {
            const projectDirectory = vscode.workspace.workspaceFolders[0].uri;
            const params: requests.projects.create.Parameters = {
                directoryUri: client.code2ProtocolConverter.asUri(projectDirectory)
            }
            client.sendRequest(requests.projects.create.type, params).then(
                (response) => {
                    const manifestUri = client.protocol2CodeConverter.asUri(response.manifestUri);
                    const manifestSelectionRange = client.protocol2CodeConverter.asRange(response.manifestContentNameRange);
    
                    const showOptions: vscode.TextDocumentShowOptions = {
                        selection: manifestSelectionRange,
                        preview: false
                    };
                    vscode.window.showTextDocument(manifestUri, showOptions).then(
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
                const projectDirectoryUri = folders[0];
                const params: requests.projects.create.Parameters = {
                    directoryUri: client.code2ProtocolConverter.asUri(projectDirectoryUri)
                }
    
                client.sendRequest(requests.projects.create.type, params).then(
                    async (response) => {
                        const manifestUri = client.protocol2CodeConverter.asUri(response.manifestUri);
                        const manifestSelectionRange = client.protocol2CodeConverter.asRange(response.manifestContentNameRange);
    
                        // check if the project directory is contained somewhere inside the workspace
                        // in this case just open the manifest
                        // otherwise, ask the user if they'd like to open the project
                        if (vscode.workspace.workspaceFolders.some(f => projectDirectoryUri.fsPath.startsWith(f.uri.fsPath))) {
                            const params: vscode.TextDocumentShowOptions = {
                                selection: manifestSelectionRange,
                                preview: false
                            };
                            vscode.window.showTextDocument(manifestUri, params).then(
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
                                    projectDirectoryUri,
                                    manifestUri,
                                    manifestSelectionRange
                                );
                                await context.globalState.update(state.OpenManifestOnInit.KEY, memento.intoDto());
        
                                const openNewWindow = answer == Answer.YES_IN_NEW;
                                await vscode.commands.executeCommand("vscode.openFolder", projectDirectoryUri, {
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
//TODO in the release description disclose that ranges won't be accurate when the document is indented using tabs instead of spaces
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
    })();

    context.subscriptions.push(vscode.workspace.registerTextDocumentContentProvider(tdcp.schema, tdcp));

    return async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            return;
        }

        const scriptPath = activeEditor.document.uri.fsPath;
        const scriptLine = activeEditor.selection.active.line + 1;
        const uri = vscode.Uri.file(scriptPath + astSuffix).with({ scheme: tdcp.schema });
        
        const doc = await vscode.workspace.openTextDocument(uri);
        const options: vscode.TextDocumentShowOptions = {
            viewColumn: vscode.ViewColumn.Two,
            preview: false,
            preserveFocus: true
        };

        tdcp.eventEmitter.fire(uri);
        
        vscode.window.showTextDocument(doc, options).then(editor => {
            const astText = editor.document.getText();
            // Searching for corresponding node in AST text.
            // A naive approach leveraging the format of returned AST text.
            // Nodes represented there together with their names have a range at which they appear.
            // E.g. Identifier [10, 1] - [10, 5]
            // with [line1, column1] - [line2, column2] being the range in question.
            // So here I just search for such a range that hopefully appears in the AST.
            const match = astText.search(new RegExp("\\[" + scriptLine));
            if (match != -1) {
                const targetPos = editor.document.positionAt(match);
                // Scroll the cursor in AST's editor to searched position
                editor.revealRange(new vscode.Range(targetPos, targetPos), vscode.TextEditorRevealType.AtTop);
            }
        })
    };
}