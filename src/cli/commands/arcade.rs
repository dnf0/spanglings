use chrono::Utc;
use colored::Colorize;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{Duration, Instant};

use crate::core::arcade::{
    generate_4choice_items, generate_showdown_items, list_showdown_pairs, ArcadeItem, ShowdownPair,
};
use crate::core::state::AppState;

/// Tracks performance, combo streaks, speed, and scoring throughout an arcade session.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ArcadeSessionStats {
    pub total_answered: usize,
    pub correct_count: usize,
    pub incorrect_count: usize,
    pub current_streak: usize,
    pub best_streak: usize,
    pub score: u64,
    pub total_time_ms: u128,
}

/// Result of evaluating a single arcade choice selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArcadeChoiceResult {
    pub is_correct: bool,
    pub points_earned: u64,
    pub speed_bonus: u64,
    pub streak_multiplier: u64,
    pub response_time_ms: u128,
}

/// Evaluates a user's choice, updates streaks, calculates combo multiplier and speed bonuses.
pub fn evaluate_arcade_choice(
    item: &ArcadeItem,
    selected_idx: usize,
    response_time_ms: u128,
    stats: &mut ArcadeSessionStats,
) -> ArcadeChoiceResult {
    let is_correct = item.is_correct(selected_idx);
    stats.total_answered += 1;
    stats.total_time_ms += response_time_ms;

    if is_correct {
        stats.correct_count += 1;
        stats.current_streak += 1;
        if stats.current_streak > stats.best_streak {
            stats.best_streak = stats.current_streak;
        }

        // Streak multiplier: 1x to 5x max
        let streak_multiplier = (stats.current_streak as u64).clamp(1, 5);

        // Speed bonus based on response time:
        // <= 500ms: 150 pts
        // <= 800ms: 100 pts
        // <= 1200ms: 50 pts
        // > 1200ms: 0 pts
        let speed_bonus = if response_time_ms <= 500 {
            150
        } else if response_time_ms <= 800 {
            100
        } else if response_time_ms <= 1200 {
            50
        } else {
            0
        };

        let base_points = 100u64;
        let points_earned = (base_points + speed_bonus) * streak_multiplier;
        stats.score += points_earned;

        ArcadeChoiceResult {
            is_correct: true,
            points_earned,
            speed_bonus,
            streak_multiplier,
            response_time_ms,
        }
    } else {
        stats.incorrect_count += 1;
        stats.current_streak = 0;

        ArcadeChoiceResult {
            is_correct: false,
            points_earned: 0,
            speed_bonus: 0,
            streak_multiplier: 1,
            response_time_ms,
        }
    }
}

/// Returns a descriptive rank title for a combo streak.
pub fn get_combo_rank(streak: usize) -> &'static str {
    match streak {
        0..=1 => "✨ Good",
        2..=4 => "⚡ Quick Focus",
        5..=9 => "🔥 ON FIRE",
        10..=19 => "💥 UNSTOPPABLE",
        _ => "👑 ULTRA INSTINCT",
    }
}

/// Plays audio feedback cue (non-blocking afplay on macOS or terminal bell on other systems).
pub fn play_arcade_sound(is_correct: bool, sound_enabled: bool) {
    if !sound_enabled {
        return;
    }

    #[cfg(target_os = "macos")]
    {
        let sound_file = if is_correct {
            "/System/Library/Sounds/Tink.aiff"
        } else {
            "/System/Library/Sounds/Sosumi.aiff"
        };
        let _ = std::process::Command::new("afplay")
            .arg(sound_file)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }

    #[cfg(not(target_os = "macos"))]
    {
        if is_correct {
            print!("\x07");
        } else {
            print!("\x07\x07");
        }
        let _ = std::io::stdout().flush();
    }
}

