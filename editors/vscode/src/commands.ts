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
        const projectDirectory = client.code2ProtocolConverter.asUri(vscode.workspace.workspaceFolders[0].uri);
        const params: requests.CreateProjectRequest.Parameters = {
            directoryUri: projectDirectory
        };
        client.sendRequest(requests.CreateProjectRequest.type, params).then(
            (value) => {
                const manifestUri = vscode.Uri.parse(value.manifestUri);
                vscode.window.showTextDocument(manifestUri);
            },
            (error) => {
                vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
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
    }).then((value) => {
        if (value) {
            const projectDirectory = client.code2ProtocolConverter.asUri(value[0]);
            const params: requests.CreateProjectRequest.Parameters = {
                directoryUri: projectDirectory
            };

            client.sendRequest(requests.CreateProjectRequest.type, params).then(
                async (value) => {
                    // ask the user if they'd like to open the project
                    // this is similar to `Git: clone` command
                    enum Answer {
                        YES_HERE = "Open in this window",
                        YES_IN_NEW = "Open in a new window",
                        NO = "No"
                    }

                    const answer = await vscode.window.showInformationMessage("Would you like to open the project?",
                        Answer.YES_HERE, Answer.YES_IN_NEW, Answer.NO);

                    if (answer && answer != Answer.NO) {
                        const projectDirectoryUri = vscode.Uri.parse(projectDirectory);
                        const openNewWindow = answer == Answer.YES_IN_NEW;
                        await vscode.commands.executeCommand("vscode.openFolder", projectDirectoryUri, {
                            forceNewWindow: openNewWindow
                        });

                        //TODO use ExtensionContext.globalState to open manifest in the new window
                        // const manifestUri = vscode.Uri.parse(value.manifestUri);
                        // vscode.window.showTextDocument(manifestUri);
                    }
                },
                (error) => {
                    vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
                }
            )
        }
    })
}