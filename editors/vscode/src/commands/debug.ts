import * as vscode from 'vscode';

import * as state from '../state';
import * as tdcp from '../providers/text_document_content_providers'
import { Cmd } from './index'


export function commandShowScriptAst(context: vscode.ExtensionContext): Cmd {
    return async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            return;
        }

        const scriptPath = activeEditor.document.uri.fsPath;
        const scriptLine = activeEditor.selection.active.line + 1;
        const uri = vscode.Uri
            .file(scriptPath + tdcp.ScriptAstProvider.pathSuffix)
            .with({ scheme: tdcp.ScriptAstProvider.scheme });
        
        const doc = await vscode.workspace.openTextDocument(uri);
        const options: vscode.TextDocumentShowOptions = {
            viewColumn: vscode.ViewColumn.Beside,
            preview: false,
            preserveFocus: true
        };

        tdcp.ScriptAstProvider.getInstance().eventEmitter.fire(uri);
        
        vscode.window.showTextDocument(doc, options).then(async editor => {
            const astText = editor.document.getText();
            // Searching for corresponding node in AST text.
            // A naive approach leveraging the format of returned AST text.
            // Nodes represented there together with their names have a range at which they appear.
            // E.g. Identifier [10, 1] - [10, 5]
            // with [line1, column1] - [line2, column2] being the range in question.
            // So here I just search for such a range that hopefully appears in the AST.
            const match = astText.search(new RegExp("\\[" + scriptLine));
            if (match != -1) {
                const targetPos = editor.document.positionAt(match);
                // Scroll the cursor in AST's editor to searched position
                editor.revealRange(new vscode.Range(targetPos, targetPos), vscode.TextEditorRevealType.AtTop);
            }

            const rememberedChoices = state.RememberedChoices.Memento.fetchOrDefault(context);
            if (!rememberedChoices.neverShowAgainDebugAstNotif) {
                enum Answer {
                    Close = "I understand",
                    NeverShowAgain = "Never show this message again"
                }

                const answer = await vscode.window.showInformationMessage(
                    "Beware! Displayed ranges in the AST may not be accurate if your document is formatted using tabs instead of spaces",
                    Answer.Close, Answer.NeverShowAgain
                );

                if (answer == Answer.NeverShowAgain) {
                    rememberedChoices.neverShowAgainDebugAstNotif = true;
                    rememberedChoices.store(context);
                }
            }
        });
    };
}

export function commandContentGraphDot(): Cmd {
    return async () => {
        const virtFileName = "WitcherScript Content Graph";
        const uri = vscode.Uri
            .file(virtFileName)
            .with({ scheme: tdcp.ContentGraphDotProvider.scheme });

        const doc = await vscode.workspace.openTextDocument(uri);
        const options: vscode.TextDocumentShowOptions = {
            viewColumn: vscode.ViewColumn.Beside,
            preview: false,
            preserveFocus: true
        };

        tdcp.ContentGraphDotProvider.getInstance().eventEmitter.fire(uri);

        await vscode.window.showTextDocument(doc, options);
    }
}

export function commandShowScriptSymbols(): Cmd {
    return async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            return;
        }

        const scriptPath = activeEditor.document.uri.fsPath;
        const uri = vscode.Uri
            .file(scriptPath + tdcp.ScriptSymbolsProvider.pathSuffix)
            .with({ scheme: tdcp.ScriptSymbolsProvider.scheme });
        
        const doc = await vscode.workspace.openTextDocument(uri);
        const options: vscode.TextDocumentShowOptions = {
            viewColumn: vscode.ViewColumn.Beside,
            preview: false,
            preserveFocus: true
        };

        tdcp.ScriptSymbolsProvider.getInstance().eventEmitter.fire(uri);
        
        vscode.window.showTextDocument(doc, options);
    };
}

export function commandShowScriptCst(context: vscode.ExtensionContext): Cmd {
    return async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            return;
        }

        const scriptPath = activeEditor.document.uri.fsPath;
        const scriptLine = activeEditor.selection.active.line + 1;
        const uri = vscode.Uri
            .file(scriptPath + tdcp.ScriptCstProvider.pathSuffix)
            .with({ scheme: tdcp.ScriptCstProvider.scheme });
        
        const doc = await vscode.workspace.openTextDocument(uri);
        const options: vscode.TextDocumentShowOptions = {
            viewColumn: vscode.ViewColumn.Beside,
            preview: false,
            preserveFocus: true
        };

        tdcp.ScriptCstProvider.getInstance().eventEmitter.fire(uri);
        
        vscode.window.showTextDocument(doc, options).then(async editor => {
            const cstText = editor.document.getText();
            const match = cstText.search(new RegExp("\\[" + scriptLine));
            if (match != -1) {
                const targetPos = editor.document.positionAt(match);
                editor.revealRange(new vscode.Range(targetPos, targetPos), vscode.TextEditorRevealType.AtTop);
            }

            const rememberedChoices = state.RememberedChoices.Memento.fetchOrDefault(context);
            // using the same memento for AST warning for simplicity
            if (!rememberedChoices.neverShowAgainDebugAstNotif) { 
                enum Answer {
                    Close = "I understand",
                    NeverShowAgain = "Never show this message again"
                }

                const answer = await vscode.window.showInformationMessage(
                    "Beware! Displayed ranges in the CST may not be accurate if your document is formatted using tabs instead of spaces",
                    Answer.Close, Answer.NeverShowAgain
                );

                if (answer == Answer.NeverShowAgain) {
                    rememberedChoices.neverShowAgainDebugAstNotif = true;
                    rememberedChoices.store(context);
                }
            }
        });
    }
}

export function commandClearGlobalState(context: vscode.ExtensionContext): Cmd {
    return async () => {
        const keys = context.globalState.keys();

        const selected = await vscode.window.showQuickPick([...keys, 'ALL']);
        if (selected) {
            if (selected == 'ALL') {
                for (const key of keys) {
                    await context.globalState.update(key, undefined);
                }
            } else {
                await context.globalState.update(selected, undefined);
            }
        }
    }
}