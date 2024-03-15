import { RequestType } from 'vscode-languageclient';
import * as lsp from 'vscode-languageserver-protocol';

export namespace projects {
    export namespace create {
        export interface Parameters {
            // Path to a directory where the project should be created
            directoryUri: string
        }
    
        export interface Response {
            // Path to the newly created manifest that should be opened by the client
            manifestUri: string
            // Range in the manifest that spans the content name
            manifestContentNameRange: lsp.Range
        }
    
        export const type = new RequestType<Parameters, Response, void>("witcherscript-ide/projects/create");
    }
}

export namespace debug {
    export namespace scriptAst {
        export interface Parameters {
            scriptUri: string
        }
    
        export interface Response {
            ast: string
        }
    
        export const type = new RequestType<Parameters, Response, void>("witcherscript-ide/debug/scriptAst");
    }
}