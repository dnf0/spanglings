use crate::core::curriculum::{find_all_exercises, find_exercise_by_query};
use crate::core::state::AppState;
use colored::Colorize;
use std::path::Path;

pub fn show_hint(exercise_query: Option<&str>) -> anyhow::Result<()> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises(exercises_dir)?;
    let state = AppState::load().unwrap_or_default();

    let target_exercise = match exercise_query {
        Some(query) => find_exercise_by_query(&exercises, query),
        None => exercises
            .iter()
            .find(|e| !state.is_completed(&e.id) && !e.is_done),
    };

    let exercise = match target_exercise {
        Some(ex) => ex,
        None => {
            if let Some(q) = exercise_query {
                println!(
                    "{}",
                    format!("No exercise found matching query: '{}'", q).yellow()
                );
            } else {
                println!(
                    "{}",
                    "All exercises completed! Specify an exercise ID for hints."
                        .green()
                        .bold()
                );
            }
            return Ok(());
        }
    };

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "💡 Hints for: {} [{}]",
        exercise.title.bold(),
        exercise.id.cyan()
    );
    println!(
        "{}",
        "==========================================================".blue()
    );

    if exercise.hints.is_empty() {
        println!(
            "{}",
            "No specific hints provided for this exercise.".yellow()
        );
        println!(
            "Use '{}' to view general grammar rules.",
            format!("spanglings explain {}", exercise.topic).cyan()
        );
    } else {
        for (i, hint) in exercise.hints.iter().enumerate() {
            let tier_label = match i {
                0 => "Tier 1 (Subtle Nudge)".yellow().bold(),
                1 => "Tier 2 (Grammar Pattern)".cyan().bold(),
                _ => "Tier 3 (Direct Guidance)".green().bold(),
            };
            println!("\n▶ {}:", tier_label);
            println!("  {}", hint);
        }
    }

    println!(
        "\n{}",
        "==========================================================".blue()
    );
    Ok(())
}
