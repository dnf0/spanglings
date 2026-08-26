use colored::Colorize;
use std::io::{self, BufRead, Write};

struct DrillItem {
    prompt: &'static str,
    target: &'static str,
    explanation: &'static str,
}

pub fn run_drill(topic: Option<&str>) -> anyhow::Result<()> {
    let t = topic.unwrap_or("all").to_lowercase();

    let preterite_drills = vec![
        DrillItem {
            prompt: "Irregular Preterite Stem for 'tener' (yo tuve -> stem: ?)",
            target: "tuv",
            explanation: "tener -> tuv- (tuve, tuviste, tuvo, tuvimos, tuvieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'poner' (yo puse -> stem: ?)",
            target: "pus",
            explanation: "poner -> pus- (puse, pusiste, puso, pusimos, pusieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'saber' (yo supe -> stem: ?)",
            target: "sup",
            explanation: "saber -> sup- (supe, supiste, supo, supimos, supieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'hacer' (yo hice -> stem: ?)",
            target: "hic",
            explanation: "hacer -> hic- (hice, hiciste, hizo, hicimos, hicieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'decir' (yo dije -> stem: ?)",
            target: "dij",
            explanation: "decir -> dij- (dije, dijiste, dijo, dijimos, dijeron)",
        },
    ];

    let subjunctive_drills = vec![
        DrillItem {
            prompt: "Present Subjunctive 'yo' root for 'tener' (que yo...)",
            target: "tenga",
            explanation: "yo tengo -> drop -o -> add -a -> tenga",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'ir' (que yo...)",
            target: "vaya",
            explanation: "ir -> vaya, vayas, vaya, vayamos, vayáis, vayan",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'saber' (que yo...)",
            target: "sepa",
            explanation: "saber -> sepa, sepas, sepa, sepamos, sepáis, sepan",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'ser' (que yo...)",
            target: "sea",
            explanation: "ser -> sea, seas, sea, seamos, seáis, sean",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' root for 'hacer' (que yo...)",
            target: "haga",
            explanation: "yo hago -> drop -o -> add -a -> haga",
        },
    ];

    let items = match t.as_str() {
        "preterite" | "pret" | "past" => preterite_drills,
        "subjunctive" | "subj" => subjunctive_drills,
        _ => {
            let mut combined = preterite_drills;
            combined.extend(subjunctive_drills);
            combined
        }
    };

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "{}",
        "          SPANGLINGS RAPID-FIRE CONJUGATION DRILL         ".bold()
    );
    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "Topic: {} (5 questions). Type your answer and press Enter.\n",
        t.cyan().bold()
    );

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut score = 0;
    let total = items.len().min(5);

    for (i, item) in items.iter().take(total).enumerate() {
        print!("Q{}/{}: {} > ", i + 1, total, item.prompt.bright_white());
        io::stdout().flush()?;

        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            // EOF reached (e.g. non-interactive pipeline)
            println!();
            break;
        }

        let answer = line.trim().to_lowercase();
        if answer == item.target.to_lowercase() {
            println!("  {} Correct!\n", "✓".green().bold());
            score += 1;
        } else {
            println!(
                "  {} Incorrect. Expected: '{}' ({})\n",
                "✗".red().bold(),
                item.target.green().bold(),
                item.explanation.dimmed()
            );
        }
    }

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "Drill Finished! Score: {} / {} ({:.0}%)",
        score.to_string().green().bold(),
        total,
        (score as f64 / total as f64) * 100.0
    );
    println!(
        "{}",
        "==========================================================".blue()
    );

    Ok(())
}
