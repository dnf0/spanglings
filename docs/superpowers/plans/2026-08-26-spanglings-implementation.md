# Spanglings Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `spanglings`, an ultra-fast developer-grade Rust CLI & TUI learning platform for intermediate-to-advanced Spanish (B1-C1 + Baseline drills), featuring compiler-grade diagnostics, multi-modal exercises, SM-2 spaced repetition, and dual execution modes (interactive TUI & headless IDE file-watcher).

**Architecture:** A modular Rust application divided into `core` (exercise parsing, curriculum metadata, state persistence, SRS scheduling), `engine` (Unicode normalization, forgiving accent diagnostics, grammar rule pattern matcher, ANSI compiler formatter), `watcher` (debounced file system watcher with `notify`), `tui` (interactive terminal app with `ratatui`/`crossterm`), `cli` (`clap` command routing), and `exercises` (100+ markdown exercises across 22 tracks).

**Tech Stack:** Rust (edition 2021), `clap` (v4), `ratatui` (v0.29), `crossterm` (v0.28), `notify` (v7), `unicode-normalization` (v0.1), `serde` / `serde_json` (v1.0), `chrono` (v0.4), `colored` (v2), `thiserror` (v2), `anyhow` (v1).

---

### Task 1: Project Scaffolding & Cargo Setup

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `.gitignore`

- [ ] **Step 1: Write Cargo.toml with dependencies**

```toml
[package]
name = "spanglings"
version = "0.1.0"
edition = "2021"
authors = ["Daniel Fisher"]
description = "Developer-grade CLI and TUI for mastering B1-C1 Spanish"

[dependencies]
clap = { version = "4.5", features = ["derive", "cargo"] }
ratatui = { version = "0.29", features = ["all-widgets"] }
crossterm = { version = "0.28", features = ["event-stream"] }
notify = "7.0"
notify-debouncer-mini = "0.5"
unicode-normalization = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
colored = "2.1"
thiserror = "2.0"
anyhow = "1.0"
regex = "1.10"
dirs = "5.0"

[dev-dependencies]
tempfile = "3.10"
```

- [ ] **Step 2: Create .gitignore**

```gitignore
/target
Cargo.lock
.spanglings_state.json
.DS_Store
```

- [ ] **Step 3: Create initial minimal main.rs**

```rust
fn main() {
    println!("¡Hola! Bienvenido a Spanglings.");
}
```

- [ ] **Step 4: Run `cargo check` to verify dependencies resolve**

Run: `cargo check`
Expected: Compiles with 0 errors.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml .gitignore src/main.rs
git commit -m "chore: initialize spanglings cargo package with dependencies"
```

---

### Task 2: Core Domain Models & Markdown Exercise Parser

**Files:**
- Create: `src/core/mod.rs`
- Create: `src/core/exercise.rs`
- Create: `src/core/curriculum.rs`
- Create: `tests/exercise_parser_tests.rs`

- [ ] **Step 1: Write failing parser test in tests/exercise_parser_tests.rs**

```rust
use spanglings::core::exercise::{Exercise, ExerciseType, Level};

#[test]
fn test_parse_valid_markdown_exercise() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01: Verbs of Influence
<!-- id: b1_subjunctive_01 | level: B1 | topic: subjunctive_weirdo | type: cloze -->

> **Grammar Rule**: Verbs of wishing/influence require subjunctive with subject change.

### Context
English: "I want you to come."

### Exercise
Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->

<!-- ALTERNATIVES
vengas tú
-->

<!-- DIAGNOSTIC_RULES
pattern: "viene" | code: "E0301" | message: "Expected Subjunctive ('vengas'), found Indicative ('viene')."
-->

<!-- HINTS
Tier 1: Check the main verb.
Tier 2: Irregular root 'veng-'.
Tier 3: Add -as -> 'vengas'.
-->
"#;

    let exercise = Exercise::from_markdown("exercises/03_subjunctive/subjunctive_01.md", content)
        .expect("Failed to parse exercise");

    assert_eq!(exercise.id, "b1_subjunctive_01");
    assert_eq!(exercise.level, Level::B1);
    assert_eq!(exercise.exercise_type, ExerciseType::Cloze);
    assert_eq!(exercise.is_done, false);
    assert_eq!(exercise.solution, "vengas");
    assert_eq!(exercise.alternatives, vec!["vengas tú"]);
    assert_eq!(exercise.hints.len(), 3);
    assert_eq!(exercise.diagnostic_rules.len(), 1);
    assert_eq!(exercise.diagnostic_rules[0].code, "E0301");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test exercise_parser_tests`
Expected: FAIL with module/type resolution error.

- [ ] **Step 3: Implement src/core/curriculum.rs and src/core/exercise.rs**

In `src/core/curriculum.rs`:
```rust
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Level {
    Baseline,
    B1,
    B2,
    C1,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::Baseline => write!(f, "Baseline"),
            Level::B1 => write!(f, "B1"),
            Level::B2 => write!(f, "B2"),
            Level::C1 => write!(f, "C1"),
        }
    }
}

impl std::str::FromStr for Level {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "baseline" | "a1" | "a2" => Ok(Level::Baseline),
            "b1" => Ok(Level::B1),
            "b2" => Ok(Level::B2),
            "c1" => Ok(Level::C1),
            _ => Err(anyhow::anyhow!("Unknown CEFR level: {}", s)),
        }
    }
}
```

In `src/core/exercise.rs`:
```rust
use crate::core::curriculum::Level;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseType {
    Cloze,
    Transformation,
    BugFix,
    Translation,
}

