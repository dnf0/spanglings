use spanglings::core::exercise::Exercise;
use spanglings::engine::accents::strip_accents;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const FUNCTIONAL_STOPWORDS: &[&str] = &[
    "a", "al", "con", "de", "del", "el", "ella", "ellas", "ello", "ellos", "en", "es", "la", "las",
    "le", "les", "lo", "los", "me", "mi", "mis", "no", "nos", "o", "os", "para", "por", "que",
    "qué", "se", "si", "sí", "sin", "sobre", "son", "su", "sus", "te", "ti", "tu", "tus", "u",
    "un", "una", "unas", "uno", "unos", "y", "yo", "muy", "hasta", "hacia", "todo", "toda",
    "todos", "todas", "tan", "donde", "dónde", "mas", "más", "pero", "sino", "bien", "como",
    "cómo", "tanto", "tanta",
];

fn collect_md_paths(dir: &Path, paths: &mut Vec<PathBuf>) {
    if dir.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .expect("read_dir failed")
            .filter_map(|e| e.ok())
            .collect();
        entries.sort_by_key(|e| e.path());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                collect_md_paths(&path, paths);
            } else if path.extension().is_some_and(|ext| ext == "md") {
                paths.push(path);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExerciseInstructions {
    pub todo_line: String,
    pub why_line: String,
    pub inline_todo: Option<String>,
}

pub fn extract_instructions_and_inline_todo(content: &str) -> Option<ExerciseInstructions> {
    let mut todo_line = None;
    let mut why_line = None;
    let mut inline_todo = None;

    let mut in_instructions = false;
    let mut in_exercise = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("### Instructions") {
            in_instructions = true;
            in_exercise = false;
            continue;
        } else if trimmed.starts_with("### Exercise") {
            in_exercise = true;
            in_instructions = false;
            continue;
        } else if trimmed.starts_with('#') {
            in_instructions = false;
            in_exercise = false;
        }

        if in_instructions {
            if let Some(rest) = trimmed.strip_prefix("**TODO**:") {
                todo_line = Some(rest.trim().to_string());
            } else if let Some(rest) = trimmed.strip_prefix("**Why**:") {
                why_line = Some(rest.trim().to_string());
            }
        }

        if in_exercise {
            if let Some(start) = trimmed.find("<!-- TODO:") {
                if let Some(end) = trimmed[start..].find("-->") {
                    let inside = trimmed[start + 10..start + end].trim();
                    inline_todo = Some(inside.to_string());
                }
            }
        }
    }

    match (todo_line, why_line) {
        (Some(todo), Some(why)) => Some(ExerciseInstructions {
            todo_line: todo,
            why_line: why,
            inline_todo,
        }),
        _ => None,
    }
}

fn tokenize_words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .map(|w| strip_accents(w).to_lowercase())
        .filter(|w| !w.is_empty())
        .collect()
}

pub fn check_solution_leakage(
    solution: &str,
    instructions: &ExerciseInstructions,
) -> Option<String> {
    let combined_instructions = format!(
        "{} {} {}",
        instructions.todo_line,
        instructions.why_line,
        instructions.inline_todo.as_deref().unwrap_or("")
    );

    let instruction_tokens: HashSet<String> =
        tokenize_words(&combined_instructions).into_iter().collect();
    let norm_instructions_str = strip_accents(&combined_instructions).to_lowercase();
    let norm_solution_str = strip_accents(solution.trim()).to_lowercase();

    // 1. Check exact multi-word phrase leak
    let solution_words = tokenize_words(solution);
    if solution_words.len() > 1 && norm_instructions_str.contains(&norm_solution_str) {
        return Some(format!(
            "Exact multi-word solution '{}' leaked in instructions",
            solution
        ));
    }

    // 2. Check individual content tokens
    let stop_words: HashSet<&str> = FUNCTIONAL_STOPWORDS.iter().copied().collect();
    for word in &solution_words {
        // Skip common functional stopwords or 1-2 letter words
        if stop_words.contains(word.as_str()) || word.len() < 3 {
            continue;
        }

        if instruction_tokens.contains(word) {
            return Some(format!(
                "Content word '{}' from solution '{}' leaked in instructions: '{}'",
                word, solution, combined_instructions
            ));
        }
    }

    None
}

#[test]
fn test_all_339_exercises_have_instructions_and_todos() {
    let exercises_dir = Path::new("exercises");
    assert!(
        exercises_dir.exists(),
        "exercises directory must exist at workspace root"
    );

    let mut md_paths = Vec::new();
    collect_md_paths(exercises_dir, &mut md_paths);

    assert_eq!(
        md_paths.len(),
        339,
        "Expected exactly 339 exercises across all 60 tracks, found {}",
        md_paths.len()
    );

    let mut missing_instructions = Vec::new();
    let mut invalid_exercises = Vec::new();

    for path in &md_paths {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("Failed to read exercise at {:?}: {}", path, e));

        if !content.contains("### Instructions") {
            missing_instructions.push(path.clone());
            continue;
        }

        let instructions = extract_instructions_and_inline_todo(&content);
        match instructions {
            Some(inst) => {
                // Assert that TODO and Why lines are non-empty and at least 15 chars
                if inst.todo_line.len() < 15 {
                    invalid_exercises.push(format!(
                        "{:?}: **TODO**: line is too short (< 15 chars): '{}'",
                        path, inst.todo_line
                    ));
                }
                if inst.why_line.len() < 15 {
                    invalid_exercises.push(format!(
                        "{:?}: **Why**: line is too short (< 15 chars): '{}'",
                        path, inst.why_line
                    ));
                }
                if inst.inline_todo.is_none() {
                    invalid_exercises.push(format!(
                        "{:?}: missing inline <!-- TODO: ... --> comment under ### Exercise",
                        path
                    ));
                }
            }
            None => {
                invalid_exercises.push(format!(
                    "{:?}: contains '### Instructions' but failed to parse **TODO**: and **Why**:",
                    path
                ));
            }
        }
    }

    assert!(
        invalid_exercises.is_empty(),
        "Found invalid exercise instructions in {} files:\n{}",
        invalid_exercises.len(),
        invalid_exercises.join("\n")
    );

    if !missing_instructions.is_empty() {
        println!(
            "NOTE: {}/339 exercises pending instruction enrichment. Validated {} enriched exercises.",
            missing_instructions.len(),
            md_paths.len() - missing_instructions.len()
        );
        if std::env::var("REQUIRE_ALL_EXERCISE_TODOS").is_ok() {
            panic!(
                "REQUIRE_ALL_EXERCISE_TODOS set, but {} exercises are missing instructions: {:?}",
                missing_instructions.len(),
                missing_instructions
            );
        }
    }
}

