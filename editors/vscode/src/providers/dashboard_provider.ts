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
            new GameLaunchOptionsHeaderItem(),
            new ProjectSystemOptionsHeaderItem(),
            new RemoteCommandsHeaderItem()
        ];
    }
}


export type Item = 
    GameLaunchOptionsHeaderItem |
    GameLaunchOptionItem |
    ProjectSystemOptionsHeaderItem |
    ProjectSystemOptionItem |
    RemoteCommandsHeaderItem |
    RemoteCommandItem;


class GameLaunchOptionsHeaderItem extends vscode.TreeItem {
    readonly parent = undefined;

    constructor() {
        super("Game launch options", vscode.TreeItemCollapsibleState.Expanded);
    }

    getChildren(): Item[] {
        return [
            new GameLaunchOptionItem(this, "Launch the game (DX12)", "launchGameDx12"),
            new GameLaunchOptionItem(this, "Launch the game for debugging (DX12)", "launchGameDx12Debug"),
            new GameLaunchOptionItem(this, "Launch the game (DX11)", "launchGameDx11"),
            new GameLaunchOptionItem(this, "Launch the game for debugging (DX11)", "launchGameDx11Debug"),
        ];
    }
}

class GameLaunchOptionItem extends vscode.TreeItem {
    constructor(
        readonly parent: GameLaunchOptionsHeaderItem,
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


class ProjectSystemOptionsHeaderItem extends vscode.TreeItem {
    readonly parent = undefined;

    constructor() {
        super("Project system", vscode.TreeItemCollapsibleState.Expanded);
    }

    getChildren(): Item[] {
        return [
            new ProjectSystemOptionItem(this, "Initialize a WitcherScript project in existing directory", "initWideProject"),
            new ProjectSystemOptionItem(this, "Create a new WitcherScript project", "createWideProject"),
        ];
    }
}

class ProjectSystemOptionItem extends vscode.TreeItem {
    constructor(
        readonly parent: ProjectSystemOptionsHeaderItem,
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


class RemoteCommandsHeaderItem extends vscode.TreeItem {
    readonly parent = undefined;

    constructor() {
        super("Remote commands", vscode.TreeItemCollapsibleState.Expanded);
    }

    getChildren(): Item[] {
        return [
            new RemoteCommandItem(this, "Recompile scripts", "recompileScripts"),
            new RemoteCommandItem(this, "Execute console command", "execConsoleCommand"),
        ];
    }
}

class RemoteCommandItem extends vscode.TreeItem {
    constructor(
        readonly parent: RemoteCommandsHeaderItem,
        override readonly label: string,
        override readonly contextValue: string
    ) {
        super(label, vscode.TreeItemCollapsibleState.None);
        this.iconPath = new vscode.ThemeIcon("remote");
    }

    getChildren(): Item[] {
        return [];
    }
}