# Interactive Terminal TUI (ratatui) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build an interactive terminal UI for `spanglings` using `ratatui` (0.29) and `crossterm` (0.28) that lets users browse, view, and solve exercises in an elegant split-pane visual interface.

**Architecture:** We will implement the TUI using the standard Model-View-Controller/Elm-like pattern of Ratatui. `App` (in `src/tui/app.rs`) holds all application state and pure logic (navigation, editing buffer, evaluating submissions). `ui.rs` contains the pure drawing function (`draw_ui`) which renders visual widgets (including Header, Left Split Prompt/Input card, Right/Bottom Diagnostic/Reference/Hint card, and Footer status/keys guide). `events.rs` manages the main event-polling loop, input mapping, and terminal entry/exit raw state.

**Tech Stack:** Rust, `ratatui` 0.29, `crossterm` 0.28, `anyhow`.

---

## File Structure

- Create: `src/tui/mod.rs` - Entry point and terminal raw mode orchestration.
- Create: `src/tui/app.rs` - Pure application state, tracking user input, cursor, and validation results.
- Create: `src/tui/ui.rs` - UI layout and custom widgets drawing.
- Create: `src/tui/events.rs` - Terminal event-loop and input handler.
- Update: `src/lib.rs` - Export `pub mod tui;`.
- Update: `src/cli/mod.rs` - Add `Commands::Tui` and configure defaults.
- Update: `src/main.rs` - Wire `Commands::Tui` and `None` to `start_tui`.
- Create: `tests/tui_tests.rs` - Test suite for `App` state, input, and submission evaluation.

---

### Task 1: App State Management (`src/tui/app.rs`)

**Files:**
- Create: `src/tui/app.rs`

- [ ] **Step 1: Define the `AppState` enum and `App` struct**
  Define `AppState` to manage different focus/visual modes if needed, but our core requirements require:
  ```rust
  use crate::core::exercise::Exercise;
  use crate::engine::validator::ValidationResult;

  pub enum AppState {
      Editing,
      // You can expand this if you have other states
  }

  pub struct App {
      pub exercises: Vec<Exercise>,
      pub current_index: usize,
      pub input_buffer: String,
      pub cursor_position: usize,
      pub state: AppState,
      pub strict_accents: bool,
      pub show_hint: bool,
      pub show_reference: bool,
      pub last_result: Option<ValidationResult>,
      pub status_message: Option<String>,
      pub should_quit: bool,
  }
  ```

- [ ] **Step 2: Implement initializers and setters**
  Create `App::new(exercises: Vec<Exercise>, strict_accents: bool) -> Self` that initializes fields. If there are exercises, pre-populate the `input_buffer` with the empty slot or an empty string, and set initial states.

- [ ] **Step 3: Implement Navigation (`next_exercise`, `prev_exercise`)**
  Ensure indices wrap around safely, clearing `input_buffer`, `cursor_position`, `last_result`, and `status_message` when navigating.

- [ ] **Step 4: Implement Input Manipulation**
  Write functions:
  - `insert_char(&mut self, c: char)`
  - `delete_char_backwards(&mut self)`
  - `move_cursor_left(&mut self)`
  - `move_cursor_right(&mut self)`
  Make sure `cursor_position` stays within bounds of `input_buffer`.

- [ ] **Step 5: Implement Logic Operations (`submit_current_answer`, `toggle_hint`, `toggle_reference`, `reset`)**
  - `submit_current_answer()`: Validates `input_buffer` against the current exercise using `validate_submission`, updating `last_result` and marking the exercise as done if passed.
  - `toggle_hint()`: Toggles the tiered hints visibility.
  - `toggle_reference()`: Toggles the grammar reference visibility.
  - `reset()`: Resets `input_buffer` to initial (empty), clears `last_result`, and optionally marks current exercise as not done.

---

### Task 2: Pure UI Rendering (`src/tui/ui.rs`)

**Files:**
- Create: `src/tui/ui.rs`

