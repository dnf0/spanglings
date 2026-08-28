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
            topic: Some("subjunctive".to_string())
        })
    );
    assert!(!cli.strict_accents);

    let cli_explain_none = Cli::parse_from(["spanglings", "explain"]);
    assert_eq!(
        cli_explain_none.command,
        Some(Commands::Explain { topic: None })
    );

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
            topic: Some("preterite".to_string()),
            concept: None,
            count: None,
        })
    );

    let cli_drill_concept = Cli::parse_from([
        "spanglings",
        "drill",
        "--concept",
        "subjunctive_wishes_desires",
    ]);
    assert_eq!(
        cli_drill_concept.command,
        Some(Commands::Drill {
            topic: None,
            concept: Some("subjunctive_wishes_desires".to_string()),
            count: None,
        })
    );

    let cli_blitz = Cli::parse_from([
        "spanglings",
        "blitz",
        "--seconds",
        "30",
        "--topic",
        "subjunctive",
    ]);
    assert_eq!(
        cli_blitz.command,
        Some(Commands::Blitz {
            seconds: Some(30),
            topic: Some("subjunctive".to_string()),
        })
    );

    let cli_check = Cli::parse_from(["spanglings", "check", "b1_subj_01"]);
    assert_eq!(
        cli_check.command,
        Some(Commands::Check {
            exercise: Some("b1_subj_01".to_string())
        })
    );

    let cli_list = Cli::parse_from(["spanglings", "list"]);
    assert_eq!(cli_list.command, Some(Commands::List { concept: None }));

    let cli_list_concept = Cli::parse_from([
        "spanglings",
        "list",
        "--concept",
        "subjunctive_wishes_desires",
    ]);
    assert_eq!(
        cli_list_concept.command,
        Some(Commands::List {
            concept: Some("subjunctive_wishes_desires".to_string()),
        })
    );

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
fn test_cli_tour_command_parsing() {
    let cli_tour = Cli::parse_from(["spanglings", "tour"]);
    assert_eq!(
        cli_tour.command,
        Some(Commands::Tour {
            skip_challenges: false
        })
    );
    let cli_tour_skip = Cli::parse_from(["spanglings", "tour", "--skip-challenges"]);
    assert_eq!(
        cli_tour_skip.command,
        Some(Commands::Tour {
            skip_challenges: true
        })
    );
}

#[test]
fn test_explain_command_executes_cleanly() {
    assert!(show_explanation(Some("subjunctive")).is_ok());
    assert!(show_explanation(Some("por-para")).is_ok());
    assert!(show_explanation(Some("ser-estar")).is_ok());
    assert!(show_explanation(Some("past")).is_ok());
    assert!(show_explanation(Some("pronouns")).is_ok());
    assert!(show_explanation(Some("prepositions")).is_ok());
    assert!(show_explanation(Some("accidental-se")).is_ok());
    assert!(show_explanation(Some("unknown_random_topic")).is_ok());
    assert!(show_explanation(None).is_ok());
    assert!(show_explanation(Some("")).is_ok());
    assert!(show_explanation(Some("   ")).is_ok());
}

#[test]
fn test_cli_explain_semantic_lookup_and_glosses() {
    assert!(show_explanation(Some("wishes")).is_ok());
    assert!(show_explanation(Some("unintentional")).is_ok());
    assert!(show_explanation(Some("body parts")).is_ok());
    assert!(show_explanation(Some("conjecture")).is_ok());
}

#[test]
fn test_list_and_progress_execute_cleanly() {
    assert!(list_exercises(false, None).is_ok());
    assert!(list_exercises(true, None).is_ok());
    assert!(list_exercises(false, Some("subjunctive_volition_influence")).is_ok());
    assert!(show_progress(false).is_ok());
    assert!(show_progress(true).is_ok());
    assert!(show_hint(None).is_ok());
}

#[test]
fn test_run_exercise_with_file_path() {
    let file = NamedTempFile::new().unwrap();
    let content = r#"# Subjunctive 01
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
    let content = r#"# Test Exercise Title
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
