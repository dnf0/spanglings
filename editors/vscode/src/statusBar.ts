import * as vscode from 'vscode';
import { execFile } from 'child_process';
import { promisify } from 'util';
import { getEffectiveWorkspaceRoot } from './pathUtils';

const execFileAsync = promisify(execFile);

export interface LevelProgress {
  total: number;
  completed: number;
  percent: number;
}

export interface ProgressActivity {
  current_streak: number;
  longest_streak: number;
  total_active_days: number;
  daily_counts?: Record<string, number>;
}

export interface ProgressData {
  total: number;
  completed: number;
  percent: number;
  due_reviews: number;
  levels?: Record<string, LevelProgress>;
  weak_topics?: string[];
  recommendations?: string[];
  activity?: ProgressActivity;
}

export class SpanglingsStatusBar implements vscode.Disposable {
  private statusBarItem: vscode.StatusBarItem;
  private isUpdating = false;

  constructor() {
    this.statusBarItem = vscode.window.createStatusBarItem(
      vscode.StatusBarAlignment.Right,
      100
    );
    this.statusBarItem.command = 'spanglings.openNextExercise';
    this.statusBarItem.text = '$(mortar-board) Spanglings';
    this.statusBarItem.tooltip = 'Spanglings: Developer Spanish Learning';
    this.statusBarItem.show();
  }

  public async update(): Promise<void> {
    if (this.isUpdating) {
      return;
    }
    this.isUpdating = true;

    const config = vscode.workspace.getConfiguration('spanglings');
    const executablePath = config.get<string>('executablePath', 'spanglings');
    const cwd = getEffectiveWorkspaceRoot();

    try {
      const { stdout } = await execFileAsync(executablePath, ['progress', '--json'], {
        cwd
      });

      const data: ProgressData = JSON.parse(stdout.trim());
      const completed = data.completed ?? 0;
      const total = data.total ?? 0;
      const percent = data.percent ?? 0;
      const streak = data.activity?.current_streak ?? 0;
      const longestStreak = data.activity?.longest_streak ?? streak;
      const dueReviews = data.due_reviews ?? 0;

      if (streak > 0) {
        this.statusBarItem.text = `$(mortar-board) ${completed}/${total} 🔥 ${streak}d`;
      } else if (total > 0) {
        this.statusBarItem.text = `$(mortar-board) ${completed}/${total} (${Math.round(percent)}%)`;
      } else {
        this.statusBarItem.text = '$(mortar-board) Spanglings';
      }

      const tooltip = new vscode.MarkdownString();
      tooltip.isTrusted = true;
      tooltip.appendMarkdown(`**Spanglings Progress**\n\n`);
      tooltip.appendMarkdown(`- **Completed:** ${completed} / ${total} (${percent.toFixed(1)}%)\n`);
      tooltip.appendMarkdown(`- **Current Streak:** ${streak} day${streak === 1 ? '' : 's'} 🔥\n`);
      tooltip.appendMarkdown(`- **Longest Streak:** ${longestStreak} day${longestStreak === 1 ? '' : 's'}\n`);
      if (dueReviews > 0) {
        tooltip.appendMarkdown(`- **Due SRS Reviews:** ${dueReviews} ⏰\n`);
      }
      tooltip.appendMarkdown(`\n---\n*Click to open next exercise*`);
      this.statusBarItem.tooltip = tooltip;
    } catch (err) {
      console.error('Error updating Spanglings status bar:', err);
      // Graceful fallback if spanglings binary is not found or fails
      this.statusBarItem.text = '$(mortar-board) Spanglings';
      this.statusBarItem.tooltip = 'Spanglings: Click to open next exercise';
    } finally {
      this.isUpdating = false;
    }
  }

  public dispose(): void {
    this.statusBarItem.dispose();
  }
}