- [ ] **Step 1: Draw the full UI Layout**
  Use `ratatui::layout::{Layout, Constraint, Direction}` to split the terminal:
  - Vertical split: Header (1 line), Main Workspace (fill), Footer (1 line).
  - Main Workspace horizontal split:
    - Left side: Prompt Card (50% height), Interactive Input Card (50% height).
    - Right side (or bottom if space is small): Diagnostics / Reference / Hints Card.

- [ ] **Step 2: Implement Header drawing**
  Draw title "Spanglings", active level e.g. `[B1]`, topic name, exercise index (e.g. `3/24`), and completion indicator.

- [ ] **Step 3: Implement Main Workspace - Prompt Card**
  Draw exercise title, description, English context/prompt, and the prompt sentence with blanks (e.g. `___`).

- [ ] **Step 4: Implement Interactive Input Card**
  Draw input buffer with borders, a visible styled cursor (at `cursor_position`), and validation status.

- [ ] **Step 5: Implement Diagnostics / Reference / Hints Card**
  - If `last_result` is some failure, show a red compiler-style error panel.
  - If `last_result` is some pass, show a green success box + optional accent tip.
  - If `show_reference`, show the grammar reference card for current topic.
  - If `show_hint`, show current exercise hints.

- [ ] **Step 6: Implement Footer keybind guide**
  Show keys: `[Enter] Submit`, `[Tab/N] Next`, `[P] Prev`, `[H] Hint`, `[E] Reference`, `[R] Reset`, `[Esc/Q] Quit`.

---

### Task 3: Event Loop & Raw Mode Setup (`src/tui/events.rs`, `src/tui/mod.rs`)

**Files:**
- Create: `src/tui/events.rs`
- Create: `src/tui/mod.rs`

- [ ] **Step 1: Implement `run_tui_app` event loop in `src/tui/events.rs`**
  Loop polling for key events, translating them into `App` state methods:
  - `Char(c)` -> `insert_char(c)` (unless 'q' or 'e' is pressed in a context where it means quit/reference, or simply map editing letters vs global controls). Note: inside text editing, all letters should enter input buffer, except control keys.
  - `Backspace` -> `delete_char_backwards()`
  - `Enter` -> `submit_current_answer()`
  - `Tab`, 'n', 'N' (when not editing or using modifier/specific keybind) -> `next_exercise()`
  - 'p', 'P' -> `prev_exercise()`
  - `Left` -> `move_cursor_left()`
  - `Right` -> `move_cursor_right()`
  - `Esc` -> `should_quit = true`
  - Control keys: `Ctrl-h` or `F1` or specific key for Hint, `Ctrl-e` for Reference, `Ctrl-r` for Reset to avoid typing interference. Or have an "Editing" vs "Normal" command mode if desired, but we can also just handle standard typing + key combination commands.

- [ ] **Step 2: Implement `start_tui` in `src/tui/mod.rs`**
  - Enable crossterm raw mode.
  - Enter alternate screen.
  - Build `ratatui::Terminal`.
  - Load all exercises from curriculum via `find_all_exercises`.
  - Instantiate `App` and run `run_tui_app`.
  - Ensure standard terminal restore occurs safely on drop / return via clean alternate screen exit.

---

### Task 4: CLI Integration (`src/lib.rs`, `src/cli/mod.rs`, `src/main.rs`)

**Files:**
- Modify: `src/lib.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Export `pub mod tui;` in `src/lib.rs`**
- [ ] **Step 2: Add `Commands::Tui` sub-command in `src/cli/mod.rs`**
- [ ] **Step 3: Wire default launch and `Commands::Tui` in `src/main.rs`**

---

### Task 5: Unit Tests (`tests/tui_tests.rs`)

**Files:**
- Create: `tests/tui_tests.rs`

- [ ] **Step 1: Write TUI App unit tests**
  Implement test cases matching the requested names:
  - `test_app_initialization_and_navigation`
  - `test_app_input_editing`
  - `test_app_submission_evaluation_passed_and_failed`
  - `test_app_toggles_hints_and_reference`

- [ ] **Step 2: Run verification and fix clippy/fmt**
  Verify all tests pass with cargo test. Run clippy and cargo fmt checks.
