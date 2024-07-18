import * as vscode from 'vscode';
import * as fspath from 'path';
import * as cp from 'child_process';

import { Cmd } from './index'
import { getConfiguration } from '../config';
import { fileExists } from '../utils';
import * as state from '../state'


const GAME_EXE_DIR_DX12 = "bin/x64_dx12";
const GAME_EXE_DIR_DX11 = "bin/x64";
const GAME_EXE_NAME = "witcher3.exe";
const GAME_DEBUG_ARGS = ['-net', '-debugscripts'];

export function commandLaunchGameDx12(): Cmd {
    return () => {
        launchGame("dx12", false)
    }
}

export function commandLaunchGameDx12Debug(): Cmd {
    return () => {
        launchGame("dx12", true)
    }
}

export function commandLaunchGameDx11(): Cmd {
    return () => {
        launchGame("dx11", false)
    }
}

export function commandLaunchGameDx11Debug(): Cmd {
    return () => {
        launchGame("dx11", true)
    }
}

async function launchGame(version: 'dx12' | 'dx11', debugMode: boolean) {
    if (process.platform != 'win32') {
        vscode.window.showErrorMessage("This action can only be performed on Windows");
        return;
    }

    const cfg = getConfiguration();
    if (cfg.gameDirectory == "") {
        vscode.window.showErrorMessage("Path to the game directory has not been specified in the configuration!");
        return;
    }

    const exeDir = fspath.join(cfg.gameDirectory, version == 'dx12' ? GAME_EXE_DIR_DX12 : GAME_EXE_DIR_DX11);
    const exePath = fspath.join(exeDir, GAME_EXE_NAME);

    if (!(await fileExists(exePath))) {
        vscode.window.showErrorMessage("The game executable could not be found!");
        return;
    }

    if (await isGameRunning()) {
        vscode.window.showErrorMessage("The game is already running!");
        return;
    }

    // spawn the game as a detached process
    let child = cp.spawn(
        exePath, 
        debugMode ? GAME_DEBUG_ARGS : [], 
        { detached: true, stdio: ['ignore'], cwd: exeDir }
    );

    // don't wait for the child to exit
    child.unref();
}

async function isGameRunning(): Promise<boolean> {
    return new Promise((resolve, _reject) => {
        cp.exec('tasklist', (_err, stdout, _stderr) => {
            resolve(stdout.toLowerCase().indexOf('witcher3.exe') > -1)
        })
    });
}



export function commandRecompileScripts(ctx: vscode.ExtensionContext): Cmd {
    return async () => {
        runRw3d(ctx, 'reload', []);
    }
}

export function commandExecConsoleCommand(ctx: vscode.ExtensionContext): Cmd {
    return async () => {
        const cmd = await vscode.window.showInputBox({
            title: "Enter a console command to be executed"
        });

        if (cmd) {
            runRw3d(ctx, `exec`, [`"${cmd}"`]);
        }
    }
}

function runRw3d(ctx: vscode.ExtensionContext, cmd: string, additionalArgs: string[]) {
    const ext = process.platform === "win32" ? ".exe" : "";
    const rw3dPath = ctx.asAbsolutePath(
        `deps/rw3d/bin/rw3d_cli${ext}`
    );

    const ip = getConfiguration().gameHostIpAddress;
    const args = [
        "--no-delay", "--log-level=output-only", `--ip=${ip}`,
        cmd, ...additionalArgs
    ];

    state.gameOutputChannel.show();

    state.gameOutputChannel.append("\n");
    state.gameOutputChannel.debug(`Executing: rw3d_cli ${args.join(" ")}`)
    const rw3d = cp.spawn(rw3dPath, args);

    rw3d.stdout.on('data', (data) => {
        const s = (data.toString() as string).trimEnd();
        for(const line of s.split("\n")) {
            state.gameOutputChannel.append(line);
        }
    });

    rw3d.stderr.on('data', (data) => {
        const s = (data.toString() as string).trimEnd();
        for(const line of s.split("\n")) {
            state.gameOutputChannel.error(line);
        }
    });
}
