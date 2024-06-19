import { NotificationType } from 'vscode-languageclient';


export namespace client {
    export namespace showForeignScriptWarning {
        export const type = new NotificationType<void>("witcherscript-ide/client/showForeignScriptWarning");
    }
}

export namespace projects {
    export namespace didImportScripts {
        export interface Parameters {
            importedScriptsUris: string[]
        }

        export const type = new NotificationType<Parameters>("witcherscript-ide/projects/didImportScripts");
    }
}

export namespace scripts {
    export namespace scriptParsingStarted {
        export interface Parameters {
            contentName: string
        }

        export const type = new NotificationType<Parameters>("witcherscript-ide/scripts/scriptParsingStarted");
    }

    export namespace scriptParsingFinished {
        export interface Parameters {
            contentName: string
        }

        export const type = new NotificationType<Parameters>("witcherscript-ide/scripts/scriptParsingFinished");
    }
}