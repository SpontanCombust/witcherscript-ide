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