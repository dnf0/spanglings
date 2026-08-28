use colored::Colorize;
use rand::seq::SliceRandom;
use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BlitzItem {
    pub prompt: &'static str,
    pub target: &'static str,
    pub topic: &'static str,
    pub explanation: &'static str,
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
        .map(|item| BlitzItem {
            prompt: item.prompt,
            target: item.target,
            topic: item.topic,
            explanation: item.explanation,
        })
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

pub fn run_blitz(duration_secs: Option<u64>, topic: Option<&str>) -> anyhow::Result<BlitzResult> {
    let duration_limit = Duration::from_secs(duration_secs.unwrap_or(60));
    let mut items = get_blitz_items(topic);
    if items.is_empty() {
        anyhow::bail!("No blitz items found for topic: {:?}", topic);
    }

    // Shuffle items randomly so each blitz run is fresh
    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);

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
        topic.unwrap_or("All Mixed Topics").cyan().bold()
    );
    println!("Answer as many as you can before the timer expires! Press Ctrl+C to abort.\n");

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let start_time = Instant::now();

    let mut total_answered: usize = 0;
    let mut correct: usize = 0;
    let mut current_streak: usize = 0;
    let mut max_streak: usize = 0;

    let mut index = 0;
    while start_time.elapsed() < duration_limit {
        let remaining = duration_limit.saturating_sub(start_time.elapsed());
        let item = &items[index % items.len()];
        index += 1;

        print!(
            "[{:02}s remaining | Streak: {}] {} > ",
            remaining.as_secs(),
            current_streak.to_string().yellow().bold(),
            item.prompt.bright_white()
        );
        io::stdout().flush()?;

        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            // EOF
            println!();
            break;
        }

        if start_time.elapsed() >= duration_limit {
            println!("\n⏳ {}", "Time's up!".red().bold());
            break;
        }

        total_answered += 1;
        if evaluate_blitz_answer(item, &line) {
            current_streak += 1;
            if current_streak > max_streak {
                max_streak = current_streak;
            }
            correct += 1;
            println!("  {} Correct!\n", "✓".green().bold());
        } else {
            current_streak = 0;
            println!(
                "  {} Incorrect. Expected: '{}' ({})\n",
                "✗".red().bold(),
                item.target.green().bold(),
                item.explanation.dimmed()
            );
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