/// Selects arcade items based on showdown pair, concept, weakness targeting, or mixed mode.
pub fn select_arcade_items(
    showdown: Option<&str>,
    concept: Option<&str>,
    weak: bool,
    count: usize,
    state: &AppState,
) -> Vec<ArcadeItem> {
    if count == 0 {
        return Vec::new();
    }

    // 1. Specific showdown pair requested
    if let Some(s) = showdown {
        if let Some(pair) = ShowdownPair::from_str(s) {
            return generate_showdown_items(pair, count);
        }
    }

    // 2. Specific concept requested
    if let Some(c) = concept {
        let items = generate_4choice_items(c, count);
        if !items.is_empty() {
            return items;
        }
        if let Some(pair) = ShowdownPair::from_str(c) {
            return generate_showdown_items(pair, count);
        }
    }

    // 3. Weakness targeting mode
    if weak {
        let weakest = state.get_weakest_concepts(5);
        let weak_slugs: Vec<String> = weakest
            .into_iter()
            .filter(|(_, mastery)| mastery.mastery_score < 0.85)
            .map(|(id, _)| id.clone())
            .collect();

        if !weak_slugs.is_empty() {
            let mut items = Vec::with_capacity(count);
            let mut rng = rand::thread_rng();
            for i in 0..count {
                let slug = &weak_slugs[i % weak_slugs.len()];
                if let Some(pair) = ShowdownPair::from_str(slug) {
                    let mut pair_items = generate_showdown_items(pair, 1);
                    if let Some(it) = pair_items.pop() {
                        items.push(it);
                    }
                } else {
                    let mut c_items = generate_4choice_items(slug, 1);
                    if let Some(it) = c_items.pop() {
                        items.push(it);
                    }
                }
            }
            if !items.is_empty() {
                items.shuffle(&mut rng);
                return items;
            }
        }
    }

    // 4. Default: Mixed pool of Showdown pairs and 4-choice items
    let mut items = Vec::with_capacity(count);
    let pairs = list_showdown_pairs();
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        let use_showdown = rng.gen_bool(0.6);
        if use_showdown && !pairs.is_empty() {
            let pair = pairs
                .choose(&mut rng)
                .copied()
                .unwrap_or(ShowdownPair::PorPara);
            let mut showdown_items = generate_showdown_items(pair, 1);
            if let Some(item) = showdown_items.pop() {
                items.push(item);
                continue;
            }
        }

        let concepts = crate::core::reference::list_grammar_concepts();
        let concept = concepts
            .choose(&mut rng)
            .map(|c| c.slug)
            .unwrap_or("subjunctive");
        let mut choice_items = generate_4choice_items(concept, 1);
        if let Some(item) = choice_items.pop() {
            items.push(item);
        }
    }

    if items.len() < count {
        let mut fallback = generate_showdown_items(ShowdownPair::PorPara, count - items.len());
        items.append(&mut fallback);
    }

    items.shuffle(&mut rng);
    items
}

/// RAII Drop Guard to restore normal terminal mode automatically.
struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
    }
}

