import { RequestType } from 'vscode-languageclient';
import * as vscode from 'vscode';
import * as lsp from 'vscode-languageserver-protocol';
import * as c2p from 'vscode-languageclient/lib/common/codeConverter';
import * as p2c from 'vscode-languageclient/lib/common/protocolConverter';


export namespace CreateProject {
    export interface ParametersDto {
        // Path to a directory where the project should be created
        directoryUri: string
    }

    export interface ResponseDto {
        // Path to the newly created manifest that should be opened by the client
        manifestUri: string
        // Range in the manifest that spans the content name
        manifestContentNameRange: lsp.Range
    }

    export const type = new RequestType<ParametersDto, ResponseDto, void>("witcherscript-ide/workspace/createProject");


    export class Parameters {
        public directoryUri: vscode.Uri;

        constructor(directoryUri: vscode.Uri) {
            this.directoryUri = directoryUri;
        }

        public intoDto(conv: c2p.Converter): ParametersDto {
            return {
                directoryUri: conv.asUri(this.directoryUri)
            }
        }

        public static fromDto(conv: p2c.Converter, dto: ParametersDto): Parameters {
            return new Parameters(
                conv.asUri(dto.directoryUri)
            )
        }
    }

    export class Response {
        public manifestUri: vscode.Uri
        public manifestContentNameRange: vscode.Range

        constructor(manifestUri: vscode.Uri, manifestContentNameRange: vscode.Range) {
            this.manifestUri = manifestUri;
            this.manifestContentNameRange = manifestContentNameRange;
        }

        public intoDto(conv: c2p.Converter): ResponseDto {
            return {
                manifestUri: conv.asUri(this.manifestUri),
                manifestContentNameRange: conv.asRange(this.manifestContentNameRange)
            }
        }

        public static fromDto(conv: p2c.Converter, dto: ResponseDto): Response {
            return new Response(
                conv.asUri(dto.manifestUri),
                conv.asRange(dto.manifestContentNameRange)
            )
        }
    }
}


export namespace ScriptAst {
    export interface ParametersDto {
        scriptUri: string
    }

    export interface ResponseDto {
        ast: string
    }

    export const type = new RequestType<ParametersDto, ResponseDto, void>("witcherscript-ide/debug/scriptAst");


    export class Parameters {
        public scriptUri: vscode.Uri;

        constructor(scriptUri: vscode.Uri) {
            this.scriptUri = scriptUri;
        }

        public intoDto(conv: c2p.Converter): ParametersDto {
            return {
                scriptUri: conv.asUri(this.scriptUri)
            }
        }

        public static fromDto(conv: p2c.Converter, dto: ParametersDto): Parameters {
            return new Parameters(
                conv.asUri(dto.scriptUri)
            )
        }
    }

    export class Response {
        public ast: string

        constructor(ast: string) {
            this.ast = ast;
        }

        public intoDto(_conv: c2p.Converter): ResponseDto {
            return {
                ast: this.ast,
            }
        }

        public static fromDto(_conv: p2c.Converter, dto: ResponseDto): Response {
            return new Response(
                dto.ast
            )
        }
    }
}