impl std::str::FromStr for ExerciseType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cloze" => Ok(ExerciseType::Cloze),
            "transformation" | "rewrite" => Ok(ExerciseType::Transformation),
            "bugfix" | "bug_fix" | "review" => Ok(ExerciseType::BugFix),
            "translation" => Ok(ExerciseType::Translation),
            _ => Ok(ExerciseType::Cloze),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticRule {
    pub pattern: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub path: PathBuf,
    pub title: String,
    pub level: Level,
    pub topic: String,
    pub exercise_type: ExerciseType,
    pub is_done: bool,
    pub rule_summary: String,
    pub prompt_text: String,
    pub solution: String,
    pub alternatives: Vec<String>,
    pub diagnostic_rules: Vec<DiagnosticRule>,
    pub hints: Vec<String>,
}

impl Exercise {
    pub fn from_markdown<P: AsRef<Path>>(path: P, content: &str) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let is_done = !content.contains("<!-- I AM NOT DONE -->");
        
        let mut title = String::new();
        let mut id = path_buf.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        let mut level = Level::B1;
        let mut topic = "general".to_string();
        let mut exercise_type = ExerciseType::Cloze;
        let mut rule_summary = String::new();
        let mut solution = String::new();
        let mut alternatives = Vec::new();
        let mut hints = Vec::new();
        let mut diagnostic_rules = Vec::new();
        let mut prompt_lines = Vec::new();

        let mut in_solution = false;
        let mut in_alternatives = false;
        let mut in_hints = false;
        let mut in_diagnostic_rules = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed == "<!-- SOLUTION" {
                in_solution = true;
                continue;
            } else if trimmed == "<!-- ALTERNATIVES" {
                in_alternatives = true;
                continue;
            } else if trimmed == "<!-- HINTS" {
                in_hints = true;
                continue;
            } else if trimmed == "<!-- DIAGNOSTIC_RULES" {
                in_diagnostic_rules = true;
                continue;
            } else if trimmed == "-->" {
                in_solution = false;
                in_alternatives = false;
                in_hints = false;
                in_diagnostic_rules = false;
                continue;
            }

            if in_solution {
                if !trimmed.is_empty() {
                    solution = trimmed.to_string();
                }
            } else if in_alternatives {
                if !trimmed.is_empty() {
                    alternatives.push(trimmed.to_string());
                }
            } else if in_hints {
                if !trimmed.is_empty() {
                    let cleaned = if let Some(stripped) = trimmed.strip_prefix("Tier ") {
                        stripped.splitn(2, ':').nth(1).unwrap_or(stripped).trim().to_string()
                    } else {
                        trimmed.to_string()
                    };
                    hints.push(cleaned);
                }
            } else if in_diagnostic_rules {
                if !trimmed.is_empty() && trimmed.contains('|') {
                    // pattern: "..." | code: "..." | message: "..."
                    let parts: Vec<&str> = trimmed.split('|').collect();
                    let mut pat = String::new();
                    let mut code = String::new();
                    let mut msg = String::new();
                    for part in parts {
                        let kv: Vec<&str> = part.splitn(2, ':').collect();
                        if kv.len() == 2 {
                            let k = kv[0].trim();
                            let v = kv[1].trim().trim_matches('"').trim_matches('\'');
                            match k {
                                "pattern" => pat = v.to_string(),
                                "code" => code = v.to_string(),
                                "message" => msg = v.to_string(),
                                _ => {}
                            }
                        }
                    }
                    if !pat.is_empty() {
                        diagnostic_rules.push(DiagnosticRule {
                            pattern: pat,
                            code,
                            message: msg,
                        });
                    }
                }
            } else if trimmed.starts_with("# ") {
                title = trimmed.trim_start_matches("# ").trim().to_string();
            } else if trimmed.starts_with("<!-- id:") {
                let meta = trimmed.trim_start_matches("<!--").trim_end_matches("-->").trim();
                for part in meta.split('|') {
                    let kv: Vec<&str> = part.splitn(2, ':').collect();
                    if kv.len() == 2 {
                        let k = kv[0].trim();
                        let v = kv[1].trim();
                        match k {
                            "id" => id = v.to_string(),
                            "level" => if let Ok(l) = v.parse::<Level>() { level = l; },
                            "topic" => topic = v.to_string(),
                            "type" => if let Ok(t) = v.parse::<ExerciseType>() { exercise_type = t; },
                            _ => {}
                        }
                    }
                }
            } else if trimmed.starts_with("> **Grammar Rule**:") || trimmed.starts_with("> Rule:") {
                rule_summary = trimmed.trim_start_matches('>').trim().to_string();
            } else if !trimmed.starts_with("<!--") {
                prompt_lines.push(line);
            }
        }

        let prompt_text = prompt_lines.join("\n").trim().to_string();

        Ok(Self {
            id,
            path: path_buf,
            title,
            level,
            topic,
            exercise_type,
            is_done,
            rule_summary,
            prompt_text,
            solution,
            alternatives,
            diagnostic_rules,
            hints,
        })
    }
}
```

In `src/core/mod.rs`:
```rust
pub mod curriculum;
pub mod exercise;
pub mod srs;
pub mod state;
pub mod reference;
```

In `src/lib.rs`:
```rust
pub mod core;
pub mod engine;
pub mod cli;
pub mod watcher;
pub mod tui;
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test exercise_parser_tests`
Expected: PASS with 1 test passed.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs src/core/ Cargo.toml tests/exercise_parser_tests.rs
git commit -m "feat(core): add domain models and markdown exercise parser"
```

---

### Task 3: Normalization & Smart Accent Warning Engine

**Files:**
- Create: `src/engine/mod.rs`
- Create: `src/engine/normalizer.rs`
- Create: `src/engine/accents.rs`
- Create: `tests/normalizer_tests.rs`

- [ ] **Step 1: Write failing tests in tests/normalizer_tests.rs**

