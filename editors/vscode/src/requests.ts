import { RequestType } from 'vscode-languageclient';
import * as vscode from 'vscode';
import * as lsp from 'vscode-languageserver-protocol';
import * as c2p from 'vscode-languageclient/lib/common/codeConverter';
import * as p2c from 'vscode-languageclient/lib/common/protocolConverter';


export namespace CreateProjectRequest {
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