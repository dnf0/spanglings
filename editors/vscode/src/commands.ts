import * as vscode from 'vscode';
import { execFile } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';
import { SpanglingsTreeProvider, RawExercise } from './exerciseTree';
import { SpanglingsStatusBar } from './statusBar';

const execFileAsync = promisify(execFile);

export function registerCommands(
  context: vscode.ExtensionContext,
  treeProvider?: SpanglingsTreeProvider,
  statusBar?: SpanglingsStatusBar
): void {
  // Command: Open Next Exercise
  context.subscriptions.push(
    vscode.commands.registerCommand('spanglings.openNextExercise', async () => {
      const config = vscode.workspace.getConfiguration('spanglings');
      const executablePath = config.get<string>('executablePath', 'spanglings');
      const workspaceRoot =
        vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || process.cwd();

      try {
        const { stdout } = await execFileAsync(executablePath, ['list', '--json'], {
          cwd: workspaceRoot
        });
        const exercises: RawExercise[] = JSON.parse(stdout.trim());
        const nextExercise = exercises.find(
          (ex) => !ex.is_done && !ex.completed
        );

        if (!nextExercise) {
          vscode.window.showInformationMessage(
            'All Spanglings exercises are completed! 🎉 ¡Felicitaciones!'
          );
          return;
        }

        const absPath = path.isAbsolute(nextExercise.path)
          ? nextExercise.path
          : path.resolve(workspaceRoot, nextExercise.path);
        const fileUri = vscode.Uri.file(absPath);

        await vscode.commands.executeCommand('vscode.open', fileUri);
      } catch (err) {
        vscode.window.showErrorMessage(
          `Failed to open next exercise: ${err instanceof Error ? err.message : String(err)}`
        );
      }
    })
  );

  // Command: Conjugate Verb
  context.subscriptions.push(
    vscode.commands.registerCommand('spanglings.conjugateVerb', async () => {
      const verb = await vscode.window.showInputBox({
        prompt: 'Enter an infinitive Spanish verb to conjugate (e.g. desplegar, rendir, hacer)',
        placeHolder: 'desplegar'
      });

      if (!verb || verb.trim().length === 0) {
        return;
      }

      const config = vscode.workspace.getConfiguration('spanglings');
      const executablePath = config.get<string>('executablePath', 'spanglings');
      const workspaceRoot =
        vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || process.cwd();

      try {
        const { stdout } = await execFileAsync(
          executablePath,
          ['conjugate', verb.trim()],
          { cwd: workspaceRoot }
        );

        const doc = await vscode.workspace.openTextDocument({
          language: 'markdown',
          content: stdout
        });
        await vscode.window.showTextDocument(doc, { preview: true });
      } catch (err) {
        vscode.window.showErrorMessage(
          `Failed to conjugate verb "${verb}": ${err instanceof Error ? err.message : String(err)}`
        );
      }
    })
  );

  // Command: Open Reference Browser
  context.subscriptions.push(
    vscode.commands.registerCommand('spanglings.openReferenceBrowser', async () => {
      const referenceTopics: vscode.QuickPickItem[] = [
        { label: 'subjunctive', description: 'Spanish Subjunctive (WEIRDO rules & triggers)' },
        { label: 'por-para', description: 'Por vs Para fundamental rules & use cases' },
        { label: 'ser-estar', description: 'Ser vs Estar distinctions & meaning shifts' },
        { label: 'past', description: 'Pretérito Indefinido vs Imperfecto vs Pluscuamperfecto' },
        { label: 'pronouns', description: 'Direct, indirect, and reflexive pronoun placement' },
        { label: 'prepositions', description: 'Spanish prepositions & verbal regimes (a, de, en, con)' },
        { label: 'accidental-se', description: 'Accidental Se constructions for unexpected events' },
        { label: 'tech', description: 'Developer Spanish terminology & cloud operations' },
        { label: 'business', description: 'Executive, enterprise governance & startup Spanish' },
        { label: 'false-friends', description: 'False cognates & common English interference traps' },
        { label: 'voseo', description: 'Rioplatense & Latin American voseo conjugation rules' },
        { label: 'accents', description: 'Spanish tilde & orthographic accent classification' }
      ];

      const selected = await vscode.window.showQuickPick(referenceTopics, {
        placeHolder: 'Select a Spanish grammar or developer reference topic to explain'
      });

      if (!selected) {
        return;
      }

      const config = vscode.workspace.getConfiguration('spanglings');
      const executablePath = config.get<string>('executablePath', 'spanglings');
      const workspaceRoot =
        vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || process.cwd();

      try {
        const { stdout } = await execFileAsync(
          executablePath,
          ['explain', selected.label],
          { cwd: workspaceRoot }
        );

        const doc = await vscode.workspace.openTextDocument({
          language: 'markdown',
          content: stdout
        });
        await vscode.window.showTextDocument(doc, { preview: true });
      } catch (err) {
        vscode.window.showErrorMessage(
          `Failed to explain topic "${selected.label}": ${err instanceof Error ? err.message : String(err)}`
        );
      }
    })
  );

  // Command: Sync State
  context.subscriptions.push(
    vscode.commands.registerCommand('spanglings.syncState', async () => {
      try {
        await Promise.all([
          treeProvider?.refresh(),
          statusBar?.update()
        ]);
        vscode.window.showInformationMessage(
          'Spanglings progress and exercise explorer synchronized.'
        );
      } catch (err) {
        vscode.window.showErrorMessage(
          `Failed to sync Spanglings state: ${err instanceof Error ? err.message : String(err)}`
        );
      }
    })
  );
}
