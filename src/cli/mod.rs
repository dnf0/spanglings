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
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Watch exercises directory and evaluate on file save
    Watch,
    /// Run and validate a specific exercise
    Run { exercise: String },
    /// Show grammatical hints for the current or specified exercise
    Hint { exercise: Option<String> },
    /// Display an in-terminal grammar reference card
    Explain { topic: String },
    /// Launch quick-fire irregular stem conjugation drills
    Drill { topic: Option<String> },
    /// Launch an SM-2 spaced repetition review session
    Review,
    /// List all curriculum exercises and completion status
    List,
    /// Display learning progress and CEFR level mastery
    Progress,
    /// Reset an exercise to its initial prompt
    Reset { exercise: String },
}
