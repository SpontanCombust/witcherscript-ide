import * as vscode from 'vscode';
import * as fspath from 'path';
import * as cp from 'child_process';

import { Cmd } from './index'
import { getConfiguration } from '../config';
import { fileExists } from '../utils';


const EXE_PATH_DX12 = "bin/x64_dx12/witcher3.exe";
const EXE_PATH_DX11 = "bin/x64/witcher3.exe";
const DEBUG_ARGS = ['-net', '-debugscripts'];

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
        vscode.window.showErrorMessage("Path to the game directory has not been specified!");
        return;
    }

    const exePath = fspath.join(cfg.gameDirectory, version == 'dx12' ? EXE_PATH_DX12 : EXE_PATH_DX11);

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
        debugMode ? DEBUG_ARGS : [], 
        { detached: true, stdio: ['ignore']}
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