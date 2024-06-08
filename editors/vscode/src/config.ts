import * as vscode from 'vscode';


export interface Config {
    gameDirectory: string,
    contentRepositories: string[],
    enableLanguageServer: boolean,
    enableSyntaxAnalysis: boolean,
    enableDebugFeatures: boolean
}

export function getConfiguration(): Config {
    const config = vscode.workspace.getConfiguration('witcherscript-ide');
    return {
        gameDirectory: config.get<string>('gameDirectory') ?? '',
        contentRepositories: config.get<string[]>('contentRepositories') ?? [],
        enableLanguageServer: config.get<boolean>('languageServer.enable') ?? true,
        enableSyntaxAnalysis: config.get<boolean>('languageServer.syntaxAnalysis') ?? true,
        enableDebugFeatures: config.get<boolean>('debug.enableDebugFeatures') ?? false
    }
}