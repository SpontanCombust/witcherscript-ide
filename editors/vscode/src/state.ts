import * as vscode from 'vscode';

import { getLanguageClient } from './lsp/lang_client';
import * as requests from './lsp/requests';
import * as model from './lsp/model';
import * as config from './config'


let contextStatusBar: vscode.StatusBarItem;
let lastActiveContentInfo: model.ContentInfo | undefined = undefined;
export let gameOutputChannel: vscode.LogOutputChannel;


/// Get info about the content to which belongs the last viewed script 
export function getLastActiveContentInfo(): model.ContentInfo | undefined {
    return lastActiveContentInfo;
}

const lastActiveContentChanged = new vscode.EventEmitter<model.ContentInfo | undefined>();

export const onLastActiveContentChanged = lastActiveContentChanged.event;


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
        updateLastActiveContentInfo();
    });

    gameOutputChannel = vscode.window.createOutputChannel("Witcher 3 Output", { log: true });
}



export async function updateLastActiveContentInfo() {
    const prevContentJson = JSON.stringify(lastActiveContentInfo);

    const client = getLanguageClient();
    if (client == undefined) {
        lastActiveContentInfo = undefined;
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

            lastActiveContentInfo = currentContent;
        }
    }

    if (prevContentJson != JSON.stringify(lastActiveContentInfo)) {
        updateContextStatusBar();
        lastActiveContentChanged.fire(lastActiveContentInfo);
    }
}

function updateContextStatusBar() {
    let text = "WitcherScript IDE";

    if (!config.getConfiguration().enableLanguageServer) {
        text += " [Disabled]";
    } else {
        const contentName = (lastActiveContentInfo != undefined) ? lastActiveContentInfo.contentName : "No active content";
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
                text += `Processing (${currentWork.contentName})`
                break;
        }

        workStatusBar.text = text;
        workStatusBar.show();
    } else {
        workStatusBar.hide();
    }
}