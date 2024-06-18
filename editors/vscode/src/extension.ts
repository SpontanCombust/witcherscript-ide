import * as vscode from 'vscode';

import * as commands from './commands';
import * as config from './config';
import * as providers from './providers';
import * as lc from './lsp/lang_client';
import * as state from './state';


export function activate(context: vscode.ExtensionContext) {
	const cfg = config.getConfiguration();

	vscode.commands.executeCommand('setContext', 'witcherscript-ide.debugFeaturesEnabled', cfg.enableDebugFeatures);
	vscode.commands.executeCommand('setContext', 'witcherscript-ide.languageServerActive', false);
	
	commands.registerCommands(context);
	providers.registerProviders(context);

	state.initializeState();

	if (cfg.enableLanguageServer) {
		state.scheduleWorkStatusUpdate({ kind: 'idle' });
		lc.createLanguageClient(context, cfg).then(() => {
			vscode.commands.executeCommand('setContext', 'witcherscript-ide.languageServerActive', true);
		});
	} else {
		state.scheduleWorkStatusUpdate({ kind: 'disabled' })
	}
}

export function deactivate(): Thenable<void> | undefined {
	return lc.stopLanguageClient()
		.then(() => state.disposeState());
}
