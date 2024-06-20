import * as vscode from 'vscode';

import { getLanguageClient } from './lsp/lang_client';
import * as requests from './lsp/requests';
import * as model from './lsp/model';
import * as config from './config'


let contextStatusBar: vscode.StatusBarItem;
let lastContentInfo: model.ContentInfo | undefined = undefined;

/// Get info about the content to which belongs the last viewed script 
export function getLastContentInfo(): model.ContentInfo | undefined {
    return lastContentInfo;
}


export interface ParsingScriptsWork {
    kind: 'parsing-scripts',
    contentName: string
}

export type Work = ParsingScriptsWork;

export interface WorkEvent {
    kind: 'begin' | 'finish',
    work: Work
}

let workStatusBar: vscode.StatusBarItem;
let currentWork: Work | undefined = undefined;
let workEventQueue: WorkEvent[] = [];
let workUpdaterRunning = false;

// Establishing a minimal time for which a status must be visible for the user to see it
const WORK_UPDATE_PERIOD_MILIS: number = 500;


export function initializeState(context: vscode.ExtensionContext) {
    contextStatusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left);
    contextStatusBar.tooltip = "Click to show available commands";
    contextStatusBar.command = 'witcherscript-ide.misc.showCommandsInPalette';
    updateContextStatusBar();
    contextStatusBar.show();
    context.subscriptions.push(contextStatusBar);

    workStatusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left);
    workStatusBar.command = 'witcherscript-ide.misc.openLogs';
    updateWorkStatusBar();
    context.subscriptions.push(workStatusBar);

    vscode.window.onDidChangeActiveTextEditor(() => {
        updateLastContentInfo();
    });
}



export async function updateLastContentInfo() {
    const client = getLanguageClient();
    if (client == undefined) {
        lastContentInfo = undefined;
    } else {
        const activeEditor = vscode.window.activeTextEditor;
        const isWitcherScript = activeEditor?.document.languageId == 'witcherscript';
        const documentUri = activeEditor?.document.uri;

        // if there is no active editor opened we leave the last content name as is
        if (documentUri != undefined && isWitcherScript) {
            let currentContent: model.ContentInfo | undefined = undefined;
            try {
                const res = await client.sendRequest(requests.scripts.parent_content.type, {
                    scriptUri: client.code2ProtocolConverter.asUri(documentUri)
                });

                currentContent = res.parentContentInfo;
            } catch(_) {}

            lastContentInfo = currentContent;
        }
    }

    updateContextStatusBar();
}

function updateContextStatusBar() {
    let text = "WitcherScript IDE";

    if (!config.getConfiguration().enableLanguageServer) {
        text += " [Disabled]";
    } else {
        const contentName = (lastContentInfo != undefined) ? lastContentInfo.contentName : "No active content";
        text += ` [${contentName}]`;
    }
    
    contextStatusBar.text = text;
}



export function scheduleWorkEvent(event: WorkEvent) {
    workEventQueue.push(event);

    if (!workUpdaterRunning) {
        workUpdaterRunning = true;
        updateWork();
    }
}

function updateWork() {
    if (currentWork == undefined) {
        beginNewWork();
    } else {
        for (let i = 0; i < workEventQueue.length; i += 1) {
            const event = workEventQueue[i];
            if (event.kind == 'finish' && JSON.stringify(event.work) == JSON.stringify(currentWork)) {
                workEventQueue.splice(i, 1);
                currentWork = undefined;
                break;
            }
        }

        if (currentWork == undefined) {
            beginNewWork();
        }
    }

    updateWorkStatusBar();

    if (currentWork != undefined) {
        setTimeout(() => {
            updateWork();
        }, WORK_UPDATE_PERIOD_MILIS);
    } else {
        workUpdaterRunning = false;
    }
}

function beginNewWork() {
    for (let i = 0; i < workEventQueue.length; i += 1) {
        const event = workEventQueue[i];
        if (event.kind == 'begin') {
            currentWork = workEventQueue.splice(i, 1)[0].work;
            break;
        }
    }
}

function updateWorkStatusBar() {
    if (currentWork != undefined) {
        let text = "$(sync~spin) ";

        switch (currentWork.kind) {
            case 'parsing-scripts':
                text += `Parsing scripts (${currentWork.contentName})`
                break;
        }

        workStatusBar.text = text;
        workStatusBar.show();
    } else {
        workStatusBar.hide();
    }
}