import * as vscode from 'vscode'

import { Config, getConfiguration } from '../config';


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

        vscode.workspace.onDidChangeConfiguration((ev) => {
            if (ev.affectsConfiguration(Config.SECTION)) {
                // making sure that game host type is synced
                instance.didChangeTreeData.fire(undefined);
            }
        });
    
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
            new RemoteCommandsHeaderItem(),
            new MiscOptionsHeaderItem()
        ];
    }
}


abstract class DashboardOptionsHeaderItem extends vscode.TreeItem {
    readonly parent = undefined;

    constructor(
        override readonly label: string
    ) {
        super(label, vscode.TreeItemCollapsibleState.Expanded);
    }

    abstract getChildren(): DashboardOptionItem[];
}

abstract class DashboardOptionItem extends vscode.TreeItem {
    constructor(
        readonly parent: DashboardOptionsHeaderItem,
        override readonly label: string,
        iconId: string | undefined
    ) {
        super(label, vscode.TreeItemCollapsibleState.None);
        if (iconId) {
            this.iconPath = new vscode.ThemeIcon(iconId);
        }
    }

    getChildren() {
        return [];
    }
}

export class DashboardCommandOptionItem extends DashboardOptionItem {
    constructor(
        readonly parent: DashboardOptionsHeaderItem,
        override readonly label: string,
        iconId: string | undefined,
        readonly btnCommand: string
    ) {
        super(parent, label, iconId);
        this.contextValue = "dashboardCommandOption";
    }
}

export type Item = 
    DashboardOptionsHeaderItem |
    DashboardOptionItem;




class GameLaunchOptionsHeaderItem extends DashboardOptionsHeaderItem {
    constructor() {
        super("Game launch options");
    }

    getChildren(): DashboardOptionItem[] {
        return [
            new DashboardCommandOptionItem(this, "Launch the game (DX12)", "run", "witcherscript-ide.game.launchDx12"),
            new DashboardCommandOptionItem(this, "Launch the game for debugging (DX12)", "debug-alt", "witcherscript-ide.game.launchDx12Debug"),
            new DashboardCommandOptionItem(this, "Launch the game (DX11)", "run", "witcherscript-ide.game.launchDx11"),
            new DashboardCommandOptionItem(this, "Launch the game for debugging (DX11)", "debug-alt", "witcherscript-ide.game.launchDx11Debug"),
        ];
    }
}


class ProjectSystemOptionsHeaderItem extends DashboardOptionsHeaderItem {
    constructor() {
        super("Project system");
    }

    getChildren(): DashboardOptionItem[] {
        return [
            new DashboardCommandOptionItem(this, "Initialize a WitcherScript project in existing directory", "package", "witcherscript-ide.projects.init"),
            new DashboardCommandOptionItem(this, "Create a new WitcherScript project", "package", "witcherscript-ide.projects.create"),
        ];
    }
}


class RemoteCommandsHeaderItem extends DashboardOptionsHeaderItem {
    constructor() {
        super("Remote commands");
    }

    getChildren(): DashboardOptionItem[] {
        return [
            new RemoteCommandsHostInfoItem(this),
            new DashboardCommandOptionItem(this, "Recompile scripts", "remote", "witcherscript-ide.game.recompileScripts"),
            new DashboardCommandOptionItem(this, "Execute console command", "remote", "witcherscript-ide.game.execConsoleCommand"),
        ];
    }
}

class RemoteCommandsHostInfoItem extends DashboardOptionItem {
    constructor(
        readonly parent: RemoteCommandsHeaderItem,
    ) {
        const cfg = getConfiguration();
        const label = `Game host: ${cfg.gameHostType.toString()}, address: ${cfg.gameHostIpAddress}`;

        super(parent, label, undefined);
        this.contextValue = "gameHostInfo";
    }
}


class MiscOptionsHeaderItem extends DashboardOptionsHeaderItem {
    constructor() {
        super("Miscellaneous");
    }

    getChildren(): DashboardOptionItem[] {
        return [
            new DashboardCommandOptionItem(this, "Open settings", "circle-filled", "witcherscript-ide.misc.openSettings"),
            new DashboardCommandOptionItem(this, "Open language server logs", "circle-filled", "witcherscript-ide.misc.openLogs")
        ];
    }
}
