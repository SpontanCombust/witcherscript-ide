import * as vscode from 'vscode'


let instance: DashboardProvider;

export class DashboardProvider implements vscode.TreeDataProvider<Item> {
    public static readonly viewId = "witcherscript-ide.dashboardView";

    private didChangeTreeData: vscode.EventEmitter<
        void | Item | Item[] | null | undefined
    > = new vscode.EventEmitter();


    private constructor() {}
    
    public static getInstance() : DashboardProvider {
        if (instance == undefined) {
            instance = new DashboardProvider();
        }
    
        return instance;
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
                return resolve(element.getChildren());
            }
        })
    }

    getParent(element: Item): vscode.ProviderResult<Item> {
        return element.parent;
    }


    private async getRootItems() : Promise<Item[]> {
        return [
            new GameLaunchOptionsHeader(),
            new ProjectSystemOptionsHeader()
        ];
    }
}


export type Item = 
    GameLaunchOptionsHeader |
    GameLaunchOption;


class GameLaunchOptionsHeader extends vscode.TreeItem {
    readonly parent = undefined;

    constructor() {
        super("Game launch options", vscode.TreeItemCollapsibleState.Expanded);
    }

    getChildren(): Item[] {
        return [
            new GameLaunchOption(this, "Launch the game (DX12)", "launchGameDx12"),
            new GameLaunchOption(this, "Launch the game for debugging (DX12)", "launchGameDx12Debug"),
            new GameLaunchOption(this, "Launch the game (DX11)", "launchGameDx11"),
            new GameLaunchOption(this, "Launch the game for debugging (DX11)", "launchGameDx11Debug"),
        ];
    }
}

class GameLaunchOption extends vscode.TreeItem {
    constructor(
        readonly parent: GameLaunchOptionsHeader,
        override readonly label: string,
        override readonly contextValue: string
    ) {
        super(label, vscode.TreeItemCollapsibleState.None);
        this.iconPath = new vscode.ThemeIcon("circle-filled");
    }

    getChildren(): Item[] {
        return [];
    }
}


class ProjectSystemOptionsHeader extends vscode.TreeItem {
    readonly parent = undefined;

    constructor() {
        super("Project system", vscode.TreeItemCollapsibleState.Expanded);
    }

    getChildren(): Item[] {
        return [
            new ProjectSystemOption(this, "Initialize a WitcherScript project in existing directory", "initWideProject"),
            new ProjectSystemOption(this, "Create a new WitcherScript project", "createWideProject"),
        ];
    }
}

class ProjectSystemOption extends vscode.TreeItem {
    constructor(
        readonly parent: ProjectSystemOptionsHeader,
        override readonly label: string,
        override readonly contextValue: string
    ) {
        super(label, vscode.TreeItemCollapsibleState.None);
        this.iconPath = new vscode.ThemeIcon("package");
    }

    getChildren(): Item[] {
        return [];
    }
}