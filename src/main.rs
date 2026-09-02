#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
#[cfg(not(target_arch = "wasm32"))]
use spanglings::cli::{Cli, Commands};

#[cfg(not(target_arch = "wasm32"))]
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
            spanglings::cli::commands::explain::show_explanation(topic.as_deref())?;
        }
        Some(Commands::Arcade {
            topic,
            showdown,
            concept,
            weak,
            count,
            sound,
        }) => {
            let selected_showdown = showdown
                .as_deref()
                .and_then(spanglings::core::arcade::ShowdownPair::from_str)
                .map(|p| p.slug().to_string())
                .or_else(|| {
                    topic
                        .as_deref()
                        .and_then(spanglings::core::arcade::ShowdownPair::from_str)
                        .map(|p| p.slug().to_string())
                })
                .or(showdown);
            let selected_concept = concept.or_else(|| {
                if selected_showdown.is_none() {
                    topic
                } else {
                    None
                }
            });
            spanglings::cli::commands::arcade::run_arcade(
                selected_showdown,
                selected_concept,
                weak,
                count,
                sound,
                cli.strict_accents,
            )?;
        }
        Some(Commands::Drill {
            topic,
            concept,
            count,
            weak,
            level,
            track,
        }) => {
            spanglings::cli::commands::drill::run_drill(
                topic.as_deref(),
                concept.as_deref(),
                count,
                weak,
                level.as_deref(),
                track,
                cli.strict_accents,
            )?;
        }
        Some(Commands::Blitz {
            seconds,
            topic,
            weak,
            level,
            track,
        }) => {
            spanglings::cli::commands::blitz::run_blitz(
                seconds,
                topic.as_deref(),
                weak,
                level.as_deref(),
                track,
            )?;
        }
        Some(Commands::Review) => {
            spanglings::cli::commands::review::run_review_session()?;
        }
        Some(Commands::List { concept }) => {
            spanglings::cli::commands::list::list_exercises(cli.json, concept.as_deref())?;
        }
        Some(Commands::Progress) => {
            spanglings::cli::commands::progress::show_progress(cli.json)?;
        }
        Some(Commands::Test { level, fast_track }) => {
            spanglings::cli::commands::test::run_test(
                level,
                fast_track,
                cli.json,
                cli.strict_accents,
            )?;
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
            spanglings::cli::commands::conjugate::run_conjugate(&verb, tense.as_deref(), cli.json)?;
        }
        Some(Commands::Hook { action }) => match action {
            spanglings::cli::HookAction::Install { hook_type } => {
                spanglings::cli::commands::hook::run_hook_install(&hook_type)?;
            }
            spanglings::cli::HookAction::Uninstall { hook_type } => {
                spanglings::cli::commands::hook::run_hook_uninstall(&hook_type)?;
            }
        },
        Some(Commands::Pack { action }) => match action {
            spanglings::cli::PackAction::Create { name } => {
                spanglings::cli::commands::pack::run_pack_create(&name)?;
            }
            spanglings::cli::PackAction::Validate { path } => {
                let passed = spanglings::cli::commands::pack::run_pack_validate(&path)?;
                if !passed {
                    std::process::exit(1);
                }
            }
        },
        Some(Commands::Tui) => {
            spanglings::tui::start_tui(cli.strict_accents)?;
        }
        Some(Commands::Export {
            format,
            out,
            level,
            topic,
            only_due,
        }) => {
            spanglings::cli::commands::export::run_export(
                &format,
                out.as_deref(),
                level.as_deref(),
                topic.as_deref(),
                only_due,
            )?;
        }
        Some(Commands::Sync { export, import }) => {
            spanglings::cli::commands::sync::run_sync(export.as_deref(), import.as_deref())?;
        }
        Some(Commands::Lsp) => {
            spanglings::lsp::start_lsp_server(cli.strict_accents)?;
        }
        Some(Commands::Reset { exercise }) => {
            spanglings::cli::commands::run::reset_exercise(&exercise)?;
        }
        Some(Commands::Tour { skip_challenges }) => {
            spanglings::cli::commands::tour::run_tour(skip_challenges)?;
        }
        None => {
            spanglings::tui::start_tui(cli.strict_accents)?;
        }
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}
