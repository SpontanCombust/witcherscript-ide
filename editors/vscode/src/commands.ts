import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs/promises';

import { client } from "./extension"
import * as requests from './requests';
import * as state from './state';


export function registerCommands(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand("witcherscript-ide.projects.init", commandInitProject(context)),
        vscode.commands.registerCommand("witcherscript-ide.projects.create", commandCreateProject(context)),
        vscode.commands.registerCommand("witcherscript-ide.scripts.importVanilla", commandImportVanillaScripts()),
        vscode.commands.registerCommand("witcherscript-ide.scripts.diffVanilla", commandDiffScriptWithVanilla()),
        vscode.commands.registerCommand("witcherscript-ide.debug.showScriptAst", commandShowScriptAst(context)),
        vscode.commands.registerCommand("witcherscript-ide.debug.contentGraphDot", commandContentGraphDot(context))
    );
}

type Cmd = (...args: any[]) => unknown;

function commandInitProject(context: vscode.ExtensionContext): Cmd {
    return async () => {
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

        
        const initProjectName = path.parse(projectDirUri.fsPath).name.replace(' ', '');

        const projectName = await vscode.window.showInputBox({
            prompt: "Enter the name of the project",
            ignoreFocusOut: true,
            value: initProjectName,
            validateInput: validateProjectName
        });

        if (!projectName) {
            return;
        }


        await initializeProjectInDirectory(projectDirUri, projectName, context);
    }
}

function commandCreateProject(context: vscode.ExtensionContext): Cmd {
    return async () => {
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
        await initializeProjectInDirectory(projectDirUri, projectName, context);
    }
}

// Returns undefined if the input is valid, error message otherwise
function validateProjectName(input: string): string | undefined {
    if (input.match(/[_a-zA-Z][_a-zA-Z0-9]*/)) {
        return undefined;
    } else {
        return "Project name is invalid. Please refer to the user manual for more information"
    }
}