```rust
use spanglings::engine::normalizer::{normalize_spanish_text, strip_accents};
use spanglings::engine::accents::{AccentMode, evaluate_accents, AccentEvaluation};

#[test]
fn test_normalize_spanish_punctuation() {
    assert_eq!(normalize_spanish_text("¿Cómo estás?"), "cómo estás");
    assert_eq!(normalize_spanish_text("¡Hola, mundo!"), "hola mundo");
    assert_eq!(normalize_spanish_text("   vengas tú...  "), "vengas tú");
}

#[test]
fn test_strip_accents() {
    assert_eq!(strip_accents("gustaría"), "gustaria");
    assert_eq!(strip_accents("explicándomelo"), "explicandomelo");
    assert_eq!(strip_accents("año"), "ano"); // normalizes for comparison
}

#[test]
fn test_forgiving_accent_evaluation_gives_advice() {
    let eval = evaluate_accents("gustaria", "gustaría", AccentMode::Forgiving);
    assert_eq!(eval.is_pass, true);
    assert_eq!(eval.warning_notice.is_some(), true);
    assert!(eval.warning_notice.unwrap().contains("gustaría"));
}

#[test]
fn test_strict_accent_fails_on_missing_accent() {
    let eval = evaluate_accents("gustaria", "gustaría", AccentMode::Strict);
    assert_eq!(eval.is_pass, false);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test normalizer_tests`
Expected: FAIL with unresolved modules.

- [ ] **Step 3: Implement src/engine/normalizer.rs and src/engine/accents.rs**

In `src/engine/normalizer.rs`:
```rust
use unicode_normalization::UnicodeNormalization;

pub fn strip_accents(input: &str) -> String {
    input
        .nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect::<String>()
        .nfc()
        .collect::<String>()
        .replace('ñ', "n")
        .replace('Ñ', "N")
}

pub fn normalize_spanish_text(input: &str) -> String {
    let mut cleaned = input.trim().to_lowercase();
    // Remove opening inverted punctuation and closing punctuation
    cleaned = cleaned
        .replace(['¿', '¡', '?', '!', '.', ',', ';', ':', '«', '»', '"', '\''], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    cleaned
}
```

In `src/engine/accents.rs`:
```rust
use crate::engine::normalizer::{normalize_spanish_text, strip_accents};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccentMode {
    Forgiving,
    Strict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccentEvaluation {
    pub is_pass: bool,
    pub warning_notice: Option<String>,
}

pub fn evaluate_accents(user_input: &str, target_solution: &str, mode: AccentMode) -> AccentEvaluation {
    let norm_user = normalize_spanish_text(user_input);
    let norm_target = normalize_spanish_text(target_solution);

    // Exact normalized match including accents
    if norm_user == norm_target {
        return AccentEvaluation {
            is_pass: true,
            warning_notice: None,
        };
    }

    // Check if stripped forms match
    let stripped_user = strip_accents(&norm_user);
    let stripped_target = strip_accents(&norm_target);

    if stripped_user == stripped_target {
        match mode {
            AccentMode::Forgiving => {
                let notice = format!(
                    "💡 Notice: Passed! Accent Tip: '{}' carries an accent mark in standard Spanish orthography.",
                    target_solution.trim()
                );
                AccentEvaluation {
                    is_pass: true,
                    warning_notice: Some(notice),
                }
            }
            AccentMode::Strict => AccentEvaluation {
                is_pass: false,
                warning_notice: Some(format!(
                    "Missing or incorrect accent mark. Expected '{}'.",
                    target_solution.trim()
                )),
            },
        }
    } else {
        AccentEvaluation {
            is_pass: false,
            warning_notice: None,
        }
    }
}
```

In `src/engine/mod.rs`:
```rust
pub mod accents;
pub mod diagnostics;
pub mod normalizer;
pub mod rules;
pub mod validator;
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test normalizer_tests`
Expected: PASS with 4 tests passed.

- [ ] **Step 5: Commit**

```bash
git add src/engine/ tests/normalizer_tests.rs
git commit -m "feat(engine): add normalizer and smart forgiving accent evaluation"
```

---

### Task 4: Grammar Diagnostics & Compiler-Style Error Formatter

**Files:**
- Create: `src/engine/diagnostics.rs`
- Create: `src/engine/rules.rs`
- Create: `src/engine/validator.rs`
- Create: `tests/diagnostic_rule_tests.rs`

- [ ] **Step 1: Write failing diagnostic test in tests/diagnostic_rule_tests.rs**

```rust
use spanglings::core::exercise::{Exercise, Level};
use spanglings::engine::accents::AccentMode;
use spanglings::engine::validator::{validate_submission, ValidationResult};

#[test]
fn test_validation_success() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let exercise = Exercise::from_markdown("test.md", content).unwrap();
    let result = validate_submission(&exercise, "vengas", AccentMode::Forgiving);
    assert_eq!(result.is_success(), true);
}

#[test]
fn test_validation_targeted_diagnostic_error() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->

<!-- DIAGNOSTIC_RULES
pattern: "viene" | code: "E0301" | message: "Expected Present Subjunctive ('vengas'), found Indicative ('viene')."
-->
"#;
    let exercise = Exercise::from_markdown("test.md", content).unwrap();
    let result = validate_submission(&exercise, "viene", AccentMode::Forgiving);
    
    match result {
        ValidationResult::Failed { diagnostic, .. } => {
            assert_eq!(diagnostic.code, "E0301");
            assert!(diagnostic.message.contains("Expected Present Subjunctive"));
        }
        _ => panic!("Expected failed validation with diagnostic"),
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test diagnostic_rule_tests`
Expected: FAIL.

- [ ] **Step 3: Implement src/engine/diagnostics.rs and src/engine/validator.rs**

