use clap_complete::Shell;
use spanglings::cli::commands::completions::generate_completions_string;

#[test]
fn test_generate_bash_completions() {
    let output = generate_completions_string(Shell::Bash);
    assert!(output.contains("spanglings"));
    assert!(output.contains("watch"));
    assert!(output.contains("run"));
    assert!(output.contains("init"));
    assert!(output.contains("search"));
}

#[test]
fn test_generate_zsh_completions() {
    let output = generate_completions_string(Shell::Zsh);
    assert!(output.contains("spanglings"));
    assert!(output.contains("watch"));
    assert!(output.contains("search"));
}

#[test]
fn test_generate_fish_completions() {
    let output = generate_completions_string(Shell::Fish);
    assert!(output.contains("spanglings"));
}