async function initializeProjectInDirectory(projectDirUri: vscode.Uri, projectName: string, context: vscode.ExtensionContext) {
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

    if (vscode.workspace.workspaceFolders?.some(wf => manifestUri.fsPath.startsWith(wf.uri.fsPath))) {
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
            const memento = new state.OpenManifestOnInit.Memento(
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


function commandShowScriptAst(context: vscode.ExtensionContext): Cmd {
    const astSuffix = " - AST";

    const tdcp = new (class implements vscode.TextDocumentContentProvider {
        readonly scheme = "witcherscript-ide-ast";

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

    context.subscriptions.push(vscode.workspace.registerTextDocumentContentProvider(tdcp.scheme, tdcp));

    return async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            return;
        }

        const scriptPath = activeEditor.document.uri.fsPath;
        const scriptLine = activeEditor.selection.active.line + 1;
        const uri = vscode.Uri.file(scriptPath + astSuffix).with({ scheme: tdcp.scheme });
        
        const doc = await vscode.workspace.openTextDocument(uri);
        const options: vscode.TextDocumentShowOptions = {
            viewColumn: vscode.ViewColumn.Two,
            preview: false,
            preserveFocus: true
        };

        tdcp.eventEmitter.fire(uri);
        
        vscode.window.showTextDocument(doc, options).then(async editor => {
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

            const rememberedChoices = state.RememberedChoices.Memento.fetchOrDefault(context);
            if (!rememberedChoices.neverShowAgainDebugAstNotif) {
                enum Answer {
                    Close = "I understand",
                    NeverShowAgain = "Never show this message again"
                }

                const answer = await vscode.window.showInformationMessage(
                    "Beware! Displayed ranges in the AST may not be accurate if your document is formatted using tabs instead of spaces",
                    Answer.Close, Answer.NeverShowAgain
                );

                if (answer == Answer.NeverShowAgain) {
                    rememberedChoices.neverShowAgainDebugAstNotif = true;
                    rememberedChoices.store(context);
                }
            }
        });
    };
}

function commandImportVanillaScripts(): Cmd {
    return async () => {
        let projectContentInfo: requests.ContentInfo;
        let content0Info: requests.ContentInfo;

        try {
            // Decide on which project in the workspace is the target of the import

            const projectInfos = (await client.sendRequest(requests.projects.list.type, {
                onlyFromWorkspace: true
            })).projectInfos;

            if (projectInfos.length == 0) {
                return vscode.window.showErrorMessage("No project available to import scripts into!");
            } else {
                const chosen = await chooseProject(projectInfos);
                if (chosen) {
                    projectContentInfo = chosen;
                } else {
                    return;
                }
            }

            // Get information about content0 (if it doesn't exist it will throw)
            content0Info = (await client.sendRequest(requests.projects.vanillaDependencyContent.type, {
                projectUri: projectContentInfo.contentUri
            })).content0Info;
        } catch (error: any) {
            return vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
        }

        // Prompt the user to choose scripts for import

        const content0ScriptsRootUri = client.protocol2CodeConverter.asUri(content0Info.scriptsRootUri);
        let scriptsToImport = await vscode.window.showOpenDialog({
            title: "Select script files",
            openLabel: "Import",
            canSelectFiles: true,
            canSelectFolders: false,
            canSelectMany: true,
            defaultUri: content0ScriptsRootUri,
            filters: {
                'WitcherScript': ['ws']
            },
        });
        
        if (scriptsToImport && scriptsToImport.length > 0) {
            let encounteredProblems = false;
            
            const content0ScriptsRootPath = content0ScriptsRootUri.fsPath;
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

            for (const content0ScriptUri of scriptsToImport) {
                const content0ScriptPath = content0ScriptUri.fsPath;
                const relativePath = path.relative(content0ScriptsRootPath, content0ScriptPath);
                const projectScriptPath = path.join(projectScriptsRootPath, relativePath);

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
                        client.info(`Successfully imported ${relativePath} into the project`);
                    } catch (err) {
                        client.error(`Failed to import script ${relativePath}: ${err}`);
                    }
                }
            }

            if (encounteredProblems) {
                vscode.window.showWarningMessage("Scripts imported with some problems (check extension output)");
            } else {
                vscode.window.showInformationMessage("Successfully imported vanilla scripts into the project!")
            }
        }
    }
}

function commandDiffScriptWithVanilla(): Cmd {
    return async () => {
        if (!vscode.window.activeTextEditor) {
            vscode.window.showErrorMessage("No active editor available!");
            return;
        }

        const currentScriptUri = vscode.window.activeTextEditor.document.uri;

        let currentContent: requests.ContentInfo;
        let vanillaContent: requests.ContentInfo;
        try {
            currentContent = (await client.sendRequest(requests.scripts.parent_content.type, {
                scriptUri: client.code2ProtocolConverter.asUri(currentScriptUri)
            })).parentContentInfo;

            vanillaContent = (await client.sendRequest(requests.projects.vanillaDependencyContent.type, {
                projectUri: currentContent.contentUri
            })).content0Info;
        } catch(error: any) {
            return vscode.window.showErrorMessage(`${error.message} [code ${error.code}]`);
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
        return await vscode.commands.executeCommand("vscode.diff", vanillaScriptUri, currentScriptUri);
    }
}

function commandContentGraphDot(context: vscode.ExtensionContext): Cmd {
    const tdcp = new (class implements vscode.TextDocumentContentProvider {
        readonly scheme = "witcherscript-ide-graph-dot";

        eventEmitter = new vscode.EventEmitter<vscode.Uri>();

        provideTextDocumentContent(_: vscode.Uri): vscode.ProviderResult<string> {
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
    })();

    context.subscriptions.push(vscode.workspace.registerTextDocumentContentProvider(tdcp.scheme, tdcp));

    return async () => {
        const virtFileName = "WitcherScript Content Graph";
        const uri = vscode.Uri.file(virtFileName).with({ scheme: tdcp.scheme });

        const doc = await vscode.workspace.openTextDocument(uri);
        const options: vscode.TextDocumentShowOptions = {
            viewColumn: vscode.ViewColumn.Two,
            preview: false,
            preserveFocus: true
        };

        tdcp.eventEmitter.fire(uri);

        await vscode.window.showTextDocument(doc, options);
    }
}




/**
 * Returns the result of user's choice of a project through a quick pick.
 * If there is only one project available, returns that without asking the user.
 * If there are no projects in the workspace or the user cancels the action, returns undefined
 * 
 * @returns ContentInfo or undefined
 */
async function chooseProject(projects: requests.ContentInfo[]): Promise<requests.ContentInfo | undefined> {
    if (!projects || projects.length == 0) {
        return undefined;
    } else if (projects.length == 1) {
        return projects[0];
    } else {
        return await new Promise<requests.ContentInfo | undefined>((resolve, _) => {
            const qp = vscode.window.createQuickPick<ContentQuickPickItem>();
            qp.placeholder = "Select a project";
            qp.canSelectMany = false;
            qp.matchOnDetail = true;
            qp.ignoreFocusOut = true;
            qp.items = projects.map(c => new ContentQuickPickItem(c));
            qp.onDidChangeSelection(items => {
                resolve(items[0].content);
                qp.hide();
            });
            qp.onDidHide(_ => {
                resolve(undefined);
                qp.dispose();
            })

            qp.show();
        })
    }
}

class ContentQuickPickItem implements vscode.QuickPickItem {
    public content: requests.ContentInfo

    label: string;
    kind?: vscode.QuickPickItemKind;
    iconPath?: vscode.Uri | { light: vscode.Uri; dark: vscode.Uri; } | vscode.ThemeIcon;
    description?: string;
    detail?: string;
    picked?: boolean;
    alwaysShow?: boolean;
    buttons?: readonly vscode.QuickInputButton[];

    constructor(content: requests.ContentInfo) {
        this.content = content;

        this.label = content.contentName;
        this.kind = vscode.QuickPickItemKind.Default;
        this.iconPath = undefined;
        this.description = undefined;
        this.detail = vscode.Uri.parse(content.contentUri).fsPath;
        this.alwaysShow = true;
        this.buttons = undefined;
    }
}