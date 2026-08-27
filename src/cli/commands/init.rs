use crate::core::embedded::init_exercises_dir;
use colored::Colorize;
use std::path::PathBuf;

pub fn run_init(target_path: Option<&str>, force: bool) -> anyhow::Result<()> {
    let target = match target_path {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from("exercises"),
    };

    println!(
        "{}",
        "Initializing Spanglings curriculum workspace..."
            .bold()
            .cyan()
    );
    let count = init_exercises_dir(&target, force)?;

    println!(
        "{} Initialized {} exercises in '{}'!",
        "✔".bold().green(),
        count.to_string().bold().yellow(),
        target.display().to_string().bold()
    );
    println!();
    println!("{}", "Next steps:".bold());
    println!(
        "  1. Run {} to start interactive TUI mode",
        "spanglings".cyan().bold()
    );
    println!(
        "  2. Or run {} to start live file-watcher mode",
        "spanglings watch".cyan().bold()
    );
    println!(
        "  3. List available exercises with {}",
        "spanglings list".cyan().bold()
    );

    Ok(())
}
