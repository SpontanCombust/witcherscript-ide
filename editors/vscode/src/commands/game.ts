import * as vscode from 'vscode';
import * as fspath from 'path';
import * as cp from 'child_process';

import { Cmd } from './index'
import { getConfiguration } from '../config';
import { fileExists } from '../utils';
import { Rw3dCli } from '../rw3d';


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
    return () => {
        const rw3d = new Rw3dCli(ctx);
        rw3d.recompileScripts();
    }
}

export function commandExecConsoleCommand(ctx: vscode.ExtensionContext): Cmd {
    return () => {
        const rw3d = new Rw3dCli(ctx);
        rw3d.execConsoleCommmand();
    }
}
