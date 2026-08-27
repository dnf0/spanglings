use crate::core::curriculum::{find_all_exercises_or_embedded, find_exercise_by_query};
use crate::core::exercise::Exercise;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{extract_user_answer, validate_submission, ValidationResult};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckDiagnosticItem {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub code: String,
    pub message: String,
    pub note: Option<String>,
    pub help: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckReport {
    pub exercise_id: String,
    pub file_path: String,
    pub passed: bool,
    pub is_done: bool,
    pub notice: Option<String>,
    pub diagnostics: Vec<CheckDiagnosticItem>,
}

pub fn evaluate_exercise_for_check(
    exercise: &Exercise,
    file_content: &str,
    accent_mode: AccentMode,
) -> CheckReport {
    let is_done = !file_content.contains("<!-- I AM NOT DONE -->");
    let user_answer = extract_user_answer(exercise, file_content);
    let result = validate_submission(exercise, &user_answer, accent_mode);

    let file_path_str = exercise.path.display().to_string();

    match result {
        ValidationResult::Passed { notice } => CheckReport {
            exercise_id: exercise.id.clone(),
            file_path: file_path_str,
            passed: true,
            is_done,
            notice,
            diagnostics: Vec::new(),
        },
        ValidationResult::Failed {
            diagnostic,
            user_input: _,
        } => {
            let item = CheckDiagnosticItem {
                file: file_path_str.clone(),
                line: diagnostic.line_number,
                column: 1,
                severity: "error".to_string(),
                code: diagnostic.code,
                message: diagnostic.message,
                note: diagnostic.note,
                help: diagnostic.help,
            };
            CheckReport {
                exercise_id: exercise.id.clone(),
                file_path: file_path_str,
                passed: false,
                is_done,
                notice: None,
                diagnostics: vec![item],
            }
        }
    }
}

pub fn run_check(
    exercise_arg: Option<&str>,
    json: bool,
    strict_accents: bool,
) -> anyhow::Result<bool> {
    let accent_mode = if strict_accents {
        AccentMode::Strict
    } else {
        AccentMode::Forgiving
    };

    let reports = if let Some(arg) = exercise_arg {
        let path = Path::new(arg);
        if path.exists() && path.is_file() {
            let content = fs::read_to_string(path)?;
            let exercise = Exercise::from_markdown(path, &content)?;
            vec![evaluate_exercise_for_check(
                &exercise,
                &content,
                accent_mode,
            )]
        } else {
            let exercises = find_all_exercises_or_embedded("exercises")?;
            let ex = find_exercise_by_query(&exercises, arg)
                .ok_or_else(|| anyhow::anyhow!("Exercise '{}' not found", arg))?;
            let content = if ex.path.exists() {
                fs::read_to_string(&ex.path)?
            } else {
                ex.raw_content.clone()
            };
            vec![evaluate_exercise_for_check(ex, &content, accent_mode)]
        }
    } else {
        let exercises = find_all_exercises_or_embedded("exercises")?;
        let mut reports = Vec::new();
        for ex in &exercises {
            let content = if ex.path.exists() {
                fs::read_to_string(&ex.path).unwrap_or_else(|_| ex.raw_content.clone())
            } else {
                ex.raw_content.clone()
            };
            let rep = evaluate_exercise_for_check(ex, &content, accent_mode);
            reports.push(rep);
        }
        reports
    };

    let all_passed = reports.iter().all(|r| r.passed);

    if json {
        let json_str = serde_json::to_string_pretty(&reports)?;
        println!("{}", json_str);
    } else {
        for rep in &reports {
            if rep.passed {
                println!(
                    "{} {}: {} [{}]",
                    "✓".green().bold(),
                    rep.file_path.cyan(),
                    "Passed".green().bold(),
                    rep.exercise_id.bold()
                );
                if let Some(ref notice) = rep.notice {
                    println!("  ℹ {}", notice.yellow());
                }
            } else {
                for diag in &rep.diagnostics {
                    println!(
                        "{}:{}:{}: {}: {}",
                        diag.file.bold(),
                        diag.line,
                        diag.column,
                        format!("error[{}]", diag.code).red().bold(),
                        diag.message
                    );
                    if let Some(ref help) = diag.help {
                        println!("  = help: {}", help.cyan());
                    }
                }
            }
        }
    }

    Ok(all_passed)
}
