import * as vscode from 'vscode';


export function getPersistence(ctx: vscode.ExtensionContext): Persistence {
    return new Persistence(ctx)
}

export class Persistence {
    constructor(
        readonly ctx: vscode.ExtensionContext
    ) {}


    // Used for opening the manifest file of the newly created project
    get openManifestOnInit(): OpenManifestOnInit | undefined {
        const dto = this.ctx.globalState.get<OpenManifestOnInitDto>("OpenManifestOnInit");
        if (dto) {
            return {
                workspaceUri: vscode.Uri.parse(dto.workspaceUriStr),
                manifestUri: vscode.Uri.parse(dto.manifestUriStr),
            }
        } else {
            return undefined;
        }
    }

    set openManifestOnInit(value: OpenManifestOnInit | undefined) {
        let dto: OpenManifestOnInitDto | undefined = undefined;
        if (value) {
            dto = {
                workspaceUriStr: value.workspaceUri.toString(),
                manifestUriStr: value.manifestUri.toString(),
            }
        }

        this.ctx.globalState.update("OpenManifestOnInit", dto);
    }


    get neverShowAgainDebugAstNotif(): boolean {
        return this.ctx.globalState.get<boolean>("NeverShowAgainDebugAstNotif") ?? false;
    }

    set neverShowAgainDebugAstNotif(value: boolean) {
        this.ctx.globalState.update("NeverShowAgainDebugAstNotif", value)
    }


    get neverShowAgainForeignScriptWarning(): boolean {
        return this.ctx.globalState.get<boolean>("NeverShowAgainForeignScriptWarning") ?? false;
    }

    set neverShowAgainForeignScriptWarning(value: boolean) {
        this.ctx.globalState.update("NeverShowAgainForeignScriptWarning", value)
    }
}

export interface OpenManifestOnInit {
    workspaceUri: vscode.Uri,
    manifestUri: vscode.Uri
}

interface OpenManifestOnInitDto {
    workspaceUriStr: string,
    manifestUriStr: string
}
