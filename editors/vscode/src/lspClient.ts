import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions
} from 'vscode-languageclient/node';

export class SpanglingsLspClient {
  private client: LanguageClient | undefined;
  private isStarting = false;
  private isStopping = false;

  public async start(_context: vscode.ExtensionContext): Promise<void> {
    if (this.isStarting || this.isStopping) {
      return;
    }

    const config = vscode.workspace.getConfiguration('spanglings');
    const enableLsp = config.get<boolean>('enableLsp', true);
    if (!enableLsp) {
      return;
    }

    if (this.client) {
      await this.stop();
    }

    this.isStarting = true;
    try {
      const executablePath = config.get<string>('executablePath', 'spanglings');
      const strictAccents = config.get<boolean>('strictAccents', false);

      const args = ['lsp'];
      if (strictAccents) {
        args.push('--strict-accents');
      }

      const cwd = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;

      const serverOptions: ServerOptions = {
        command: executablePath,
        args,
        options: cwd ? { cwd } : undefined
      };

      const clientOptions: LanguageClientOptions = {
        documentSelector: [
          { scheme: 'file', language: 'markdown' },
          { scheme: 'untitled', language: 'markdown' }
        ],
        synchronize: {
          fileEvents: vscode.workspace.createFileSystemWatcher('**/exercises/**/*.md')
        }
      };

      this.client = new LanguageClient(
        'spanglingsLsp',
        'Spanglings Language Server',
        serverOptions,
        clientOptions
      );

      await this.client.start();
    } catch (err) {
      vscode.window.showErrorMessage(
        `Failed to start Spanglings LSP: ${err instanceof Error ? err.message : String(err)}`
      );
    } finally {
      this.isStarting = false;
    }
  }

  public async stop(): Promise<void> {
    if (!this.client || this.isStopping) {
      return;
    }
    this.isStopping = true;
    const clientToStop = this.client;
    this.client = undefined;
    try {
      await clientToStop.stop();
    } catch (err) {
      console.error('Error stopping Spanglings LSP client:', err);
    } finally {
      this.isStopping = false;
    }
  }

  public async restart(context: vscode.ExtensionContext): Promise<void> {
    if (this.isStarting || this.isStopping) {
      return;
    }
    await this.stop();
    await this.start(context);
  }

  public getClient(): LanguageClient | undefined {
    return this.client;
  }
}
