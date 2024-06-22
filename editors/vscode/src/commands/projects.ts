import * as vscode from 'vscode';
import * as lsp from 'vscode-languageclient/node';
import * as path from 'path';
import * as fs from 'fs/promises';

import { getLanguageClient } from "../lsp/lang_client"
import { getVanillaFilesProvider } from '../providers/vanilla_files_provider';
import { getScriptContentProvider } from '../providers/script_content_provider';
import * as requests from '../lsp/requests';
import * as persistence from '../persistence';
import * as utils from '../utils';
import { Cmd } from './index'


export function commandInitProject(context: vscode.ExtensionContext): Cmd {
    return async () => {
        const client = getLanguageClient();
        if (client == undefined) {
            vscode.window.showErrorMessage("Language Server needs to be active!");
            return;
        }

        const initialDirUri = vscode.workspace.workspaceFolders ? vscode.workspace.workspaceFolders[0].uri : undefined;

        const projectDirUri = await vscode.window.showOpenDialog({
            canSelectFiles: false,
            canSelectFolders: true,
            canSelectMany: false,
            defaultUri: initialDirUri,
            title: "Choose project directory"
        }).then(folders => {
            return folders ? folders[0] : undefined;
        });

        if (!projectDirUri) {
            return;
        }

        let initProjectName = path.basename(projectDirUri.fsPath).replace(' ', '');
        if (initProjectName == "content") {
            const parentDir = path.dirname(projectDirUri.fsPath);
            initProjectName = path.basename(parentDir).replace(' ', '');
        }

        const projectName = await vscode.window.showInputBox({
            prompt: "Enter the name of the project",
            ignoreFocusOut: true,
            value: initProjectName,
            validateInput: validateProjectName
        });

        if (!projectName) {
            return;
        }


        await initializeProjectInDirectory(client, projectDirUri, projectName, context);
    }
}

export function commandCreateProject(context: vscode.ExtensionContext): Cmd {
    return async () => {
        const client = getLanguageClient();
        if (client == undefined) {
            vscode.window.showErrorMessage("Language Server needs to be active!");
            return;
        }

        const projectName = await vscode.window.showInputBox({
            prompt: "Enter the name of the project",
            ignoreFocusOut: true,
            validateInput: validateProjectName
        });

        if (!projectName) {
            return;
        }


        const initialDirUri = vscode.workspace.workspaceFolders ? vscode.workspace.workspaceFolders[0].uri : undefined;

        const containingDirUri = await vscode.window.showOpenDialog({
            canSelectFiles: false,
            canSelectFolders: true,
            canSelectMany: false,
            defaultUri: initialDirUri,
            title: "Choose project folder destination"
        }).then(folders => {
            return folders ? folders[0] : undefined;
        });

        if (!containingDirUri) {
            return;
        }

        const projectDir = path.join(containingDirUri.fsPath, projectName);

        try {
            await fs.mkdir(projectDir, { recursive: false });
        } catch (error: any) {
            vscode.window.showErrorMessage("Failed to create project folder: " + error);
            return;
        }


        const projectDirUri = vscode.Uri.file(projectDir);
        await initializeProjectInDirectory(client, projectDirUri, projectName, context);
    }
}

// Returns undefined if the input is valid, error message otherwise
function validateProjectName(input: string): string | undefined {
    if (/^[_a-zA-Z][_a-zA-Z0-9]*$/.test(input)) {
        return undefined;
    } else {
        return "Project name is invalid. Please refer to the user manual for more information"
    }
}

async function initializeProjectInDirectory(client: lsp.LanguageClient, projectDirUri: vscode.Uri, projectName: string, context: vscode.ExtensionContext) {
    let manifestUri: vscode.Uri;
    try {
        const params: requests.projects.create.Parameters = {
            directoryUri: client.code2ProtocolConverter.asUri(projectDirUri),
            projectName: projectName
        }

        const response = await client.sendRequest(requests.projects.create.type, params);
        manifestUri = client.protocol2CodeConverter.asUri(response.manifestUri);

    } catch (error: any) {
        vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
        return;
    }

    if (vscode.workspace.workspaceFolders?.some(wf => utils.isSubpathOf(manifestUri.fsPath, wf.uri.fsPath))) {
        const showOptions: vscode.TextDocumentShowOptions = {
            preview: false
        };
        vscode.window.showTextDocument(manifestUri, showOptions).then(
            _ => {},
            (err) => client.debug('Manifest could not be shown: ' + err)
        );
    } else {
        enum Answer {
            YesHere = "Open in this window",
            YesInNew = "Open in a new window",
            No = "No"
        }

        const answer = await vscode.window.showInformationMessage("Would you like to open the project?",
            Answer.YesHere, Answer.YesInNew, Answer.No);

        if (answer != undefined && answer != Answer.No) {
            const memento = new persistence.OpenManifestOnInit.Memento(
                projectDirUri,
                manifestUri
            );

            await memento.store(context);

            const openNewWindow = answer == Answer.YesInNew;
            await vscode.commands.executeCommand("vscode.openFolder", projectDirUri, {
                forceNewWindow: openNewWindow
            });
        }
    }
}

export function commandRefreshVanillaFilesView(): Cmd {
    return () => {
        getVanillaFilesProvider().refreshAll();
    }
}

export function commandRefreshScriptContentView(): Cmd {
    return () => {
        getScriptContentProvider().refreshAll();
    }
}