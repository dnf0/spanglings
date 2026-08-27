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
        Some(Commands::Blitz { seconds, topic }) => {
            spanglings::cli::commands::blitz::run_blitz(seconds, topic.as_deref())?;
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
        Some(Commands::Search { query }) => {
            spanglings::cli::commands::search::run_search(&query, cli.json)?;
        }
        Some(Commands::Completions { shell }) => {
            spanglings::cli::commands::completions::run_completions(shell)?;
        }
        Some(Commands::Check { exercise }) => {
            let passed = spanglings::cli::commands::check::run_check(
                exercise.as_deref(),
                cli.json,
                cli.strict_accents,
            )?;
            if !passed {
                std::process::exit(1);
            }
        }
        Some(Commands::Conjugate { verb, tense }) => {
            spanglings::cli::commands::conjugate::run_conjugate(
                &verb,
                tense.as_deref(),
                cli.json,
            )?;
        }
        Some(Commands::Hook { action }) => match action {
            spanglings::cli::HookAction::Install { hook_type } => {
                spanglings::cli::commands::hook::run_hook_install(&hook_type)?;
            }
            spanglings::cli::HookAction::Uninstall { hook_type } => {
                spanglings::cli::commands::hook::run_hook_uninstall(&hook_type)?;
            }
        },
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
