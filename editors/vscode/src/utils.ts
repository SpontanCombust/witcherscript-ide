import * as vscode from 'vscode';
import * as path from 'path';

import * as state from './state';
import * as requests from './lsp/requests'


export async function showForeignScriptWarning(context: vscode.ExtensionContext) {
    const rememberedChoices = state.RememberedChoices.Memento.fetchOrDefault(context);
    if (!rememberedChoices.neverShowAgainForeignScriptWarning) {
        enum Answer {
            Close = "I understand",
            NeverShowAgain = "Don't show this message again",
            SeeManual = "See manual"
        }

        const manualUri = vscode.Uri.parse("https://spontancombust.github.io/witcherscript-ide/user-manual/");

        const answer = await vscode.window.showWarningMessage(
            "This script file is not included through any workspace project or their dependencies.\n" +
            "If you want to use more than the most basic features of the extension you need to create a script project.\n" +
            "To learn about creating projects see the manual:\n" + manualUri.toString(),
            Answer.Close, Answer.NeverShowAgain, Answer.SeeManual
        );

        if (answer == Answer.NeverShowAgain) {
            rememberedChoices.neverShowAgainForeignScriptWarning = true;
            rememberedChoices.store(context);
        }
        else if (answer == Answer.SeeManual) {
            await vscode.env.openExternal(manualUri);
        }
    }
}



/**
 * Returns the result of user's choice of a project through a quick pick.
 * If there is only one project available, returns that without asking the user.
 * If there are no projects in the workspace or the user cancels the action, returns undefined
 * 
 * @returns ContentInfo or undefined
 */
export async function chooseProject(projects: requests.ContentInfo[]): Promise<requests.ContentInfo | undefined> {
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


export function isSubpathOf(dir: string, parent: string): boolean {
    if (dir === parent) return false;

    let parentComps = parent.split(path.sep).filter(i => i.length);
    let dirComps = dir.split(path.sep).filter(i => i.length);

    return parentComps.every((comp, i) => dirComps[i] === comp);
}