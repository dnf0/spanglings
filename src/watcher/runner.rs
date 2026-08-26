use crate::core::curriculum::find_all_exercises;
use crate::core::exercise::Exercise;
use crate::core::state::AppState;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{extract_user_answer, validate_submission, ValidationResult};
use anyhow::Result;
use chrono::Utc;
use colored::Colorize;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn start_watch_mode(strict_accents: bool) -> Result<()> {
    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "{}",
        "  Spanglings Watch Mode: Watching exercises/ directory... ".bold()
    );
    println!(
        "{}",
        "  Edit files in your editor. Save to evaluate automatically.".cyan()
    );
    println!("{}", "  Press Ctrl+C to exit.".dimmed());
    println!(
        "{}",
        "==========================================================".blue()
    );

    let accent_mode = if strict_accents {
        AccentMode::Strict
    } else {
        AccentMode::Forgiving
    };

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(200), tx)?;

    let exercises_dir = Path::new("exercises");
    if exercises_dir.exists() {
        debouncer
            .watcher()
            .watch(exercises_dir, RecursiveMode::Recursive)?;
    }

    // Run initial evaluation
    let _ = evaluate_current_exercise(accent_mode);

    for res in rx {
        match res {
            Ok(events) => {
                let has_md_change = events
                    .iter()
                    .any(|e| e.path.extension().is_some_and(|ext| ext == "md"));
                if has_md_change {
                    let _ = evaluate_current_exercise(accent_mode);
                }
            }
            Err(err) => eprintln!("Watcher error: {:?}", err),
        }
    }

    Ok(())
}

pub fn evaluate_current_exercise(mode: AccentMode) -> Result<bool> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises(exercises_dir)?;
    let mut state = AppState::load().unwrap_or_default();

    if exercises.is_empty() {
        println!(
            "{}",
            "No exercises found in 'exercises/' directory. Waiting for exercise files...".yellow()
        );
        return Ok(false);
    }

    // Find the first exercise that is either not marked done in file or not recorded complete in state
    let active_exercise = exercises
        .iter()
        .find(|e| !e.is_done || !state.is_completed(&e.id));

    let ex = match active_exercise {
        Some(e) => e,
        None => {
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            println!(
                "{}",
                "==========================================================".green()
            );
            println!(
                "{}",
                "🎉 ¡FELICITACIONES! You have completed all curriculum exercises!"
                    .green()
                    .bold()
            );
            println!(
                "{}",
                "==========================================================".green()
            );
            println!(
                "Run '{}' to practice spaced repetition or '{}' for fast stem drills.",
                "spanglings review".cyan(),
                "spanglings drill".cyan()
            );
            return Ok(true);
        }
    };

    let content = fs::read_to_string(&ex.path)?;
    let parsed_ex = Exercise::from_markdown(&ex.path, &content)?;
    let user_answer = extract_user_answer(&parsed_ex, &content);

    // Clear terminal screen for clean compiler-like experience
    print!("\x1B[2J\x1B[1;1H");
    println!(
        "{} {} [{}] - {}",
        "Testing:".blue().bold(),
        parsed_ex.title.bold(),
        parsed_ex.id.cyan(),
        parsed_ex.path.display().to_string().dimmed()
    );
    println!(
        "{}",
        "----------------------------------------------------------".dimmed()
    );

    let result = validate_submission(&parsed_ex, &user_answer, mode);

    match result {
        ValidationResult::Passed { notice } => {
            println!(
                "{}",
                format!("✅ Passed: {}", parsed_ex.title).green().bold()
            );
            println!("   Submission: '{}'", user_answer.cyan());

            if let Some(note) = notice {
                println!("\n  ⚠️  {}", note.yellow());
            }

            state.mark_completed(&parsed_ex.id);
            state.update_srs(&parsed_ex.id, 5, Utc::now());
            state.save()?;

            if !parsed_ex.is_done {
                println!(
                    "\n{}",
                    "💡 Next step: Remove '<!-- I AM NOT DONE -->' from the file to advance!"
                        .yellow()
                        .bold()
                );
            } else {
                println!(
                    "\n{}",
                    "🌟 Exercise completed and marked done! Moving forward...".green()
                );
            }
            Ok(true)
        }
        ValidationResult::Failed { diagnostic, .. } => {
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
            Ok(false)
        }
    }
}
