use colored::Colorize;
use rand::seq::SliceRandom;
use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlitzItem {
    pub topic: String,
    pub formula_cue: String,
    pub trigger_sentence: String,
    pub target_verb: String,
    pub target_subject: String,
    pub target: String,
    pub explanation: String,
}

impl From<crate::core::generator::DrillItem> for BlitzItem {
    fn from(item: crate::core::generator::DrillItem) -> Self {
        Self {
            topic: item.topic,
            formula_cue: item.formula_cue,
            trigger_sentence: item.trigger_sentence,
            target_verb: item.target_verb,
            target_subject: item.target_subject,
            target: item.target,
            explanation: item.explanation,
        }
    }
}

impl BlitzItem {
    pub fn format_prompt(&self, remaining_secs: u64, streak: usize) -> String {
        let concept_header =
            if let Some(concept) = crate::core::reference::get_grammar_concept(&self.topic) {
                if !concept.gloss.is_empty() {
                    format!("{} ({})", concept.title, concept.gloss)
                } else {
                    concept.title.to_string()
                }
            } else if self.topic.is_empty() {
                String::new()
            } else {
                let parts: Vec<String> = self
                    .topic
                    .split('_')
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect();
                parts.join(" ")
            };

        let badge = if self.formula_cue.is_empty() {
            format!("[{concept_header}]")
        } else if concept_header.is_empty() {
            format!("[{}]", self.formula_cue)
        } else {
            format!("[{concept_header} | {}]", self.formula_cue)
        };

        format!(
            "[{remaining_secs}s remaining | Streak: {streak}] {}\nSentence: \"{}\" (verb: {} | subject: {}) > ",
            badge, self.trigger_sentence, self.target_verb, self.target_subject
        )
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BlitzResult {
    pub total_answered: usize,
    pub correct: usize,
    pub incorrect: usize,
    pub accuracy: f64,
    pub max_streak: usize,
    pub duration_secs: u64,
    pub items_per_minute: f64,
}

pub fn get_blitz_items(topic_filter: Option<&str>) -> Vec<BlitzItem> {
    crate::cli::commands::drill::get_drill_items(topic_filter)
        .into_iter()
        .map(BlitzItem::from)
        .collect()
}

pub fn evaluate_blitz_answer(item: &BlitzItem, user_input: &str) -> bool {
    let clean_user = user_input.trim().to_lowercase();
    let clean_target = item.target.trim().to_lowercase();
    if clean_user.is_empty() {
        return false;
    }
    if clean_user == clean_target {
        return true;
    }
    // Forgiving accent check in blitz
    crate::engine::accents::strip_accents(&clean_user)
        == crate::engine::accents::strip_accents(&clean_target)
}

pub fn run_blitz(
    duration_secs: Option<u64>,
    topic: Option<&str>,
    weak: bool,
    level: Option<&str>,
    track: Option<usize>,
) -> anyhow::Result<BlitzResult> {
    let parsed_level = level
        .map(|l| l.parse::<crate::core::curriculum::Level>())
        .transpose()?;
    let mut state = crate::core::state::AppState::load().unwrap_or_default();
    let initial_masteries = state.get_concept_mastery_scores();

    let duration_limit = Duration::from_secs(duration_secs.unwrap_or(60));
    let filter = crate::cli::commands::drill::DrillFilter {
        weak_only: weak,
        topic: topic.map(|s| s.to_string()),
        level: parsed_level,
        track,
        count: 100,
    };

    let drill_items = crate::cli::commands::drill::select_drill_items(&state, filter);
    let mut items: Vec<BlitzItem> = if !drill_items.is_empty() {
        drill_items.into_iter().map(BlitzItem::from).collect()
    } else {
        get_blitz_items(topic)
    };

    if items.is_empty() {
        anyhow::bail!("No blitz items found for the given criteria");
    }

    // Shuffle items randomly so each blitz run is fresh
    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);

    let topic_display = if weak {
        "Weakest Concepts (Adaptive)".to_string()
    } else if let Some(t) = topic {
        t.to_string()
    } else if let Some(ref lvl) = parsed_level {
        format!("Level {}", lvl)
    } else if let Some(tr) = track {
        format!("Track {}", tr)
    } else {
        "All Mixed Topics".to_string()
    };

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "{}",
        "          ⚡ SPANGLINGS RAPID-FIRE BLITZ MODE ⚡           ".bold()
    );
    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "Time limit: {} seconds | Topic: {}",
        duration_limit.as_secs().to_string().yellow().bold(),
        topic_display.cyan().bold()
    );
    println!("Answer as many as you can before the timer expires! Press Ctrl+C to abort.\n");

    if let Some(t) = topic {
        if let Some(sheet) = crate::cli::commands::drill::get_topic_cheat_sheet(t) {
            println!("{}", "--- [TOPIC CHEAT SHEET] ---".yellow().bold());
            println!("{}\n", sheet.cyan());
            println!("{}", "---------------------------".yellow());
        }
    }

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let start_time = Instant::now();

    let mut total_answered: usize = 0;
    let mut correct: usize = 0;
    let mut current_streak: usize = 0;
    let mut max_streak: usize = 0;
    let mut attempted_topics = Vec::new();

    let mut index = 0;
    'blitz: while start_time.elapsed() < duration_limit {
        let item = &items[index % items.len()];
        index += 1;
        let mut hint_used = false;

        loop {
            let remaining = duration_limit.saturating_sub(start_time.elapsed());
            if start_time.elapsed() >= duration_limit {
                println!("\n⏳ {}", "Time's up!".red().bold());
                break 'blitz;
            }

            print!(
                "{}",
                item.format_prompt(remaining.as_secs(), current_streak)
            );
            io::stdout().flush()?;

            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 {
                // EOF
                println!();
                break 'blitz;
            }

            if start_time.elapsed() >= duration_limit {
                println!("\n⏳ {}", "Time's up!".red().bold());
                break 'blitz;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.eq_ignore_ascii_case("?") || trimmed.eq_ignore_ascii_case("hint") {
                hint_used = true;
                println!("  💡 Hint: {}\n", item.explanation.yellow());
                continue;
            }

            let now = chrono::Utc::now();
            attempted_topics.push(item.topic.clone());
            total_answered += 1;

            if evaluate_blitz_answer(item, trimmed) {
                current_streak += 1;
                if current_streak > max_streak {
                    max_streak = current_streak;
                }
                correct += 1;
                let quality = if hint_used { 3 } else { 5 };
                state.update_concept_mastery(&item.topic, quality, now);
                let _ = state.save();
                println!("  {} Correct!\n", "✓".green().bold());
            } else {
                current_streak = 0;
                state.update_concept_mastery(&item.topic, 1, now);
                let _ = state.save();
                println!(
                    "  {} Incorrect. Expected: '{}' ({})\n",
                    "✗".red().bold(),
                    item.target.green().bold(),
                    item.explanation.dimmed()
                );
            }
            break;
        }
    }

    let elapsed = start_time.elapsed();
    let actual_secs = elapsed.as_secs().max(1);
    let incorrect = total_answered.saturating_sub(correct);
    let accuracy = if total_answered > 0 {
        (correct as f64 / total_answered as f64) * 100.0
    } else {
        0.0
    };
    let items_per_minute = (total_answered as f64 / actual_secs as f64) * 60.0;

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "{}",
        "                      BLITZ FINISHED!                     ".bold()
    );
    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "  • Total Answered:   {}",
        total_answered.to_string().cyan().bold()
    );
    println!(
        "  • Correct Answers:  {}",
        correct.to_string().green().bold()
    );
    println!("  • Accuracy:         {:.1}%", accuracy);
    println!(
        "  • Max Streak:       {}",
        max_streak.to_string().yellow().bold()
    );
    println!("  • Speed:            {:.1} answers/min", items_per_minute);
    println!(
        "{}",
        "==========================================================".blue()
    );

    let final_masteries = state.get_concept_mastery_scores();
    let touched_topics: std::collections::BTreeSet<String> = attempted_topics.into_iter().collect();
    if !touched_topics.is_empty() {
        println!("\n📊 Concept Mastery Progress:");
        for topic in &touched_topics {
            let old_score = initial_masteries.get(topic).copied().unwrap_or(0.0);
            let new_score = final_masteries.get(topic).copied().unwrap_or(0.0);
            let old_pct = (old_score * 100.0).round() as i32;
            let new_pct = (new_score * 100.0).round() as i32;
            let delta = new_pct - old_pct;
            let title = if let Some(concept) = crate::core::reference::get_grammar_concept(topic) {
                concept.title.to_string()
            } else {
                let parts: Vec<String> = topic
                    .split(['_', '-'])
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect();
                parts.join(" ")
            };
            let delta_str = if delta > 0 {
                format!("(+{}%)", delta).green().bold()
            } else if delta < 0 {
                format!("({}%)", delta).red().bold()
            } else {
                "(±0%)".dimmed()
            };
            println!(
                "  • {:<16} {:>3}% ➔ {:>3}% {}",
                format!("{}:", title),
                old_pct,
                new_pct,
                delta_str
            );
        }
    }

    Ok(BlitzResult {
        total_answered,
        correct,
        incorrect,
        accuracy,
        max_streak,
        duration_secs: actual_secs,
        items_per_minute,
    })
}
