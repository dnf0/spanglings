# Rapid Single-Key ADHD Arcade & Showdown Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a zero-friction, single-key rapid arcade drilling engine featuring Binary Grammatical Showdowns (`j`/`k`) and 4-Choice Rapid Clozes (`1`/`2`/`3`/`4`) with instant auto-advancing, combo multipliers, speed bonuses, and dual CLI (`spanglings arcade`) and TUI (`[x]`/`[d]`) interfaces.

**Architecture:** A standalone core module `src/core/arcade.rs` generates binary contrast showdowns and 4-choice items with algorithmic distractor synthesis across all 24 concepts. The CLI command `src/cli/commands/arcade.rs` uses raw terminal event loops to capture single keypresses with zero Enter keys, triggering 150ms visual feedback flashes and updating SM-2 mastery state in real time. The TUI (`src/tui/arcade_modal.rs`, `ui.rs`, `app.rs`) embeds a responsive full-screen arcade overlay.

**Tech Stack:** Rust (1.80+), Ratatui, Crossterm, Clap v4, Serde, Rand, Std::time::Instant.

---

### Task 1: Core Arcade & Showdown Engine (`src/core/arcade.rs`)

**Files:**
- Create: `src/core/arcade.rs`
- Modify: `src/core/mod.rs`
- Modify: `src/lib.rs` (if necessary)
- Test: `tests/arcade_tests.rs`

- [ ] **Step 1: Write the failing tests in `tests/arcade_tests.rs`**

```rust
use spanglings::core::arcade::{
    generate_4choice_items, generate_showdown_items, list_showdown_pairs, ArcadeItem, ShowdownPair,
};
use spanglings::core::reference::list_grammar_concepts;

#[test]
fn test_all_8_showdown_pairs_generate_valid_items() {
    let pairs = list_showdown_pairs();
    assert_eq!(pairs.len(), 8);
    for pair in pairs {
        let items = generate_showdown_items(pair, 10);
        assert_eq!(items.len(), 10, "Should generate 10 items for pair {:?}", pair);
        for item in &items {
            assert_eq!(item.options.len(), 2, "Showdown must have exactly 2 options");
            assert!(item.correct_index < 2);
            assert!(!item.trigger_sentence.is_empty());
            assert!(!item.explanation.is_empty());
            assert_ne!(item.options[0], item.options[1], "Options must be distinct");
        }
    }
}

#[test]
fn test_4choice_generator_across_all_24_concepts() {
    let concepts = list_grammar_concepts();
    for concept in concepts {
        let items = generate_4choice_items(concept.slug, 5);
        assert_eq!(items.len(), 5, "Should generate 5 choice items for {}", concept.slug);
        for item in &items {
            assert_eq!(item.options.len(), 4, "Choice items must have exactly 4 options");
            assert!(item.correct_index < 4);
            assert!(!item.trigger_sentence.is_empty());
            assert!(!item.explanation.is_empty());
            // Assert all 4 options are distinct
            let mut set = std::collections::HashSet::new();
            for opt in &item.options {
                set.insert(opt.clone());
            }
            assert_eq!(set.len(), 4, "All 4 options must be unique: {:?}", item.options);
        }
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test arcade_tests`
Expected: FAIL with "cannot find module or file `arcade`"

- [ ] **Step 3: Implement `src/core/arcade.rs`**

Implement `ShowdownPair`, `ArcadeItem`, `list_showdown_pairs()`, `generate_showdown_items()`, and `generate_4choice_items()` with algorithmic distractor generator. Export `pub mod arcade;` in `src/core/mod.rs`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test arcade_tests`
Expected: PASS (2/2 passed)

- [ ] **Step 5: Commit**

```bash
git add src/core/arcade.rs src/core/mod.rs tests/arcade_tests.rs
git commit --no-gpg-sign -m "feat(core): implement rapid single-key arcade and showdown generator"
```

---

### Task 2: CLI Arcade Command (`src/cli/commands/arcade.rs` & CLI Dispatch)

**Files:**
- Create: `src/cli/commands/arcade.rs`
- Modify: `src/cli/commands/mod.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`
- Test: `tests/cli_arcade_tests.rs`

- [ ] **Step 1: Write the failing tests in `tests/cli_arcade_tests.rs`**

```rust
use spanglings::cli::commands::arcade::{evaluate_arcade_choice, ArcadeSessionStats};
use spanglings::core::arcade::ArcadeItem;