In `src/engine/diagnostics.rs`:
```rust
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: String,
    pub title: String,
    pub file_path: String,
    pub line_number: usize,
    pub user_snippet: String,
    pub message: String,
    pub note: Option<String>,
    pub help: Option<String>,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn format_terminal(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{}: {}\n",
            format!("error[{}]", self.code).red().bold(),
            self.title.bold()
        ));
        out.push_str(&format!(
            "  {} {}:{}\n",
            "-->".blue().bold(),
            self.file_path,
            self.line_number
        ));
        out.push_str(&format!("   {}\n", "|".blue().bold()));
        out.push_str(&format!(
            "{:2} {} {}\n",
            self.line_number,
            "|".blue().bold(),
            self.user_snippet
        ));
        out.push_str(&format!(
            "   {} {}\n",
            "|".blue().bold(),
            format!("^^^^^ {}", self.message).red().bold()
        ));
        if let Some(ref note) = self.note {
            out.push_str(&format!("   {} {}: {}\n", "=".blue().bold(), "note".bold(), note));
        }
        if let Some(ref help) = self.help {
            out.push_str(&format!("   {} {}: {}\n", "=".blue().bold(), "help".bold(), help));
        }
        if let Some(ref hint) = self.hint {
            out.push_str(&format!("   {} {}: {}\n", "=".blue().bold(), "hint".bold(), hint));
        }
        out
    }
}
```

In `src/engine/validator.rs`:
```rust
use crate::core::exercise::Exercise;
use crate::engine::accents::{evaluate_accents, AccentMode};
use crate::engine::diagnostics::Diagnostic;
use crate::engine::normalizer::normalize_spanish_text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    Passed {
        notice: Option<String>,
    },
    Failed {
        diagnostic: Diagnostic,
        user_input: String,
    },
}

impl ValidationResult {
    pub fn is_success(&self) -> bool {
        matches!(self, ValidationResult::Passed { .. })
    }
}

pub fn extract_user_answer(exercise: &Exercise, file_content: &str) -> String {
    // If input is an inline fill or prompt
    for line in file_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('>') || trimmed.starts_with("<!--") {
            continue;
        }
        if trimmed.starts_with("English:") || trimmed.starts_with("Prompt:") || trimmed.starts_with("Context:") {
            continue;
        }
        if !trimmed.contains("___") && !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    file_content.trim().to_string()
}

pub fn validate_submission(exercise: &Exercise, user_input: &str, accent_mode: AccentMode) -> ValidationResult {
    // 1. Check primary solution
    let primary_eval = evaluate_accents(user_input, &exercise.solution, accent_mode);
    if primary_eval.is_pass {
        return ValidationResult::Passed {
            notice: primary_eval.warning_notice,
        };
    }

    // 2. Check alternative accepted forms
    for alt in &exercise.alternatives {
        let alt_eval = evaluate_accents(user_input, alt, accent_mode);
        if alt_eval.is_pass {
            return ValidationResult::Passed {
                notice: alt_eval.warning_notice,
            };
        }
    }

    // 3. Match against targeted diagnostic rules
    let norm_user = normalize_spanish_text(user_input);
    for rule in &exercise.diagnostic_rules {
        let norm_pat = normalize_spanish_text(&rule.pattern);
        if norm_user.contains(&norm_pat) || user_input.contains(&rule.pattern) {
            return ValidationResult::Failed {
                diagnostic: Diagnostic {
                    code: rule.code.clone(),
                    title: "grammatical rule violation".to_string(),
                    file_path: exercise.path.to_string_lossy().to_string(),
                    line_number: 1,
                    user_snippet: user_input.to_string(),
                    message: rule.message.clone(),
                    note: Some(exercise.rule_summary.clone()),
                    help: exercise.hints.first().cloned(),
                    hint: exercise.hints.get(1).cloned(),
                },
                user_input: user_input.to_string(),
            };
        }
    }

    // 4. Fallback general diagnostic
    ValidationResult::Failed {
        diagnostic: Diagnostic {
            code: "E0001".to_string(),
            title: "mismatch with target Spanish sentence".to_string(),
            file_path: exercise.path.to_string_lossy().to_string(),
            line_number: 1,
            user_snippet: user_input.to_string(),
            message: format!("Expected valid conjugation/sentence matching '{}'.", exercise.solution),
            note: Some(exercise.rule_summary.clone()),
            help: exercise.hints.first().cloned(),
            hint: exercise.hints.get(1).cloned(),
        },
        user_input: user_input.to_string(),
    }
}
```

In `src/engine/rules.rs`:
```rust
// Registry of global Spanish rule triggers
pub const CODE_SER_ESTAR: &str = "E0101";
pub const CODE_PAST_ASPECT: &str = "E0201";
pub const CODE_SUBJUNCTIVE_WEIRDO: &str = "E0301";
pub const CODE_SUBJUNCTIVE_RELATIVE: &str = "E0401";
pub const CODE_SUBJUNCTIVE_CONJUNCTION: &str = "E0501";
pub const CODE_CONDITIONALS: &str = "E0601";
pub const CODE_POR_PARA: &str = "E0701";
pub const CODE_CLITIC_STACKING: &str = "E0801";
pub const CODE_PREPOSITIONAL_REGIME: &str = "E0901";
pub const CODE_ACCIDENTAL_SE: &str = "E1001";
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test diagnostic_rule_tests`
Expected: PASS with 2 tests passed.

- [ ] **Step 5: Commit**

```bash
git add src/engine/ tests/diagnostic_rule_tests.rs
git commit -m "feat(engine): add diagnostic rules, validator, and compiler formatter"
```

---

### Task 5: SM-2 Spaced Repetition System & State Persistence

**Files:**
- Create: `src/core/srs.rs`
- Create: `src/core/state.rs`
- Create: `tests/srs_tests.rs`

- [ ] **Step 1: Write failing SRS test in tests/srs_tests.rs**

