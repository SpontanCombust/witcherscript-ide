import * as path from 'path';
import * as vscode from 'vscode';
import * as lsp from 'vscode-languageclient/node';

import * as state from './state';
import * as config from './config';
import * as handlers from './handlers';


let client: lsp.LanguageClient | undefined;

export async function createLanguageClient(ctx: vscode.ExtensionContext, cfg: config.Config) {
    const ext = process.platform === "win32" ? ".exe" : "";
	const serverPath = ctx.asAbsolutePath(
		path.join('server', 'bin', `witcherscript-lsp${ext}`)
	);
	const nativeContentUri = vscode.Uri.joinPath(
		ctx.extensionUri, 'server', 'assets', 'content0_native'
	).toString();

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

	const initializationOptions: InitializationOptions = {
		nativeContentUri: nativeContentUri,
		gameDirectory: cfg.gameDirectory,
		contentRepositories: cfg.contentRepositories,
		enableSyntaxAnalysis: cfg.enableSyntaxAnalysis
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

	handlers.registerHandlers(client, ctx);

	// Start the client. This will also launch the server
	return client.start().then(_ => {
		const memento = state.OpenManifestOnInit.Memento.fetch(ctx);
		
		if (memento != undefined) {
			// If a new project has just been created in this directory and the user agreed to open it, show them the manifest of said project
			if (vscode.workspace.workspaceFolders?.some(f => f.uri.fsPath == memento.workspaceUri.fsPath)) {
				const params: vscode.TextDocumentShowOptions = {
					preview: false
				};
				vscode.window.showTextDocument(memento.manifestUri, params).then(
					_ => {},
					(err) => client?.debug('Manifest could not be shown: ' + err)
				);

				state.OpenManifestOnInit.Memento.erase(ctx);
			}
		}
	});
}

// Configuration needed by the server. The format in both client and server must match!
interface InitializationOptions {
	nativeContentUri: string,
	gameDirectory: string,
    contentRepositories: string[]
	enableSyntaxAnalysis: boolean
}


export function getLanguageClient(): lsp.LanguageClient | undefined {
    return client;
}

export async function stopLanguageClient() {
    return client?.stop();
}