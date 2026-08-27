# Interactive Onboarding Guided Tour (`spanglings tour`) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Provide an interactive, 6-station guided onboarding tour (`spanglings tour`) with hands-on micro-challenges, first-run TUI prompt, and persistent state tracking.

**Architecture:** A dedicated tour engine in `src/cli/commands/tour.rs` models 6 educational stations with formatted cards, interactive input evaluation, and non-interactive CI safety. `AppState` tracks `tour_completed: bool` to trigger an optional first-run welcome dialog in the TUI without polluting exercise scores.

**Tech Stack:** Rust (2021 edition), `crossterm` (raw mode & keyboard events), `colored` (ANSI styling), `ratatui` (TUI first-run modal), `serde` / `serde_json` (state persistence).

---

### Task 1: Persistent State Onboarding Support

**Files:**
- Modify: `src/core/state.rs:1-120`
- Test: `tests/weakness_profiler_tests.rs` or `tests/tour_tests.rs`

- [ ] **Step 1: Write the failing test for `tour_completed` state**

```rust
// In tests/tour_tests.rs
use spanglings::core::state::AppState;
use tempfile::NamedTempFile;

#[test]
fn test_tour_state_defaults_and_toggle() {
    let mut state = AppState::default();
    assert!(!state.tour_completed);
    state.mark_tour_completed();
    assert!(state.tour_completed);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test tour_tests test_tour_state_defaults_and_toggle`
Expected: FAIL with method `mark_tour_completed` not found.

- [ ] **Step 3: Implement `tour_completed` field and methods in `AppState`**

In `src/core/state.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    #[serde(default)]
    pub completed: HashMap<String, ExerciseProgress>,
    #[serde(default)]
    pub srs_items: HashMap<String, SrsItem>,
    #[serde(default)]
    pub concept_mastery: HashMap<String, ConceptMastery>,
    #[serde(default)]
    pub tour_completed: bool,
    #[serde(default)]
    pub version: u32,
}

impl AppState {
    pub fn mark_tour_completed(&mut self) {
        self.tour_completed = true;
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test tour_tests test_tour_state_defaults_and_toggle`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/core/state.rs tests/tour_tests.rs
git commit --no-gpg-sign -m "feat(state): add tour_completed tracking to AppState"
```

---

### Task 2: Core Tour Engine & 6 Interactive Stations

**Files:**
- Create: `src/cli/commands/tour.rs`
- Modify: `src/cli/commands/mod.rs`
- Test: `tests/tour_tests.rs`

- [ ] **Step 1: Write the failing test for tour stations and non-interactive runner**

In `tests/tour_tests.rs`:
```rust
use spanglings::cli::commands::tour::{get_tour_stations, run_tour};

#[test]
fn test_get_tour_stations_contains_all_six_stations() {
    let stations = get_tour_stations();
    assert_eq!(stations.len(), 6);
    assert_eq!(stations[0].id, "philosophy");
    assert_eq!(stations[1].id, "anatomy_accents");
    assert_eq!(stations[2].id, "diagnostics");
    assert_eq!(stations[3].id, "hints_reference");
    assert_eq!(stations[4].id, "tools_placement");
    assert_eq!(stations[5].id, "workflows");
}

