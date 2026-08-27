import * as vscode from 'vscode';
import { SpanglingsLspClient } from './lspClient';
import { SpanglingsStatusBar } from './statusBar';
import { SpanglingsTreeProvider } from './exerciseTree';
import { registerCommands } from './commands';

let lspClient: SpanglingsLspClient | undefined;
let statusBar: SpanglingsStatusBar | undefined;
let treeProvider: SpanglingsTreeProvider | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  lspClient = new SpanglingsLspClient();
  statusBar = new SpanglingsStatusBar();
  treeProvider = new SpanglingsTreeProvider();

  context.subscriptions.push(statusBar);

  // Register TreeDataProvider for the sidebar exercise explorer
  context.subscriptions.push(
    vscode.window.registerTreeDataProvider('spanglings.exerciseTree', treeProvider)
  );

  // Register all extension command handlers
  registerCommands(context, treeProvider, statusBar);

  // Start LSP client and initialize status bar
  await lspClient.start(context);
  await statusBar.update();

  // Update status bar and tree provider when exercise files are saved
  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument(async (doc) => {
      if (doc.languageId === 'markdown' || doc.fileName.endsWith('.md')) {
        await Promise.all([
          statusBar?.update(),
          treeProvider?.refresh()
        ]);
      }
    })
  );

  // Handle configuration changes
  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration(async (e) => {
      if (e.affectsConfiguration('spanglings')) {
        await lspClient?.restart(context);
        await Promise.all([
          statusBar?.update(),
          treeProvider?.refresh()
        ]);
      }
    })
  );
}

export async function deactivate(): Promise<void> {
  if (lspClient) {
    await lspClient.stop();
  }
}
