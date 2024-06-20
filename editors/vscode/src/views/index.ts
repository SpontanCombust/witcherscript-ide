import * as vscode from 'vscode'

import { getVanillaFilesProvider } from '../providers/vanilla_files_provider'


export function createViews(context: vscode.ExtensionContext) {
    const vanillaFilesView = vscode.window.createTreeView('witcherscript-ide.vanillaFilesView', {
        treeDataProvider: getVanillaFilesProvider()
    })

    context.subscriptions.push(vanillaFilesView);
}