```rust
use chrono::{Duration, Utc};
use spanglings::core::srs::{calculate_sm2_review, SrsItem};
use spanglings::core::state::AppState;
use tempfile::NamedTempFile;

#[test]
fn test_sm2_repetition_intervals() {
    let mut item = SrsItem::default();
    
    // First successful review (quality 5) -> interval 1 day
    item = calculate_sm2_review(&item, 5, Utc::now());
    assert_eq!(item.repetitions, 1);
    assert_eq!(item.interval_days, 1);

    // Second review (quality 5) -> interval 6 days
    item = calculate_sm2_review(&item, 5, Utc::now());
    assert_eq!(item.repetitions, 2);
    assert_eq!(item.interval_days, 6);

    // Failed review (quality 1) -> resets repetitions
    item = calculate_sm2_review(&item, 1, Utc::now());
    assert_eq!(item.repetitions, 0);
    assert_eq!(item.interval_days, 1);
}

#[test]
fn test_state_save_and_load() {
    let tmp = NamedTempFile::new().unwrap();
    let path = tmp.path().to_path_buf();

    let mut state = AppState::default();
    state.completed_exercises.insert("b1_sub_01".to_string());
    state.save_to_path(&path).unwrap();

    let loaded = AppState::load_from_path(&path).unwrap();
    assert!(loaded.completed_exercises.contains("b1_sub_01"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test srs_tests`
Expected: FAIL.

- [ ] **Step 3: Implement src/core/srs.rs and src/core/state.rs**

In `src/core/srs.rs`:
```rust
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrsItem {
    pub repetitions: u32,
    pub interval_days: u32,
    pub ease_factor: f32,
    pub next_review_due: DateTime<Utc>,
    pub last_reviewed: Option<DateTime<Utc>>,
}

impl Default for SrsItem {
    fn default() -> Self {
        Self {
            repetitions: 0,
            interval_days: 0,
            ease_factor: 2.5,
            next_review_due: Utc::now(),
            last_reviewed: None,
        }
    }
}

pub fn calculate_sm2_review(item: &SrsItem, quality: u8, now: DateTime<Utc>) -> SrsItem {
    let q = quality.clamp(0, 5) as f32;
    let mut new_ef = item.ease_factor + (0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02));
    if new_ef < 1.3 {
        new_ef = 1.3;
    }

    let (new_reps, new_interval) = if quality < 3 {
        (0, 1)
    } else {
        match item.repetitions {
            0 => (1, 1),
            1 => (2, 6),
            n => {
                let interval = ((item.interval_days as f32) * new_ef).round() as u32;
                (n + 1, interval.max(1))
            }
        }
    };

    let next_due = now + Duration::days(new_interval as i64);

    SrsItem {
        repetitions: new_reps,
        interval_days: new_interval,
        ease_factor: new_ef,
        next_review_due: next_due,
        last_reviewed: Some(now),
    }
}
```

In `src/core/state.rs`:
```rust
use crate::core::srs::SrsItem;
use crate::engine::accents::AccentMode;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseStat {
    pub attempts: u32,
    pub completed_at: Option<DateTime<Utc>>,
    pub hints_used: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub version: u32,
    pub completed_exercises: HashSet<String>,
    pub current_exercise: Option<String>,
    pub accent_mode: AccentMode,
    pub srs: HashMap<String, SrsItem>,
    pub stats: HashMap<String, ExerciseStat>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            version: 1,
            completed_exercises: HashSet::new(),
            current_exercise: None,
            accent_mode: AccentMode::Forgiving,
            srs: HashMap::new(),
            stats: HashMap::new(),
        }
    }
}

impl AppState {
    pub fn default_path() -> PathBuf {
        dirs::config_dir()
            .map(|p| p.join("spanglings").join("state.json"))
            .unwrap_or_else(|| PathBuf::from(".spanglings_state.json"))
    }

    pub fn load() -> Self {
        let path = Self::default_path();
        Self::load_from_path(&path).unwrap_or_default()
    }

    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = fs::read_to_string(path)
            .with_context(|| format!("Failed to read state file at {:?}", path))?;
        let state: Self = serde_json::from_str(&data)
            .with_context(|| format!("Failed to deserialize state JSON at {:?}", path))?;
        Ok(state)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::default_path();
        self.save_to_path(&path)
    }

    pub fn save_to_path(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn mark_completed(&mut self, exercise_id: &str) {
        self.completed_exercises.insert(exercise_id.to_string());
        let stat = self.stats.entry(exercise_id.to_string()).or_insert(ExerciseStat {
            attempts: 0,
            completed_at: None,
            hints_used: 0,
        });
        stat.completed_at = Some(Utc::now());
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test srs_tests`
Expected: PASS with 2 tests passed.

- [ ] **Step 5: Commit**

```bash
git add src/core/srs.rs src/core/state.rs tests/srs_tests.rs
git commit -m "feat(core): add SM-2 spaced repetition scheduler and state persistence"
```

---

### Task 6: In-Terminal Grammar Cheat Sheets Engine

**Files:**
- Create: `src/core/reference.rs`
- Create: `tests/reference_tests.rs`

- [ ] **Step 1: Write reference card test in tests/reference_tests.rs**

```rust
use spanglings::core::reference::get_reference_card;

#[test]
fn test_get_subjunctive_reference_card() {
    let card = get_reference_card("subjunctive").expect("Subjunctive card not found");
    assert!(card.contains("WEIRDO"));
    assert!(card.contains("Wishes"));
    assert!(card.contains("Present Subjunctive Endings"));
}

#[test]
fn test_get_por_para_reference_card() {
    let card = get_reference_card("por-para").expect("Por/Para card not found");
    assert!(card.contains("Por"));
    assert!(card.contains("Para"));
    assert!(card.contains("Cause/Motive"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test reference_tests`
Expected: FAIL.

- [ ] **Step 3: Implement src/core/reference.rs**

