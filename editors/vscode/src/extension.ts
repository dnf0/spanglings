import * as vscode from 'vscode';
import { SpanglingsLspClient } from './lspClient';
import { SpanglingsStatusBar } from './statusBar';

let lspClient: SpanglingsLspClient | undefined;
let statusBar: SpanglingsStatusBar | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  lspClient = new SpanglingsLspClient();
  statusBar = new SpanglingsStatusBar();

  context.subscriptions.push(statusBar);

  await lspClient.start(context);
  await statusBar.update();

  // Update status bar when exercise files are saved
  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument(async (doc) => {
      if (doc.languageId === 'markdown' || doc.fileName.endsWith('.md')) {
        await statusBar?.update();
      }
    })
  );

  // Handle configuration changes
  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration(async (e) => {
      if (e.affectsConfiguration('spanglings')) {
        await lspClient?.restart(context);
        await statusBar?.update();
      }
    })
  );
}

export async function deactivate(): Promise<void> {
  if (lspClient) {
    await lspClient.stop();
  }
}
