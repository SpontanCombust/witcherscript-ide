import * as path from 'path';
import * as vscode from 'vscode';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node';


let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
	const ext = process.platform === "win32" ? ".exe" : "";
	const serverPath = context.asAbsolutePath(
		path.join('server', 'bin', `witcherscript-lsp${ext}`)
	);

	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	const serverOptions: ServerOptions = {
		run: { 
			command: serverPath, 
			transport: TransportKind.stdio 
		},
		debug: { 
			command: serverPath, 
			transport: TransportKind.stdio 
		}
	};

	const clientOptions: LanguageClientOptions = {
		// Register the server for WitcherScript documents
		documentSelector: [{ scheme: 'file', language: 'witcherscript' }],
		synchronize: {
			// Notify the server about file changes to *.ws files contained in the workspace
			fileEvents: vscode.workspace.createFileSystemWatcher('**/.ws')
		}
	};

	client = new LanguageClient(
		'witcherscript-ide',
		'WitcherScript IDE',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	client.start();
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}