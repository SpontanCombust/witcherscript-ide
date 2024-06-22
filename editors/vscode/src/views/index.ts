import * as vscode from 'vscode'

import { initVanillaFilesView } from './vanilla_files_view'
import { initScriptContentView } from './script_content_view';


export function createViews(context: vscode.ExtensionContext) {
    //TODO make sure views fetch data only after the initial parsing phase
    initVanillaFilesView(context);
    initScriptContentView(context);
}