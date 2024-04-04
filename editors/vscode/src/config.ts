import * as vscode from 'vscode';


export interface Config {
    gameDirectory: string,
    contentRepositories: string[]
}

export function getConfiguration(): Config {
    const config = vscode.workspace.getConfiguration('witcherscript-ide');
    return {
        gameDirectory: config.get<string>('gameDirectory') ?? '',
        contentRepositories: config.get<string[]>('contentRepositories') ?? []
    }
}