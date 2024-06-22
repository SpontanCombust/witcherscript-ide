import * as vscode from 'vscode'

import * as tdcp from './text_document_content_providers'


export class ReadOnlyFileDecorationsProvider implements vscode.FileDecorationProvider {
    onDidChangeFileDecorations?: vscode.Event<
        vscode.Uri | vscode.Uri[] | undefined
    > = undefined;

    provideFileDecoration(uri: vscode.Uri, _token: vscode.CancellationToken): vscode.ProviderResult<vscode.FileDecoration> {
        if (uri.scheme == tdcp.ReadOnlyContentProvider.scheme) {
            return new vscode.FileDecoration("R", "This file is read-only", new vscode.ThemeColor("icon.foreground"));
        }

        return undefined;
    }
}