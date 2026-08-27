use crate::core::state::AppState;
use colored::Colorize;
use std::io::{self, BufRead, IsTerminal, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TourChallenge {
    pub prompt: String,
    pub expected_input: String,
    pub explanation: String,
    pub tip: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TourStation {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub bullet_points: Vec<String>,
    pub challenge: Option<TourChallenge>,
    pub simulated_command: Option<String>,
    pub simulated_output: Option<String>,
}

pub fn get_tour_stations() -> Vec<TourStation> {
    vec![
        TourStation {
            id: "philosophy".to_string(),
            title: "Welcome & The Spanglings Philosophy".to_string(),
            subtitle: "Active recall, production context, and zero busywork".to_string(),
            description: "Spanglings is a developer-grade learning environment for mastering intermediate and advanced Spanish (B1-C1).".to_string(),
            bullet_points: vec![
                "Active Recall: Learn grammar by writing code and real Spanish sentences rather than passive multiple-choice guessing.".to_string(),
                "Zero Busywork: No need to delete '<!-- I AM NOT DONE -->' comments. Just fill in the blanks and save.".to_string(),
                "Developer Ergonomics: Native support for watch mode in your editor (VS Code, Neovim, Zed) or full terminal TUI.".to_string(),
            ],
            challenge: None,
            simulated_command: None,
            simulated_output: None,
        },
        TourStation {
            id: "anatomy_accents".to_string(),
            title: "Exercise Anatomy & UTF-8 Spanish Accents".to_string(),
            subtitle: "Mastering the cloze format and special characters".to_string(),
            description: "Every Spanglings exercise is a focused markdown file containing grammar explanations and cloze blanks '___'.".to_string(),
            bullet_points: vec![
                "Cloze blanks are marked with '___'. Fill them in with the target conjugated verb or grammar token.".to_string(),
                "UTF-8 Spanish characters (á, é, í, ó, ú, ñ, ü, ¿, ¡) are fully supported and validated.".to_string(),
                "Accent modes: Configure forgiving (default) or strict accent validation in your config or state.".to_string(),
            ],
            challenge: Some(TourChallenge {
                prompt: "Quiero que tú (venir) ___ a la reunión.".to_string(),
                expected_input: "vengas".to_string(),
                explanation: "Venir in present subjunctive (tú) is 'vengas'. 'Quiero que...' triggers the subjunctive mood.".to_string(),
                tip: Some("Type 'vengas' and press Enter (or press Enter to auto-solve).".to_string()),
            }),
            simulated_command: None,
            simulated_output: None,
        },
        TourStation {
            id: "diagnostics".to_string(),
            title: "Concept-Aware Compiler Diagnostics".to_string(),
            subtitle: "Compiler-grade error feedback linked to linguistic ontology".to_string(),
            description: "When an answer is incorrect, Spanglings provides rich compiler-like diagnostics instead of vague hints.".to_string(),
            bullet_points: vec![
                "Error codes (e.g. error[E0301]) categorize the precise linguistic mistake.".to_string(),
                "Diagnostic cards highlight the linked grammatical concept and required prerequisite topics.".to_string(),
                "Concrete remediation advice helps you correct the underlying misconception immediately.".to_string(),
            ],
            challenge: None,
            simulated_command: Some("spanglings check exercises/01_subjunctive/01_intro.md".to_string()),
            simulated_output: Some("error[E0301]: expected present subjunctive form 'vengas', found indicative 'viene'\n  --> exercises/01_subjunctive/01_intro.md:12:15\n   |\n12 | Quiero que tú viene a la reunión.\n   |               ^^^^^ incorrect mood\n   |\n   = concept: subjunctive_volition_influence\n   = prerequisite: irregular_subjunctive_stems\n   = help: 'Quiero que' expresses desire/will, requiring subjunctive ('vengas')".to_string()),
        },
        TourStation {
            id: "hints_reference".to_string(),
            title: "Progressive Hints & Grammar Reference".to_string(),
            subtitle: "3-tier hint progression and instant grammar reference cards".to_string(),
            description: "Get unstuck without spoiling solutions using progressive tiered hints and comprehensive reference cards.".to_string(),
            bullet_points: vec![
                "Tier 1 (Rule): Reminds you of the underlying grammar principle without giving stems.".to_string(),
                "Tier 2 (Stem): Gives the root / stem change or conjugation pattern.".to_string(),
                "Tier 3 (Solution): Reveals the exact answer with detailed linguistic explanation.".to_string(),
                "Instant reference: Run 'spanglings explain <topic>' anytime for comprehensive grammar summaries.".to_string(),
            ],
            challenge: None,
            simulated_command: Some("spanglings explain subjunctive".to_string()),
            simulated_output: Some("[Grammar Reference Card: Present Subjunctive]\nFormation: 'yo' present indicative -> drop -o -> add opposite endings (-AR: -e, -es, -e...; -ER/-IR: -a, -as, -a...)\nTriggers: WEIRDOS (Wishes, Emotions, Impersonal expressions, Requests, Doubts, Ojalá, Speculation)".to_string()),
        },
        TourStation {
            id: "tools_placement".to_string(),
            title: "Integrated Tools: Conjugator & Placement Battery".to_string(),
            subtitle: "Built-in high-speed conjugation tables and CEFR assessment".to_string(),
            description: "Spanglings includes powerful offline developer utilities to accelerate your learning loop.".to_string(),
            bullet_points: vec![
                "High-speed verb lookup: 'spanglings conjugate <verb>' displays all moods, tenses, and irregular stems.".to_string(),
                "Placement test: 'spanglings test' runs a diagnostic assessment to calibrate your starting CEFR level (A1 to C1).".to_string(),
                "Spaced Repetition: 'spanglings review' uses the SM-2 algorithm to schedule review intervals for challenging items.".to_string(),
                "Rapid-Fire Drills: 'spanglings drill' tests verb stem transformations under timed active recall.".to_string(),
            ],
            challenge: None,
            simulated_command: Some("spanglings conjugate proponer".to_string()),
            simulated_output: Some("Verb: proponer (to propose / suggest) [irregular]\n  Present: propongo, propones, propone, proponemos, proponéis, proponen\n  Preterite: propuse, propusiste, propuso, propusimos, propusisteis, propusieron\n  Subjunctive: proponga, propongas, proponga, propongamos, propongáis, propongan".to_string()),
        },
        TourStation {
            id: "workflows".to_string(),
            title: "Choose Your Workflow: Watch Mode vs TUI".to_string(),
            subtitle: "Tailor Spanglings to your personal editor and terminal preferences".to_string(),
            description: "Learn the way you code best — in your favorite editor with automated watch evaluation, or full interactive TUI.".to_string(),
            bullet_points: vec![
                "Watch Mode ('spanglings watch'): Keep your terminal beside VS Code, Zed, or Neovim. Hotkeys: [n] Next, [p] Prev, [r] Reload, [q] Quit.".to_string(),
                "Interactive TUI ('spanglings'): Full ratatui terminal interface with exercise explorer, split-pane diffs, and search.".to_string(),
                "Progress Tracking ('spanglings progress'): Visual heatmap and CEFR level mastery radar.".to_string(),
                "Ready to start? Run 'spanglings watch' or 'spanglings' to begin your first exercise!".to_string(),
            ],
            challenge: None,
            simulated_command: None,
            simulated_output: None,
        },
    ]
}

pub fn render_station_card(station: &TourStation, index: usize, total: usize) {
    println!("{}", "═".repeat(64).cyan());
    println!(
        " {} [Station {}/{}] {}",
        "✦".yellow().bold(),
        index + 1,
        total,
        station.title.bright_white().bold()
    );
    println!("   {}", station.subtitle.dimmed().italic());
    println!("{}", "─".repeat(64).cyan());
    println!("\n{}\n", station.description);

    for point in &station.bullet_points {
        println!("  {} {}", "•".cyan().bold(), point);
    }

    if let Some(cmd) = &station.simulated_command {
        println!(
            "\n  {} {}",
            "Example Command:".yellow().bold(),
            format!("$ {}", cmd).bright_black()
        );
    }

    if let Some(out) = &station.simulated_output {
        println!("  {}", "Output:".yellow().bold());
        for line in out.lines() {
            println!("    {}", line.dimmed());
        }
    }

    if let Some(challenge) = &station.challenge {
        println!(
            "\n  {} {}",
            "Challenge:".magenta().bold(),
            challenge.prompt.bright_white()
        );
        if let Some(tip) = &challenge.tip {
            println!("  {} {}", "Tip:".dimmed(), tip.dimmed());
        }
    }
    println!();
}

fn run_interactive_tour(stations: &[TourStation]) -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut idx = 0;

    while idx < stations.len() {
        let station = &stations[idx];
        println!("{}", "═".repeat(64).cyan());
        println!(
            " {} [Station {}/{}] {}",
            "✦".yellow().bold(),
            idx + 1,
            stations.len(),
            station.title.bright_white().bold()
        );
        println!("   {}", station.subtitle.dimmed().italic());
        println!("{}", "─".repeat(64).cyan());
        println!("\n{}\n", station.description);

        for point in &station.bullet_points {
            println!("  {} {}", "•".cyan().bold(), point);
        }

        if let Some(cmd) = &station.simulated_command {
            println!(
                "\n  {} {}",
                "Example Command:".yellow().bold(),
                format!("$ {}", cmd).bright_black()
            );
        }

        if let Some(out) = &station.simulated_output {
            println!("  {}", "Output:".yellow().bold());
            for line in out.lines() {
                println!("    {}", line.dimmed());
            }
        }

        if let Some(challenge) = &station.challenge {
            println!(
                "\n  {} {}",
                "Challenge:".magenta().bold(),
                challenge.prompt.bright_white().bold()
            );
            if let Some(tip) = &challenge.tip {
                println!("  {} {}", "Tip:".dimmed(), tip.dimmed());
            }
            print!("\n  Type your answer (or press Enter to auto-solve) > ");
            io::stdout().flush()?;

            let mut input = String::new();
            if reader.read_line(&mut input)? == 0 {
                // EOF
                break;
            }
            let trimmed = input.trim();
            if trimmed.is_empty() {
                println!(
                    "  {} Auto-solved: '{}'",
                    "ℹ".cyan().bold(),
                    challenge.expected_input.green().bold()
                );
                println!("    {}", challenge.explanation.dimmed());
            } else if trimmed.eq_ignore_ascii_case(&challenge.expected_input) {
                println!(
                    "  {} Correct! '{}'",
                    "✓".green().bold(),
                    challenge.expected_input.green().bold()
                );
                println!("    {}", challenge.explanation.dimmed());
            } else {
                println!(
                    "  {} Got '{}', expected '{}'",
                    "✗".yellow().bold(),
                    trimmed.red(),
                    challenge.expected_input.green().bold()
                );
                println!("    {}", challenge.explanation.dimmed());
            }
            println!();
        }

        // Navigation prompt
        let is_last = idx + 1 == stations.len();
        let prompt_text = if is_last {
            format!(
                "Navigation: [{}] Finish  |  [{}] Prev  |  [{}] Quit > ",
                "Enter / n".green().bold(),
                "p".yellow().bold(),
                "q".red().bold()
            )
        } else {
            format!(
                "Navigation: [{}] Next  |  [{}] Prev  |  [{}] Quit > ",
                "Enter / n".green().bold(),
                "p".yellow().bold(),
                "q".red().bold()
            )
        };

        print!("{}", prompt_text);
        io::stdout().flush()?;

        let mut nav = String::new();
        if reader.read_line(&mut nav)? == 0 {
            // EOF
            break;
        }

        let choice = nav.trim().to_lowercase();
        match choice.as_str() {
            "" | "n" | "next" | "f" | "finish" => {
                idx += 1;
            }
            "p" | "prev" | "previous" => {
                idx = idx.saturating_sub(1);
            }
            "q" | "quit" | "exit" => {
                println!(
                    "\n{}",
                    "Exiting tour early. You can return anytime with 'spanglings tour'.".yellow()
                );
                return Ok(());
            }
            _ => {
                idx += 1;
            }
        }
        println!();
    }

    Ok(())
}

pub fn run_tour(skip_challenges: bool) -> anyhow::Result<()> {
    let is_interactive = io::stdin().is_terminal() && io::stdout().is_terminal() && !skip_challenges;
    let stations = get_tour_stations();

    if is_interactive {
        run_interactive_tour(&stations)?;
    } else {
        println!("{}", "═".repeat(64).blue());
        println!(
            "{}",
            "             SPANGLINGS ONBOARDING TOUR (OVERVIEW)            ".bold()
        );
        println!("{}", "═".repeat(64).blue());
        println!();

        for (i, station) in stations.iter().enumerate() {
            render_station_card(station, i, stations.len());
        }
    }

    println!("{}", "═".repeat(64).green());
    println!(
        "{}",
        "              TOUR COMPLETE — ¡BUEN VIAJE!                    ".green().bold()
    );
    println!("{}", "═".repeat(64).green());
    println!("You are now ready to start your Spanglings journey!\n");
    println!("Recommended next steps:");
    println!(
        "  1. {} - Start live-evaluating exercises in your editor",
        "spanglings watch".cyan().bold()
    );
    println!(
        "  2. {}          - Launch the full interactive terminal TUI",
        "spanglings".cyan().bold()
    );
    println!(
        "  3. {}     - Take the diagnostic CEFR placement test",
        "spanglings test".cyan().bold()
    );
    println!(
        "  4. {}  - View reference cards on any grammar topic",
        "spanglings explain".cyan().bold()
    );
    println!("{}", "═".repeat(64).green());

    let mut state = AppState::load().unwrap_or_default();
    state.mark_tour_completed();
    let _ = state.save();

    Ok(())
}
