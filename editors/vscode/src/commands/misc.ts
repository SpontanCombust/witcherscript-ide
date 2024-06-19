import * as vscode from 'vscode';

import { Cmd } from './index'
import { getLanguageClient } from '../lsp/lang_client';


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