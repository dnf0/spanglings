# Grammar Diagnostics & Compiler-Style Error Formatter Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a compiler-style diagnostic engine for the Spanish learning application, providing rich error messages similar to rustc when user submissions violate grammar rules.

**Architecture:** Create modules for grammar rules (`rules.rs`), diagnostics representation and formatting (`diagnostics.rs`), and validation orchestration (`validator.rs`). Integrate these into `mod.rs` and configure the accent-forgiving enum. Verify correctness using comprehensive test suites.

**Tech Stack:** Rust (edition 2021), colored crate, serde, standard testing framework.

---

### Task 1: Scaffolding and Failing Tests

**Files:**
- Create: `tests/diagnostic_rule_tests.rs`

- [ ] **Step 1: Create the test suite with failing tests**

```rust
use spanglings::core::curriculum::Level;
use spanglings::core::exercise::{Exercise, ExerciseType};
use spanglings::engine::accents::AccentMode;
use spanglings::engine::validator::{extract_user_answer, validate_submission, ValidationResult};

#[test]
fn test_validation_success() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let exercise = Exercise::from_markdown("exercises/03_subjunctive/b1_sub_01.md", content).unwrap();
    let result = validate_submission(&exercise, "vengas", AccentMode::Forgiving);
    assert!(result.is_success());
}

#[test]
fn test_validation_alternative_success() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->

<!-- ALTERNATIVES
vengas tú
-->
"#;
    let exercise = Exercise::from_markdown("exercises/03_subjunctive/b1_sub_01.md", content).unwrap();
    let result = validate_submission(&exercise, "vengas tú", AccentMode::Forgiving);
    assert!(result.is_success());
}

#[test]
fn test_validation_targeted_diagnostic_error() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->

<!-- DIAGNOSTIC_RULES
pattern: "viene" | code: "E0301" | message: "Expected Present Subjunctive ('vengas'), found Indicative ('viene')."
-->

<!-- HINTS
Tier 1: Check the main clause verb.
Tier 2: Use the irregular root 'veng-'.
Tier 3: Add '-as' to form 'vengas'.
-->
"#;
    let exercise = Exercise::from_markdown("exercises/03_subjunctive/b1_sub_01.md", content).unwrap();
    let result = validate_submission(&exercise, "viene", AccentMode::Forgiving);

    match result {
        ValidationResult::Failed { diagnostic, .. } => {
            assert_eq!(diagnostic.code, "E0301");
            assert!(diagnostic.message.contains("Expected Present Subjunctive"));
            assert_eq!(diagnostic.file_path, "exercises/03_subjunctive/b1_sub_01.md");
            let formatted = diagnostic.format_terminal();
            assert!(formatted.contains("error[E0301]"));
            assert!(formatted.contains("Expected Present Subjunctive"));
        }
        _ => panic!("Expected failed validation with targeted diagnostic"),
    }
}

#[test]
fn test_validation_general_fallback_diagnostic() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let exercise = Exercise::from_markdown("exercises/03_subjunctive/b1_sub_01.md", content).unwrap();
    let result = validate_submission(&exercise, "comas", AccentMode::Forgiving);

    match result {
        ValidationResult::Failed { diagnostic, .. } => {
            assert_eq!(diagnostic.code, "E0001");
            assert!(diagnostic.message.contains("vengas"));
        }
        _ => panic!("Expected general fallback diagnostic"),
    }
}

#[test]
fn test_extract_user_answer() {
    let exercise_content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

> **Grammar Rule**: Subjunctive required.

### Context
English: "I want you to come."

### Exercise
Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let exercise = Exercise::from_markdown("test.md", exercise_content).unwrap();

    let user_edited = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

> **Grammar Rule**: Subjunctive required.

### Context
English: "I want you to come."

### Exercise
Quiero que tú (venir) vengas a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let answer = extract_user_answer(&exercise, user_edited);
    assert!(!answer.is_empty());
}
```

- [ ] **Step 2: Run cargo test to verify compilation fails**
Run: `cargo test`
Expected: Fails to compile because `validator` module and `AccentMode` elements are not yet implemented.

---

### Task 2: Standard Error Codes

**Files:**
- Create: `src/engine/rules.rs`

- [ ] **Step 1: Implement standard grammar diagnostic codes**

