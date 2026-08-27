pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "spanglings",
    author,
    version,
    about = "Developer-grade CLI and TUI for mastering B1-C1 Spanish"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Strict accent mode (fails if missing accents instead of warning)
    #[arg(long, global = true)]
    pub strict_accents: bool,

    /// Output results in JSON format
    #[arg(long, global = true, help = "Output results in JSON format")]
    pub json: bool,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Watch exercises directory and evaluate on file save
    Watch,
    /// Initialize exercises in the current directory or target path
    Init {
        /// Directory path to extract exercises into (defaults to ./exercises)
        path: Option<String>,
        /// Overwrite existing files if directory is not empty
        #[arg(short, long)]
        force: bool,
    },
    /// Run and validate a specific exercise
    Run { exercise: String },
    /// Show grammatical hints for the current or specified exercise
    Hint { exercise: Option<String> },
    /// Display an in-terminal grammar reference card
    Explain { topic: String },
    /// Launch quick-fire irregular stem conjugation drills
    Drill { topic: Option<String> },
    /// Launch 60-second rapid-fire blitz speed drill
    Blitz {
        /// Time limit in seconds (default: 60)
        #[arg(short, long)]
        seconds: Option<u64>,
        /// Filter blitz drills by topic
        #[arg(short, long)]
        topic: Option<String>,
    },
    /// Launch an SM-2 spaced repetition review session
    Review,
    /// List all curriculum exercises and completion status
    List,
    /// Display learning progress and CEFR level mastery
    Progress,
    /// Search exercises by topic, keyword, or grammar concept
    Search {
        /// Search keyword or query
        query: String,
    },
    /// Generate shell auto-completions
    Completions {
        /// Shell to generate completions for (bash, zsh, fish, powershell, elvish)
        shell: clap_complete::Shell,
    },
    /// Check exercise file for errors or validate curriculum for editor diagnostics
    Check {
        /// Specific exercise path or ID to check (checks all if omitted)
        exercise: Option<String>,
    },
    /// Look up full conjugation tables and tenses for any Spanish verb
    Conjugate {
        /// Verb infinitive to conjugate (e.g. 'haber', 'ser', 'poner', 'hablar')
        verb: String,
        /// Optional tense filter (e.g. 'subjuntivo', 'imperativo', 'preterito')
        tense: Option<String>,
    },
    /// Manage Git pre-commit / pre-push Spanish practice hooks
    Hook {
        #[command(subcommand)]
        action: HookAction,
    },
    /// Launch the interactive terminal UI
    Tui,
    /// Reset an exercise to its initial prompt
    Reset { exercise: String },
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum HookAction {
    /// Install Spanish practice git hook in current repository
    Install {
        /// Hook type (e.g. pre-commit, pre-push)
        #[arg(short = 't', long, default_value = "pre-commit")]
        hook_type: String,
    },
    /// Remove Spanish practice git hook from current repository
    Uninstall {
        /// Hook type (e.g. pre-commit, pre-push)
        #[arg(short = 't', long, default_value = "pre-commit")]
        hook_type: String,
    },
}
