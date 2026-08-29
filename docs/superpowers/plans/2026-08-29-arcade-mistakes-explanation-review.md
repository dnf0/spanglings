# Arcade Mistakes Explanation & Review Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Provide detailed explanations for each incorrect answer at the end of an arcade session in both the CLI dopamine summary and the interactive TUI Arcade Arena recap modal.

**Architecture:**
1. Extend `ArcadeSessionStats` with `ArcadeMistake` records capturing the question, selected option, correct answer, cue, and pedagogical explanation.
2. Update `evaluate_arcade_choice` to record `ArcadeMistake` on every failed choice.
3. Update `print_arcade_summary` in CLI to render a high-contrast, structured review table of missed questions with explanations.
4. Update `draw_arcade_modal` in TUI to split the session completion layout and render the missed questions breakdown.

**Tech Stack:** Rust (2021 edition), `crossterm`, `colored`, `ratatui` (0.29).

---

### Task 1: Core Mistake Recording & Data Model (`src/cli/commands/arcade.rs`, `tests/cli_arcade_tests.rs`)

**Files:**
- Modify: `src/cli/commands/arcade.rs:15-100`
- Test: `tests/cli_arcade_tests.rs`

- [ ] **Step 1: Write failing test in `tests/cli_arcade_tests.rs`**
```rust
#[test]
fn test_arcade_choice_records_mistakes() {
    let item = ArcadeItem {
        topic: "por-para".to_string(),
        trigger_sentence: "Estudio ____ aprender español.".to_string(),
        prompt_cue: "purpose/goal -> para".to_string(),
        options: vec!["por".to_string(), "para".to_string()],
        correct_index: 1,
        explanation: "Para indicates purpose or goal.".to_string(),
    };

    let mut stats = ArcadeSessionStats::default();

    // Correct choice: no mistake recorded
    let result = evaluate_arcade_choice(&item, 1, 400, &mut stats);
    assert!(result.is_correct);
    assert!(stats.mistakes.is_empty());

    // Incorrect choice: mistake recorded with all details
    let result_fail = evaluate_arcade_choice(&item, 0, 700, &mut stats);
    assert!(!result_fail.is_correct);
    assert_eq!(stats.mistakes.len(), 1);
    let mistake = &stats.mistakes[0];
    assert_eq!(mistake.topic, "por-para");
    assert_eq!(mistake.trigger_sentence, "Estudio ____ aprender español.");
    assert_eq!(mistake.user_answer, "por");
    assert_eq!(mistake.correct_answer, "para");
    assert_eq!(mistake.prompt_cue, "purpose/goal -> para");
    assert_eq!(mistake.explanation, "Para indicates purpose or goal.");
}
```

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test --test cli_arcade_tests test_arcade_choice_records_mistakes`
Expected: FAIL (missing `mistakes` field on `ArcadeSessionStats`).

- [ ] **Step 3: Implement `ArcadeMistake` and update `evaluate_arcade_choice`**
In `src/cli/commands/arcade.rs`:
```rust
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ArcadeMistake {
    pub topic: String,
    pub trigger_sentence: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub prompt_cue: String,
    pub explanation: String,
}

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
Update `evaluate_arcade_choice`:
```rust
    } else {
        stats.incorrect_count += 1;
        stats.current_streak = 0;
        stats.mistakes.push(ArcadeMistake {
            topic: item.topic.clone(),
            trigger_sentence: item.trigger_sentence.clone(),
            user_answer: item.options.get(selected_idx).cloned().unwrap_or_default(),
            correct_answer: item.correct_option().to_string(),
            prompt_cue: item.prompt_cue.clone(),
            explanation: item.explanation.clone(),
        });
```

- [ ] **Step 4: Run tests to verify they pass**
Run: `cargo test --test cli_arcade_tests`
Expected: PASS.

- [ ] **Step 5: Commit changes**
```bash
git add src/cli/commands/arcade.rs tests/cli_arcade_tests.rs
git commit --no-gpg-sign -m "feat(arcade): record detailed mistakes on incorrect arcade choices"
```

---

### Task 2: CLI Summary Formatting & Breakdown (`src/cli/commands/arcade.rs`, `tests/cli_arcade_tests.rs`)

**Files:**
- Modify: `src/cli/commands/arcade.rs:530-630`
- Test: `tests/cli_arcade_tests.rs`

- [ ] **Step 1: Write tests for summary formatting**
In `tests/cli_arcade_tests.rs`:
```rust
#[test]
fn test_arcade_session_stats_json_includes_mistakes() {
    let mut stats = ArcadeSessionStats::default();
    stats.mistakes.push(spanglings::cli::commands::arcade::ArcadeMistake {
        topic: "se-matrix".to_string(),
        trigger_sentence: "Se ____ olvidaron las llaves.".to_string(),
        user_answer: "me".to_string(),
        correct_answer: "le".to_string(),
        prompt_cue: "involuntary dative -> le".to_string(),
        explanation: "Involuntary dative requires 'le' for 3rd person.".to_string(),
    });

    let json = serde_json::to_string(&stats).unwrap();
    assert!(json.contains("mistakes"));
    assert!(json.contains("Involuntary dative"));
}
```

