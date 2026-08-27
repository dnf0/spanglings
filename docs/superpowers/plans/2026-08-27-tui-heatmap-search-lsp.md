# Interactive TUI Heatmap, Fuzzy Search & Editor Diagnostics Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement GitHub-style ANSI activity heatmap in CLI progress & TUI, interactive live fuzzy search (`/`) in Ratatui TUI, and structured editor diagnostic checker (`spanglings check --json`).

---

### Task 1: Activity Heatmap Engine in CLI Progress & TUI

**Files:**
- Modify: `src/core/state.rs`
- Modify: `src/cli/commands/progress.rs`
- Modify: `src/tui/ui.rs`
- Create: `tests/heatmap_tests.rs`

- [ ] **Step 1: Activity calendar builder**
- Calculate daily completions from `AppState.stats` and `AppState.srs` over the last 12 weeks (84 days).
- Render 7 rows (Mon-Sun) x 12 columns with intensity blocks: `·` (0), `░` (1-2), `▒` (3-5), `▓` (6-9), `█` (10+).
- Integrate into `spanglings progress` CLI output and JSON summary (`activity_history`).

- [ ] **Step 2: Activity heatmap widget in TUI**
- Add visual activity streak and completion counter to TUI status header.

- [ ] **Step 3: Add integration tests in `tests/heatmap_tests.rs`**
- [ ] **Step 4: Run `cargo test --test heatmap_tests` and commit**
```bash
git add src/core/state.rs src/cli/commands/progress.rs src/tui/ui.rs tests/heatmap_tests.rs
git commit -m "feat(progress): add ANSI activity calendar heatmap to CLI and TUI"
```

---

### Task 2: Live Fuzzy Search (`/`) in Interactive TUI

**Files:**
- Modify: `src/tui/app.rs`
- Modify: `src/tui/events.rs`
- Modify: `src/tui/ui.rs`
- Modify: `tests/tui_tests.rs`

- [ ] **Step 1: Add Search State to `App`**
- Add fields: `is_searching: bool`, `search_query: String`, `filtered_indices: Vec<usize>`.
- Add methods: `enter_search()`, `exit_search()`, `update_search_filter()`, `current_exercise_ref()`.
- Filter exercises by substring match across `id`, `title`, `topic`, `level`, and `prompt`.

- [ ] **Step 2: Handle `/` and Search Keybindings in `events.rs`**
- In normal mode: `/` activates search.
- In search mode: typing appends to `search_query`, `Backspace` deletes, `Esc` cancels search, `Enter` confirms selection and exits search mode, `Up`/`Down` navigates filtered results.

- [ ] **Step 3: Render Search Modal / Input Bar in `ui.rs`**
- Render interactive search overlay bar when `app.is_searching` is active.

- [ ] **Step 4: Update TUI tests in `tests/tui_tests.rs`**
- [ ] **Step 5: Run `cargo test --test tui_tests` and commit**
```bash
git add src/tui/app.rs src/tui/events.rs src/tui/ui.rs tests/tui_tests.rs
git commit -m "feat(tui): add live interactive search and filtering mode"
```

---

### Task 3: Editor Diagnostic Checker & JSON Streamer (`spanglings check`)

**Files:**
- Create: `src/cli/commands/check.rs`
- Modify: `src/cli/commands/mod.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`
- Create: `tests/check_tests.rs`

- [ ] **Step 1: Implement `spanglings check <file> [--json]`**
- Parses an exercise file or reads from stdin.
- Evaluates solution/answer and produces editor-friendly diagnostic issues (file, line, col, severity: Error/Warning/Info, message, rule ID, fix).

- [ ] **Step 2: Wire `Commands::Check` in CLI dispatcher**
- [ ] **Step 3: Add unit/integration tests in `tests/check_tests.rs`**
- [ ] **Step 4: Run `cargo test` and commit**
```bash
git add src/cli/commands/check.rs src/cli/commands/mod.rs src/cli/mod.rs src/main.rs tests/check_tests.rs
git commit -m "feat(cli): add check command for editor diagnostics and tooling integration"
```

---

### Task 4: Documentation & Final Verification

**Files:**
- Modify: `README.md`
- Modify: `docs/BACKLOG.md`

- [ ] **Step 1: Update README.md with `/` TUI search, activity heatmap, and `spanglings check`**
- [ ] **Step 2: Mark Focus Area 4 completed in `docs/BACKLOG.md`**
- [ ] **Step 3: Run full verification suite (`cargo test`, `cargo clippy`, `cargo fmt`)**
- [ ] **Step 4: Update knowledge graph with `graphify`**
- [ ] **Step 5: Commit and push**
