import * as vscode from 'vscode';

import * as commands from './commands';
import * as config from './config';
import * as providers from './providers';
import * as lc from './lsp/lang_client';
import * as state from './state';
import { getContextKeys } from './context_keys';


export function activate(context: vscode.ExtensionContext) {
	const cfg = config.getConfiguration();
	const ctxKeys = getContextKeys();

	ctxKeys.debugFeaturesEnabled = cfg.enableDebugFeatures;
	ctxKeys.languageServerActive = false;
	
	commands.registerCommands(context);
	providers.registerProviders(context);

	state.initializeState(context);

	if (cfg.enableLanguageServer) {
		lc.createLanguageClient(context, cfg).then(() => {
			ctxKeys.languageServerActive = true;
		});
	}
}

export function deactivate(): Thenable<void> | undefined {
	return lc.stopLanguageClient();
}
