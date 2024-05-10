import * as vscode from 'vscode';


export interface Config {
    gameDirectory: string,
    contentRepositories: string[],
    enableDebugFeatures: boolean
}

export function getConfiguration(): Config {
    const config = vscode.workspace.getConfiguration('witcherscript-ide');
    return {
        gameDirectory: config.get<string>('gameDirectory') ?? '',
        contentRepositories: config.get<string[]>('contentRepositories') ?? [],
        enableDebugFeatures: config.get<boolean>('debug.enableDebugFeatures') ?? false
    }
}