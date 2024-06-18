import * as vscode from 'vscode';

import { getConfiguration } from '../config';
import * as debug from './debug';
import * as projects from './projects';
import * as scripts from './scripts';
import * as misc from './misc';



export function registerCommands(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand("witcherscript-ide.projects.init", projects.commandInitProject(context)),
        vscode.commands.registerCommand("witcherscript-ide.projects.create", projects.commandCreateProject(context)),
        vscode.commands.registerCommand("witcherscript-ide.scripts.importVanilla", scripts.commandImportVanillaScripts()),
        vscode.commands.registerCommand("witcherscript-ide.scripts.diffVanilla", scripts.commandDiffScriptWithVanilla(context)),
        vscode.commands.registerCommand("witcherscript-ide.misc.showCommandsInPalette", misc.commandShowCommandsInPalette()),
    );

    const cfg = getConfiguration();
    if (cfg.enableDebugFeatures) {
        context.subscriptions.push(
            vscode.commands.registerCommand("witcherscript-ide.debug.showScriptAst", debug.commandShowScriptAst(context)),
            vscode.commands.registerCommand("witcherscript-ide.debug.showScriptCst", debug.commandShowScriptCst(context)),
            vscode.commands.registerCommand("witcherscript-ide.debug.contentGraphDot", debug.commandContentGraphDot()),
            vscode.commands.registerCommand("witcherscript-ide.debug.showScriptSymbols", debug.commandShowScriptSymbols()),
            vscode.commands.registerCommand("witcherscript-ide.debug.clearGlobalState", debug.commandClearGlobalState(context))
        );
    }
}

export type Cmd = (...args: any[]) => unknown;