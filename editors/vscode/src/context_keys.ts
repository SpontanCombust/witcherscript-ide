import * as vscode from 'vscode';


export function getContextKeys() {
    if (instance == undefined) {
        instance = new ContextKeys();
    }

    return instance;
}

let instance: ContextKeys;

export class ContextKeys {
    private _debugFeaturesEnabled!: boolean;
    private _languageServerActive!: boolean;
    
    constructor() {
        this.debugFeaturesEnabled = false;
        this.languageServerActive = false;
    }


    get debugFeaturesEnabled(): boolean {
        return this._debugFeaturesEnabled;
    }

    set debugFeaturesEnabled(value: boolean) {
        this._debugFeaturesEnabled = value;
        this.setContextKey('debugFeaturesEnabled', value);
    }


    get languageServerActive(): boolean {
        return this._languageServerActive;
    }

    set languageServerActive(value: boolean) {
        this._languageServerActive = value;
        this.setContextKey('languageServerActive', value);
    }


    private setContextKey(key: string, value: any) {
        vscode.commands.executeCommand('setContext', `witcherscript-ide.${key}`, value);
    }
}