- [ ] **Step 2: Implement CLI mistakes review in `print_arcade_summary`**
In `src/cli/commands/arcade.rs`:
```rust
    // Missed questions review
    if !stats.mistakes.is_empty() {
        println!("\n{}", format!("❌ Review Missed Questions ({}):", stats.mistakes.len()).bold().red());
        println!("{}", "─────────────────────────────────────────────────────────────".dimmed());
        for (i, m) in stats.mistakes.iter().enumerate() {
            println!(
                "  {}. [{}] {}",
                (i + 1).to_string().bold(),
                m.topic.cyan(),
                m.trigger_sentence.white().bold()
            );
            println!(
                "     {} {}",
                "✗ Your answer:   ".red(),
                m.user_answer.red().bold()
            );
            println!(
                "     {} {}",
                "✓ Correct answer:".green(),
                m.correct_answer.green().bold()
            );
            println!(
                "     {} {}\n",
                "💡 Rule / Why:  ".yellow(),
                m.explanation.dimmed()
            );
        }
        println!("{}", "─────────────────────────────────────────────────────────────".dimmed());
    } else if stats.total_answered > 0 {
        println!("\n{}", "✨ Perfect Run! 100% Accuracy — No mistakes to review! ✨".bold().green());
    }
```

- [ ] **Step 3: Run tests and verify**
Run: `cargo test --test cli_arcade_tests`
Expected: PASS.

- [ ] **Step 4: Commit changes**
```bash
git add src/cli/commands/arcade.rs tests/cli_arcade_tests.rs
git commit --no-gpg-sign -m "feat(cli): render structured mistakes review in arcade session summary"
```

---

### Task 3: TUI Modal Recap Review Card & Layout (`src/tui/ui.rs`, `src/tui/app.rs`, `tests/tui_arcade_tests.rs`)

**Files:**
- Modify: `src/tui/ui.rs:2560-2660`
- Test: `tests/tui_arcade_tests.rs`

- [ ] **Step 1: Write failing TUI test in `tests/tui_arcade_tests.rs`**
```rust
#[test]
fn test_tui_arcade_session_recap_renders_mistakes() {
    let mut app = App::new();
    app.enter_arcade_mode(Some(ShowdownPair::PorPara));
    
    // Intentionally submit wrong answers
    for _ in 0..app.arcade_items.len() {
        let wrong_idx = if app.arcade_items[app.arcade_item_idx].correct_index == 0 { 1 } else { 0 };
        app.handle_arcade_key_code(match wrong_idx {
            0 => KeyCode::Char('j'),
            _ => KeyCode::Char('k'),
        });
    }

    assert!(app.arcade_item_idx >= app.arcade_items.len());
    assert!(!app.arcade_stats.mistakes.is_empty());

    let backend = ratatui::backend::TestBackend::new(120, 40);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();
    terminal.draw(|f| draw_arcade_modal(f, &app, f.area())).unwrap();

    let buffer = terminal.backend().buffer();
    let rendered: String = buffer.content().iter().map(|c| c.symbol()).collect();
    assert!(rendered.contains("Review Missed Questions") || rendered.contains("Missed Questions"));
    assert!(rendered.contains("Your answer") || rendered.contains("Your:"));
}
```

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test --test tui_arcade_tests test_tui_arcade_session_recap_renders_mistakes`
Expected: FAIL (TUI recap doesn't render missed questions).

- [ ] **Step 3: Implement TUI recap layout with missed questions review card**
In `src/tui/ui.rs`:
- When `app.arcade_stats.mistakes.is_empty()`: render the compact stats card with green perfection banner.
- When `!app.arcade_stats.mistakes.is_empty()`:
  - Split `inner_area` into:
    - Banner: `Constraint::Length(3)`
    - Stats: `Constraint::Length(7)`
    - Missed review: `Constraint::Min(8)` (renders scrollable list of missed items with sentence, red user answer, green correct answer, and explanation)
    - Footer: `Constraint::Length(3)`

- [ ] **Step 4: Run tests to verify they pass**
Run: `cargo test --test tui_arcade_tests`
Expected: PASS.

- [ ] **Step 5: Commit changes**
```bash
git add src/tui/ui.rs tests/tui_arcade_tests.rs
git commit --no-gpg-sign -m "feat(tui): render missed questions review card in arcade recap modal"
```

---

### Task 4: Complete Verification, Formatting & Release Polish

- [ ] **Step 1: Run full test suite across all 20 packages**
Run: `cargo test`
- [ ] **Step 2: Run clippy and format checks**
Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`
- [ ] **Step 3: Update knowledge graph**
Run: `uvx --from graphifyy graphify update .`
- [ ] **Step 4: PR & Merge into main**