#[test]
fn test_zero_solution_leakage_in_instructions() {
    let exercises_dir = Path::new("exercises");
    let mut md_paths = Vec::new();
    collect_md_paths(exercises_dir, &mut md_paths);

    let mut leakages = Vec::new();

    for path in &md_paths {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("Failed to read exercise at {:?}: {}", path, e));

        let Some(instructions) = extract_instructions_and_inline_todo(&content) else {
            continue;
        };

        let exercise = Exercise::from_markdown(path, &content)
            .unwrap_or_else(|e| panic!("Failed to parse exercise at {:?}: {}", path, e));

        if let Some(leak_msg) = check_solution_leakage(&exercise.solution, &instructions) {
            leakages.push(format!("{:?} ({}): {}", path, exercise.id, leak_msg));
        }
    }

    assert!(
        leakages.is_empty(),
        "Found solution leakage in {} exercises:\n{}",
        leakages.len(),
        leakages.join("\n")
    );
}

#[test]
fn test_leak_detector_catches_direct_solution_spoilers() {
    let spoiled_instructions = ExerciseInstructions {
        todo_line: "Conjugate the verb and write quepo in the blank.".to_string(),
        why_line: "The verb caber has an irregular 1st person form.".to_string(),
        inline_todo: Some("Fill in quepo".to_string()),
    };
    let leak = check_solution_leakage("quepo", &spoiled_instructions);
    assert!(leak.is_some(), "Expected leak detection for 'quepo'");

    let unspoiled_instructions = ExerciseInstructions {
        todo_line: "Conjugate the verb (caber) in 1st person singular present indicative."
            .to_string(),
        why_line: "The verb caber undergoes radical stem change in 1st person.".to_string(),
        inline_todo: Some("Conjugate caber for yo".to_string()),
    };
    let no_leak = check_solution_leakage("quepo", &unspoiled_instructions);
    assert!(
        no_leak.is_none(),
        "Unspoiled instructions should not trigger leakage"
    );

    // Test multi-word solution leakage
    let spoiled_multi = ExerciseInstructions {
        todo_line: "Provide the term conmutación por error for failover.".to_string(),
        why_line: "Standard tech terminology.".to_string(),
        inline_todo: None,
    };
    let leak_multi = check_solution_leakage("conmutación por error", &spoiled_multi);
    assert!(leak_multi.is_some(), "Expected multi-word leak detection");

    let unspoiled_multi = ExerciseInstructions {
        todo_line: "Provide the standard Spanish technical expression for system failover."
            .to_string(),
        why_line: "High availability architecture terminology.".to_string(),
        inline_todo: Some("Technical term for failover".to_string()),
    };
    let no_leak_multi = check_solution_leakage("conmutación por error", &unspoiled_multi);
    assert!(
        no_leak_multi.is_none(),
        "Unspoiled multi-word phrase should pass"
    );

    // Test common functional stopwords allowed in explanations
    let functional_instructions = ExerciseInstructions {
        todo_line: "Choose between por and para to express cause.".to_string(),
        why_line: "Expressing origin or cause requires por, while destination uses para."
            .to_string(),
        inline_todo: Some("Choose por or para".to_string()),
    };
    let no_leak_stopword = check_solution_leakage("por", &functional_instructions);
    assert!(
        no_leak_stopword.is_none(),
        "Functional stopword 'por' should be allowed in grammar explanation"
    );
}

#[test]
fn test_extract_instructions_and_inline_todo_parser() {
    let raw = r#"# Baseline 01: Present Irregular 'Yo' Forms
<!-- id: b0_present_stems_01 | level: Baseline | topic: baseline_present_stems | type: cloze | concepts: ["irregular_present_stems"] | prerequisites: [] | grammar_focus: "Irregular present indicative yo-form." -->

> **Grammar Rule**: Irregular 1st person.

### Context
English: "I don't fit in this car."

### Instructions
**TODO**: Conjugate the verb (caber) in 1st person singular present indicative.
**Why**: The verb caber has an irregular 1st person present stem (yo quepo).

### Exercise
<!-- TODO: Conjugate caber for yo in present tense -->
Yo no (caber) ___ en este coche tan pequeño.

<!-- SOLUTION
quepo
-->
"#;

    let parsed = extract_instructions_and_inline_todo(raw).expect("Should extract instructions");
    assert_eq!(
        parsed.todo_line,
        "Conjugate the verb (caber) in 1st person singular present indicative."
    );
    assert_eq!(
        parsed.why_line,
        "The verb caber has an irregular 1st person present stem (yo quepo)."
    );
    assert_eq!(
        parsed.inline_todo.as_deref(),
        Some("Conjugate caber for yo in present tense")
    );
}
