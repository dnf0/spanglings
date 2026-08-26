use crate::core::curriculum::{find_all_exercises, Level};
use crate::core::state::AppState;
use chrono::Utc;
use colored::Colorize;
use std::collections::HashMap;
use std::path::Path;

pub fn show_progress() -> anyhow::Result<()> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises(exercises_dir)?;
    let state = AppState::load().unwrap_or_default();
    let now = Utc::now();

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "{}",
        "              SPANGLINGS LEARNING PROGRESS                ".bold()
    );
    println!(
        "{}",
        "==========================================================".blue()
    );

    let total = exercises.len();
    let completed = exercises
        .iter()
        .filter(|e| state.is_completed(&e.id) || e.is_done)
        .count();
    let pct = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "\nOverall Mastery: {} / {} completed ({:.1}%)\n{}",
        completed.to_string().green().bold(),
        total.to_string().cyan().bold(),
        pct,
        render_progress_bar(pct, 40)
    );

    println!("\n{}", "Progress by CEFR Level:".bold());
    let levels = [Level::Baseline, Level::B1, Level::B2, Level::C1];
    let mut level_totals: HashMap<Level, usize> = HashMap::new();
    let mut level_completed: HashMap<Level, usize> = HashMap::new();

    for ex in &exercises {
        *level_totals.entry(ex.level).or_insert(0) += 1;
        if state.is_completed(&ex.id) || ex.is_done {
            *level_completed.entry(ex.level).or_insert(0) += 1;
        }
    }

    for lvl in levels {
        let tot = level_totals.get(&lvl).copied().unwrap_or(0);
        let comp = level_completed.get(&lvl).copied().unwrap_or(0);
        let lvl_pct = if tot > 0 {
            (comp as f64 / tot as f64) * 100.0
        } else {
            0.0
        };
        println!(
            "  {:<10} {:>3}/{:<3} ({:>5.1}%)  {}",
            lvl.to_string().cyan(),
            comp,
            tot,
            lvl_pct,
            render_progress_bar(lvl_pct, 20)
        );
    }

    println!("\n{}", "Spaced Repetition (SM-2) Memory Retention:".bold());
    let srs_items = &state.srs;
    let due_count = srs_items
        .values()
        .filter(|item| item.next_review_due <= now)
        .count();
    let mastered_count = srs_items
        .values()
        .filter(|item| item.interval_days >= 21)
        .count();
    let avg_ease: f32 = if !srs_items.is_empty() {
        srs_items.values().map(|i| i.ease_factor).sum::<f32>() / srs_items.len() as f32
    } else {
        2.5
    };

    println!(
        "  • Cards Tracked:   {}",
        srs_items.len().to_string().cyan()
    );
    println!(
        "  • Reviews Due Now: {}",
        if due_count > 0 {
            due_count.to_string().yellow().bold()
        } else {
            "0 (All caught up!)".green()
        }
    );
    println!(
        "  • Mastered Cards:  {}",
        mastered_count.to_string().green()
    );
    println!("  • Avg Ease Factor: {:.2}", avg_ease);
    println!(
        "{}",
        "==========================================================".blue()
    );

    Ok(())
}

fn render_progress_bar(pct: f64, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    let bar: String = "█".repeat(filled) + &"░".repeat(empty);
    if pct >= 100.0 {
        bar.green().to_string()
    } else if pct >= 50.0 {
        bar.cyan().to_string()
    } else {
        bar.yellow().to_string()
    }
}
