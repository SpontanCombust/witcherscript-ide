import * as vscode from 'vscode';
import * as state from './state';


export async function showForeignScriptWarning(context: vscode.ExtensionContext) {
    const rememberedChoices = state.RememberedChoices.Memento.fetchOrDefault(context);
    if (!rememberedChoices.neverShowAgainForeignScriptWarning) {
        enum Answer {
            Close = "I understand",
            NeverShowAgain = "Don't show this message again",
            SeeManual = "See manual"
        }

        const manualUri = vscode.Uri.parse("https://spontancombust.github.io/witcherscript-ide/user-manual/project-system/");

        const answer = await vscode.window.showWarningMessage(
            "This script file is not included through any workspace project or their dependencies.\n" +
            "If you want to use more than the most basic features of the extension you need to create a script project.\n" +
            "To learn about creating projects see the manual:\n" + manualUri.toString(),
            Answer.Close, Answer.NeverShowAgain, Answer.SeeManual
        );

        if (answer == Answer.NeverShowAgain) {
            rememberedChoices.neverShowAgainForeignScriptWarning = true;
            rememberedChoices.store(context);
        }
        else if (answer == Answer.SeeManual) {
            await vscode.env.openExternal(manualUri);
        }
    }
}