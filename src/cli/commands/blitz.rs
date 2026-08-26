use colored::Colorize;
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
    let all_items = vec![
        // Irregular Preterite Stems
        BlitzItem {
            prompt: "Preterite Stem: 'tener' (yo tuve -> stem: ?)",
            target: "tuv",
            topic: "preterite",
            explanation: "tener -> tuv- (tuve, tuviste, tuvo)",
        },
        BlitzItem {
            prompt: "Preterite Stem: 'poner' (yo puse -> stem: ?)",
            target: "pus",
            topic: "preterite",
            explanation: "poner -> pus- (puse, pusiste, puso)",
        },
        BlitzItem {
            prompt: "Preterite Stem: 'saber' (yo supe -> stem: ?)",
            target: "sup",
            topic: "preterite",
            explanation: "saber -> sup- (supe, supiste, supo)",
        },
        BlitzItem {
            prompt: "Preterite Stem: 'hacer' (yo hice -> stem: ?)",
            target: "hic",
            topic: "preterite",
            explanation: "hacer -> hic- (hice, hiciste, hizo)",
        },
        BlitzItem {
            prompt: "Preterite Stem: 'decir' (yo dije -> stem: ?)",
            target: "dij",
            topic: "preterite",
            explanation: "decir -> dij- (dije, dijiste, dijo)",
        },
        BlitzItem {
            prompt: "Preterite Stem: 'estar' (yo estuve -> stem: ?)",
            target: "estuv",
            topic: "preterite",
            explanation: "estar -> estuv- (estuve, estuviste, estuvo)",
        },
        BlitzItem {
            prompt: "Preterite Stem: 'querer' (yo quise -> stem: ?)",
            target: "quis",
            topic: "preterite",
            explanation: "querer -> quis- (quise, quisiste, quiso)",
        },
        BlitzItem {
            prompt: "Preterite Stem: 'venir' (yo vine -> stem: ?)",
            target: "vin",
            topic: "preterite",
            explanation: "venir -> vin- (vine, viniste, vino)",
        },
        BlitzItem {
            prompt: "Preterite Stem: 'caber' (yo cupe -> stem: ?)",
            target: "cup",
            topic: "preterite",
            explanation: "caber -> cup- (cupe, cupiste, cupo)",
        },
        BlitzItem {
            prompt: "Preterite Stem: 'andar' (yo anduve -> stem: ?)",
            target: "anduv",
            topic: "preterite",
            explanation: "andar -> anduv- (anduve, anduviste, anduvo)",
        },
        // Subjunctive Forms
        BlitzItem {
            prompt: "Subjunctive 'yo': 'tener' (que yo...)",
            target: "tenga",
            topic: "subjunctive",
            explanation: "yo tengo -> drop -o -> tenga",
        },
        BlitzItem {
            prompt: "Subjunctive 'yo': 'salir' (que yo...)",
            target: "salga",
            topic: "subjunctive",
            explanation: "yo salgo -> drop -o -> salga",
        },
        BlitzItem {
            prompt: "Subjunctive 'yo': 'poner' (que yo...)",
            target: "ponga",
            topic: "subjunctive",
            explanation: "yo pongo -> drop -o -> ponga",
        },
        BlitzItem {
            prompt: "Subjunctive 'yo': 'decir' (que yo...)",
            target: "diga",
            topic: "subjunctive",
            explanation: "yo digo -> drop -o -> diga",
        },
        BlitzItem {
            prompt: "Subjunctive 'yo': 'hacer' (que yo...)",
            target: "haga",
            topic: "subjunctive",
            explanation: "yo hago -> drop -o -> haga",
        },
        BlitzItem {
            prompt: "Subjunctive 'yo': 'ver' (que yo...)",
            target: "vea",
            topic: "subjunctive",
            explanation: "yo veo -> drop -o -> vea",
        },
        BlitzItem {
            prompt: "Subjunctive 'yo': 'caber' (que yo...)",
            target: "quepa",
            topic: "subjunctive",
            explanation: "caber -> quepa, quepas, quepa",
        },
        BlitzItem {
            prompt: "Subjunctive 'yo': 'ir' (que yo...)",
            target: "vaya",
            topic: "subjunctive",
            explanation: "ir -> vaya, vayas, vaya",
        },
        BlitzItem {
            prompt: "Subjunctive 'yo': 'saber' (que yo...)",
            target: "sepa",
            topic: "subjunctive",
            explanation: "saber -> sepa, sepas, sepa",
        },
        BlitzItem {
            prompt: "Subjunctive 'yo': 'ser' (que yo...)",
            target: "sea",
            topic: "subjunctive",
            explanation: "ser -> sea, seas, sea",
        },
        // Clitics & Pronouns
        BlitzItem {
            prompt: "Replace 'le lo' with cacophony rule: 'Le doy el libro' -> '___ doy'",
            target: "se lo",
            topic: "pronouns",
            explanation: "le + lo -> se lo",
        },
        BlitzItem {
            prompt: "Replace 'les las' with cacophony rule: 'Les compro las flores' -> '___ compro'",
            target: "se las",
            topic: "pronouns",
            explanation: "les + las -> se las",
        },
        // False friends
        BlitzItem {
            prompt: "Translate 'currently / at present' to Spanish (looks like 'actually'):",
            target: "actualmente",
            topic: "false_friends",
            explanation: "actualmente = currently",
        },
        BlitzItem {
            prompt: "Translate 'to pretend / feign' to Spanish (verb starting with f):",
            target: "fingir",
            topic: "false_friends",
            explanation: "fingir = to pretend / feign",
        },
        BlitzItem {
            prompt: "Translate 'sensible / prudent' to Spanish (not sensible):",
            target: "sensato",
            topic: "false_friends",
            explanation: "sensato = sensible/prudent; sensible = sensitive",
        },
    ];

    if let Some(filt) = topic_filter {
        let f = filt.to_lowercase().replace('_', "-");
        all_items
            .into_iter()
            .filter(|item| item.topic.replace('_', "-").contains(&f))
            .collect()
    } else {
        all_items
    }
}

pub fn evaluate_blitz_answer(item: &BlitzItem, user_input: &str) -> bool {
    let clean_user = user_input.trim().to_lowercase();
    let clean_target = item.target.trim().to_lowercase();
    clean_user == clean_target
}

pub fn run_blitz(duration_secs: Option<u64>, topic: Option<&str>) -> anyhow::Result<BlitzResult> {
    let duration_limit = Duration::from_secs(duration_secs.unwrap_or(60));
    let items = get_blitz_items(topic);
    if items.is_empty() {
        anyhow::bail!("No blitz items found for topic: {:?}", topic);
    }

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
    println!("  • Total Answered:   {}", total_answered.to_string().cyan().bold());
    println!("  • Correct Answers:  {}", correct.to_string().green().bold());
    println!("  • Accuracy:         {:.1}%", accuracy);
    println!("  • Max Streak:       {}", max_streak.to_string().yellow().bold());
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