#[test]
fn test_arcade_choice_evaluation_and_scoring() {
    let item = ArcadeItem {
        topic: "por-para".to_string(),
        trigger_sentence: "Estudio ____ aprender español.".to_string(),
        prompt_cue: "purpose/goal -> para".to_string(),
        options: vec!["por".to_string(), "para".to_string()],
        correct_index: 1,
        explanation: "Para indicates purpose or goal.".to_string(),
    };

    let mut stats = ArcadeSessionStats::default();
    
    // Correct selection with <800ms speed
    let result = evaluate_arcade_choice(&item, 1, 500, &mut stats);
    assert!(result.is_correct);
    assert_eq!(stats.current_streak, 1);
    assert!(stats.score >= 200);

    // Incorrect selection
    let result2 = evaluate_arcade_choice(&item, 0, 1200, &mut stats);
    assert!(!result2.is_correct);
    assert_eq!(stats.current_streak, 0);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test cli_arcade_tests`
Expected: FAIL

- [ ] **Step 3: Implement `src/cli/commands/arcade.rs` & CLI args**

1. Define `Commands::Arcade` in `src/cli/mod.rs`:
   - `showdown: Option<String>`
   - `concept: Option<String>`
   - `weak: bool`
   - `count: Option<usize>`
   - `sound: bool`
2. Implement `run_arcade(...)` in `src/cli/commands/arcade.rs`:
   - Crossterm raw mode single-key event listener.
   - Immediate key resolution (`1`-`4`, `j`/`k`, `←`/`→`, `q`/`Esc`).
   - Visual result rendering with color flashes, streak combo multiplier badges, and instant auto-advancing.
   - Live SM-2 concept mastery updates and end-of-session summary.
3. Wire into `src/main.rs`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test cli_arcade_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/cli/commands/arcade.rs src/cli/commands/mod.rs src/cli/mod.rs src/main.rs tests/cli_arcade_tests.rs
git commit --no-gpg-sign -m "feat(cli): implement single-key rapid arcade drill runner"
```

---

### Task 3: In-TUI Interactive Arcade Arena Modal (`[x]` / `[d]`)

**Files:**
- Create: `src/tui/arcade_modal.rs` (or extend `src/tui/ui.rs`)
- Modify: `src/tui/app.rs`
- Modify: `src/tui/events.rs`
- Modify: `src/tui/ui.rs`
- Test: `tests/tui_arcade_tests.rs`

- [ ] **Step 1: Write the failing tests in `tests/tui_arcade_tests.rs`**

```rust
use spanglings::core::exercise::Exercise;
use spanglings::core::state::AppState;
use spanglings::tui::app::App;
use ratatui::backend::TestBackend;
use ratatui::Terminal;

#[test]
fn test_tui_arcade_modal_lifecycle_and_single_key_navigation() {
    let app = App::new_with_state(vec![], false, AppState::default());
    let mut app = app;
    
    // Trigger arcade mode via 'x'
    app.enter_arcade_mode(None);
    assert!(app.show_arcade_modal);
    assert!(!app.arcade_items.is_empty());
    
    // Answer question with '1' or 'j'
    let initial_score = app.arcade_stats.score;
    app.handle_arcade_key('1');
    assert!(app.arcade_stats.score > initial_score || app.arcade_stats.incorrect_count > 0);
    
    // Close modal with 'q'
    app.handle_arcade_key('q');
    assert!(!app.show_arcade_modal);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test tui_arcade_tests`
Expected: FAIL

- [ ] **Step 3: Implement TUI Arcade modal state & drawing**

1. In `src/tui/app.rs`:
   - Add `show_arcade_modal: bool`, `arcade_items: Vec<ArcadeItem>`, `arcade_item_idx: usize`, `arcade_stats: ArcadeSessionStats`, `arcade_flash: Option<(bool, Instant)>`.
   - Implement `enter_arcade_mode`, `exit_arcade_mode`, `handle_arcade_key`.
2. In `src/tui/events.rs`:
   - Intercept key events when `show_arcade_modal` is true.
   - Bind `[x]` and `[d]` in editing mode to launch Arcade Arena.
3. In `src/tui/ui.rs`:
   - Implement `draw_arcade_modal(f, app, area)`:
     - Combo flame banner `🔥 5x STREAK! | Score: 1,250 XP (+200 Speed Bonus)`.
     - Big bold highlighted sentence cloze.
     - Large styled choice buttons: `[ J ] Por` vs `[ K ] Para` or 4-box layout.
     - 200ms green/red flash border overlay.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test tui_arcade_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/tui/app.rs src/tui/events.rs src/tui/ui.rs tests/tui_arcade_tests.rs
git commit --no-gpg-sign -m "feat(tui): implement full-screen interactive Arcade Arena modal"
```

---

### Task 4: End-to-End Verification & Knowledge Graph Update

**Files:**
- Check all modified files
- Update Knowledge Graph: `uvx --from graphifyy graphify update .`

- [ ] **Step 1: Run full test suite**

Run: `cargo test`
Expected: 100% tests passing across all test suites.

- [ ] **Step 2: Run clippy and format check**

Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`
Expected: 0 warnings, clean formatting.

- [ ] **Step 3: Update knowledge graph**

Run: `uvx --from graphifyy graphify update .`

- [ ] **Step 4: Commit & Verification Handoff**

```bash
git add -A && git commit --no-gpg-sign -m "chore: complete rapid arcade and showdown engine verification"
```
