import * as vscode from 'vscode';


export function getConfiguration(): Config {
    return new Config();
}

export class Config {
    readonly config: vscode.WorkspaceConfiguration;

    constructor() {
        this.config = vscode.workspace.getConfiguration('witcherscript-ide');
    }


    get gameDirectory(): string {
        return this.config.get<string>('gameDirectory') ?? '';
    }

    get contentRepositories(): string[] {
        return this.config.get<string[]>('contentRepositories') ?? [];
    }

    get gameHostIpAddress(): string {
        return this.config.get<string>('gameHost.ipAddress') ?? '';
    }

    get enableLanguageServer(): boolean {
        return this.config.get<boolean>('languageServer.enable') ?? true;
    }

    get rayonThreads(): number {
        return this.config.get<number>('languageServer.rayonThreads') ?? 0;
    }

    get enableSyntaxAnalysis(): boolean {
        return this.config.get<boolean>('languageServer.syntaxAnalysis') ?? true;
    }

    get enableDebugFeatures(): boolean {
        return this.config.get<boolean>('debug.enableDebugFeatures') ?? false;
    }
}