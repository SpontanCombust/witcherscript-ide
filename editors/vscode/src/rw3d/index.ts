/// Handling the Rusty Witcher 3 Debugger CLI executable shipped together with the extension

import * as vscode from 'vscode';
import * as cp from 'child_process';

import { GameHostType, getConfiguration } from '../config';
import * as state from '../state'


export class Rw3dCli {
    private readonly exePath: string;
    
    constructor(ctx: vscode.ExtensionContext) {
        const ext = process.platform === "win32" ? ".exe" : "";
        this.exePath = ctx.asAbsolutePath(
            `deps/rw3d/bin/rw3d_cli${ext}`
        );
    }


    public recompileScripts() {
        this.run('reload', []);
    }

    public async execConsoleCommmand(cmd?: string) {
        if (!cmd) {
            cmd = await vscode.window.showInputBox({
                title: "Enter a console command to be executed"
            });
        }

        if (cmd) {
            this.run('exec', [`"${cmd}"`]);
        }
    }


    private run(cmd: string, additionalArgs: string[]) {
        const cfg = getConfiguration();

        let target: string;
        switch (cfg.gameHostType) {
            case GameHostType.Standalone:
                target = "game";
                break;
            case GameHostType.Editor:
                target = "editor";
                break;
            case GameHostType.Auto:
                target = "auto";
                break;
            default:
                target = "auto";
        }

        const ip = cfg.gameHostIpAddress;
        const args = [
            "--no-delay", "--log-level=output-only", `--target=${target}`, `--ip=${ip}`,
            cmd, ...additionalArgs
        ];

        state.gameOutputChannel.show();

        state.gameOutputChannel.append("\n");
        state.gameOutputChannel.debug(`Executing: rw3d_cli ${args.join(" ")}`)
        const rw3d = cp.spawn(this.exePath, args);

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
}