#[test]
fn test_run_tour_non_interactive_skip_challenges() {
    let res = run_tour(true);
    assert!(res.is_ok());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test tour_tests test_get_tour_stations_contains_all_six_stations`
Expected: FAIL with module/functions not found.

- [ ] **Step 3: Implement `src/cli/commands/tour.rs`**

Implement:
- Struct `TourStation`: `id`, `title`, `description`, `bullet_points`, `challenge: Option<TourChallenge>`.
- `get_tour_stations() -> Vec<TourStation>`:
  1. *Welcome & Spanglings Philosophy*
  2. *Anatomy of an Exercise & UTF-8 Accents* (Prompt: `Quiero que tú (venir) ___ a la reunión.` -> `vengas`)
  3. *Concept-Aware Compiler Diagnostics* (Tests `viene` -> shows `E0301` diagnostic card)
  4. *Progressive 3-Tier Hints & Reference Cards* (Simulates `[h]` hints and `spanglings explain subjunctive`)
  5. *Integrated Tools: Verb Conjugator & Placement Test* (Simulates `spanglings conjugate proponer` and CEFR placement)
  6. *Workflow Choices: Watch Mode vs TUI* (Explains `spanglings watch`, `spanglings`, `spanglings review`, `spanglings drill`)
- `run_tour(skip_challenges: bool) -> anyhow::Result<()>`:
  - Supports interactive terminal raw mode (`crossterm`) when TTY is present.
  - Automatically falls back to non-interactive batch output when running without a terminal (CI/testing).
  - Updates `state.mark_tour_completed()` and saves upon completion.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test tour_tests`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/cli/commands/tour.rs src/cli/commands/mod.rs tests/tour_tests.rs
git commit --no-gpg-sign -m "feat(tour): implement 6-station interactive tour engine"
```

---

### Task 3: CLI Command Registration & Global Routing

**Files:**
- Modify: `src/cli/mod.rs:1-80`
- Modify: `src/main.rs:1-60`
- Test: `tests/cli_tests.rs`

- [ ] **Step 1: Write the failing CLI parsing test**

In `tests/cli_tests.rs`:
```rust
let cli_tour = Cli::parse_from(["spanglings", "tour"]);
assert_eq!(
    cli_tour.command,
    Some(Commands::Tour {
        skip_challenges: false
    })
);

let cli_tour_skip = Cli::parse_from(["spanglings", "tour", "--skip-challenges"]);
assert_eq!(
    cli_tour_skip.command,
    Some(Commands::Tour {
        skip_challenges: true
    })
);
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test cli_tests test_cli_parsing_subcommands`
Expected: FAIL with unknown variant `Tour`.

- [ ] **Step 3: Update `src/cli/mod.rs` and `src/main.rs`**

In `src/cli/mod.rs`:
```rust
#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Take an interactive guided onboarding tour of Spanglings
    Tour {
        /// Skip hands-on micro-challenges and view station overviews
        #[arg(long)]
        skip_challenges: bool,
    },
    // ... existing variants
}
```

In `src/main.rs`:
```rust
Some(Commands::Tour { skip_challenges }) => {
    spanglings::cli::commands::tour::run_tour(skip_challenges)?;
}
```

- [ ] **Step 4: Run tests to verify CLI parsing passes**

Run: `cargo test --test cli_tests`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/cli/mod.rs src/main.rs tests/cli_tests.rs
git commit --no-gpg-sign -m "feat(cli): wire spanglings tour subcommand and flags"
```

---

### Task 4: TUI First-Run Welcome Modal & Help Shortcut

**Files:**
- Modify: `src/tui/app.rs`
- Modify: `src/tui/ui.rs`
- Test: `tests/tui_tests.rs`

- [ ] **Step 1: Write test for TUI first-run state detection and help shortcut**

In `tests/tui_tests.rs`:
```rust
#[test]
fn test_tui_first_run_tour_modal_state() {
    let mut state = AppState::default();
    state.tour_completed = false;
    let app = App::new_with_state(state);
    assert!(app.show_first_run_modal);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test tui_tests test_tui_first_run_tour_modal_state`
Expected: FAIL with field not found.

- [ ] **Step 3: Implement first-run modal in `src/tui/app.rs` and `src/tui/ui.rs`**

- In `App`: Add `show_first_run_modal: bool`. Initialized to `true` if `!state.tour_completed && state.completed.is_empty()`.
- If active, pressing `[y]` or `[Enter]` dismisses modal and triggers `run_tour(false)`. Pressing `[n]` or `[Esc]` dismisses modal and sets `state.tour_completed = true`.
- In Help Modal (`?`): Add `T: Launch Onboarding Tour`.
- In `src/tui/ui.rs`: Render clean centered popup modal for first-run welcome dialog with styled borders and prompt.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test tui_tests`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/tui/app.rs src/tui/ui.rs tests/tui_tests.rs
git commit --no-gpg-sign -m "feat(tui): add first-run onboarding tour prompt and help shortcut"
```

---

### Task 5: Documentation & Backlog Completion

**Files:**
- Modify: `README.md`
- Modify: `docs/BACKLOG.md`
- Test: `tests/tour_tests.rs`, `tests/exercise_validity_tests.rs`

- [ ] **Step 1: Update `README.md` with `spanglings tour` documentation**
- Document `spanglings tour` and `spanglings tour --skip-challenges` under Getting Started and Command Reference.

- [ ] **Step 2: Update `docs/BACKLOG.md`**
- Mark Focus Area 15 (SPANG-140) as Completed with subtask checklist.

- [ ] **Step 3: Run full verification suite**

Run:
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```
Expected: All checks passing with 0 warnings.

- [ ] **Step 4: Commit**

```bash
git add README.md docs/BACKLOG.md
git commit --no-gpg-sign -m "docs: document spanglings tour and update backlog"
```
