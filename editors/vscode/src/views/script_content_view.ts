import * as vscode from 'vscode'

import { Item, getScriptContentProvider } from '../providers/script_content_provider'


let instance: vscode.TreeView<Item>;

export function initScriptContentView(context: vscode.ExtensionContext) {
    instance = vscode.window.createTreeView('witcherscript-ide.scriptContentView', {
        treeDataProvider: getScriptContentProvider()
    });

    context.subscriptions.push(instance);
}

export function getScriptContentView() {
    return instance;
}