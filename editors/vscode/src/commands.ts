import * as vscode from 'vscode';
import { client } from "./extension"
import * as requests from './requests';
import * as state from './state';


export function registerCommands(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand("witcherscript-ide.workspace.initProject", () => commandInitProject()),
        vscode.commands.registerCommand("witcherscript-ide.workspace.createProject", () => commandCreateProject(context))
    );
}

async function commandInitProject() {
    if (vscode.workspace.workspaceFolders) {
        const projectDirectory = vscode.workspace.workspaceFolders[0].uri;
        const params = new requests.CreateProjectRequest.Parameters(projectDirectory);
        client.sendRequest(requests.CreateProjectRequest.type, params.intoDto(client.code2ProtocolConverter)).then(
            (dto) => {
                const response = requests.CreateProjectRequest.Response.fromDto(client.protocol2CodeConverter, dto);

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

async function commandCreateProject(context: vscode.ExtensionContext) {
    vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        title: "Choose the project folder",
    }).then((folders) => {
        if (folders) {
            const projectDirectory = folders[0];
            const params = new requests.CreateProjectRequest.Parameters(projectDirectory);

            client.sendRequest(requests.CreateProjectRequest.type, params.intoDto(client.code2ProtocolConverter)).then(
                async (dto) => {
                    const response = requests.CreateProjectRequest.Response.fromDto(client.protocol2CodeConverter, dto);

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