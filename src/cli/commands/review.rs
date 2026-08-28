use crate::core::curriculum::find_all_exercises_or_embedded;
use crate::core::state::AppState;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{validate_submission, ValidationResult};
use chrono::Utc;
use colored::Colorize;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn run_review_session() -> anyhow::Result<()> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises_or_embedded(exercises_dir)?;
    let mut state = AppState::load().unwrap_or_default();
    let now = Utc::now();

    let due_exercises: Vec<_> = exercises
        .iter()
        .filter(|e| state.is_due_for_review(&e.id, now))
        .collect();

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "{}",
        "         SPANGLINGS SPACED REPETITION (SM-2) REVIEW       ".bold()
    );
    println!(
        "{}",
        "==========================================================".blue()
    );

    if due_exercises.is_empty() {
        println!(
            "{}",
            "🎉 All caught up! No reviews are currently due."
                .green()
                .bold()
        );
        println!(
            "Use '{}' to list curriculum or '{}' for fast stem drilling.",
            "spanglings list".cyan(),
            "spanglings drill".cyan()
        );
        return Ok(());
    }

    println!(
        "You have {} exercise(s) due for review today.\n",
        due_exercises.len().to_string().yellow().bold()
    );

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut reviewed_count = 0;

    for (i, ex) in due_exercises.iter().enumerate() {
        println!(
            "--- Review {}/{} [{}] ---",
            i + 1,
            due_exercises.len(),
            ex.id.cyan()
        );
        println!("Title: {}", ex.title.bold());

        print!("Your answer > ");
        io::stdout().flush()?;

        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            // EOF reached
            println!();
            break;
        }

        let user_ans = line.trim();
        let val_result = validate_submission(ex, user_ans, AccentMode::Forgiving);

        let quality = match val_result {
            ValidationResult::Passed { notice } => {
                println!("  {} Correct!", "✓".green().bold());
                if let Some(note) = notice {
                    println!("  ⚠️  Note: {}", note.yellow());
                    4 // Good with slight accent hesitation
                } else {
                    5 // Perfect recall
                }
            }
            ValidationResult::Failed { .. } => {
                println!("  {} Incorrect.", "✗".red().bold());
                println!("  Expected: '{}'", ex.solution.green().bold());
                if !ex.hints.is_empty() {
                    println!("  Hint: {}", ex.hints[0].dimmed());
                }
                1 // Incorrect recall, resets interval
            }
        };

        state.update_srs(&ex.id, quality, Utc::now());
        reviewed_count += 1;
        println!();
    }

    state.save()?;

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "Review session completed! Reviewed {} card(s). Progress saved.",
        reviewed_count.to_string().green().bold()
    );
    println!(
        "{}",
        "==========================================================".blue()
    );

    Ok(())
}
