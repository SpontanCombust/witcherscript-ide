import * as vscode from 'vscode';

import { Cmd } from './index'


export function commandShowCommandsInPalette(): Cmd {
    return () => {
        const category = 'WitcherScript-IDE';
        vscode.commands.executeCommand('workbench.action.quickOpen', `>${category}`);
    }
}