In `src/core/reference.rs`:
```rust
use std::collections::HashMap;

pub fn get_reference_card(topic: &str) -> Option<&'static str> {
    match topic.to_lowercase().replace('_', "-").as_str() {
        "subjunctive" | "subj" | "weirdo" => Some(SUBJUNCTIVE_CARD),
        "por-para" | "por_para" | "por" | "para" => Some(POR_PARA_CARD),
        "ser-estar" | "ser_estar" | "ser" | "estar" => Some(SER_ESTAR_CARD),
        "past" | "preterite" | "imperfect" => Some(PAST_TENSES_CARD),
        "pronouns" | "clitics" | "stacking" => Some(PRONOUN_STACKING_CARD),
        _ => None,
    }
}

pub const SUBJUNCTIVE_CARD: &str = r#"
================================================================================
                     SPANISH SUBJUNCTIVE CHEAT SHEET (WEIRDO)
================================================================================
Triggers require: [Main Clause Verb] + "QUE" + [Subject Change]

  W - Wishes / Desires:       querer, desear, preferir, esperar, exigir
  E - Emotions:               alegrarse de, sentir, temer, molestar, encantar
  I - Impersonal Expressions: es necesario que, es importante que, es bueno que
  R - Recommendations:        recomendar, aconsejar, sugerir, pedir
  D - Doubt / Denial:         dudar, no creer, no pensar, negar, no estar seguro
  O - Ojalá:                  ojalá (que)... (Always triggers subjunctive)

FORMING PRESENT SUBJUNCTIVE:
  1. Take the "YO" form of the present indicative (e.g. tengo, hablo, como)
  2. Drop the "-o"
  3. Add OPPOSITE endings:
     -AR verbs:  -e,  -es,  -e,  -emos,  -éis,  -en
     -ER/-IR:    -a,  -as,  -a,  -amos,  -áis,  -an

DOUBT VS CERTAINTY:
  Indicative:  Creo que viene. / Es cierto que viene. / No dudo que viene.
  Subjunctive: No creo que venga. / No es cierto que venga. / Dudo que venga.
================================================================================
"#;

pub const POR_PARA_CARD: &str = r#"
================================================================================
                          POR VS PARA REFERENCE CARD
================================================================================
POR (Cause, Motive, Passage, Exchange):
  - Cause / Reason:           Lo hizo por amor. (Out of love)
  - Means / Communication:    Hablamos por teléfono. / Enviado por correo.
  - Duration / Approximate:   Estudié por tres horas. / Por la mañana.
  - Movement through/along:   Caminamos por el parque.
  - Exchange / Price:         Te doy 10 euros por el libro.
  - In search of:             Fue a la tienda por leche.

PARA (Goal, Recipient, Deadline, Destination):
  - Purpose / "In order to":  Estudio para aprender. (para + infinitive)
  - Recipient:                Este regalo es para ti.
  - Deadline / Specific time: La tarea es para el lunes.
  - Destination:              Salgo para Madrid mañana.
  - Opinion:                  Para mí, es la mejor opción.
  - Standard of comparison:   Para un niño, habla muy bien.
================================================================================
"#;

pub const SER_ESTAR_CARD: &str = r#"
================================================================================
                         SER VS ESTAR REFERENCE CARD
================================================================================
SER (Identity, Essence, Characteristics, Origin, Time):
  - Identity & Profession:    Soy ingeniero. / Es Daniel.
  - Origin & Material:        Soy de España. / La mesa es de madera.
  - Inherent characteristics: Es alto, inteligente y generoso.
  - Time, Date, Events:       Son las tres. / La fiesta es en mi casa.

ESTAR (States, Conditions, Locations, Progressive):
  - Physical Location:        El libro está en la mesa. / Estoy en Londres.
  - Temporary condition/mood: Está cansado. / Está rota la ventana.
  - Present Continuous:       Estoy estudiando español.

ADJECTIVE MEANING SHIFTS:
  - ser listo (smart)         vs estar listo (ready)
  - ser rico (wealthy)        vs estar rico (delicious - food)
  - ser atento (courteous)    vs estar atento (paying attention)
  - ser verde (green color)   vs estar verde (unripe / inexperienced)
================================================================================
"#;

pub const PAST_TENSES_CARD: &str = r#"
================================================================================
                    PRETERITE VS IMPERFECT REFERENCE CARD
================================================================================
PRETERITE (Completed, Bounded Actions):
  - Specific completed event: Ayer compré un coche.
  - Action with time limit:   Vivió en Madrid durante cinco años.
  - Chain of events:          Llegó, abrió la puerta y salió.
  - Interrupting action:      ...cuando sonó el teléfono.

IMPERFECT (Ongoing, Habitual, Background Setting):
  - Habitual past actions:    De niño, jugaba en la calle todos los días.
  - Ongoing background:       Llovía y hacía frío.
  - Age, Time, Weather:       Tenía 20 años. / Eran las seis.
  - Mental/Emotional state:   Quería salir, pero no sabía adónde ir.

MEANING CHANGERS:
  - conocer: conocí (met for 1st time)  vs conocía (knew / was familiar)
  - saber:   supe (found out / learned) vs sabía (knew information)
  - querer:  quise (attempted/tried)    vs quería (wanted/desired)
  - poder:   pude (managed/succeeded)   vs podía (had the capability)
================================================================================
"#;

pub const PRONOUN_STACKING_CARD: &str = r#"
================================================================================
                    DOUBLE OBJECT PRONOUNS & ACCENTS
================================================================================
ORDER RULE: [REFLEXIVE] -> [INDIRECT] -> [DIRECT]
  Indirect: me, te, le, nos, os, les
  Direct:   me, te, lo/la, nos, os, los/las

THE "LE / LES -> SE" RULE:
  When Indirect (le/les) is followed by Direct (lo/la/los/las), 'le' becomes 'SE':
  * Le lo doy -> SE LO DOY.

PLACEMENT RULES:
  1. BEFORE conjugated verb:  "Se lo dije ayer."
  2. ATTACHED to infinitive:  "Voy a decírselo." (Needs written accent!)
  3. ATTACHED to gerund:      "Estoy explicándotelo." (Needs written accent!)
  4. ATTACHED to command:     "¡Dímelo ahora!" (Needs written accent!)
================================================================================
"#;
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test reference_tests`
Expected: PASS with 2 tests passed.

