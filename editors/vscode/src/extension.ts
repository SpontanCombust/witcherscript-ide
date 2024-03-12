import * as path from 'path';
import * as vscode from 'vscode';
import * as lsp from 'vscode-languageclient/node';

import * as commands from './commands';
import * as state from './state';


export let client: lsp.LanguageClient;

export function activate(context: vscode.ExtensionContext) {
	commands.registerCommands(context);

	const ext = process.platform === "win32" ? ".exe" : "";
	const serverPath = context.asAbsolutePath(
		path.join('server', 'bin', `witcherscript-lsp${ext}`)
	);

	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	const serverOptions: lsp.ServerOptions = {
		run: { 
			command: serverPath, 
			transport: lsp.TransportKind.stdio 
		},
		debug: { 
			command: serverPath, 
			transport: lsp.TransportKind.stdio 
		}
	};

	const clientOptions: lsp.LanguageClientOptions = {
		// Register the server for WitcherScript documents
		documentSelector: [
			{ scheme: 'file', language: 'witcherscript' },
			{ scheme: 'file', pattern: '**/witcherscript.toml' }
		],
		synchronize: {
			// Notify the server about file changes to *.ws files contained in the workspace
			fileEvents: [
				vscode.workspace.createFileSystemWatcher('**/.ws'),
				vscode.workspace.createFileSystemWatcher('**/witcherscript.toml')
			]
		}
	};

	client = new lsp.LanguageClient(
		'witcherscript-ide',
		'WitcherScript IDE',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	client.start().then(_ => {
		const mementoDto = context.globalState.get<state.OpenManifestOnInit.MementoDto>(state.OpenManifestOnInit.KEY);
		
		if (mementoDto != undefined) {
			const memento = state.OpenManifestOnInit.Memento.fromDto(mementoDto);
			
			// If a new project has just been created in this directory and the user agreed to open it, show them the manifest of said project
			if (vscode.workspace.workspaceFolders.some(f => f.uri.fsPath == memento.workspaceUri.fsPath)) {
				const params: vscode.TextDocumentShowOptions = {
					selection: memento.selectionRange,
					preview: false
				};
				vscode.window.showTextDocument(memento.manifestUri, params).then(
					_ => {},
					(err) => client.debug('Manifest could not be shown: ' + err)
				);
	
				context.globalState.update(state.OpenManifestOnInit.KEY, undefined);
			}
		}
	});
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}