/// Runs the rapid single-key arcade session in raw terminal mode.
pub fn run_arcade(
    showdown: Option<String>,
    concept: Option<String>,
    weak: bool,
    count: Option<usize>,
    sound: bool,
    _strict_accents: bool,
) -> std::io::Result<()> {
    let count = count.unwrap_or(10);
    let mut state = AppState::load().unwrap_or_default();
    let initial_masteries = state.get_concept_mastery_scores();

    let items = select_arcade_items(showdown.as_deref(), concept.as_deref(), weak, count, &state);

    if items.is_empty() {
        println!("{}", "No arcade items could be generated.".yellow());
        return Ok(());
    }

    println!("\r\n{}", "⚡ SPANGLINGS RAPID ARCADE".bold().cyan());
    println!(
        "{}",
        "Single-Key Zero-Friction Drills (No Enter required!)".dimmed()
    );
    if weak {
        println!("{}", "🎯 Mode: Adaptive Weakness Targeting".yellow());
    } else if let Some(ref s) = showdown {
        println!("🎯 Mode: Showdown Duel ({})", s.bold().magenta());
    } else if let Some(ref c) = concept {
        println!("🎯 Mode: Concept Cloze ({})", c.bold().blue());
    } else {
        println!("{}", "🎯 Mode: Mixed Rapid Arcade".green());
    }
    if sound {
        println!("{}", "🔊 Sound effects enabled".dimmed());
    }
    println!(
        "{}\r\n",
        "Press any key to start... (or Ctrl+C / q to cancel)".dimmed()
    );

    crossterm::terminal::enable_raw_mode()?;
    let _guard = RawModeGuard;

    let mut stats = ArcadeSessionStats::default();
    let total_items = items.len();

    for (idx, item) in items.iter().enumerate() {
        let q_num = idx + 1;
        let combo_title = get_combo_rank(stats.current_streak);
        let multiplier = (stats.current_streak as u64).clamp(1, 5);

        // Header info bar
        print!("\r\n\r\n");
        print!(
            "⚡ [Q {}/{}] Score: {} | Combo: {} (x{} Multiplier) | Best Streak: {}\r\n",
            q_num,
            total_items,
            stats.score.to_string().bold().yellow(),
            format!("🔥 {}x [{}]", stats.current_streak, combo_title)
                .bold()
                .magenta(),
            multiplier,
            stats.best_streak.to_string().bold().green()
        );
        print!("{}\r\n", "─".repeat(70).dimmed());

        // Display cue & sentence
        let cue_badge = if !item.prompt_cue.is_empty() {
            format!("[{}]", item.prompt_cue)
        } else {
            format!("[{}]", item.topic)
        };
        print!("{} {}\r\n", "Cue:".bold().cyan(), cue_badge.dimmed());
        print!(
            "{} {}\r\n\r\n",
            "Sentence:".bold(),
            item.trigger_sentence.bold().white()
        );

        // Display options
        if item.options.len() == 2 {
            print!(
                "   {}  {}             {}  {}\r\n",
                "[ J / ← / 1 ]".bold().cyan(),
                item.options[0].bold().bright_white(),
                "[ K / → / 2 ]".bold().cyan(),
                item.options[1].bold().bright_white()
            );
        } else {
            let mut opt_str = String::new();
            for (opt_i, opt) in item.options.iter().enumerate() {
                opt_str.push_str(&format!(
                    "  {} {}   ",
                    format!("[ {} ]", opt_i + 1).bold().cyan(),
                    opt.bold().bright_white()
                ));
            }
            print!("{}\r\n", opt_str);
        }

        print!("{}\r\n", "─".repeat(70).dimmed());
        print!(
            "{}\r\n",
            "⚡ Press key to answer • [q] or [Esc] to exit".dimmed()
        );
        io::stdout().flush()?;

        // Wait for single key event
        let start_time = Instant::now();
        let mut chosen_idx: Option<usize> = None;
        let mut should_exit = false;

        loop {
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        // Exit handling
                        if key_event.code == KeyCode::Char('c')
                            && key_event.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            should_exit = true;
                            break;
                        }
                        match key_event.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                                should_exit = true;
                                break;
                            }
                            // Showdown binary keys
                            KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Left
                                if !item.options.is_empty() =>
                            {
                                chosen_idx = Some(0);
                                break;
                            }
                            KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Right
                                if item.options.len() >= 2 =>
                            {
                                chosen_idx = Some(1);
                                break;
                            }
                            // Number keys 1..=4
                            KeyCode::Char('1') if !item.options.is_empty() => {
                                chosen_idx = Some(0);
                                break;
                            }
                            KeyCode::Char('2') if item.options.len() >= 2 => {
                                chosen_idx = Some(1);
                                break;
                            }
                            KeyCode::Char('3') if item.options.len() >= 3 => {
                                chosen_idx = Some(2);
                                break;
                            }
                            KeyCode::Char('4') if item.options.len() >= 4 => {
                                chosen_idx = Some(3);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if should_exit {
            print!("\r\n{}\r\n", "Arcade session interrupted by user.".yellow());
            break;
        }

        let selected = chosen_idx.unwrap_or(0);
        let elapsed_ms = start_time.elapsed().as_millis();
        let eval_result = evaluate_arcade_choice(item, selected, elapsed_ms, &mut stats);

        // Update SM-2 concept mastery in app state
        let quality = if eval_result.is_correct {
            if elapsed_ms < 800 {
                5
            } else if elapsed_ms < 1500 {
                4
            } else {
                3
            }
        } else {
            1
        };
        state.update_concept_mastery(&item.topic, quality, Utc::now());
        let _ = state.save();

        // Visual flash & audio sound
        play_arcade_sound(eval_result.is_correct, sound);

        if eval_result.is_correct {
            let speed_tag = if eval_result.speed_bonus > 0 {
                format!(
                    " (+{} speed bonus [{}ms])",
                    eval_result.speed_bonus, elapsed_ms
                )
                .yellow()
            } else {
                format!(" ({}ms)", elapsed_ms).dimmed()
            };
            print!(
                "\r\n{} {}{}\r\n   {}\r\n",
                "✓ CORRECT!".bold().green(),
                format!("+{} PTS", eval_result.points_earned)
                    .bold()
                    .yellow(),
                speed_tag,
                item.explanation.dimmed()
            );
        } else {
            print!(
                "\r\n{} Correct answer: {} ({}ms)\r\n   {}\r\n",
                "✗ INCORRECT!".bold().red(),
                item.correct_option().bold().green(),
                elapsed_ms.to_string().dimmed(),
                item.explanation.dimmed()
            );
        }
        io::stdout().flush()?;

        // Short visual flash sleep
        std::thread::sleep(Duration::from_millis(250));
    }

    // Disable raw mode explicitly before printing summary
    let _ = crossterm::terminal::disable_raw_mode();

    // Print end-of-session dopamine summary
    print_arcade_summary(&stats, &initial_masteries, &state);

    Ok(())
}

