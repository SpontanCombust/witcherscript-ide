import * as vscode from 'vscode';

import * as tdcp from './text_document_content_providers';
import { DashboardProvider } from './dashboard_provider';
import { VanillaFilesProvider } from './vanilla_files_provider';
import { ScriptContentProvider } from './script_content_provider';
import * as fdp from './file_decorations_providers'


export function registerProviders(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.workspace.registerTextDocumentContentProvider(tdcp.ScriptAstProvider.scheme, tdcp.ScriptAstProvider.getInstance()),
        vscode.workspace.registerTextDocumentContentProvider(tdcp.ScriptCstProvider.scheme, tdcp.ScriptCstProvider.getInstance()),
        vscode.workspace.registerTextDocumentContentProvider(tdcp.ContentGraphDotProvider.scheme, tdcp.ContentGraphDotProvider.getInstance()),
        vscode.workspace.registerTextDocumentContentProvider(tdcp.ScriptSymbolsProvider.scheme, tdcp.ScriptSymbolsProvider.getInstance()),
        vscode.workspace.registerTextDocumentContentProvider(tdcp.ReadOnlyContentProvider.scheme, tdcp.ReadOnlyContentProvider.getInstance()),
        vscode.window.registerTreeDataProvider(DashboardProvider.viewId, DashboardProvider.getInstance()),
        vscode.window.registerTreeDataProvider(VanillaFilesProvider.viewId, VanillaFilesProvider.getInstance()),
        vscode.window.registerTreeDataProvider(ScriptContentProvider.viewId, ScriptContentProvider.getInstance()),
        vscode.window.registerFileDecorationProvider(new fdp.ReadOnlyFileDecorationsProvider())
    );
}