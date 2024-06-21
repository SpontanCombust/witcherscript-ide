import * as vscode from 'vscode'
import * as fs from 'fs'
import * as fspath from 'path'

import * as requests from '../lsp/requests'
import { getLanguageClient } from '../lsp/lang_client'
import * as utils from '../utils'


let instance: VanillaFilesProvider | undefined = undefined;
export function getVanillaFilesProvider(): VanillaFilesProvider {
    if (!instance) {
        instance = new VanillaFilesProvider();
    }

    return instance;
}

export class VanillaFilesProvider implements vscode.TreeDataProvider<VanillaFile> {
    private vanillaContentUri: string;
    private vanillaScriptsRootPath: string;
    private vanillaLocalFilePaths: string[];

    private didChangeTreeData: vscode.EventEmitter<
        void | VanillaFile | VanillaFile[] | null | undefined
    > = new vscode.EventEmitter();

    constructor() {
        this.vanillaContentUri = "";
        this.vanillaScriptsRootPath = "";
        this.vanillaLocalFilePaths = [];
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
                return resolve(this.makeChildrenFiles(this.vanillaLocalFilePaths, this.vanillaScriptsRootPath, element));
            } else {
                return resolve(this.makeVanillaRootFiles());
            }
        })
    }

    getParent(element: VanillaFile): vscode.ProviderResult<VanillaFile> {
        return element.parent;
    }


    private async makeVanillaRootFiles(): Promise<VanillaFile[]> {
        await this.fetchVanillaSourcePaths();
        return this.makeRootFiles(this.vanillaLocalFilePaths, this.vanillaScriptsRootPath);
    }

    private makeRootFiles(localPaths: string[], fullPathRoot: string): VanillaFile[] {
        const rootFiles = localPaths
            .filter(p => utils.pathComponents(p).length == 1)
            .map(localPath => {
                const fullPath = fspath.join(fullPathRoot, localPath);
                const isDir = fs.lstatSync(fullPath).isDirectory();

                return new VanillaFile(this.vanillaScriptsRootPath, fullPath, localPath, isDir, undefined);
            })
            .sort((f1, f2) => f1.cmp(f2));

        return rootFiles;
    }

    private makeChildrenFiles(localPaths: string[], fullPathRoot: string, parent: VanillaFile): VanillaFile[] {
        const childFiles = localPaths
            .filter(p => utils.isSubpathOf(p, parent.localPath))
            .filter(p => utils.pathComponents(p).length == utils.pathComponents(parent.localPath).length + 1)
            .map(localPath => {
                const fullPath = fspath.join(fullPathRoot, localPath);
                const isDir = fs.lstatSync(fullPath).isDirectory();

                return new VanillaFile(this.vanillaScriptsRootPath, fullPath, localPath, isDir, undefined);
            })
            .sort((f1, f2) => f1.cmp(f2));

        return childFiles;
    }

    private async fetchVanillaSourcePaths() {
        const client = getLanguageClient();
        if (!client) {
            return [];
        }

        try {
            var vanillaContentRes = await client.sendRequest(requests.projects.vanillaContent.type, {});
            if (!vanillaContentRes.content0Info) {
                client.info("[Vanilla Files View] No content0 to create a view for");
                return [];
            }

            this.vanillaContentUri = vanillaContentRes.content0Info.contentUri;
            this.vanillaScriptsRootPath = vscode.Uri.parse(vanillaContentRes.content0Info.scriptsRootUri).fsPath;

            var sourceTreeRes = await client.sendRequest(requests.projects.sourceTree.type, {
                contentUri: this.vanillaContentUri
            });

            this.vanillaLocalFilePaths = this.fillIntermediaryPaths(sourceTreeRes.localScriptPaths);
        } catch(err: any) {
            vscode.window.showErrorMessage("Failed to get info about content0: " + err.message)
            return [];
        }
    }

    /// Only paths to files are returned from the server
    /// We also need paths to all their parent directories
    private fillIntermediaryPaths(paths: string[]): string[] {
        let pathSet = new Set(paths);

        for (const p of paths) {
            let i = p.indexOf(fspath.sep, 0);
            while (i != -1) {
                const partialPath = p.slice(0, i + 1);
                pathSet.add(partialPath);

                i = p.indexOf(fspath.sep, i + 1);
            }
        }

        const arr = Array.from(pathSet);
        arr.sort();
        return arr;
    }
}

export class VanillaFile extends vscode.TreeItem {
    readonly scriptsRootPath: string;
    readonly fullPath: string;
    // relative to the scripts root
    readonly localPath: string;
    readonly isDir: boolean;
    readonly parent?: VanillaFile;

    constructor(
        scriptsRootPath: string,
        fullPath: string, 
        localPath: string,
        isDir: boolean,
        parent?: VanillaFile
    ) {
        const resourceUri = vscode.Uri.file(fullPath);
        const collapsibleState = isDir ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None;
        super(resourceUri, collapsibleState);

        this.scriptsRootPath = scriptsRootPath;
        this.fullPath = fullPath;
        this.localPath = localPath;
        this.isDir = isDir;
        this.parent = parent;
        this.id = fullPath.toLowerCase();
        this.label = fspath.basename(fullPath);
        this.contextValue = isDir ? undefined : 'script';

        if (!isDir) {
            this.command = {
                command: 'witcherscript-ide.misc.openFileReadOnly',
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