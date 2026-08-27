use crate::core::curriculum::find_all_exercises_or_embedded;
use crate::core::state::AppState;
use colored::Colorize;
use std::path::Path;

pub fn get_exercises_json() -> anyhow::Result<String> {
    let exercises = find_all_exercises_or_embedded("exercises")?;
    let json_str = serde_json::to_string_pretty(&exercises)?;
    Ok(json_str)
}

pub fn list_exercises(json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", get_exercises_json()?);
        return Ok(());
    }
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises_or_embedded(exercises_dir)?;
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
