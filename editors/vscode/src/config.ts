import * as vscode from 'vscode';


export interface Config {
    gameDirectory: string,
    projectRepositories: string[]
}

export function getConfiguration(): Config {
    const config = vscode.workspace.getConfiguration('witcherscript-ide');
    return {
        gameDirectory: config.get<string>('gameDirectory') ?? '',
        projectRepositories: config.get<string[]>('projectRepositories') ?? []
    }
}