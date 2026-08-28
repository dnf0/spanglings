import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';

/**
 * Returns a valid, writable workspace directory.
 * If no folder is open in VS Code, checks standard locations (~/repos/spanglings, ~/spanglings)
 * before falling back to os.homedir(). Never returns '/' or read-only system root.
 */
export function getEffectiveWorkspaceRoot(): string {
  const wsFolder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  if (wsFolder) {
    return wsFolder;
  }

  // Check known existing spanglings directories
  const repoSpanglings = path.join(os.homedir(), 'repos', 'spanglings');
  if (fs.existsSync(repoSpanglings)) {
    return repoSpanglings;
  }

  const homeSpanglings = path.join(os.homedir(), 'spanglings');
  if (fs.existsSync(homeSpanglings)) {
    return homeSpanglings;
  }

  return os.homedir();
}

/**
 * Robustly resolves an exercise relative path (e.g. "exercises/00_baseline/01_present_irregular_stems.md")
 * against the current workspace or common locations, checking:
 * 1. Absolute paths
 * 2. Direct resolution against effective workspace root
 * 3. Resolution with "exercises/" prefix stripped
 * 4. Ascending parent directory traversal up to 6 levels
 * 5. Checking standard ~/repos/spanglings and ~/spanglings locations
 */
export function resolveExercisePath(exPath: string, workspaceRoot?: string): string {
  const root = workspaceRoot || getEffectiveWorkspaceRoot();

  // 1. If path is already absolute and exists on disk
  if (path.isAbsolute(exPath) && fs.existsSync(exPath)) {
    return exPath;
  }

  // 2. Direct resolve with root (e.g. root/exercises/00_baseline/...)
  const directPath = path.resolve(root, exPath);
  if (fs.existsSync(directPath)) {
    return directPath;
  }

  // 3. If root is itself inside 'exercises' or ends with 'exercises', strip leading 'exercises/'
  if (exPath.startsWith('exercises/') || exPath.startsWith('exercises\\')) {
    const stripped = exPath.replace(/^exercises[/\\]/, '');
    const strippedPath = path.resolve(root, stripped);
    if (fs.existsSync(strippedPath)) {
      return strippedPath;
    }
  }

  // 4. If root is in a subfolder, search parent directories
  let cur = root;
  for (let i = 0; i < 6; i++) {
    const candidateFull = path.resolve(cur, exPath);
    if (fs.existsSync(candidateFull)) {
      return candidateFull;
    }

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

  // 5. Check ~/repos/spanglings and ~/spanglings
  const standardLocations = [
    path.join(os.homedir(), 'repos', 'spanglings'),
    path.join(os.homedir(), 'spanglings')
  ];

  for (const loc of standardLocations) {
    const cand = path.resolve(loc, exPath);
    if (fs.existsSync(cand)) {
      return cand;
    }
  }

  // 6. Default fallback
  return directPath;
}
