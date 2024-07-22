import * as vscode from 'vscode';

import * as commands from './commands';
import * as config from './config';
import * as providers from './providers';
import * as lc from './lsp/lang_client';
import * as state from './state';
import { getContextKeys } from './context_keys';
import { getPersistence } from './persistence';


export function activate(context: vscode.ExtensionContext) {
	const cfg = config.getConfiguration();
	const ctxKeys = getContextKeys();
	const db = getPersistence(context);

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


	if (db.shouldSeeWelcomeMessage) {
		enum Answer {
			ShowTuto = "Show tutorial",
			Skip = "I know what I'm doing"
		}

		vscode.window.showInformationMessage(
			"Thank you for installing WitcherScript IDE!\nCheck out the tutorial to get started.",
			Answer.ShowTuto, Answer.Skip,
		).then((answer) => {
			if (answer) {
				switch (answer) {
					case Answer.ShowTuto:
						vscode.commands.executeCommand("workbench.action.openWalkthrough", "SpontanCombust.witcherscript-ide#witcherscript-ide.walkthrough", false);
						break;
					case Answer.Skip:
						break;
				}

				db.shouldSeeWelcomeMessage = false;
			}
		});
	}

	const version = context.extension.packageJSON.version as string;
	if (db.version != version) {
		const changelogUri = vscode.Uri.parse("https://spontancombust.github.io/witcherscript-ide/user-manual/changelog/");

		enum Answer {
			ShowChangelog = "Show changelog",
		}

		vscode.window.showInformationMessage(
			`WitcherScript IDE has been updated to version ${version}`,
			Answer.ShowChangelog
		).then((answer) => {
			if (answer) {
				switch (answer) {
					case Answer.ShowChangelog:
						vscode.env.openExternal(changelogUri);
						break;
				}
			}
		})

		db.version = version;
	}
}

export function deactivate(): Thenable<void> | undefined {
	return lc.stopLanguageClient();
}
