use crate::core::curriculum::{find_all_exercises_or_embedded, Level};
use crate::core::placement::{
    check_placement_answer, evaluate_placement_test, get_placement_battery,
};
use crate::core::state::{AppState, EvaluatedLevel};
use crate::engine::accents::AccentMode;
use anyhow::{Context, Result};
use chrono::Utc;
use colored::Colorize;
use serde_json::json;
use std::io::{self, BufRead, Write};

pub fn run_test(
    level_filter: Option<String>,
    fast_track_flag: bool,
    json_mode: bool,
    strict_accents: bool,
) -> Result<()> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    run_test_with_io(
        level_filter,
        fast_track_flag,
        json_mode,
        strict_accents,
        &mut stdin_lock,
        &mut stdout,
    )
}

pub fn run_test_with_io<R: BufRead, W: Write>(
    level_filter: Option<String>,
    fast_track_flag: bool,
    json_mode: bool,
    strict_accents: bool,
    reader: &mut R,
    writer: &mut W,
) -> Result<()> {
    let parsed_level = level_filter
        .as_deref()
        .map(|s| s.parse::<Level>().map_err(|e| anyhow::anyhow!(e)))
        .transpose()?;

    let battery = get_placement_battery(parsed_level);
    if battery.is_empty() {
        anyhow::bail!("No diagnostic questions available for the specified level.");
    }

    let accent_mode = if strict_accents {
        AccentMode::Strict
    } else {
        AccentMode::Forgiving
    };

    if !json_mode {
        writeln!(writer)?;
        writeln!(
            writer,
            "{}",
            "==========================================================".cyan()
        )?;
        writeln!(
            writer,
            "{}",
            "        SPANGLINGS DIAGNOSTIC PLACEMENT TEST              "
                .bold()
                .cyan()
        )?;
        writeln!(
            writer,
            "{}",
            "==========================================================".cyan()
        )?;
        writeln!(
            writer,
            "{}",
            "Test your active Spanish proficiency across CEFR levels.".white()
        )?;
        writeln!(
            writer,
            "{}",
            "Type your answer for each blank (___) and press Enter.".white()
        )?;
        writeln!(writer)?;
    }

    let mut user_answers = Vec::new();

    for (i, question) in battery.iter().enumerate() {
        if !json_mode {
            writeln!(
                writer,
                "{} [{}/{}] — CEFR Tier: {}",
                "Question".bold().blue(),
                i + 1,
                battery.len(),
                format!("{:?}", question.level).bold().yellow()
            )?;
            writeln!(
                writer,
                "  {} {}",
                "Context:".dimmed(),
                question.context_en.italic()
            )?;
            writeln!(writer, "  {}  {}", "Prompt: ".bold(), question.prompt_es)?;
            write!(writer, "  {} ", "Answer >".green().bold())?;
            writer.flush()?;
        }

        let mut line = String::new();
        if reader.read_line(&mut line).is_err() || line.is_empty() {
            user_answers.push("".to_string());
        } else {
            user_answers.push(line.trim().to_string());
        }

        if !json_mode {
            let last_ans = user_answers.last().unwrap();
            let is_correct = check_placement_answer(
                last_ans,
                &question.solution,
                &question.alternatives,
                accent_mode,
            );
            if is_correct {
                writeln!(writer, "  {}", "✅ Correct!".bold().green())?;
            } else {
                writeln!(
                    writer,
                    "  {} Expected: {} ({})",
                    "❌ Incorrect.".bold().red(),
                    question.solution.bold().yellow(),
                    question.explanation.dimmed()
                )?;
            }
            writeln!(writer)?;
        }
    }

    let result = evaluate_placement_test(&battery, &user_answers, accent_mode);

    let mut state = AppState::load().unwrap_or_default();
    state.evaluated_level = Some(EvaluatedLevel {
        level: result.assessed_level,
        score_percent: result.percentage,
        evaluated_at: Utc::now(),
    });

    let exercises = find_all_exercises_or_embedded("./exercises")
        .unwrap_or_else(|_| crate::core::embedded::get_embedded_exercises().unwrap_or_default());

    if json_mode {
        let out = json!({
            "assessed_level": format!("{:?}", result.assessed_level),
            "percentage": result.percentage,
            "total_questions": result.total_questions,
            "total_correct": result.total_correct,
            "passed_levels": result.passed_levels.iter().map(|l| format!("{:?}", l)).collect::<Vec<_>>(),
            "scores_by_level": result.scores_by_level.iter().map(|(k, v)| {
                (format!("{:?}", k), json!({ "correct": v.0, "total": v.1, "percent": if v.1 > 0 { (v.0 as f64 / v.1 as f64) * 100.0 } else { 0.0 } }))
            }).collect::<serde_json::Map<_, _>>(),
        });
        writeln!(writer, "{}", serde_json::to_string_pretty(&out)?)?;
        state.save()?;
        return Ok(());
    }

    writeln!(
        writer,
        "{}",
        "==========================================================".cyan()
    )?;
    writeln!(
        writer,
        "{}",
        "               PLACEMENT ASSESSMENT RESULTS               "
            .bold()
            .cyan()
    )?;
    writeln!(
        writer,
        "{}",
        "==========================================================".cyan()
    )?;
    writeln!(writer)?;
    writeln!(
        writer,
        "🎯 Evaluated CEFR Placement Level: {}",
        format!("{:?}", result.assessed_level).bold().green()
    )?;
    writeln!(
        writer,
        "📊 Overall Diagnostic Score:     {} ({} / {} correct)",
        format!("{:.1}%", result.percentage).bold().yellow(),
        result.total_correct,
        result.total_questions
    )?;
    writeln!(writer)?;
    writeln!(writer, "{}", "Breakdown by CEFR Level:".bold())?;
    for level in [Level::Baseline, Level::B1, Level::B2, Level::C1] {
        if let Some(&(correct, total)) = result.scores_by_level.get(&level) {
            let pct = if total > 0 {
                (correct as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            let status = if pct >= 75.0 {
                "PASS (Tested Out)".bold().green()
            } else {
                "In Progress".yellow()
            };
            writeln!(
                writer,
                "  • {:<10} {:>2}/{} ({:>5.1}%)  [{}]",
                format!("{:?}", level),
                correct,
                total,
                pct,
                status
            )?;
        }
    }
    writeln!(writer)?;

    for &passed_lvl in &result.passed_levels {
        let count_for_lvl = exercises.iter().filter(|e| e.level == passed_lvl).count();
        if count_for_lvl == 0 {
            continue;
        }

        let should_fast_track = if fast_track_flag {
            true
        } else {
            write!(
                writer,
                "⚡ Fast-track level {}? Mark all {} exercises as completed & seed spaced reviews? [Y/n] ",
                format!("{:?}", passed_lvl).bold().cyan(),
                count_for_lvl
            )?;
            writer.flush()?;
            let mut confirm = String::new();
            if reader.read_line(&mut confirm).is_ok() {
                let trimmed = confirm.trim().to_lowercase();
                trimmed.is_empty() || trimmed == "y" || trimmed == "yes"
            } else {
                false
            }
        };

        if should_fast_track {
            let marked = state.fast_track_level(passed_lvl, &exercises);
            writeln!(
                writer,
                "  ✨ Fast-tracked {} exercises for level {:?} into spaced review queue!",
                marked, passed_lvl
            )?;
        }
    }

    state
        .save()
        .context("Failed to save updated learning state")?;
    writeln!(writer)?;
    writeln!(
        writer,
        "{}",
        "Learning state updated successfully. Run 'spanglings progress' to view your updated dashboard."
            .green()
    )?;
    writeln!(writer)?;

    Ok(())
}
