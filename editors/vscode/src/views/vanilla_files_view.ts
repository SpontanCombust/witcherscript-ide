import * as vscode from 'vscode'

import { VanillaFile, getVanillaFilesProvider } from '../providers/vanilla_files_provider'


let instance: vscode.TreeView<VanillaFile>;

export function initVanillaFilesView(context: vscode.ExtensionContext) {
    instance = vscode.window.createTreeView('witcherscript-ide.vanillaFilesView', {
        treeDataProvider: getVanillaFilesProvider()
    });

    context.subscriptions.push(instance);
}

export function getVanillaFilesView() {
    return instance;
}