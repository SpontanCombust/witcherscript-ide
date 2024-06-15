import { LanguageClient } from 'vscode-languageclient/node'
import * as vscode from 'vscode';
import * as notifications from './notifications';
import * as utils from '../utils';

export function registerHandlers(client: LanguageClient, context: vscode.ExtensionContext) {
    client.onNotification(notifications.client.showForeignScriptWarning.type, handleShowForeignScriptWarningNotification(context))
}

type Handler = (...args: any[]) => void;

function handleShowForeignScriptWarningNotification(context: vscode.ExtensionContext): Handler {
    return async () => {
        await utils.showForeignScriptWarning(context);
    }
}