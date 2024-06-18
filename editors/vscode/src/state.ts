import * as vscode from 'vscode';

import { getLanguageClient } from './lsp/lang_client';
import * as requests from './lsp/requests';
import * as model from './lsp/model';


export interface WorkDisabled {
    kind: 'disabled'
}

export interface WorkIdle {
    kind: 'idle'
}

export interface WorkParsingScripts {
    kind: 'parsing-scripts',
    contentName: string
}

export interface WorkScanningSymbols {
    kind: 'symbol-scan',
    contentName: string
}

export type WorkStatus = WorkDisabled | WorkIdle | WorkParsingScripts | WorkScanningSymbols;

let contextStatusBar: vscode.StatusBarItem;
let lastProjectName: string | undefined = undefined;

let workStatusBar: vscode.StatusBarItem;
let workStatus: WorkStatus = { kind: 'disabled' };
let workStatusLocked: boolean = false;
let workStatusUpdateQueue: WorkStatus[] = [];

// Establishing a minimal time for which a status must be visible for the user to see it
const MIN_STATUS_TIME_MILIS: number = 500;


export function initializeState() {
    contextStatusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left);
    contextStatusBar.tooltip = "Click to show available commands";
    contextStatusBar.command = 'witcherscript-ide.misc.showCommandsInPalette';
    updateContextStatusBar();
    contextStatusBar.show();

    workStatusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left);
    updateWorkStatusBar();

    vscode.window.onDidChangeActiveTextEditor(() => {
        updateLastProjectName();
        updateContextStatusBar();
    });
}

export function disposeState() {
    contextStatusBar.dispose()
}



async function updateLastProjectName() {
    const client = getLanguageClient();
    if (client == undefined) {
        lastProjectName = undefined;
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

            lastProjectName = currentContent?.contentName;
        }
    }
}

function updateContextStatusBar() {
    let text = "WitcherScript IDE";

    if (workStatus.kind == 'disabled') {
        text += " [Disabled]";
    } else {
        const projectName = (lastProjectName != undefined) ? lastProjectName : "No active project";
        text += ` [${projectName}]`;
    }
    
    contextStatusBar.text = text;
}



export function scheduleWorkStatusUpdate(status: WorkStatus) {
    if (workStatusLocked) {
        workStatusUpdateQueue.push(status);
    } else {
        workStatus = status;

        if (status.kind != 'disabled' && status.kind != 'idle') {
            workStatusLocked = true;
        }
        
        updateContextStatusBar();

        setTimeout(() => {
            workStatusLocked = false;
            if (workStatusUpdateQueue.length > 0) {
                const nextStatus = workStatusUpdateQueue.splice(0, 1)[0];
                scheduleWorkStatusUpdate(nextStatus);
            }
        }, MIN_STATUS_TIME_MILIS);
    }
}

function updateWorkStatusBar() {
    if (workStatus.kind != 'disabled' && workStatus.kind != 'idle') {
        let text = "$(loading~spin)";

        switch (workStatus.kind) {
            case 'parsing-scripts':
                text = `Parsing scripts (${workStatus.contentName})`
                break;
            case 'symbol-scan':
                text = `Building symbol table (${workStatus.contentName})`
                break;
        }

        workStatusBar.text = text;
        workStatusBar.show();
    } else {
        workStatusBar.hide();
    }
}