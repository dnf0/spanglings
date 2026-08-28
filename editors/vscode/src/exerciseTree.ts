import * as vscode from 'vscode';
import { execFile } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';
import { resolveExercisePath } from './pathUtils';

const execFileAsync = promisify(execFile);

export interface DiagnosticRule {
  pattern: string;
  code: string;
  message: string;
}

export interface RawExercise {
  path: string;
  id: string;
  level: string;
  topic: string;
  exercise_type: string;
  is_done?: boolean;
  completed?: boolean;
  title: string;
  solution?: string;
  alternatives?: string[];
  diagnostic_rules?: DiagnosticRule[];
  hints?: string[];
  raw_content?: string;
}

export class TreeItemNode extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly contextValue: string,
    public readonly topic?: string,
    public readonly exercise?: RawExercise
  ) {
    super(label, collapsibleState);
  }
}

export class SpanglingsTreeProvider implements vscode.TreeDataProvider<TreeItemNode> {
  private _onDidChangeTreeData: vscode.EventEmitter<TreeItemNode | undefined | null | void> =
    new vscode.EventEmitter<TreeItemNode | undefined | null | void>();
  readonly onDidChangeTreeData: vscode.Event<TreeItemNode | undefined | null | void> =
    this._onDidChangeTreeData.event;

  private exercises: RawExercise[] = [];
  private isRefreshing = false;

  constructor() {
    this.refresh();
  }

  public async refresh(): Promise<void> {
    if (this.isRefreshing) {
      return;
    }
    this.isRefreshing = true;

    try {
      await this.loadExercises();
      this._onDidChangeTreeData.fire();
    } catch (err) {
      console.error('Error refreshing Spanglings exercise tree:', err);
    } finally {
      this.isRefreshing = false;
    }
  }

  private async loadExercises(): Promise<void> {
    const config = vscode.workspace.getConfiguration('spanglings');
    const executablePath = config.get<string>('executablePath', 'spanglings');
    const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;

    try {
      const { stdout } = await execFileAsync(executablePath, ['list', '--json'], {
        cwd: workspaceRoot
      });
      this.exercises = JSON.parse(stdout.trim());
    } catch (err) {
      console.error('Error loading exercises via spanglings list --json:', err);
      this.exercises = [];
    }
  }

  getTreeItem(element: TreeItemNode): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: TreeItemNode): Promise<TreeItemNode[]> {
    if (this.exercises.length === 0 && !this.isRefreshing) {
      await this.loadExercises();
    }

    const workspaceRoot =
      vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || process.cwd();

    if (!element) {
      // Root level: Group exercises by topic/track
      const topicMap = new Map<string, RawExercise[]>();
      for (const ex of this.exercises) {
        const topic = ex.topic || 'general';
        if (!topicMap.has(topic)) {
          topicMap.set(topic, []);
        }
        topicMap.get(topic)!.push(ex);
      }

      const nodes: TreeItemNode[] = [];
      for (const [topic, exercises] of topicMap.entries()) {
        const total = exercises.length;
        const completed = exercises.filter(
          (e) => e.is_done === true || e.completed === true
        ).length;
        const topicName = this.formatTopicName(topic);
        const label = `${topicName} (${completed}/${total})`;

        const node = new TreeItemNode(
          label,
          vscode.TreeItemCollapsibleState.Collapsed,
          'topic',
          topic
        );
        node.iconPath =
          completed === total && total > 0
            ? new vscode.ThemeIcon('check-all')
            : new vscode.ThemeIcon('folder');
        node.tooltip = `${topicName}: ${completed} of ${total} completed`;
        nodes.push(node);
      }
      return nodes;
    }

    if (element.contextValue === 'topic' && element.topic) {
      // Child level: Exercises in topic
      const topicExercises = this.exercises.filter(
        (e) => (e.topic || 'general') === element.topic
      );

      return topicExercises.map((ex) => {
        const isCompleted = ex.is_done === true || ex.completed === true;
        const label = `[${ex.level}] ${ex.title}`;
        const node = new TreeItemNode(
          label,
          vscode.TreeItemCollapsibleState.None,
          'exercise',
          element.topic,
          ex
        );

        node.iconPath = isCompleted
          ? new vscode.ThemeIcon('pass')
          : new vscode.ThemeIcon('circle-outline');

        node.command = {
          command: 'spanglings.openExerciseFile',
          title: 'Open Exercise',
          arguments: [ex.path]
        };

        const statusText = isCompleted ? 'Completed ✓' : 'Incomplete ⏳';
        const tooltip = new vscode.MarkdownString();
        tooltip.isTrusted = true;
        tooltip.appendMarkdown(`**${ex.title}**\n\n`);
        tooltip.appendMarkdown(`- **Level:** \`${ex.level}\`\n`);
        tooltip.appendMarkdown(`- **Type:** \`${ex.exercise_type}\`\n`);
        tooltip.appendMarkdown(`- **Status:** ${statusText}\n`);
        tooltip.appendMarkdown(`- **File:** \`${ex.path}\`\n\n`);
        tooltip.appendMarkdown(`*Click to open exercise file*`);
        node.tooltip = tooltip;

        return node;
      });
    }

    return [];
  }

  private formatTopicName(topic: string): string {
    return topic
      .replace(/^(\d+)[_-]/, '')
      .replace(/[_-]/g, ' ')
      .replace(/\b\w/g, (c) => c.toUpperCase());
  }
}
