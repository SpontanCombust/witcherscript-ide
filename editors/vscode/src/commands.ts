import * as vscode from 'vscode';
import { client } from "./extension"
import * as requests from './requests';


export function registerCommands(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand("witcherscript-ide.workspace.initProject", commandInitProject),
        vscode.commands.registerCommand("witcherscript-ide.workspace.createProject", commandCreateProject)
    );
}

async function commandInitProject() {
    if (vscode.workspace.workspaceFolders) {
        const params: requests.CreateProjectRequest.Parameters = {
            directoryUri: vscode.workspace.workspaceFolders[0].uri
        };
        client.sendRequest(requests.CreateProjectRequest.type, params).then(
            (value) => {
                vscode.workspace.openTextDocument(value.manifestUri);
            },
            (error) => {
                vscode.window.showErrorMessage(error);
            }
        )
    } else {
        vscode.window.showErrorMessage("No workspace folder is opened in the editor");
    }
}

async function commandCreateProject() {
    vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        title: "Choose the project folder",
    }).then(async (value) => {
        if (value) {
            const projectDirectory = value[0];
            const params: requests.CreateProjectRequest.Parameters = {
                directoryUri: projectDirectory
            };

            //FIXME no response?
            await client.sendRequest(requests.CreateProjectRequest.type, params).then(
                async (value) => {
                    // if the chosen directory is not a subdirectory in the workspace
                    // ask the user if they'd like to open the project
                    // this is similar to `Git: clone` command
                    if (vscode.workspace.workspaceFolders.some(folder => projectDirectory.fsPath.startsWith(folder.uri.fsPath))) {
                        vscode.workspace.openTextDocument(value.manifestUri);
                    } else {
                        enum Answer {
                            YES_HERE = "Open in this window",
                            YES_IN_NEW = "Open in a new window",
                            NO = "No"
                        }

                        const answer = await vscode.window.showInformationMessage("Would you like to open the project?",
                            Answer.YES_HERE, Answer.YES_IN_NEW, Answer.NO);

                        if (answer && answer != Answer.NO) {
                            const openNewWindow = answer == Answer.YES_IN_NEW;
                            vscode.commands.executeCommand("vscode.openFolder", projectDirectory, {
                                forceNewWindow: openNewWindow
                            });
                        }
                    }
                },
                (error) => {
                    vscode.window.showErrorMessage(error);
                }
            )
        }
    })
}