import * as vscode from 'vscode';


// Used for opening the manifest file of the newly created project
export namespace OpenManifestOnInit {
    export const KEY = "OpenManifestOnInit";

    export class Memento {
        public workspaceUri: vscode.Uri;
        public manifestUri: vscode.Uri;

        constructor(workspaceUri: vscode.Uri, manifestUri: vscode.Uri) {
            this.workspaceUri = workspaceUri;
            this.manifestUri = manifestUri;
        }

        public async store(context: vscode.ExtensionContext) {
            const dto: MementoDto = {
                workspaceUriStr: this.workspaceUri.toString(),
                manifestUriStr: this.manifestUri.toString(),
            };

            await context.globalState.update(KEY, dto);
        }

        public static fetch(context: vscode.ExtensionContext): Memento | undefined {
            const dto = context.globalState.get<MementoDto>(KEY);
            if (dto) {
                const memento = new Memento(
                    vscode.Uri.parse(dto.workspaceUriStr),
                    vscode.Uri.parse(dto.manifestUriStr),
                );

                return memento;
            } else {
                return undefined;
            }
        }

        public static erase(context: vscode.ExtensionContext) {
            context.globalState.update(KEY, undefined);
        }
    }

    interface MementoDto {
        workspaceUriStr: string,
        manifestUriStr: string
    }
}

export namespace RememberedChoices {
    export const KEY = "RememberedChoices";

    export class Memento {
        public neverShowAgainDebugAstNotif: boolean

        constructor(neverShowAgainDebugAstNotif: boolean) {
            this.neverShowAgainDebugAstNotif = neverShowAgainDebugAstNotif;
        }

        public async store(context: vscode.ExtensionContext) {
            const dto: MementoDto = {
                neverShowAgainDebugAstNotif: this.neverShowAgainDebugAstNotif
            };

            context.globalState.update(KEY, dto);
        }

        public static fetchOrDefault(context: vscode.ExtensionContext): Memento {
            const dto = context.globalState.get<MementoDto>(KEY);

            if (dto) {
                return new Memento(
                    dto.neverShowAgainDebugAstNotif
                );
            } else {
                return new Memento(
                    false
                )
            }
        }
    }

    interface MementoDto {
        neverShowAgainDebugAstNotif: boolean
    }
}