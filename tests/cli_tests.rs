use clap::Parser;
use spanglings::cli::commands::explain::show_explanation;
use spanglings::cli::commands::hint::show_hint;
use spanglings::cli::commands::list::list_exercises;
use spanglings::cli::commands::progress::show_progress;
use spanglings::cli::commands::run::run_exercise;
use spanglings::cli::{Cli, Commands};
use spanglings::core::curriculum::{find_all_exercises, find_exercise_by_query};
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_cli_parsing_subcommands() {
    let cli = Cli::parse_from(["spanglings", "explain", "subjunctive"]);
    assert_eq!(
        cli.command,
        Some(Commands::Explain {
            topic: "subjunctive".to_string()
        })
    );
    assert!(!cli.strict_accents);

    let cli_strict =
        Cli::parse_from(["spanglings", "--strict-accents", "run", "b1_subjunctive_01"]);
    assert_eq!(
        cli_strict.command,
        Some(Commands::Run {
            exercise: "b1_subjunctive_01".to_string()
        })
    );
    assert!(cli_strict.strict_accents);

    let cli_hint = Cli::parse_from(["spanglings", "hint", "b1_subjunctive_01"]);
    assert_eq!(
        cli_hint.command,
        Some(Commands::Hint {
            exercise: Some("b1_subjunctive_01".to_string())
        })
    );

    let cli_drill = Cli::parse_from(["spanglings", "drill", "preterite"]);
    assert_eq!(
        cli_drill.command,
        Some(Commands::Drill {
            topic: Some("preterite".to_string())
        })
    );

    let cli_list = Cli::parse_from(["spanglings", "list"]);
    assert_eq!(cli_list.command, Some(Commands::List));

    let cli_progress = Cli::parse_from(["spanglings", "progress"]);
    assert_eq!(cli_progress.command, Some(Commands::Progress));

    let cli_review = Cli::parse_from(["spanglings", "review"]);
    assert_eq!(cli_review.command, Some(Commands::Review));

    let cli_watch = Cli::parse_from(["spanglings", "watch"]);
    assert_eq!(cli_watch.command, Some(Commands::Watch));

    let cli_reset = Cli::parse_from(["spanglings", "reset", "b1_subj_01"]);
    assert_eq!(
        cli_reset.command,
        Some(Commands::Reset {
            exercise: "b1_subj_01".to_string()
        })
    );
}

#[test]
fn test_explain_command_executes_cleanly() {
    assert!(show_explanation("subjunctive").is_ok());
    assert!(show_explanation("por-para").is_ok());
    assert!(show_explanation("ser-estar").is_ok());
    assert!(show_explanation("past").is_ok());
    assert!(show_explanation("pronouns").is_ok());
    assert!(show_explanation("prepositions").is_ok());
    assert!(show_explanation("accidental-se").is_ok());
    assert!(show_explanation("unknown_random_topic").is_ok());
}

#[test]
fn test_list_and_progress_execute_cleanly() {
    assert!(list_exercises().is_ok());
    assert!(show_progress().is_ok());
    assert!(show_hint(None).is_ok());
}

#[test]
fn test_run_exercise_with_file_path() {
    let file = NamedTempFile::new().unwrap();
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_subj_test | level: B1 | topic: subjunctive | type: cloze -->

### Exercise
Quiero que tú vengas.

<!-- SOLUTION
vengas
-->
"#;
    fs::write(file.path(), content).unwrap();

    let path_str = file.path().to_str().unwrap();
    let res = run_exercise(path_str, false);
    assert!(res.is_ok());
}

#[test]
fn test_curriculum_discovery_and_query() {
    let temp_dir = tempfile::tempdir().unwrap();
    let track_dir = temp_dir.path().join("01_test_track");
    fs::create_dir_all(&track_dir).unwrap();

    let file_path = track_dir.join("ex01.md");
    let content = r#"<!-- I AM NOT DONE -->
# Test Exercise Title
<!-- id: b1_test_ex | level: B1 | topic: subjunctive | type: cloze -->

### Exercise
Haga ___ ahora.

<!-- SOLUTION
esto
-->
"#;
    fs::write(&file_path, content).unwrap();

    let exercises = find_all_exercises(temp_dir.path()).unwrap();
    assert_eq!(exercises.len(), 1);
    assert_eq!(exercises[0].id, "b1_test_ex");

    let by_id = find_exercise_by_query(&exercises, "b1_test_ex");
    assert!(by_id.is_some());
    assert_eq!(by_id.unwrap().title, "Test Exercise Title");

    let by_title = find_exercise_by_query(&exercises, "Exercise Title");
    assert!(by_title.is_some());

    let not_found = find_exercise_by_query(&exercises, "nonexistent");
    assert!(not_found.is_none());
}
