# Design Specification: Universal Dual-Layer Explanations & Plain-English Mental Models

**Date**: 2026-09-01  
**Status**: APPROVED  
**Objective**: Transform grammatical feedback, cheat sheets, reference cards, and TUI dashboards across Spanglings into a dual-layer learning experience that pairs intuitive plain-English mental models with precise structural grammar rules.

---

## 1. Motivation & Pedagogical Philosophy
Learners struggle when presented solely with abstract linguistic terminology (e.g., `Present subjunctive (tú) of 'saber' uses stem 'sep-': sepas`). While structural rules are essential for production, learners first need an immediate **communicative mental model**—understanding *why* a native speaker reaches for this form in plain English—before applying the morphological formula.

---

## 2. Universal Surfaces & Architecture

### 2.1 Surface 1: Post-Workout Mistakes Breakdown (CLI & TUI Arcade)
* **CLI (`spanglings arcade`)**:
  ```text
    1. [subjunctive] Es fundamental que tú ____ la contraseña maestra.
       ✗ Your answer:    sabes
       ✓ Correct answer: sepas
       💡 Meaning: Expresses a requirement ("need you to know") rather than an established fact.
       📐 Rule:    Present subjunctive (tú) of 'saber' uses irregular stem 'sep-' ➔ sepas.
  ```
* **TUI Arena Recap Modal (`draw_arcade_modal`)**:
  Renders both `💡 Meaning: ...` and `📐 Rule: ...` lines with distinct styling (Yellow & Cyan).

### 2.2 Surface 2: In-Drill & In-Blitz Interactive Prompts (`spanglings drill`, `blitz`)
* When pressing `?` / `hint` or on incorrect submission:
  ```text
  💡 Meaning: Expresses a requirement ("need you to know"); uncertainty triggers virtual mode.
  📐 Rule:    Present subjunctive (tú) of 'saber' uses irregular stem 'sep-' ➔ sepas.
  ```

### 2.3 Surface 3: Pre-Session Grammar Topic Cheat Sheets
* Displayed when launching `spanglings drill <topic>` or `spanglings blitz <topic>`:
  Includes a prominent `🧠 PLAIN-ENGLISH MENTAL MODEL` card at the top before the derivation tables.

### 2.4 Surface 4: Grammar Reference Cards (`spanglings ref <topic>`)
* In `src/core/reference.rs`, each `GrammarConcept` features a dedicated `mental_model: &'static str` detailing:
  - The intuitive everyday analogy (e.g. *Subjunctive = "The Virtual Reality Mode"*, *Por/Para = "Cause/Exchange vs Destination/Deadline"*).
  - English equivalents and cognitive cues.

### 2.5 Surface 5: TUI Concept Mastery Dashboard (`[m]`) & Showdown Selector (`[s]`)
* **Concept Mastery Modal**: Displays the plain-English mental model description alongside SM-2 stability scores, intervals, and repetition counts.
* **Showdown Selector**: Displays intuitive contrast summaries (e.g. `saber (facts / how-to)` vs `conocer (people / places / acquaintance)`).

---

## 3. Data Structures & Serde Backward Compatibility

### 3.1 `ArcadeMistake` in `src/cli/commands/arcade.rs`
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArcadeMistake {
    pub topic: String,
    pub trigger_sentence: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub explanation: String,
    #[serde(default)]
    pub plain_english: String,
}
```

### 3.2 `DrillItem` & `SentenceFrame` in `src/core/generator.rs`
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillItem {
    pub topic: String,
    pub formula_cue: String,
    pub trigger_sentence: String,
    pub target_verb: String,
    pub target_subject: String,
    pub target: String,
    pub explanation: String,
    #[serde(default)]
    pub plain_english: String,
}
```

### 3.3 `GrammarConcept` in `src/core/reference.rs`
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarConcept {
    pub slug: &'static str,
    pub title: &'static str,
    pub gloss: &'static str,
    pub mental_model: &'static str,
    pub keywords: &'static [&'static str],
    pub card: &'static str,
}
```

### 3.4 Fallback Logic
If `plain_english` is empty, formatters cleanly fall back to `💡 Rule / Why: <explanation>`, ensuring full backward compatibility.

---

## 4. Verification & Testing
1. **Unit Tests (`tests/reference_tests.rs`)**:
   - Verify every grammar concept in `CONCEPTS` has a non-empty `mental_model`.
2. **Unit Tests (`tests/cli_arcade_tests.rs`)**:
   - Verify `ArcadeMistake` serialization/deserialization with and without `plain_english`.
   - Verify dual-line formatting in CLI summary output.
3. **Unit Tests (`tests/tui_arcade_tests.rs`)**:
   - Verify TUI recap rendering handles dual-layer mistake cards properly on `TestBackend`.
4. **End-to-End Suite**:
   - `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo fmt --check`.
