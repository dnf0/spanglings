use crate::core::curriculum::find_all_exercises;
use crate::core::state::AppState;
use colored::Colorize;
use std::path::Path;

pub fn list_exercises() -> anyhow::Result<()> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises(exercises_dir)?;
    let state = AppState::load().unwrap_or_default();

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "{}",
        "               SPANGLINGS CURRICULUM                     ".bold()
    );
    println!(
        "{}",
        "==========================================================".blue()
    );

    if exercises.is_empty() {
        println!(
            "{}",
            "No exercises found in 'exercises/' directory.".yellow()
        );
        return Ok(());
    }

    let mut current_level = None;

    for ex in &exercises {
        if current_level != Some(ex.level) {
            current_level = Some(ex.level);
            println!(
                "\n{} {}",
                "─── Level:".dimmed(),
                ex.level.to_string().cyan().bold()
            );
        }

        let is_completed = state.is_completed(&ex.id) || ex.is_done;
        let status_mark = if is_completed {
            "[✓]".green().bold()
        } else {
            "[ ]".red().bold()
        };

        println!(
            "  {} {:<22} {:<35} ({})",
            status_mark,
            ex.id.bright_white(),
            ex.title,
            ex.exercise_type.to_string().dimmed()
        );
    }

    println!(
        "\n{}",
        "==========================================================".blue()
    );
    let total = exercises.len();
    let completed = exercises
        .iter()
        .filter(|e| state.is_completed(&e.id) || e.is_done)
        .count();
    println!(
        "Total: {} exercises | Completed: {} | Remaining: {}",
        total.to_string().cyan().bold(),
        completed.to_string().green().bold(),
        (total - completed).to_string().yellow().bold()
    );

    Ok(())
}
