import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';

/**
 * Robustly resolves an exercise relative path (e.g. "exercises/00_baseline/01_present_irregular_stems.md")
 * against the current workspace, checking:
 * 1. Absolute paths
 * 2. Direct resolution against workspace root
 * 3. Resolution with "exercises/" prefix stripped (if workspace is already inside exercises/)
 * 4. Ascending parent directory traversal up to 6 levels
 */
export function resolveExercisePath(exPath: string, workspaceRoot?: string): string {
  const root =
    workspaceRoot ||
    vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ||
    process.cwd();

  // 1. If path is already absolute and exists on disk
  if (path.isAbsolute(exPath) && fs.existsSync(exPath)) {
    return exPath;
  }

  // 2. Direct resolve with workspaceRoot (e.g. workspaceRoot/exercises/00_baseline/...)
  const directPath = path.resolve(root, exPath);
  if (fs.existsSync(directPath)) {
    return directPath;
  }

  // 3. If workspaceRoot is itself inside 'exercises' or ends with 'exercises', strip leading 'exercises/'
  if (exPath.startsWith('exercises/') || exPath.startsWith('exercises\\')) {
    const stripped = exPath.replace(/^exercises[/\\]/, '');
    const strippedPath = path.resolve(root, stripped);
    if (fs.existsSync(strippedPath)) {
      return strippedPath;
    }
  }

  // 4. If workspaceRoot is in a subfolder of a repo (like editors/vscode or 00_baseline), search parent directories
  let cur = root;
  for (let i = 0; i < 6; i++) {
    // Check with full relative path (e.g. parent/exercises/00_baseline/...)
    const candidateFull = path.resolve(cur, exPath);
    if (fs.existsSync(candidateFull)) {
      return candidateFull;
    }

    // Check with stripped path if exPath starts with exercises/
    if (exPath.startsWith('exercises/') || exPath.startsWith('exercises\\')) {
      const stripped = exPath.replace(/^exercises[/\\]/, '');
      const candidateStripped = path.resolve(cur, stripped);
      if (fs.existsSync(candidateStripped)) {
        return candidateStripped;
      }
    }

    const parent = path.dirname(cur);
    if (parent === cur) {
      break;
    }
    cur = parent;
  }

  // 5. Default fallback to direct resolution
  return directPath;
}
