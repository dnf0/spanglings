# Diagnostic Placement Test & Level Fast-Track Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Provide an intelligent multi-stage diagnostic placement test and level test-out engine so users can assess their Spanish CEFR level and fast-track past mastered levels without tedious drilling.

**Architecture:**
- `src/core/placement.rs`: Calibration battery, scoring algorithms, and CEFR level calculation.
- `src/core/state.rs`: `EvaluatedLevel` struct and `fast_track_level()` method to batch-complete exercises and seed SM-2 cards.
- `src/cli/commands/test.rs`: Interactive terminal runner for `spanglings test` with `--level`, `--fast-track`, and `--json`.
- `src/tui/`: `PlacementTest` modal inside the TUI with keybinding `T`.
- `src/cli/commands/progress.rs` & `editors/vscode/src/statusBar.ts`: Display verified placement level badges.

---

### Task 1: Diagnostic Placement Engine (`src/core/placement.rs` & `tests/placement_tests.rs`)
- [ ] Create `tests/placement_tests.rs` with test cases:
  - `test_placement_battery_generation_and_filtering`
  - `test_placement_evaluation_accuracy_and_scoring`
  - `test_placement_cefr_level_calculation`
  - `test_fast_track_marks_level_exercises_and_seeds_srs`
- [ ] Implement `src/core/placement.rs`:
  - `PlacementQuestion` struct
  - `PlacementResult` struct
  - `get_placement_battery(level: Option<Level>) -> Vec<PlacementQuestion>`
  - `evaluate_placement_test(battery: &[PlacementQuestion], answers: &[String], accent_mode: AccentMode) -> PlacementResult`
  - `calculate_cefr_level(scores: &HashMap<Level, (usize, usize)>) -> (Level, f64)`
- [ ] Register `pub mod placement;` in `src/core/mod.rs`.
- [ ] Run `cargo test --test placement_tests` and ensure all pass.
- [ ] Commit with `--no-gpg-sign`.

---

### Task 2: State Model & Fast-Track Logic (`src/core/state.rs` & `tests/srs_tests.rs`)
- [ ] Add `EvaluatedLevel` struct in `src/core/state.rs`:
  ```rust
  #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
  pub struct EvaluatedLevel {
      pub level: Level,
      pub score_percent: f64,
      pub evaluated_at: DateTime<Utc>,
  }
  ```
- [ ] Add `pub evaluated_level: Option<EvaluatedLevel>` to `AppState`.
- [ ] Implement `fast_track_level(&mut self, level: Level, exercises: &[Exercise]) -> usize` in `AppState`.
- [ ] Add unit test in `tests/srs_tests.rs` verifying fast-track state mutations and SM-2 initial interval seeding.
- [ ] Verify with `cargo test`.
- [ ] Commit with `--no-gpg-sign`.

---

### Task 3: CLI Command `spanglings test` (`src/cli/commands/test.rs` & `src/cli/mod.rs`)
- [ ] Add `Test` variant to `Commands` in `src/cli/mod.rs`:
  ```rust
  Test {
      /// Test out of a specific level (e.g. B1, B2, C1)
      #[arg(short, long)]
      level: Option<String>,
      /// Automatically fast-track and skip passed levels without prompting
      #[arg(short, long)]
      fast_track: bool,
  }
  ```
- [ ] Create `src/cli/commands/test.rs` implementing `run_test(level, fast_track, json, strict_accents)`.
- [ ] Export in `src/cli/commands/mod.rs` and wire in `src/main.rs`.
- [ ] Add CLI integration test in `tests/test_command_tests.rs`.
- [ ] Verify with `cargo test --test test_command_tests`.
- [ ] Commit with `--no-gpg-sign`.

---

### Task 4: TUI Interactive Diagnostic Modal (`src/tui/`)
- [ ] Add `TuiModal::PlacementTest` to `src/tui/app.rs`.
- [ ] Add placement test state tracking (`PlacementTestState`) in `src/tui/app.rs`.
- [ ] Handle `T` key to open modal in `src/tui/events.rs`, text input, `Enter` to submit answer, `F` to fast-track.
- [ ] Render diagnostic test UI in `src/tui/ui.rs`:
  - Question gauge & progress bar.
  - English context & Spanish cloze prompt.
  - Final results card with score breakdown, CEFR badge, and fast-track action.
- [ ] Add UI tests in `tests/tui_tests.rs`.
- [ ] Verify with `cargo test --test tui_tests`.
- [ ] Commit with `--no-gpg-sign`.

---

### Task 5: Progress Badges & VS Code Status Bar Update
- [ ] Update `src/cli/commands/progress.rs` to display `🎯 Verified Level: [B2 High]` when present.
- [ ] Update `editors/vscode/src/statusBar.ts` to show verified level badge.
- [ ] Run full test suite: `cargo clippy --all-targets -- -D warnings && cargo fmt --check && cargo test`.
- [ ] Update `docs/BACKLOG.md` and `README.md`.
- [ ] Rebuild knowledge graph with `uvx --from graphifyy graphify update .`.
- [ ] Commit with `--no-gpg-sign`.
