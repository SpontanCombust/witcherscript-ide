import { LanguageClient } from 'vscode-languageclient/node'
import * as vscode from 'vscode';

import * as notifications from './notifications';
import * as utils from '../utils';
import * as state from '../state'


export function registerHandlers(client: LanguageClient, context: vscode.ExtensionContext) {
    client.onNotification(notifications.client.showForeignScriptWarning.type, handleShowForeignScriptWarningNotification(context))
    client.onNotification(notifications.scripts.scriptParsingStarted.type, handleScriptParsingStartedNotification())
    client.onNotification(notifications.scripts.scriptParsingFinished.type, handleScriptParsingFinishedNotification())
}


function handleShowForeignScriptWarningNotification(context: vscode.ExtensionContext) {
    return async () => {
        await utils.showForeignScriptWarning(context);
    }
}

function handleScriptParsingStartedNotification() {
    return (params: notifications.scripts.scriptParsingStarted.Parameters) => {
        state.scheduleWorkEvent({ 
            kind: 'begin', 
            work: {
                kind: 'parsing-scripts',
                contentName: params.contentName
            }
        });
    }
}

function handleScriptParsingFinishedNotification() {
    return (params: notifications.scripts.scriptParsingFinished.Parameters) => {
        state.scheduleWorkEvent({ 
            kind: 'finish', 
            work: {
                kind: 'parsing-scripts',
                contentName: params.contentName
            }
        });

        state.updateLastContentInfo();
    }
}