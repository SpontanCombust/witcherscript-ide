import * as vscode from 'vscode'
import * as fs from 'fs'
import * as fspath from 'path'

import * as model from '../lsp/model'
import * as requests from '../lsp/requests'
import { getLanguageClient } from '../lsp/lang_client';


let instance: ScriptContentProvider;

export class ScriptContentProvider implements vscode.TreeDataProvider<Item> {
    public static readonly viewId = "witcherscript-ide.scriptContentView";

    private didChangeTreeData: vscode.EventEmitter<
        void | Item | Item[] | null | undefined
    > = new vscode.EventEmitter();


    private constructor() {}
    
    public static getInstance() : ScriptContentProvider {
        if (instance == undefined) {
            instance = new ScriptContentProvider();
        }
    
        return instance;
    }

    refreshAll() {
        this.didChangeTreeData.fire(undefined)
    }


    onDidChangeTreeData: vscode.Event<
        void | Item | Item[] | null | undefined
    > = this.didChangeTreeData.event;

    getTreeItem(element: Item): vscode.TreeItem | Thenable<vscode.TreeItem> {
        return element;
    }

    getChildren(element?: Item | undefined): vscode.ProviderResult<Item[]> {
        return new Promise((resolve, _reject) => {
            if (element == undefined) {
                return resolve(this.getRootItems());
            } else {
                return resolve(element.getChildren())
            }
        })
    }

    getParent(element: Item): vscode.ProviderResult<Item> {
        return element.parent;
    }


    private async getRootItems() : Promise<ScriptContentItem[]> {
        const client = getLanguageClient();
        if (!client) {
            return [];
        }
        
        try {
            const res = await client.sendRequest(requests.projects.list.type, {
                onlyFromWorkspace: false
            });

            return res.projectInfos
                .sort((ci1, ci2) => ci1.contentName.localeCompare(ci2.contentName))
                .map(ci => new ScriptContentItem(ci));
                
        } catch(err: any) {
            vscode.window.showErrorMessage("Failed to get info about workspace contents: " + err.message)
            return [];
        }
    }
}


export type Item = 
    ScriptContentItem | 
    ScriptContentMetadataHeaderItem | 
    ScriptContentMetadataItem | 
    ScriptContentFileHeaderItem |
    ScriptContentFileItem;

class ScriptContentItem extends vscode.TreeItem {
    readonly parent = undefined;

    constructor(
        readonly contentInfo: model.ContentInfo
    ) {
        super(contentInfo.contentName, vscode.TreeItemCollapsibleState.Collapsed);

        const contentPath = vscode.Uri.parse(contentInfo.contentUri).fsPath;
        this.iconPath = new vscode.ThemeIcon("package");
        this.description = contentPath;
        this.tooltip = contentPath;
    }

    getChildren(): Item[] {
        return [
            new ScriptContentMetadataHeaderItem(this),
            new ScriptContentFileHeaderItem(this)
        ]
    }
}

class ScriptContentMetadataHeaderItem extends vscode.TreeItem {
    constructor(
        readonly parent: ScriptContentItem
    ) {
        super("Metadata", vscode.TreeItemCollapsibleState.Expanded);

        this.iconPath = new vscode.ThemeIcon("gear");
    }

    getChildren(): Item[] {
        const ci = this.parent.contentInfo;
        const kindStr = (() => {
            switch (ci.contentKind) {
                case model.ContentKind.Raw: 
                    return "Raw";
                case model.ContentKind.WideProject:
                    return "WIDE project";
                case model.ContentKind.RedkitProject:
                    return "REDkit project";
                default:
                    return "";
            }
        })();

        return [
            new ScriptContentMetadataItem(this, "Path", vscode.Uri.parse(ci.contentUri).fsPath),
            new ScriptContentMetadataItem(this, "Kind", kindStr),
            new ScriptContentMetadataItem(this, "Name", ci.contentName),
            new ScriptContentMetadataItem(this, "Scripts root", vscode.Uri.parse(ci.scriptsRootUri).fsPath),
            new ScriptContentMetadataItem(this, "In workspace", ci.isInWorkspace.toString()),
            new ScriptContentMetadataItem(this, "In repository", ci.isInRepository.toString()),
        ];
    }
}

class ScriptContentMetadataItem extends vscode.TreeItem {
    constructor(
        readonly parent: ScriptContentMetadataHeaderItem,
        override readonly label: string,
        override readonly description: string,
    ) {
        super(label, vscode.TreeItemCollapsibleState.None);

        this.iconPath = new vscode.ThemeIcon("gear");
        this.tooltip = description;
    }

    getChildren(): Item[] {
        return [];
    }
}

class ScriptContentFileHeaderItem extends vscode.TreeItem {
    override readonly label = "Files";

    constructor(
        readonly parent: ScriptContentItem,
    ) {
        const resourceUri = vscode.Uri.parse(parent.contentInfo.contentUri);
        super(resourceUri, vscode.TreeItemCollapsibleState.Collapsed);

        this.id = resourceUri.fsPath.toLowerCase();
    }

    getChildren(): Item[] {
        const fullPath = this.resourceUri!.fsPath;
        return readFilesInDir(this, fullPath, this.parent.contentInfo.isInWorkspace);
    }
}

class ScriptContentFileItem extends vscode.TreeItem {
    override readonly label: string;

    constructor(
        readonly parent: ScriptContentFileItem | ScriptContentFileHeaderItem,
        readonly fullPath: string,
        readonly isDir: boolean,
        readonly inWorkspace: boolean,
    ) {
        const resourceUri = vscode.Uri.file(fullPath);
        const collapsibleState = isDir ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None;
        super(resourceUri, collapsibleState);

        this.id = fullPath.toLowerCase();
        this.label = fspath.basename(fullPath);

        if (!isDir) {
            // disallow editing the file if it's from outside the workspace
            const cmd = inWorkspace ? 'vscode.open' :  'witcherscript-ide.misc.openFileReadOnly';

            this.command = {
                command: cmd,
                title: 'Open file',
                arguments: [resourceUri]
            }
        }
    }

    getChildren(): Item[] {
        return this.isDir ? readFilesInDir(this, this.fullPath, this.inWorkspace) : [];
    }

    cmp(other: ScriptContentFileItem) : number {
        if (this.isDir == other.isDir) {
            return this.label.localeCompare(other.label);
        } else {
            if (this.isDir && !other.isDir) {
                return -1;
            } else {
                return 1;
            }
        }
    }
}


function readFilesInDir(parent: ScriptContentFileItem | ScriptContentFileHeaderItem, fullDirPath: string, inWorkspace: boolean): ScriptContentFileItem[] {
    return fs.readdirSync(fullDirPath)
        .map((fileName) => {
            const fullPath = fspath.join(fullDirPath, fileName);
            const isDir = fs.lstatSync(fullPath).isDirectory();

            return new ScriptContentFileItem(parent, fullPath, isDir, inWorkspace)
        })
        .sort((f1, f2) => f1.cmp(f2));
}