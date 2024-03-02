import * as vscode from 'vscode';
import { RequestType } from 'vscode-languageclient';


export namespace CreateProjectRequest {
    export interface Parameters {
        // Path to a directory where the project should be created
        directoryUri: string
    }

    export interface Response {
        // Path to the newly created manifest that should be opened by the client
        manifestUri: string
    }

    export const type = new RequestType<Parameters, Response, void>("witcherscript-ide/workspace/createProject");
}