use crate::core::curriculum::{find_all_exercises, find_exercise_by_query};
use crate::core::exercise::Exercise;
use crate::core::state::AppState;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{extract_user_answer, validate_submission, ValidationResult};
use chrono::Utc;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn run_exercise(exercise_query: &str, strict_accents: bool) -> anyhow::Result<()> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises(exercises_dir)?;

    let target_path = Path::new(exercise_query);
    let (parsed_ex, content) = if target_path.exists() && target_path.is_file() {
        let content = fs::read_to_string(target_path)?;
        let parsed = Exercise::from_markdown(target_path, &content)?;
        (parsed, content)
    } else {
        match find_exercise_by_query(&exercises, exercise_query) {
            Some(ex) => {
                let content = fs::read_to_string(&ex.path)?;
                let parsed = Exercise::from_markdown(&ex.path, &content)?;
                (parsed, content)
            }
            None => {
                anyhow::bail!("No exercise found matching query: '{}'", exercise_query);
            }
        }
    };

    let user_answer = extract_user_answer(&parsed_ex, &content);

    let accent_mode = if strict_accents {
        AccentMode::Strict
    } else {
        AccentMode::Forgiving
    };

    let result = validate_submission(&parsed_ex, &user_answer, accent_mode);

    match result {
        ValidationResult::Passed { notice } => {
            println!(
                "{}",
                format!("✅ Passed: {} [{}]", parsed_ex.title, parsed_ex.id)
                    .green()
                    .bold()
            );
            println!("   Submission: '{}'", user_answer.cyan());

            if let Some(note) = notice {
                println!("\n  ⚠️  {}", note.yellow());
            }

            let mut state = AppState::load().unwrap_or_default();
            state.mark_completed(&parsed_ex.id);
            state.update_srs(&parsed_ex.id, 5, Utc::now());
            for concept in &parsed_ex.concept_tags {
                state.update_concept_mastery(concept, 5, Utc::now());
            }
            state.save()?;
        }
        ValidationResult::Failed { diagnostic, .. } => {
            let mut state = AppState::load().unwrap_or_default();
            for concept in &parsed_ex.concept_tags {
                state.update_concept_mastery(concept, 1, Utc::now());
            }
            let _ = state.save();

            println!("{}", diagnostic.format_terminal());
            println!(
                "\n{} For hints, run: {}",
                "Need help?".yellow(),
                format!("spanglings hint {}", parsed_ex.id).cyan()
            );
            println!(
                "For grammar rules, run: {}",
                format!("spanglings explain {}", parsed_ex.topic).cyan()
            );
        }
    }

    Ok(())
}

pub fn reset_exercise(exercise_query: &str) -> anyhow::Result<()> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises(exercises_dir)?;

    let exercise = match find_exercise_by_query(&exercises, exercise_query) {
        Some(ex) => ex,
        None => anyhow::bail!("No exercise found matching query: '{}'", exercise_query),
    };

    let mut state = AppState::load().unwrap_or_default();
    state.unmark_completed(&exercise.id);
    state.save()?;

    println!(
        "{}",
        format!(
            "Reset exercise '{}' ({:?}) to unfinished.",
            exercise.id, exercise.path
        )
        .green()
    );

    Ok(())
}