```rust
// Registry of global Spanish rule triggers
pub const CODE_GENERAL_MISMATCH: &str = "E0001";
pub const CODE_SER_ESTAR_STATE: &str = "E0101";
pub const CODE_SER_ESTAR_MEANING: &str = "E0102";
pub const CODE_PAST_ASPECT_PRETERITE: &str = "E0201";
pub const CODE_PAST_ASPECT_IMPERFECT: &str = "E0202";
pub const CODE_PAST_MEANING_CHANGE: &str = "E0203";
pub const CODE_SUBJUNCTIVE_WEIRDO: &str = "E0301";
pub const CODE_SUBJUNCTIVE_RELATIVE: &str = "E0401";
pub const CODE_SUBJUNCTIVE_CONJUNCTION: &str = "E0501";
pub const CODE_CONDITIONALS: &str = "E0601";
pub const CODE_POR_PARA: &str = "E0701";
pub const CODE_CLITIC_STACKING: &str = "E0801";
pub const CODE_CLITIC_ACCENT: &str = "E0802";
pub const CODE_PREPOSITIONAL_REGIME: &str = "E0901";
pub const CODE_ACCIDENTAL_SE: &str = "E1001";

pub fn get_rule_title(code: &str) -> String {
    match code {
        "E0001" => "general mismatch".to_string(),
        "E0101" => "ser vs estar state mismatch".to_string(),
        "E0102" => "ser vs estar meaning change".to_string(),
        "E0201" => "past aspect preterite mismatch".to_string(),
        "E0202" => "past aspect imperfect mismatch".to_string(),
        "E0203" => "past meaning change".to_string(),
        "E0301" => "grammatical rule violation".to_string(),
        "E0401" => "subjunctive relative clause".to_string(),
        "E0501" => "subjunctive conjunction".to_string(),
        "E0601" => "conditional tense violation".to_string(),
        "E0701" => "por vs para mismatch".to_string(),
        "E0801" => "clitic stacking error".to_string(),
        "E0802" => "clitic accentuation error".to_string(),
        "E0901" => "prepositional regime mismatch".to_string(),
        "E1001" => "accidental se usage".to_string(),
        _ => "grammatical rule violation".to_string(),
    }
}
```

---

### Task 3: Compiler-Style Diagnostics

**Files:**
- Create: `src/engine/diagnostics.rs`

- [ ] **Step 1: Implement Diagnostic struct and format_terminal function**

```rust
use serde::{Deserialize, Serialize};
use colored::Colorize;

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
        let err_code = format!("error[{}]", self.code).red().bold();
        let title_label = self.title.bold();
        let arrow_color = "-->".blue().bold();
        let pipe_color = "|".blue().bold();
        let line_str = format!("{}", self.line_number);
        let line_padding = " ".repeat(line_str.len());

        let underline_len = if self.user_snippet.is_empty() { 1 } else { self.user_snippet.len() };
        let underline = "^".repeat(underline_len).red().bold();

        let mut lines = Vec::new();
        lines.push(format!("{}: {}", err_code, title_label));
        lines.push(format!("{} {}:{}:{}", " ".repeat(line_str.len() + 1), arrow_color, self.file_path, self.line_number));
        lines.push(format!("{} {}", line_padding, pipe_color));
        lines.push(format!(" {} {} {}", line_str, pipe_color, self.user_snippet));
        lines.push(format!("{} {} {} {}", line_padding, pipe_color, underline, self.message));

        if let Some(ref note) = self.note {
            lines.push(format!("{} {} {}: {}", line_padding, "=".blue().bold(), "note".bold(), note));
        }
        if let Some(ref help) = self.help {
            lines.push(format!("{} {} {}: {}", line_padding, "=".blue().bold(), "help".bold(), help));
        }
        if let Some(ref hint) = self.hint {
            lines.push(format!("{} {} {}: {}", line_padding, "=".blue().bold(), "hint".bold(), hint));
        }

        lines.join("\n")
    }
}
```

---

### Task 4: Accent Mode Integration and Module Exporting

**Files:**
- Modify: `src/engine/accents.rs`
- Modify: `src/engine/mod.rs`

- [ ] **Step 1: Add AccentMode enum to src/engine/accents.rs**

Add at the top of `src/engine/accents.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AccentMode {
    #[default]
    Forgiving,
    Strict,
}
```

- [ ] **Step 2: Export all required modules in src/engine/mod.rs**

Rewrite `src/engine/mod.rs`:
```rust
pub mod accents;
pub mod diagnostics;
pub mod normalizer;
pub mod rules;
pub mod validator;
```

---

### Task 5: Core Validation Logic & Submission Evaluator

**Files:**
- Create: `src/engine/validator.rs`

- [ ] **Step 1: Implement extract_user_answer and validate_submission**

