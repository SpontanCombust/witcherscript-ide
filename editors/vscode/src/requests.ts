import { RequestType } from 'vscode-languageclient';
import * as lsp from 'vscode-languageserver-protocol';


export interface ContentInfo {
    contentUri: string,
    scriptsRootUri: string,
    contentName: string,
    isInWorkspace: boolean,
    isInRepository: boolean
}


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

    export namespace list {
        export interface Parameters {
            // Defaults to true
            onlyFromWorkspace?: boolean
        }
    
        export interface Response {
            projectInfos: ContentInfo[]
        }
    
        export const type = new RequestType<Parameters, Response, void>("witcherscript-ide/projects/list");
    }

    export namespace vanillaDependencyContent {
        export interface Parameters {
            projectUri: string
        }

        export interface Response {
            content0Info: ContentInfo
        }

        export const type = new RequestType<Parameters, Response, void>("witcherscript-ide/projects/vanillaDependencyContent");
    }
}

export namespace scripts {
    export namespace parent_content {
        export interface Parameters {
            scriptUri: string
        }

        export interface Response {
            parentContentInfo: ContentInfo
        }

        export const type = new RequestType<Parameters, Response, void>("witcherscript-ide/scripts/parentContent");
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