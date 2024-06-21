import * as vscode from 'vscode'

import { initVanillaFilesView } from './vanilla_files_view'


export function createViews(context: vscode.ExtensionContext) {
    initVanillaFilesView(context);
}