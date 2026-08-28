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
    /// Launch quick-fire irregular stem conjugation and grammar drills
    Drill {
        /// Topic or concept to drill (e.g. preterite, subjunctive, por-para, ser-estar, pronouns, false-friends, prepositions, idioms)
        topic: Option<String>,
        /// Filter drill items by specific linguistic concept
        #[arg(short, long)]
        concept: Option<String>,
        /// Number of drill questions to ask (default: 5)
        #[arg(short = 'n', long)]
        count: Option<usize>,
    },
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
    List {
        /// Filter exercises by linguistic concept (e.g. subjunctive_wishes_desires, ser_vs_estar_identity)
        #[arg(short, long)]
        concept: Option<String>,
    },
    /// Display learning progress and CEFR level mastery
    Progress,
    /// Take an adaptive diagnostic placement test to assess CEFR level or test out of levels
    Test {
        /// Filter by specific level to test out of (e.g. B1, B2, C1)
        #[arg(short, long)]
        level: Option<String>,
        /// Automatically fast-track and skip passed levels without interactive confirmation
        #[arg(short, long)]
        fast_track: bool,
    },
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
    /// Manage, scaffold, and validate custom curriculum exercise packs
    Pack {
        #[command(subcommand)]
        action: PackAction,
    },
    /// Launch the interactive terminal UI
    Tui,
    /// Export study materials to Anki TSV, Markdown guide, or JSON
    Export {
        /// Export format: 'anki', 'markdown', or 'json'
        #[arg(short, long, default_value = "anki")]
        format: String,
        /// Output file path (writes to stdout if omitted)
        #[arg(short, long)]
        out: Option<String>,
        /// Filter by CEFR level (e.g. B1, B2, C1)
        #[arg(short, long)]
        level: Option<String>,
        /// Filter by topic
        #[arg(short, long)]
        topic: Option<String>,
        /// Export only exercises due for review in SRS
        #[arg(long)]
        only_due: bool,
    },
    /// Backup, restore, or merge learning state and review history
    Sync {
        /// Export portable learning state JSON to a file
        #[arg(long)]
        export: Option<String>,
        /// Import and merge learning state JSON from a file
        #[arg(long)]
        import: Option<String>,
    },
    /// Start Language Server Protocol (LSP) stdio server for editor integrations (VS Code, Neovim, Helix)
    Lsp,
    /// Reset an exercise to its initial prompt
    Reset { exercise: String },
    /// Launch the interactive onboarding guided tour
    Tour {
        /// Skip interactive challenge prompts and print overview
        #[arg(long)]
        skip_challenges: bool,
    },
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

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum PackAction {
    /// Scaffold a new custom curriculum track with starter markdown exercises
    Create {
        /// Name of the new curriculum track
        name: String,
    },
    /// Validate all exercises in a directory for schema correctness and solvability
    Validate {
        /// Directory path containing markdown exercises
        path: String,
    },
}