/// Prints formatted dopamine session summary with score, accuracy, best streak, and concept deltas.
pub fn print_arcade_summary(
    stats: &ArcadeSessionStats,
    initial_masteries: &HashMap<String, f32>,
    final_state: &AppState,
) {
    println!(
        "\n{}",
        "═══════════════════════════════════════════════════════════════".dimmed()
    );
    println!(
        "{}",
        "          ⚡ ARCADE DOPAMINE SESSION SUMMARY ⚡          "
            .bold()
            .cyan()
    );
    println!(
        "{}\n",
        "═══════════════════════════════════════════════════════════════".dimmed()
    );

    let accuracy = if stats.total_answered > 0 {
        (stats.correct_count as f64 / stats.total_answered as f64) * 100.0
    } else {
        0.0
    };

    let avg_time_ms = if stats.total_answered > 0 {
        stats.total_time_ms / stats.total_answered as u128
    } else {
        0
    };

    let rank = get_combo_rank(stats.best_streak);

    println!(
        "  {} {}",
        "Total Answered:".bold(),
        stats.total_answered.to_string().white()
    );
    println!(
        "  {} {} correct, {} incorrect ({:.1}%)",
        "Accuracy:      ".bold(),
        stats.correct_count.to_string().green(),
        stats.incorrect_count.to_string().red(),
        accuracy
    );
    println!(
        "  {} {}",
        "Total Score:   ".bold(),
        stats.score.to_string().bold().yellow()
    );
    println!(
        "  {} {} ({})",
        "Best Streak:   ".bold(),
        stats.best_streak.to_string().bold().magenta(),
        rank
    );
    println!(
        "  {} {}ms / question",
        "Average Speed: ".bold(),
        avg_time_ms.to_string().cyan()
    );

    // Concept mastery deltas
    let current_masteries = final_state.get_concept_mastery_scores();
    let mut updated_concepts: Vec<_> = current_masteries
        .iter()
        .filter(|(k, v)| {
            let initial = initial_masteries.get(*k).copied().unwrap_or(0.0);
            (**v - initial).abs() > 0.001
        })
        .collect();

    if !updated_concepts.is_empty() {
        println!("\n{}", "📈 Concept Mastery Deltas:".bold().green());
        updated_concepts.sort_by(|a, b| a.0.cmp(b.0));
        for (concept_id, &curr) in updated_concepts {
            let prev = initial_masteries.get(concept_id).copied().unwrap_or(0.0);
            let delta = curr - prev;
            let delta_str = if delta >= 0.0 {
                format!("+{:.2}", delta).green()
            } else {
                format!("{:.2}", delta).red()
            };
            println!(
                "  • {}: {:.0}% ➔ {:.0}% ({})",
                concept_id.bold(),
                prev * 100.0,
                curr * 100.0,
                delta_str
            );
        }
    }

    println!(
        "\n{}",
        "═══════════════════════════════════════════════════════════════".dimmed()
    );
}
