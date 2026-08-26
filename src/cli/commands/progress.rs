use crate::core::curriculum::{find_all_exercises_or_embedded, Level};
use crate::core::exercise::Exercise;
use crate::core::state::AppState;
use chrono::{DateTime, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelProgress {
    pub total: usize,
    pub completed: usize,
    pub percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicWeaknessStat {
    pub topic: String,
    pub total: usize,
    pub completed: usize,
    pub avg_ease_factor: f32,
    pub lapses: usize,
    pub due_reviews: usize,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressSummary {
    pub total: usize,
    pub completed: usize,
    pub percent: f64,
    pub due_reviews: usize,
    pub levels: std::collections::BTreeMap<String, LevelProgress>,
    pub weak_topics: Vec<TopicWeaknessStat>,
    pub recommendations: Vec<String>,
}

pub fn compute_weakness_profile(
    exercises: &[Exercise],
    state: &AppState,
    now: DateTime<Utc>,
) -> Vec<TopicWeaknessStat> {
    let mut topic_exercises: HashMap<String, Vec<&Exercise>> = HashMap::new();
    for ex in exercises {
        topic_exercises.entry(ex.topic.clone()).or_default().push(ex);
    }

    let mut profiles = Vec::new();
    for (topic, ex_list) in topic_exercises {
        let total = ex_list.len();
        let completed = ex_list
            .iter()
            .filter(|e| state.is_completed(&e.id) || e.is_done)
            .count();
        let mut ease_sum = 0.0f32;
        let mut srs_count = 0usize;
        let mut lapses = 0usize;
        let mut due_reviews = 0usize;

        for ex in &ex_list {
            if let Some(stat) = state.stats.get(&ex.id) {
                if stat.attempts > 1 {
                    lapses += (stat.attempts - 1) as usize;
                }
            }
            if let Some(srs) = state.srs.get(&ex.id) {
                ease_sum += srs.ease_factor;
                srs_count += 1;
                if srs.repetitions == 0 && srs.interval_days <= 1 {
                    lapses += 1;
                }
                if srs.next_review_due <= now {
                    due_reviews += 1;
                }
            }
        }

        let avg_ease_factor = if srs_count > 0 {
            ease_sum / srs_count as f32
        } else {
            2.5
        };

        let is_weak = avg_ease_factor < 2.35 || lapses > 0 || due_reviews > 0;
        if is_weak {
            let recommendation = if avg_ease_factor < 2.1 || lapses >= 2 {
                format!("spanglings explain {}", topic.replace('_', "-"))
            } else {
                format!("spanglings drill --topic {}", topic)
            };

            profiles.push(TopicWeaknessStat {
                topic,
                total,
                completed,
                avg_ease_factor,
                lapses,
                due_reviews,
                recommendation,
            });
        }
    }

    profiles.sort_by(|a, b| {
        let a_score = (2.5 - a.avg_ease_factor) * 10.0
            + (a.lapses as f32 * 2.0)
            + (a.due_reviews as f32 * 1.5);
        let b_score = (2.5 - b.avg_ease_factor) * 10.0
            + (b.lapses as f32 * 2.0)
            + (b.due_reviews as f32 * 1.5);
        b_score
            .partial_cmp(&a_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    profiles
}

pub fn get_progress_json() -> anyhow::Result<String> {
    let exercises = find_all_exercises_or_embedded("exercises")?;
    let state = AppState::load().unwrap_or_default();
    let now = Utc::now();

    let total = exercises.len();
    let completed = exercises
        .iter()
        .filter(|e| state.is_completed(&e.id) || e.is_done)
        .count();
    let percent = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let srs_items = &state.srs;
    let due_reviews = srs_items
        .values()
        .filter(|item| item.next_review_due <= now)
        .count();

    let levels = [Level::Baseline, Level::B1, Level::B2, Level::C1];
    let mut level_totals: HashMap<Level, usize> = HashMap::new();
    let mut level_completed: HashMap<Level, usize> = HashMap::new();

    for ex in &exercises {
        *level_totals.entry(ex.level).or_insert(0) += 1;
        if state.is_completed(&ex.id) || ex.is_done {
            *level_completed.entry(ex.level).or_insert(0) += 1;
        }
    }

    let mut levels_map = std::collections::BTreeMap::new();
    for lvl in levels {
        let tot = level_totals.get(&lvl).copied().unwrap_or(0);
        let comp = level_completed.get(&lvl).copied().unwrap_or(0);
        let lvl_pct = if tot > 0 {
            (comp as f64 / tot as f64) * 100.0
        } else {
            0.0
        };
        levels_map.insert(
            lvl.to_string(),
            LevelProgress {
                total: tot,
                completed: comp,
                percent: lvl_pct,
            },
        );
    }

    let weak_topics = compute_weakness_profile(&exercises, &state, now);
    let recommendations: Vec<String> = weak_topics
        .iter()
        .take(3)
        .map(|w| w.recommendation.clone())
        .collect();

    let summary = ProgressSummary {
        total,
        completed,
        percent,
        due_reviews,
        levels: levels_map,
        weak_topics,
        recommendations,
    };

    let json_str = serde_json::to_string_pretty(&summary)?;
    Ok(json_str)
}

pub fn show_progress(json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", get_progress_json()?);
        return Ok(());
    }
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises_or_embedded(exercises_dir)?;
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

    let weak_topics = compute_weakness_profile(&exercises, &state, now);
    println!("\n{}", "Targeted Weakness Profiler & Recommendations:".bold());
    if weak_topics.is_empty() {
        println!("  ✨ {}", "No critical weak areas detected! All reviewed topics in good standing.".green());
    } else {
        for w in weak_topics.iter().take(4) {
            println!(
                "  ⚠️  {:<22} (Avg Ease: {:.2}, Lapses: {}, Due: {}) -> {}",
                w.topic.yellow().bold(),
                w.avg_ease_factor,
                w.lapses,
                w.due_reviews,
                w.recommendation.cyan()
            );
        }
    }

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