```rust
use crate::core::exercise::Exercise;
use crate::engine::accents::{check_accent_match, AccentMode, AccentResult};
use crate::engine::diagnostics::Diagnostic;
use crate::engine::normalizer::normalize;
use crate::engine::rules::get_rule_title;

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
    // 1. Try to read the original template file from exercise.path
    if let Ok(orig_content) = std::fs::read_to_string(&exercise.path) {
        let orig_lines: Vec<&str> = orig_content.lines().collect();
        let user_lines: Vec<&str> = file_content.lines().collect();
        
        for (i, &orig_line) in orig_lines.iter().enumerate() {
            if orig_line.contains("___") {
                if let Some(&user_line) = user_lines.get(i) {
                    if let Some((prefix, suffix)) = orig_line.split_once("___") {
                        let user_trimmed = user_line.trim();
                        let prefix_trimmed = prefix.trim();
                        let suffix_trimmed = suffix.trim();
                        
                        let mut answer = user_trimmed;
                        if !prefix_trimmed.is_empty() && answer.starts_with(prefix_trimmed) {
                            answer = &answer[prefix_trimmed.len()..];
                        }
                        if !suffix_trimmed.is_empty() && answer.ends_with(suffix_trimmed) {
                            answer = &answer[..answer.len() - suffix_trimmed.len()];
                        }
                        return answer.trim().to_string();
                    }
                }
            }
        }
    }

    // 2. Fallback heuristic: search for the exercise line
    for line in file_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('>') || trimmed.starts_with("<!--") {
            continue;
        }
        if trimmed.starts_with("English:") || trimmed.starts_with("Prompt:") || trimmed.starts_with("Context:") {
            continue;
        }
        
        let mut candidates = vec![exercise.solution.as_str()];
        for alt in &exercise.alternatives {
            candidates.push(alt.as_str());
        }
        for cand in candidates {
            if trimmed.contains(cand) {
                return cand.to_string();
            }
        }
        
        if !trimmed.contains("___") && !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    
    if !exercise.solution.is_empty() {
        exercise.solution.clone()
    } else {
        "".to_string()
    }
}

pub fn validate_submission(
    exercise: &Exercise,
    user_input: &str,
    accent_mode: AccentMode,
) -> ValidationResult {
    // 1. Check primary solution
    match check_accent_match(user_input, &exercise.solution, accent_mode == AccentMode::Strict) {
        AccentResult::ExactMatch => return ValidationResult::Passed { notice: None },
        AccentResult::ForgivenMatch { tip, .. } => return ValidationResult::Passed { notice: Some(tip) },
        AccentResult::Mismatch => {}
    }

    // 2. Check alternative accepted forms
    for alt in &exercise.alternatives {
        match check_accent_match(user_input, alt, accent_mode == AccentMode::Strict) {
            AccentResult::ExactMatch => return ValidationResult::Passed { notice: None },
            AccentResult::ForgivenMatch { tip, .. } => return ValidationResult::Passed { notice: Some(tip) },
            AccentResult::Mismatch => {}
        }
    }

    // 3. Match against targeted diagnostic rules
    let norm_user = normalize(user_input);
    for rule in &exercise.diagnostic_rules {
        let norm_pat = normalize(&rule.pattern);
        if norm_user.contains(&norm_pat) || user_input.contains(&rule.pattern) {
            return ValidationResult::Failed {
                diagnostic: Diagnostic {
                    code: rule.code.clone(),
                    title: get_rule_title(&rule.code),
                    file_path: exercise.path.to_string_lossy().to_string(),
                    line_number: 1,
                    user_snippet: user_input.to_string(),
                    message: rule.message.clone(),
                    note: exercise.hints.get(2).cloned(),
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
            title: get_rule_title("E0001"),
            file_path: exercise.path.to_string_lossy().to_string(),
            line_number: 1,
            user_snippet: user_input.to_string(),
            message: format!("Expected '{}'.", exercise.solution),
            note: exercise.hints.get(2).cloned(),
            help: exercise.hints.first().cloned(),
            hint: exercise.hints.get(1).cloned(),
        },
        user_input: user_input.to_string(),
    }
}
```

---

### Task 6: Verification and Automated Quality Control

**Files:**
- Modify: None

- [ ] **Step 1: Run cargo test to verify all tests pass**
Run: `cargo test`
Expected: PASS

- [ ] **Step 2: Run clippy and fmt**
Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS with zero warnings/formatting violations

- [ ] **Step 3: Run graphify update**
Run: `uvx --from graphifyy graphify update .`
Expected: PASS

- [ ] **Step 4: Commit and finalize**
Run standard commits.
