# Dual-Layer Explanations & Plain-English Mental Models Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Provide intuitive, plain-English mental models alongside precise structural grammar rules across all Spanglings feedback touchpoints: Arcade mistake reviews, TUI arena recap modals, pre-session cheat sheets, reference cards, and interactive drill hints.

**Architecture:** Extend core grammar reference models (`GrammarConcept`) and item models (`DrillItem`, `SentenceFrame`, `ArcadeItem`, `ArcadeMistake`) with a `plain_english` field. Update CLI summary formatters, interactive prompt hint handlers, pre-session cheat sheets, and TUI ratatui modals to render dual-layer explanations with graceful single-line fallback.

**Tech Stack:** Rust (2021 edition), Serde (JSON), Ratatui (TUI), Colored (terminal ANSI).

## Global Constraints
- All commit messages must follow Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`).
- All git commits must be created with `--no-gpg-sign`.
- No direct commits to `main`; work on branch `feat/dual-layer-explanations`.
- Zero compiler warnings (`cargo clippy --all-targets -- -D warnings`).
- 100% test pass rate with backward compatibility for existing serialized states.

---

### Task 1: Core Models & Grammar Mental Model Reference Data

**Files:**
- Modify: `src/core/reference.rs`
- Modify: `src/core/generator.rs`
- Modify: `src/core/arcade.rs`
- Test: `tests/reference_tests.rs`

**Interfaces:**
- Produces:
  - `GrammarConcept.mental_model: &'static str` for all 24 topics in `CONCEPTS`.
  - `DrillItem.plain_english: String`, `SentenceFrame.plain_english: &'static str`.
  - `ShowdownSentence.plain_english: &'static str`, `SpecializedEngineSentence.plain_english: &'static str`, `ArcadeItem.plain_english: String`.
  - `get_mental_model_for_topic(topic: &str) -> Option<&'static str>`.

- [ ] **Step 1: Write failing test in `tests/reference_tests.rs`**
  - Add test `test_all_grammar_concepts_have_rich_mental_models` verifying each topic in `CONCEPTS` has a non-empty `mental_model` with at least 15 characters describing the everyday intuitive analogy.

- [ ] **Step 2: Run test to confirm failure**
  - Run: `cargo test --test reference_tests` (fails because `mental_model` field does not exist yet).

- [ ] **Step 3: Update `src/core/reference.rs`**
  - Add `pub mental_model: &'static str` to `GrammarConcept`.
  - Populate high-clarity plain-English mental models for all 24 grammar topics.
  - Export helper `pub fn get_mental_model_for_topic(slug: &str) -> Option<&'static str>`.
  - Enrich `card` constants with a `🧠 PLAIN-ENGLISH MENTAL MODEL` section.

- [ ] **Step 4: Update `src/core/generator.rs` and `src/core/arcade.rs`**
  - Add `plain_english` field to `SentenceFrame`, `DrillItem`, `ShowdownSentence`, `SpecializedEngineSentence`, and `ArcadeItem`.
  - In `SentenceFrame::render`, substitute slot tokens into `plain_english`.

- [ ] **Step 5: Run tests and verify clean pass**
  - Run: `cargo test --test reference_tests`
  - Run: `cargo test`

- [ ] **Step 6: Commit changes**
  - `git add src/core/reference.rs src/core/generator.rs src/core/arcade.rs tests/reference_tests.rs`
  - `git commit --no-gpg-sign -m "feat(core): add plain-english mental models to grammar concepts and item generators"`

---

### Task 2: CLI & TUI Formatting Integration

**Files:**
- Modify: `src/cli/commands/arcade.rs`
- Modify: `src/cli/commands/drill.rs`
- Modify: `src/cli/commands/blitz.rs`
- Modify: `src/tui/ui.rs`
- Test: `tests/cli_arcade_tests.rs`
- Test: `tests/tui_arcade_tests.rs`

**Interfaces:**
- Produces:
  - `ArcadeMistake.plain_english: String` with `#[serde(default)]`.
  - Formatted dual-layer output in CLI `print_arcade_summary`:
    ```text
    💡 Meaning: <plain_english>
    📐 Rule:    <explanation>
    ```
  - Dual-layer lines in TUI `draw_arcade_modal` recap card.
  - Pre-session cheat sheets with `🧠 PLAIN-ENGLISH MENTAL MODEL` card.
  - In-session hints (`?` / `hint`) showing both Meaning and Rule.

- [ ] **Step 1: Write failing tests in `tests/cli_arcade_tests.rs` & `tests/tui_arcade_tests.rs`**
  - Add `test_arcade_mistake_dual_layer_serde_and_fallback` testing serialization/deserialization with and without `plain_english`.
  - Add `test_tui_arcade_recap_modal_renders_dual_layer_explanations` testing TUI recap rendering on `TestBackend`.

- [ ] **Step 2: Run tests to confirm failure**
  - Run: `cargo test --test cli_arcade_tests --test tui_arcade_tests`

- [ ] **Step 3: Update `src/cli/commands/arcade.rs`**
  - Add `#[serde(default)] pub plain_english: String` to `ArcadeMistake`.
  - In `run_arcade`, populate `plain_english` when recording mistakes.
  - In `print_arcade_summary`, render dual-layer cards (`💡 Meaning:` and `📐 Rule:`), with fallback to `💡 Rule / Why:` if `plain_english` is empty.

- [ ] **Step 4: Update `src/tui/ui.rs`**
  - In `draw_arcade_modal` missed questions section, push both `💡 Meaning:` (yellow) and `📐 Rule:` (cyan/light yellow) lines when `plain_english` is present.

- [ ] **Step 5: Update `src/cli/commands/drill.rs` & `src/cli/commands/blitz.rs`**
  - In `show_topic_cheat_sheet`, print the `🧠 Plain-English Mental Model` section retrieved from `reference::get_mental_model_for_topic`.
  - In interactive prompt loops, update hint/explanation printing to display both Meaning and Rule.

- [ ] **Step 6: Run tests and lint checks**
  - Run: `cargo test --test cli_arcade_tests --test tui_arcade_tests`
  - Run: `cargo test`
  - Run: `cargo clippy --all-targets -- -D warnings`
  - Run: `cargo fmt --check`

- [ ] **Step 7: Commit changes**
  - `git add src/cli/commands/arcade.rs src/cli/commands/drill.rs src/cli/commands/blitz.rs src/tui/ui.rs tests/cli_arcade_tests.rs tests/tui_arcade_tests.rs`
  - `git commit --no-gpg-sign -m "feat(ui): render dual-layer explanations across CLI arcade, TUI recap, and drill cheat sheets"`

---

### Task 3: Full End-to-End Verification, Knowledge Graph Update & Release

**Files:**
- Modify: `graphify-out/`

- [ ] **Step 1: Run full test suite and clippy**
  - `cargo test`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo fmt --check`

- [ ] **Step 2: Update knowledge graph**
  - `uvx --from graphifyy graphify update .`

- [ ] **Step 3: Commit and push**
  - `git add -f graphify-out && git commit --no-gpg-sign -m "chore: update graphify knowledge graph"`
  - `git push origin feat/dual-layer-explanations`

- [ ] **Step 4: Create PR and merge**
  - `gh pr create --title "feat(core): universal dual-layer explanations and plain-english mental models" --body "..."`
  - `gh pr merge --merge --delete-branch`
  - `git checkout main && git pull && cargo install --path . --force`
