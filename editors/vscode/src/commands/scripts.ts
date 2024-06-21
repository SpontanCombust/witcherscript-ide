import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs/promises';

import { getLanguageClient } from "../lsp/lang_client"
import * as requests from '../lsp/requests';
import * as notifications from '../lsp/notifications';
import * as model from '../lsp/model'
import * as utils from '../utils';
import { Cmd } from './index'
import { VanillaFile } from '../providers/vanilla_files_provider';


export function commandImportVanillaScripts(): Cmd {
    return async (params?: VanillaFile) => {
        const client = getLanguageClient();
        if (client == undefined) {
            vscode.window.showErrorMessage("Language Server is not active!");
            return;
        }
        
        let projectContentInfo: model.ContentInfo;
        let content0Info: model.ContentInfo;

        try {
            // Decide on which project in the workspace is the target of the import

            const projectInfos = (await client.sendRequest(requests.projects.list.type, {
                onlyFromWorkspace: true
            })).projectInfos;

            if (projectInfos.length == 0) {
                enum Answer {
                    Close = "I understand",
                    SeeManual = "See manual"
                }

                const manualUri = vscode.Uri.parse("https://spontancombust.github.io/witcherscript-ide/user-manual/");

                const answer = await vscode.window.showErrorMessage(
                    "No project available to import scripts into.\n" +
                    "To learn about creating projects see the manual:\n" + manualUri.toString(),
                    Answer.Close, Answer.SeeManual
                );

                if (answer == Answer.SeeManual) {
                    await vscode.env.openExternal(manualUri);
                }

                return;
            } else {
                const chosen = await utils.chooseProject(projectInfos);
                if (chosen) {
                    projectContentInfo = chosen;
                } else {
                    return;
                }
            }

        } catch (error: any) {
            return vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
        }
        

        let content0ScriptsRootPath: string;
        let scriptsToImport: vscode.Uri[];

        if (params != undefined) {
            content0ScriptsRootPath = params.scriptsRootPath;
            scriptsToImport = params.resourceUri ? [params.resourceUri] : [];
        } else {
            try {
                content0Info = (await client.sendRequest(requests.projects.vanillaDependencyContent.type, {
                    projectUri: projectContentInfo.contentUri
                })).content0Info;

                const content0ScriptsRootUri = client.protocol2CodeConverter.asUri(content0Info.scriptsRootUri);

                content0ScriptsRootPath = content0ScriptsRootUri.fsPath;
                scriptsToImport = await vscode.window.showOpenDialog({
                    title: "Select script files",
                    openLabel: "Import",
                    canSelectFiles: true,
                    canSelectFolders: false,
                    canSelectMany: true,
                    defaultUri: content0ScriptsRootUri,
                    filters: {
                        'WitcherScript': ['ws']
                    },
                }) ?? [];
            } catch (error: any) {
                return vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
            } 
        }

        if (scriptsToImport.length > 0) {
            let encounteredProblems = false;
            const projectScriptsRootPath = client.protocol2CodeConverter.asUri(projectContentInfo.scriptsRootUri).fsPath;

            scriptsToImport = scriptsToImport.filter(uri => {
                const isContent0Script = uri.fsPath.startsWith(content0ScriptsRootPath);

                if (!isContent0Script) {
                    client.warn(uri.fsPath + " is not a content0 script!", undefined, false);
                    encounteredProblems = true;
                }

                return isContent0Script;
            });

            // Finally import scripts while doing a little validation

            let scriptsImported = [];
            for (const content0ScriptUri of scriptsToImport) {
                const content0ScriptPath = content0ScriptUri.fsPath;
                const relativePath = path.relative(content0ScriptsRootPath, content0ScriptPath);
                const projectScriptPath = path.join(projectScriptsRootPath, relativePath);
                const projectScriptUri = vscode.Uri.file(projectScriptPath);

                let fileAlreadyExists = true;
                try {
                    await fs.stat(projectScriptPath);
                } catch(_) {
                    fileAlreadyExists = false;
                }

                if (fileAlreadyExists) {
                    client.warn(`Script ${relativePath} already exists in project's source tree`);
                    encounteredProblems = true;
                } else {
                    try {
                        const projectScriptDir = path.dirname(projectScriptPath);
                        // make sure that all the intermediary path components exist
                        await fs.mkdir(projectScriptDir, { recursive: true });
                        await fs.copyFile(content0ScriptPath, projectScriptPath);
                        await vscode.window.showTextDocument(projectScriptUri, { preview: false });
                        client.info(`Successfully imported ${relativePath} into the project`);
                        scriptsImported.push(projectScriptUri);
                    } catch (err) {
                        client.error(`Failed to import script ${relativePath}: ${err}`);
                    }
                }
            }

            client.sendNotification(notifications.projects.didImportScripts.type, {
                importedScriptsUris: scriptsImported.map(uri => client.code2ProtocolConverter.asUri(uri))
            })

            if (encounteredProblems) {
                vscode.window.showWarningMessage("Scripts imported with some problems (check extension output)");
            } else {
                vscode.window.showInformationMessage("Successfully imported vanilla scripts into the project!")
            }
        }
    }
}

export function commandDiffScriptWithVanilla(context: vscode.ExtensionContext): Cmd {
    return async () => {
        const client = getLanguageClient();
        if (client == undefined) {
            vscode.window.showErrorMessage("Language Server is not active!");
            return;
        }
        
        if (!vscode.window.activeTextEditor) {
            vscode.window.showErrorMessage("No active editor available!");
            return;
        }

        const currentScriptUri = vscode.window.activeTextEditor.document.uri;

        let currentContent: model.ContentInfo;
        let vanillaContent: model.ContentInfo;
        try {
            currentContent = (await client.sendRequest(requests.scripts.parent_content.type, {
                scriptUri: client.code2ProtocolConverter.asUri(currentScriptUri)
            })).parentContentInfo;

            vanillaContent = (await client.sendRequest(requests.projects.vanillaDependencyContent.type, {
                projectUri: currentContent.contentUri
            })).content0Info;
        } catch(error: any) {
            vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);

            if (error.code == -1021) {
                utils.showForeignScriptWarning(context);
            }

            return;
        }

        const currentScriptPath = currentScriptUri.fsPath;
        const currentScriptRootPath = client.protocol2CodeConverter.asUri(currentContent.scriptsRootUri).fsPath;
        const vanillaScriptRootPath = client.protocol2CodeConverter.asUri(vanillaContent.scriptsRootUri).fsPath;

        const relativePath = path.relative(currentScriptRootPath, currentScriptPath);
        const vanillaScriptPath = path.join(vanillaScriptRootPath, relativePath);

        let counterpartExists = true;
        try {
            await fs.stat(vanillaScriptPath);
        } catch(_) {
            counterpartExists = false;
        }

        if (!counterpartExists) {
            return vscode.window.showErrorMessage(`Script ${relativePath} does not have a vanilla counterpart`);
        }

        const vanillaScriptUri = vscode.Uri.file(vanillaScriptPath);
        const scriptName = path.basename(vanillaScriptPath);
        const title = `${scriptName} (vanilla) ↔ ${scriptName} (modded)`;
        return await vscode.commands.executeCommand("vscode.diff", vanillaScriptUri, currentScriptUri, title);
    }
}