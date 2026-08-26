use clap::Parser;
use spanglings::cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Watch) => {
            spanglings::watcher::runner::start_watch_mode(cli.strict_accents)?;
        }
        Some(Commands::Init { path, force }) => {
            spanglings::cli::commands::init::run_init(path.as_deref(), force)?;
        }
        Some(Commands::Run { exercise }) => {
            spanglings::cli::commands::run::run_exercise(&exercise, cli.strict_accents)?;
        }
        Some(Commands::Hint { exercise }) => {
            spanglings::cli::commands::hint::show_hint(exercise.as_deref())?;
        }
        Some(Commands::Explain { topic }) => {
            spanglings::cli::commands::explain::show_explanation(&topic)?;
        }
        Some(Commands::Drill { topic }) => {
            spanglings::cli::commands::drill::run_drill(topic.as_deref())?;
        }
        Some(Commands::Review) => {
            spanglings::cli::commands::review::run_review_session()?;
        }
        Some(Commands::List) => {
            spanglings::cli::commands::list::list_exercises(cli.json)?;
        }
        Some(Commands::Progress) => {
            spanglings::cli::commands::progress::show_progress(cli.json)?;
        }
        Some(Commands::Tui) => {
            spanglings::tui::start_tui(cli.strict_accents)?;
        }
        Some(Commands::Reset { exercise }) => {
            spanglings::cli::commands::run::reset_exercise(&exercise)?;
        }
        None => {
            spanglings::tui::start_tui(cli.strict_accents)?;
        }
    }
    Ok(())
}
