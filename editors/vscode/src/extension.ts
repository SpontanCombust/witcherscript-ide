import * as path from 'path';
import * as vscode from 'vscode';
import * as lsp from 'vscode-languageclient/node';

import * as commands from './commands';
import * as state from './state';
import * as config from './config';
import * as providers from './providers';
import * as handlers from './handlers';


export let client: lsp.LanguageClient;

export function activate(context: vscode.ExtensionContext) {
	commands.registerCommands(context);
	providers.registerProviders(context);

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

	const cfg = config.getConfiguration();
	const initializationOptions: InitializationOptions = {
		gameDirectory: cfg.gameDirectory,
		contentRepositories: cfg.contentRepositories
	};

	const clientOptions: lsp.LanguageClientOptions = {
		// Register the server for WitcherScript documents
		documentSelector: [
			{ scheme: 'file', language: 'witcherscript' },
			{ scheme: 'file', pattern: '**/*.w3edit' },
			{ scheme: 'file', pattern: '**/witcherscript.toml' }
		],
		synchronize: {
			// Notify the server about file changes to files we care about
			fileEvents: [
				vscode.workspace.createFileSystemWatcher('**/*.ws'),
				vscode.workspace.createFileSystemWatcher('**/witcherscript.toml', false, true),
				vscode.workspace.createFileSystemWatcher('**/*.w3edit', false, true)
			]
		},
		initializationOptions: initializationOptions
	};

	client = new lsp.LanguageClient(
		'witcherscript-ide',
		'WitcherScript IDE',
		serverOptions,
		clientOptions
	);

	handlers.registerHandlers(client, context);

	// Start the client. This will also launch the server
	client.start().then(_ => {
		const memento = state.OpenManifestOnInit.Memento.fetch(context);
		
		if (memento != undefined) {
			// If a new project has just been created in this directory and the user agreed to open it, show them the manifest of said project
			if (vscode.workspace.workspaceFolders?.some(f => f.uri.fsPath == memento.workspaceUri.fsPath)) {
				const params: vscode.TextDocumentShowOptions = {
					preview: false
				};
				vscode.window.showTextDocument(memento.manifestUri, params).then(
					_ => {},
					(err) => client.debug('Manifest could not be shown: ' + err)
				);

				state.OpenManifestOnInit.Memento.erase(context);
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


interface InitializationOptions {
	gameDirectory: string,
    contentRepositories: string[]
}