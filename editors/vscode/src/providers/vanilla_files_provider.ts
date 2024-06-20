import * as vscode from 'vscode'
import * as fs from 'fs'
import * as fspath from 'path'

import * as requests from '../lsp/requests'
import { getLanguageClient } from '../lsp/lang_client'


let instance: VanillaFilesProvider | undefined = undefined;
export function getVanillaFilesProvider(): VanillaFilesProvider {
    if (!instance) {
        instance = new VanillaFilesProvider();
    }

    return instance;
}

export class VanillaFilesProvider implements vscode.TreeDataProvider<VanillaFile> {
    private scriptsRootPath: string | undefined;

    private didChangeTreeData: vscode.EventEmitter<
        void | VanillaFile | VanillaFile[] | null | undefined
    > = new vscode.EventEmitter();

    constructor() {
        this.scriptsRootPath = undefined;
    }


    public refreshAll() {
        this.didChangeTreeData.fire(undefined);
    }


    onDidChangeTreeData: vscode.Event<
        void | VanillaFile | VanillaFile[] | null | undefined
    > = this.didChangeTreeData.event;

    getTreeItem(element: VanillaFile): vscode.TreeItem | Thenable<vscode.TreeItem> {
        return element;
    }

    getChildren(element?: VanillaFile | undefined): vscode.ProviderResult<VanillaFile[]> {
        return new Promise((resolve, _reject) => {
            if (element) {
                if (element.isDir) {
                    return resolve(this.readFilesInDir(element.fullPath, element));
                } else {
                    return [];
                }
            } else {
                return resolve(this.fetchRootFiles());
            }
        })
    }

    private async fetchRootFiles(): Promise<VanillaFile[]> {
        const client = getLanguageClient();
        if (!client) {
            return [];
        }

        try {
            const res = await client.sendRequest(requests.projects.vanillaContent.type, {});
            if (!res.content0Info) {
                client.info("[Vanilla Files View] No content0 to create a view for");
                return [];
            }

            const scriptsRootPath = vscode.Uri.parse(res.content0Info.scriptsRootUri).fsPath;
            this.scriptsRootPath = scriptsRootPath;

            return this.readFilesInDir(scriptsRootPath, undefined);
        } catch(err: any) {
            vscode.window.showErrorMessage("Failed to get info about content0: " + err.message)
            return [];
        }
    }

    private readFilesInDir(fullDirPath: string, parent?: VanillaFile): VanillaFile[] {
        return fs.readdirSync(fullDirPath)
            .map((fileName) => {
                const fullPath = fspath.join(fullDirPath, fileName);
                const localPath = fspath.relative(this.scriptsRootPath ?? "", fullPath);
                const isDir = fs.lstatSync(fullPath).isDirectory();

                if (!isDir && fspath.extname(localPath) != '.ws') {
                    return undefined;
                } else {
                    return new VanillaFile(fullPath, localPath, isDir, parent);
                }
            })
            .filter((file): file is VanillaFile => !!file)
            .sort((f1, f2) => f1.cmp(f2));
    }

    getParent(element: VanillaFile): vscode.ProviderResult<VanillaFile> {
        return element.parent;
    }
}

export class VanillaFile extends vscode.TreeItem {
    readonly fullPath: string;
    // relative to the scripts root
    readonly localPath: string;
    readonly isDir: boolean;
    readonly parent?: VanillaFile;

    constructor(
        fullPath: string, 
        localPath: string,
        isDir: boolean,
        parent?: VanillaFile
    ) {
        const resourceUri = vscode.Uri.file(fullPath);
        const collapsibleState = isDir ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None;
        super(resourceUri, collapsibleState);

        this.fullPath = fullPath;
        this.localPath = localPath;
        this.isDir = isDir;
        this.parent = parent;
        this.id = fullPath.toLowerCase();
        this.label = fspath.basename(fullPath);

        if (!isDir) {
            this.command = {
                command: 'vscode.open',
                title: 'Open file',
                arguments: [resourceUri]
            }
        }
    }

    cmp(other: VanillaFile) : number {
        if (this.isDir == other.isDir) {
            return this.label!.toString().localeCompare(other.label!.toString());
        } else {
            if (this.isDir && !other.isDir) {
                return -1;
            } else {
                return 1;
            }
        }
    }
}