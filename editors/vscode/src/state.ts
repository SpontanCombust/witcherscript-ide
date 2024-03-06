import * as vscode from 'vscode';


// Used for opening the manifest file of the newly created project
export namespace OpenManifestOnInit {
    export const KEY = "OpenManifestOnInit";

    export class Memento {
        public workspaceUri: vscode.Uri;
        public manifestUri: vscode.Uri;
        public selectionRange: vscode.Range

        constructor(workspaceUri: vscode.Uri, manifestUri: vscode.Uri, selectionRange: vscode.Range) {
            this.workspaceUri = workspaceUri;
            this.manifestUri = manifestUri;
            this.selectionRange = selectionRange;
        }

        public intoDto(): MementoDto {
            return {
                workspaceUriStr: this.workspaceUri.toString(),
                manifestUriStr: this.manifestUri.toString(),
                selectionRange: [
                    this.selectionRange.start.line,
                    this.selectionRange.start.character,
                    this.selectionRange.end.line,
                    this.selectionRange.end.character
                ]
            }
        }

        public static fromDto(dto: MementoDto): Memento {
            const memento = new Memento(
                vscode.Uri.parse(dto.workspaceUriStr),
                vscode.Uri.parse(dto.manifestUriStr),
                new vscode.Range(dto.selectionRange[0], dto.selectionRange[1], dto.selectionRange[2], dto.selectionRange[3])  
            );

            return memento;
        }
    }

    export interface MementoDto {
        workspaceUriStr: string,
        manifestUriStr: string,
        selectionRange: [number, number, number, number]
    }
}