- [ ] **Step 5: Commit**

```bash
git add src/core/reference.rs tests/reference_tests.rs
git commit -m "feat(core): add in-terminal grammar cheat sheet cards"
```

---

### Task 7: CLI Dispatcher & Command Implementations

**Files:**
- Create: `src/cli/mod.rs`
- Create: `src/cli/commands/mod.rs`
- Create: `src/cli/commands/run.rs`
- Create: `src/cli/commands/hint.rs`
- Create: `src/cli/commands/explain.rs`
- Create: `src/cli/commands/list.rs`
- Create: `src/cli/commands/progress.rs`
- Create: `src/cli/commands/drill.rs`
- Create: `src/cli/commands/review.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Implement src/cli/mod.rs with Clap Subcommands**

```rust
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "spanglings", author, version, about = "Developer-grade CLI for Spanish mastery")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Strict accent mode (fails if missing accents)
    #[arg(long, global = true)]
    pub strict_accents: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Watch exercises directory and evaluate on file save
    Watch,
    /// Run and validate a specific exercise
    Run {
        exercise: String,
    },
    /// Show grammatical hints for the current or specified exercise
    Hint {
        exercise: Option<String>,
    },
    /// Display an in-terminal grammar reference card
    Explain {
        topic: String,
    },
    /// Launch quick-fire irregular stem conjugation drills
    Drill {
        topic: Option<String>,
    },
    /// Launch an SM-2 spaced repetition review session
    Review,
    /// List all curriculum exercises and completion status
    List,
    /// Display learning progress and CEFR level mastery
    Progress,
    /// Reset an exercise to its initial prompt
    Reset {
        exercise: String,
    },
}
```

- [ ] **Step 2: Implement command handlers in src/cli/commands/**

Implement handlers:
- `explain`: prints `get_reference_card(&topic)`.
- `hint`: loads exercise, displays Tier 1/2/3 hints.
- `list`: lists exercises with green `[✓]` or red `[ ]`.
- `progress`: prints CEFR level bars and completion percentage.
- `run`: validates exercise, displays compiler-style ANSI diagnostic.
- `drill`: interactive rapid-fire terminal prompt for irregular stems.
- `review`: queries due SRS exercises and runs interactive prompt.

- [ ] **Step 3: Wire up src/main.rs**

```rust
use clap::Parser;
use spanglings::cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Watch) => spanglings::watcher::runner::start_watch_mode(cli.strict_accents)?,
        Some(Commands::Run { exercise }) => spanglings::cli::commands::run::run_exercise(&exercise, cli.strict_accents)?,
        Some(Commands::Hint { exercise }) => spanglings::cli::commands::hint::show_hint(exercise.as_deref())?,
        Some(Commands::Explain { topic }) => spanglings::cli::commands::explain::show_explanation(&topic)?,
        Some(Commands::Drill { topic }) => spanglings::cli::commands::drill::run_drill(topic.as_deref())?,
        Some(Commands::Review) => spanglings::cli::commands::review::run_review_session()?,
        Some(Commands::List) => spanglings::cli::commands::list::list_exercises()?,
        Some(Commands::Progress) => spanglings::cli::commands::progress::show_progress()?,
        Some(Commands::Reset { exercise }) => spanglings::cli::commands::run::reset_exercise(&exercise)?,
        None => spanglings::tui::start_tui(cli.strict_accents)?,
    }
    Ok(())
}
```

- [ ] **Step 4: Test CLI commands execution**

Run: `cargo run -- explain subjunctive`
Expected: Displays Spanish Subjunctive Cheat Sheet.
Run: `cargo run -- list`
Expected: Lists empty curriculum or sample exercises.

- [ ] **Step 5: Commit**

```bash
git add src/cli/ src/main.rs
git commit -m "feat(cli): add clap CLI dispatcher and command handlers"
```

---

### Task 8: Headless File Watcher Runner

**Files:**
- Create: `src/watcher/mod.rs`
- Create: `src/watcher/runner.rs`

- [ ] **Step 1: Implement file watcher with notify in src/watcher/runner.rs**

```rust
use crate::core::exercise::Exercise;
use crate::core::state::AppState;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{extract_user_answer, validate_submission, ValidationResult};
use anyhow::Result;
use colored::Colorize;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn start_watch_mode(strict_accents: bool) -> Result<()> {
    println!("{}", "==========================================================".blue());
    println!("{}", "  Spanglings Watch Mode: Watching exercises/ directory... ".bold());
    println!("{}", "  Edit files in your editor. Save to evaluate automatically.".cyan());
    println!("{}", "  Press Ctrl+C to exit.".dimmed());
    println!("{}", "==========================================================".blue());

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(150), tx)?;

    let exercises_dir = Path::new("exercises");
    if exercises_dir.exists() {
        debouncer.watcher().watch(exercises_dir, RecursiveMode::Recursive)?;
    }

    let accent_mode = if strict_accents {
        AccentMode::Strict
    } else {
        AccentMode::Forgiving
    };

    // Run initial check
    evaluate_current_exercise(accent_mode);

    for res in rx {
        match res {
            Ok(events) => {
                for event in events {
                    if let Some(ext) = event.path.extension() {
                        if ext == "md" {
                            evaluate_current_exercise(accent_mode);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }

    Ok(())
}

pub fn evaluate_current_exercise(mode: AccentMode) {
    // Find next unfinished exercise and run evaluation
}
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add src/watcher/
git commit -m "feat(watcher): add debounced notify file watcher for IDE mode"
```

---

### Task 9: Interactive Terminal TUI (`ratatui`)

**Files:**
- Create: `src/tui/mod.rs`
- Create: `src/tui/ui.rs`
- Create: `src/tui/events.rs`

- [ ] **Step 1: Implement TUI state and event loop in src/tui/mod.rs**

Features:
- Split screen: Top pane = Exercise prompt & grammar context; Bottom pane = Interactive editor / prompt input; Right/Bottom = Real-time diagnostic diff or cheat sheet.
- Key bindings: `Enter` to submit, `H` to toggle hint, `E` to open grammar cheat sheet, `N` for next, `P` for previous, `R` for review mode, `Q` to quit.

- [ ] **Step 2: Implement UI layout in src/tui/ui.rs**

- [ ] **Step 3: Verify TUI compiles and tests pass**

Run: `cargo test`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/tui/
git commit -m "feat(tui): add split-pane interactive ratatui user interface"
```

---

### Task 10: Complete Curriculum Authoring (Tracks 00-21, 100+ Exercises)

**Files:**
- Create: `exercises/00_baseline_drills/` (Preterite roots, Future roots, Subjunctive roots, False cognates)
- Create: `exercises/01_ser_vs_estar/` (Adjective shifts: listo, rico, atento, verde)
- Create: `exercises/02_past_aspects/` (Preterite vs imperfect: supe/sabía, quise/quería, pude/podía)
- Create: `exercises/03_subjunctive_weirdo/` (Wishes, emotions, impersonal, recommendations, doubt, ojalá)
- Create: `exercises/04_subjunctive_relative/` (Indefinite vs definite antecedents)
- Create: `exercises/05_subjunctive_conjunctions/` (En cuanto, para que, a menos que, con tal de que)
- Create: `exercises/06_imperfect_subjunctive_conditionals/` (Si tuviera..., como si fuera...)
- Create: `exercises/07_por_vs_para/` (Cause vs Purpose vs Deadline vs Recipient)
- Create: `exercises/08_pronoun_stacking/` (Double clitics: se lo dije, explicándomelo)
- Create: `exercises/09_prepositional_regimes/` (Darse cuenta de, acordarse de, soñar con, pensar en)
- Create: `exercises/10_accidental_se/` (Se me cayó, se nos olvidó, impersonal vs passive se)
- Create: `exercises/11_pluperfect_subjunctive/` (Si me hubieras dicho...)
- Create: `exercises/12_verbal_periphrases/` (Llevar + gerundio, acabar de, dejar de)
- Create: `exercises/13_advanced_concessives/` (Hagas lo que hagas, por más que)
- Create: `exercises/14_connectors/` (De ahí que + subj, dado que, puesto que)
- Create: `exercises/15_indirect_speech/` (Reported speech tense harmonization)
- Create: `exercises/16_idioms/` (Tomar el pelo, dar la lata, meter la pata)
- Create: `exercises/17_negated_perception/` (No digo que sea..., ¿no crees que...?)
- Create: `exercises/18_cleft_sentences/` (Fue entonces cuando..., lo que pasa es que...)
- Create: `exercises/19_formal_inversion/` (Habiendo considerado..., el hecho de que)
- Create: `exercises/20_passive_refleja/` (Se busca a los culpables vs se buscan casas)
- Create: `exercises/21_nuanced_collocations/` (Advanced register collocations)

- [ ] **Step 1: Author markdown exercise files for all tracks with complete solutions, alternatives, diagnostic rules, and hints**

- [ ] **Step 2: Commit curriculum catalog**

```bash
git add exercises/
git commit -m "feat(curriculum): add 100+ comprehensive B1-C1 exercises and baseline drills"
```

---

### Task 11: Automated Golden Test Suite & Verification

**Files:**
- Create: `tests/exercise_validity_tests.rs`

- [ ] **Step 1: Write Golden Curriculum Validator in tests/exercise_validity_tests.rs**

```rust
use spanglings::core::exercise::Exercise;
use spanglings::engine::accents::AccentMode;
use spanglings::engine::validator::validate_submission;
use std::fs;
use std::path::Path;

#[test]
fn test_all_curriculum_exercises_are_valid_and_solvable() {
    let exercises_dir = Path::new("exercises");
    assert!(exercises_dir.exists(), "exercises directory must exist");

    let mut exercise_count = 0;
    for entry in walkdir::WalkDir::new(exercises_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(entry.path()).unwrap();
            let exercise = Exercise::from_markdown(entry.path(), &content)
                .unwrap_or_else(|e| panic!("Failed to parse exercise {:?}: {:?}", entry.path(), e));

            // Verify solution validates
            let result = validate_submission(&exercise, &exercise.solution, AccentMode::Forgiving);
            assert!(
                result.is_success(),
                "Exercise {:?} solution '{}' failed validation!",
                entry.path(),
                exercise.solution
            );

            // Verify alternatives validate
            for alt in &exercise.alternatives {
                let alt_result = validate_submission(&exercise, alt, AccentMode::Forgiving);
                assert!(
                    alt_result.is_success(),
                    "Exercise {:?} alternative '{}' failed validation!",
                    entry.path(),
                    alt
                );
            }

            exercise_count += 1;
        }
    }

    println!("Successfully validated {} exercises!", exercise_count);
    assert!(exercise_count >= 50, "Expected at least 50 exercises, found {}", exercise_count);
}
```

- [ ] **Step 2: Run all tests**

Run: `cargo test --all`
Expected: 100% test pass rate across all unit, integration, and curriculum validity tests.

- [ ] **Step 3: Run `cargo clippy` and `cargo fmt`**

Run: `cargo clippy -- -D warnings && cargo fmt --check`
Expected: Zero warnings, cleanly formatted code.

- [ ] **Step 4: Commit**

```bash
git add tests/
git commit -m "test: add golden validation test suite ensuring 100% exercise solvability"
```
