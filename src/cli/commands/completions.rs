use crate::cli::Cli;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

pub fn generate_completions_string(shell: Shell) -> String {
    let mut cmd = Cli::command();
    let mut buf = Vec::new();
    generate(shell, &mut cmd, "spanglings", &mut buf);
    String::from_utf8_lossy(&buf).to_string()
}

pub fn run_completions(shell: Shell) -> anyhow::Result<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "spanglings", &mut io::stdout());
    Ok(())
}
