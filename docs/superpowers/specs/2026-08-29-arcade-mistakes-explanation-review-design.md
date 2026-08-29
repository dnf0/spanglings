# Design Specification: Arcade Mode End-of-Session Mistakes Explanation & Review

**Date:** 2026-08-29  
**Status:** Approved  
**Target:** CLI (`src/cli/commands/arcade.rs`) and TUI (`src/tui/ui.rs`, `src/tui/app.rs`)

---

## 1. Problem Statement & User Need
In Spanglings' rapid-fire Arcade Mode (`spanglings arcade` & TUI modal `[x]`), users answer single-key questions at speeds under 800ms. While real-time 200ms flashes indicate correctness, users with ADHD or fast typing reflexes need a dedicated, stress-free **review breakdown at the end of the session** to study why each incorrect answer was wrong and reinforce the underlying grammatical rules.

---

## 2. Architecture & Data Model

### `ArcadeMistake` Struct (`src/cli/commands/arcade.rs` & `src/core/arcade.rs`)
```rust
/// Detailed record of a question answered incorrectly during an arcade session.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ArcadeMistake {
    pub topic: String,
    pub trigger_sentence: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub prompt_cue: String,
    pub explanation: String,
}
```

### `ArcadeSessionStats` Extension
```rust
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ArcadeSessionStats {
    pub total_answered: usize,
    pub correct_count: usize,
    pub incorrect_count: usize,
    pub current_streak: usize,
    pub best_streak: usize,
    pub score: u64,
    pub total_time_ms: u128,
    #[serde(default)]
    pub mistakes: Vec<ArcadeMistake>,
}
```

### Evaluation Logic (`evaluate_arcade_choice`)
When `item.is_correct(selected_idx)` evaluates to `false`:
- Construct `ArcadeMistake`:
  - `topic`: `item.topic.clone()`
  - `trigger_sentence`: `item.trigger_sentence.clone()`
  - `user_answer`: `item.options.get(selected_idx).cloned().unwrap_or_default()`
  - `correct_answer`: `item.correct_option().to_string()`
  - `prompt_cue`: `item.prompt_cue.clone()`
  - `explanation`: `item.explanation.clone()`
- Push to `stats.mistakes`.

---

## 3. Presentation Layer

### CLI Session Summary (`print_arcade_summary`)
When session completes:
1. Print the dopamine score card, accuracy, best streak, speed, and concept deltas.
2. If `stats.mistakes.is_empty()`:
   - Print `✨ Perfect Run! 100% Accuracy — No mistakes to review! ✨`.
3. If `!stats.mistakes.is_empty()`:
   - Print formatted review section:
     ```text
     ❌ Review Missed Questions (2):
     ─────────────────────────────────────────────────────────────
     1. [por-para] Trabajo ____ ganar dinero.
        ✗ Your answer:    por
        ✓ Correct answer: para
        💡 Rule / Why:   "Para" expresses purpose or objective ("in order to").
     ─────────────────────────────────────────────────────────────
     2. [regimen] Sueño ____ viajar por el mundo.
        ✗ Your answer:    de
        ✓ Correct answer: con
        💡 Rule / Why:   "Soñar con" is the fixed prepositional bond for "to dream about".
     ─────────────────────────────────────────────────────────────
     ```
4. If `--json` is supplied, output serialized JSON containing the `mistakes` array.

### TUI Arcade Arena Modal (`draw_arcade_modal`)
When `app.arcade_item_idx >= app.arcade_items.len()`:
1. If `app.arcade_stats.mistakes.is_empty()`:
   - Render single full stats block with `✨ Perfect Accuracy! ✨`.
2. If `!app.arcade_stats.mistakes.is_empty()`:
   - Split `inner_area` into:
     - Top: Header banner (`Length(3)`)
     - Middle-Top: Session stats card (`Length(7)`)
     - Middle-Bottom: `Review Missed Questions (N)` block (`Min(6)`) with structured lines for each missed item: prompt, user answer (red), correct answer (green), and rule explanation (cyan/yellow).
     - Bottom: Footer shortcuts (`Length(3)`).

---

## 4. Test Strategy
1. **Unit Tests (`tests/cli_arcade_tests.rs`)**:
   - Verify `evaluate_arcade_choice` records `ArcadeMistake` when answering incorrectly.
   - Verify `evaluate_arcade_choice` does not append to `mistakes` on correct answers.
   - Verify JSON serialization and deserialization of `ArcadeSessionStats` with `mistakes`.
2. **TUI Tests (`tests/tui_arcade_tests.rs`)**:
   - Complete an arcade session in `App` with intentional incorrect selections.
   - Render to `ratatui::backend::TestBackend` and assert that the buffer contains `"Review Missed Questions"`, user answer, correct answer, and explanation string.
