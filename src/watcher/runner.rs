use crate::core::curriculum::find_all_exercises;
use crate::core::exercise::Exercise;
use crate::core::state::AppState;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{extract_user_answer, validate_submission, ValidationResult};
use anyhow::Result;
use chrono::Utc;
use colored::Colorize;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

struct RawModeGuard {
    active: bool,
}

impl RawModeGuard {
    fn new() -> Self {
        let is_tty = io::stdin().is_terminal() && io::stdout().is_terminal();
        let active = if is_tty {
            crossterm::terminal::enable_raw_mode().is_ok()
        } else {
            false
        };
        Self { active }
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = crossterm::terminal::disable_raw_mode();
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

fn render_output(s: &str, in_raw_mode: bool) {
    if in_raw_mode {
        let converted = s.replace("\r\n", "\n").replace('\n', "\r\n");
        print!("{}\r\n", converted);
    } else {
        println!("{}", s);
    }
    let _ = io::stdout().flush();
}

pub fn evaluate_exercise(
    exercise: &Exercise,
    all_exercises: &[Exercise],
    state: &mut AppState,
    mode: AccentMode,
    in_raw_mode: bool,
) -> Result<bool> {
    let content = fs::read_to_string(&exercise.path)?;
    let parsed_ex = Exercise::from_markdown(&exercise.path, &content)?;
    let user_answer = extract_user_answer(&parsed_ex, &content);

    clear_screen();
    let mut output = Vec::new();
    output.push(format!(
        "{} {} [{}] - {}",
        "Testing:".blue().bold(),
        parsed_ex.title.bold(),
        parsed_ex.id.cyan(),
        parsed_ex.path.display().to_string().dimmed()
    ));
    output.push(
        "----------------------------------------------------------"
            .dimmed()
            .to_string(),
    );

    let result = validate_submission(&parsed_ex, &user_answer, mode);

    match result {
        ValidationResult::Passed { notice } => {
            state.mark_completed(&parsed_ex.id);
            state.update_srs(&parsed_ex.id, 5, Utc::now());
            for concept in &parsed_ex.concept_tags {
                state.update_concept_mastery(concept, 5, Utc::now());
            }
            let _ = state.save();

            let completed_count = all_exercises
                .iter()
                .filter(|e| state.is_completed(&e.id))
                .count();
            let total_count = all_exercises.len();
            let percent = if total_count > 0 {
                (completed_count as f64 / total_count as f64 * 100.0).round() as usize
            } else {
                0
            };

            output.push(
                format!("✅ Passed: {}", parsed_ex.title)
                    .green()
                    .bold()
                    .to_string(),
            );
            output.push(format!("   Submission: '{}'", user_answer.cyan()));
            output.push(
                format!(
                    "   Progress:   [{}/{}] ({}%)",
                    completed_count, total_count, percent
                )
                .bold()
                .to_string(),
            );

            if let Some(note) = notice {
                output.push(format!("\n  ⚠️  {}", note.yellow()));
            }

            output.push(
                "----------------------------------------------------------"
                    .dimmed()
                    .to_string(),
            );
            output.push(format!(
                "{}: Next | {}: Previous | {}: Rerun | {}: Quit",
                "[n / Enter]".cyan().bold(),
                "[p]".cyan().bold(),
                "[r]".cyan().bold(),
                "[q]".cyan().bold()
            ));
            render_output(&output.join("\n"), in_raw_mode);
            Ok(true)
        }
        ValidationResult::Failed { diagnostic, .. } => {
            for concept in &parsed_ex.concept_tags {
                state.update_concept_mastery(concept, 1, Utc::now());
            }
            let _ = state.save();

            let completed_count = all_exercises
                .iter()
                .filter(|e| state.is_completed(&e.id))
                .count();
            let total_count = all_exercises.len();
            let percent = if total_count > 0 {
                (completed_count as f64 / total_count as f64 * 100.0).round() as usize
            } else {
                0
            };

            output.push(diagnostic.format_terminal());
            output.push(format!(
                "\n{} For hints, run: {}",
                "Need help?".yellow(),
                format!("spanglings hint {}", parsed_ex.id).cyan()
            ));
            output.push(format!(
                "For grammar rules, run: {}",
                format!("spanglings explain {}", parsed_ex.topic).cyan()
            ));
            output.push(
                format!(
                    "Progress: [{}/{}] ({}%)",
                    completed_count, total_count, percent
                )
                .dimmed()
                .to_string(),
            );
            output.push(
                "----------------------------------------------------------"
                    .dimmed()
                    .to_string(),
            );
            output.push(format!(
                "{}: Skip/Next | {}: Previous | {}: Rerun | {}: Quit",
                "[n]".cyan().bold(),
                "[p]".cyan().bold(),
                "[r]".cyan().bold(),
                "[q]".cyan().bold()
            ));
            render_output(&output.join("\n"), in_raw_mode);
            Ok(false)
        }
    }
}

pub fn render_all_completed(exercises: &[Exercise], in_raw_mode: bool) {
    clear_screen();
    let mut output = Vec::new();
    output.push(
        "=========================================================="
            .green()
            .to_string(),
    );
    output.push(
        "🎉 ¡FELICITACIONES! You have completed all curriculum exercises!"
            .green()
            .bold()
            .to_string(),
    );
    output.push(
        "=========================================================="
            .green()
            .to_string(),
    );
    output.push(format!(
        "Run '{}' to practice spaced repetition or '{}' for fast stem drills.",
        "spanglings review".cyan(),
        "spanglings drill".cyan()
    ));
    output.push(
        format!("Total completed: {}/{}", exercises.len(), exercises.len())
            .dimmed()
            .to_string(),
    );
    output.push(
        "----------------------------------------------------------"
            .dimmed()
            .to_string(),
    );
    output.push(format!(
        "{}: Next | {}: Previous | {}: Rerun | {}: Quit",
        "[n / Enter]".cyan().bold(),
        "[p]".cyan().bold(),
        "[r]".cyan().bold(),
        "[q]".cyan().bold()
    ));
    render_output(&output.join("\n"), in_raw_mode);
}

pub fn evaluate_current_exercise_in<P: AsRef<Path>>(
    exercises_dir: P,
    state: &mut AppState,
    mode: AccentMode,
) -> Result<bool> {
    let exercises = find_all_exercises(exercises_dir)?;
    if exercises.is_empty() {
        println!(
            "{}",
            "No exercises found in directory. Waiting for exercise files...".yellow()
        );
        return Ok(false);
    }

    let active_index = exercises.iter().position(|e| !state.is_completed(&e.id));

    match active_index {
        Some(idx) => evaluate_exercise(&exercises[idx], &exercises, state, mode, false),
        None => {
            render_all_completed(&exercises, false);
            Ok(true)
        }
    }
}

pub fn evaluate_current_exercise(mode: AccentMode) -> Result<bool> {
    let exercises_dir = Path::new("exercises");
    let mut state = AppState::load().unwrap_or_default();
    let result = evaluate_current_exercise_in(exercises_dir, &mut state, mode)?;
    state.save()?;
    Ok(result)
}

pub fn start_watch_mode(strict_accents: bool) -> Result<()> {
    let exercises_dir = Path::new("exercises");
    let mut exercises = find_all_exercises(exercises_dir).unwrap_or_default();
    let mut state = AppState::load().unwrap_or_default();

    let accent_mode = if strict_accents {
        AccentMode::Strict
    } else {
        AccentMode::Forgiving
    };

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(200), tx)?;

    if exercises_dir.exists() {
        debouncer
            .watcher()
            .watch(exercises_dir, RecursiveMode::Recursive)?;
    }

    let raw_guard = RawModeGuard::new();
    let in_raw = raw_guard.is_active();

    let mut current_index = exercises
        .iter()
        .position(|e| !state.is_completed(&e.id))
        .unwrap_or(0);

    let mut last_passed = false;

    let evaluate_at = |idx: usize, exercises: &[Exercise], state: &mut AppState| -> Result<bool> {
        if exercises.is_empty() {
            clear_screen();
            println!(
                "{}",
                "No exercises found in directory. Waiting for exercise files...".yellow()
            );
            return Ok(false);
        }
        if exercises.iter().all(|e| state.is_completed(&e.id)) && idx >= exercises.len() {
            render_all_completed(exercises, in_raw);
            return Ok(true);
        }
        let safe_idx = idx.min(exercises.len().saturating_sub(1));
        evaluate_exercise(&exercises[safe_idx], exercises, state, accent_mode, in_raw)
    };

    if let Ok(res) = evaluate_at(current_index, &exercises, &mut state) {
        last_passed = res;
    }

    loop {
        let mut had_file_change = false;
        while let Ok(res) = rx.try_recv() {
            match res {
                Ok(events) => {
                    if events
                        .iter()
                        .any(|e| e.path.extension().is_some_and(|ext| ext == "md"))
                    {
                        had_file_change = true;
                    }
                }
                Err(err) => eprintln!("Watcher error: {:?}", err),
            }
        }

        if had_file_change {
            if let Ok(updated) = find_all_exercises(exercises_dir) {
                exercises = updated;
            }
            if let Ok(res) = evaluate_at(current_index, &exercises, &mut state) {
                last_passed = res;
            }
        }

        if in_raw {
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                break;
                            }
                            KeyCode::Char('c')
                                if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                            {
                                break;
                            }
                            KeyCode::Char('n') | KeyCode::Right => {
                                if !exercises.is_empty() && current_index + 1 < exercises.len() {
                                    current_index += 1;
                                    if let Ok(res) =
                                        evaluate_at(current_index, &exercises, &mut state)
                                    {
                                        last_passed = res;
                                    }
                                } else if !exercises.is_empty()
                                    && exercises.iter().all(|e| state.is_completed(&e.id))
                                {
                                    current_index = exercises.len();
                                    render_all_completed(&exercises, in_raw);
                                }
                            }
                            KeyCode::Enter if last_passed => {
                                if !exercises.is_empty() && current_index + 1 < exercises.len() {
                                    current_index += 1;
                                    if let Ok(res) =
                                        evaluate_at(current_index, &exercises, &mut state)
                                    {
                                        last_passed = res;
                                    }
                                } else if !exercises.is_empty()
                                    && exercises.iter().all(|e| state.is_completed(&e.id))
                                {
                                    current_index = exercises.len();
                                    render_all_completed(&exercises, in_raw);
                                }
                            }
                            KeyCode::Char('p') | KeyCode::Left if current_index > 0 => {
                                current_index -= 1;
                                if let Ok(res) = evaluate_at(current_index, &exercises, &mut state)
                                {
                                    last_passed = res;
                                }
                            }
                            KeyCode::Char('r') => {
                                if let Ok(res) = evaluate_at(current_index, &exercises, &mut state)
                                {
                                    last_passed = res;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            if let Ok(res) = rx.recv_timeout(Duration::from_millis(200)) {
                match res {
                    Ok(events) => {
                        if events
                            .iter()
                            .any(|e| e.path.extension().is_some_and(|ext| ext == "md"))
                        {
                            if let Ok(updated) = find_all_exercises(exercises_dir) {
                                exercises = updated;
                            }
                            if let Ok(res) = evaluate_at(current_index, &exercises, &mut state) {
                                last_passed = res;
                            }
                        }
                    }
                    Err(err) => eprintln!("Watcher error: {:?}", err),
                }
            }
        }
    }

    Ok(())
}
