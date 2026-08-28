# Rich Context Drill Prompts & Topic Cheat Sheets Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform rapid-fire conjugation drills (`spanglings drill` and `spanglings blitz`) from abstract grammar terminology into clear, contextual trigger sentences with formula cues, pre-session grammar cheat sheets, and live `?`/`hint` support.

**Architecture:** 
- Expand `DrillItem` struct in `src/cli/commands/drill.rs` with `formula_cue`, `trigger_sentence`, `target_verb`, and `target_subject`.
- Implement `get_topic_cheat_sheet(topic: &str) -> Option<&'static str>` providing concise 3-line formula summaries rendered before drills.
- Upgrade prompt display in both `spanglings drill` and `spanglings blitz` to render sentence triggers with blanks and formula badges.
- Add live in-drill hint processing (`?` or `hint`) allowing learners to see step-by-step derivation without penalty.

**Tech Stack:** Rust, Clap CLI, Colored crate, Rand.

---

### Task 1: Data Model Expansion & Topic Cheat Sheets Engine

**Files:**
- Modify: `src/cli/commands/drill.rs`
- Modify: `tests/drill_tests.rs`

- [ ] **Step 1: Write failing unit tests for `get_topic_cheat_sheet` and updated `DrillItem` fields**

In `tests/drill_tests.rs`:
```rust
#[test]
fn test_get_topic_cheat_sheet_all_topics() {
    use spanglings::cli::commands::drill::get_topic_cheat_sheet;

    assert!(get_topic_cheat_sheet("subjunctive").is_some());
    assert!(get_topic_cheat_sheet("preterite").is_some());
    assert!(get_topic_cheat_sheet("por_para").is_some());
    assert!(get_topic_cheat_sheet("ser_estar").is_some());
    assert!(get_topic_cheat_sheet("pronouns").is_some());
    assert!(get_topic_cheat_sheet("prepositions").is_some());
    assert!(get_topic_cheat_sheet("accidental_se").is_some());
    assert!(get_topic_cheat_sheet("all").is_some());

    let subj_sheet = get_topic_cheat_sheet("subjunctive").unwrap();
    assert!(subj_sheet.contains("Subjunctive Formula"));
    assert!(subj_sheet.contains("opposite vowel"));
}

#[test]
fn test_drill_items_rich_fields() {
    use spanglings::cli::commands::drill::get_drill_items;

    let items = get_drill_items(None);
    assert!(!items.is_empty());

    for item in &items {
        assert!(!item.formula_cue.is_empty(), "formula_cue must not be empty for {}", item.target);
        assert!(!item.trigger_sentence.is_empty(), "trigger_sentence must not be empty for {}", item.target);
        assert!(!item.target_verb.is_empty(), "target_verb must not be empty for {}", item.target);
        assert!(!item.target_subject.is_empty(), "target_subject must not be empty for {}", item.target);
        assert!(!item.explanation.is_empty(), "explanation must not be empty for {}", item.target);
    }
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `cargo test --test drill_tests`  
Expected: FAIL due to missing `formula_cue`, `trigger_sentence`, `target_verb`, `target_subject` and `get_topic_cheat_sheet`.

- [ ] **Step 3: Implement `DrillItem` expansion, `get_topic_cheat_sheet`, and enriched question bank**

In `src/cli/commands/drill.rs`:
1. Update `DrillItem`:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrillItem {
    pub topic: &'static str,
    pub formula_cue: &'static str,
    pub trigger_sentence: &'static str,
    pub target_verb: &'static str,
    pub target_subject: &'static str,
    pub target: &'static str,
    pub explanation: &'static str,
}
```
2. Implement `get_topic_cheat_sheet(topic: &str) -> Option<&'static str>`.
3. Enrich all 70+ drill items with contextual trigger sentences, target verbs, target subjects, and formula cues.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test drill_tests`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/cli/commands/drill.rs tests/drill_tests.rs
git commit --no-gpg-sign -m "feat(drill): expand DrillItem model with sentence triggers, formula cues, and topic cheat sheets"
```

---

### Task 2: Interactive Prompt Layout & In-Drill Live Hint Handling

**Files:**
- Modify: `src/cli/commands/drill.rs`
- Modify: `tests/drill_tests.rs`

- [ ] **Step 1: Write tests for prompt formatting and live hint handling**

In `tests/drill_tests.rs`:
```rust
#[test]
fn test_drill_item_prompt_formatting() {
    use spanglings::cli::commands::drill::DrillItem;

    let item = DrillItem {
        topic: "subjunctive",
        formula_cue: "drop -o -> opposite vowel -a",
        trigger_sentence: "Dudo que yo ____ los libros en la mesa.",
        target_verb: "poner",
        target_subject: "yo",
        target: "ponga",
        explanation: "yo pongo -> drop -o -> add -a -> ponga",
    };

    let formatted = item.format_prompt(1, 5);
    assert!(formatted.contains("Q1/5 [Subjunctive | drop -o -> opposite vowel -a]"));
    assert!(formatted.contains("Sentence: \"Dudo que yo ____ los libros en la mesa.\""));
    assert!(formatted.contains("(verb: poner | subject: yo)"));
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `cargo test --test drill_tests`  
Expected: FAIL due to missing `format_prompt` method on `DrillItem`.

- [ ] **Step 3: Implement `format_prompt`, pre-session cheat sheet display, and interactive `?` / `hint` loop**

In `src/cli/commands/drill.rs`:
1. Implement `format_prompt(&self, current: usize, total: usize) -> String` on `DrillItem`.
2. In `run_drill`:
   - Print topic cheat sheet before Q1.
   - Loop on user input for each question:
     - If user input is `"?"` or `"hint"`: print `💡 Hint: {item.explanation}` and re-prompt `Answer > `.
     - Otherwise: evaluate answer, display result, and proceed.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test drill_tests`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/cli/commands/drill.rs tests/drill_tests.rs
git commit --no-gpg-sign -m "feat(drill): add rich prompt formatting and in-drill interactive hint loop"
```

---

### Task 3: Blitz Command Synchronization & Full Integration Verification

**Files:**
- Modify: `src/cli/commands/blitz.rs`
- Modify: `tests/blitz_tests.rs`

- [ ] **Step 1: Update blitz tests for enriched prompt output**

In `tests/blitz_tests.rs`:
Verify that `get_blitz_items` returns enriched drill items with non-empty trigger sentences and formula cues.

- [ ] **Step 2: Run test to verify failure / updates needed**

Run: `cargo test --test blitz_tests`

- [ ] **Step 3: Update `src/cli/commands/blitz.rs` to display rich trigger sentences and formula cues**

Update `run_blitz` rendering to display:
`[{remaining_secs}s remaining | Streak: {streak}] [{item.topic} | {item.formula_cue}]`
`Sentence: "{item.trigger_sentence}" (verb: {item.target_verb} | subject: {item.target_subject})`

- [ ] **Step 4: Run full test suite & clippy**

Run:
```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```
Expected: PASS (100% clean)

- [ ] **Step 5: Commit**

```bash
git add src/cli/commands/blitz.rs tests/blitz_tests.rs
git commit --no-gpg-sign -m "feat(blitz): synchronize blitz question layout with contextual trigger sentences and formula cues"
```
