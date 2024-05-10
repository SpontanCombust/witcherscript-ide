import { NotificationType } from 'vscode-languageclient';


export namespace client {
    export namespace showForeignScriptWarning {
        export const type = new NotificationType<void>("witcherscript-ide/client/showForeignScriptWarning");
    }
}