import * as vscode from 'vscode';

import { Cmd } from './index'
import { getLanguageClient } from '../lsp/lang_client';
import * as tdcp from '../providers/text_document_content_providers'


export function commandShowCommandsInPalette(): Cmd {
    return () => {
        const category = 'WitcherScript-IDE';
        vscode.commands.executeCommand('workbench.action.quickOpen', `>${category}`);
    }
}

export function commandOpenLogs(): Cmd {
    return () => {
        const client = getLanguageClient();
        if (client != undefined) {
            client.outputChannel.show();
        }
    }
}

export function commandOpenFileReadOnly(): Cmd {
    return (uri: vscode.Uri) => {
        uri = uri.with({ scheme: tdcp.ReadOnlyContentProvider.scheme });
        tdcp.ReadOnlyContentProvider.getInstance().eventEmitter.fire(uri);
        vscode.window.showTextDocument(uri);
    }
}