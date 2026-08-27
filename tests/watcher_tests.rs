use spanglings::core::state::AppState;
use spanglings::engine::accents::AccentMode;
use spanglings::watcher::runner::evaluate_current_exercise_in;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_watcher_evaluate_current_exercise_runs_cleanly() {
    let temp_dir = tempdir().unwrap();
    let exercises_path = temp_dir.path().join("exercises");
    fs::create_dir_all(&exercises_path).unwrap();

    let mut state = AppState::default();
    let result = evaluate_current_exercise_in(&exercises_path, &mut state, AccentMode::Forgiving);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_watcher_detects_and_evaluates_uncompleted_exercise_in_dir() {
    let temp_dir = tempdir().unwrap();
    let exercises_path = temp_dir.path().join("exercises");
    fs::create_dir_all(&exercises_path).unwrap();

    let ex_file = exercises_path.join("ex01.md");
    let content = r#"# Test Watcher Exercise
<!-- id: b1_watch_test | level: B1 | topic: subjunctive | type: cloze | concepts: ["subjunctive_volition"] -->

### Context
English: "I want you to come."

### Exercise
Quiero que tú (venir) vengas a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    fs::write(&ex_file, content).unwrap();

    let mut state = AppState::default();
    assert!(!state.is_completed("b1_watch_test"));

    let evaluated =
        evaluate_current_exercise_in(&exercises_path, &mut state, AccentMode::Forgiving);
    assert!(evaluated.is_ok());
    assert!(evaluated.unwrap());
    assert!(state.is_completed("b1_watch_test"));
    assert!(state.srs.contains_key("b1_watch_test"));
    assert!(state.concept_mastery.contains_key("subjunctive_volition"));
    assert_eq!(
        state
            .concept_mastery
            .get("subjunctive_volition")
            .unwrap()
            .total_reviews,
        1
    );
}

#[test]
fn test_watcher_detects_failing_submission() {
    let temp_dir = tempdir().unwrap();
    let exercises_path = temp_dir.path().join("exercises");
    fs::create_dir_all(&exercises_path).unwrap();

    let ex_file = exercises_path.join("ex01.md");
    let content = r#"# Test Watcher Exercise
<!-- id: b1_watch_failing | level: B1 | topic: subjunctive | type: cloze | concepts: ["subjunctive_volition"] -->

### Context
English: "I want you to come."

### Exercise
Quiero que tú (venir) viene a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    fs::write(&ex_file, content).unwrap();

    let mut state = AppState::default();
    let evaluated =
        evaluate_current_exercise_in(&exercises_path, &mut state, AccentMode::Forgiving);
    assert!(evaluated.is_ok());
    assert!(!evaluated.unwrap());
    assert!(!state.is_completed("b1_watch_failing"));
    assert_eq!(
        state
            .concept_mastery
            .get("subjunctive_volition")
            .unwrap()
            .lapses,
        1
    );
}

#[test]
fn test_watcher_advances_to_next_uncompleted_exercise() {
    let temp_dir = tempdir().unwrap();
    let exercises_path = temp_dir.path().join("exercises");
    fs::create_dir_all(&exercises_path).unwrap();

    let ex1_file = exercises_path.join("01_ex.md");
    let content1 = r#"# Exercise 1
<!-- id: ex_01 | level: B1 | topic: ser_estar | type: cloze -->
Ella ___ médica.
<!-- SOLUTION
es
-->
"#;
    fs::write(&ex1_file, content1).unwrap();

    let ex2_file = exercises_path.join("02_ex.md");
    let content2 = r#"# Exercise 2
<!-- id: ex_02 | level: B1 | topic: ser_estar | type: cloze -->
Ella ___ cansada.
<!-- SOLUTION
está
-->
"#;
    fs::write(&ex2_file, content2).unwrap();

    let mut state = AppState::default();
    state.mark_completed("ex_01");

    let evaluated =
        evaluate_current_exercise_in(&exercises_path, &mut state, AccentMode::Forgiving);
    assert!(evaluated.is_ok());
    // ex_02 is not yet filled in correctly ("___"), so it will fail evaluation
    assert!(!evaluated.unwrap());
    assert!(state.is_completed("ex_01"));
    assert!(!state.is_completed("ex_02"));
}

#[test]
fn test_watcher_all_completed_returns_true() {
    let temp_dir = tempdir().unwrap();
    let exercises_path = temp_dir.path().join("exercises");
    fs::create_dir_all(&exercises_path).unwrap();

    let ex1_file = exercises_path.join("01_ex.md");
    let content1 = r#"# Exercise 1
<!-- id: ex_01 | level: B1 | topic: ser_estar | type: cloze -->
Ella es médica.
<!-- SOLUTION
es
-->
"#;
    fs::write(&ex1_file, content1).unwrap();

    let mut state = AppState::default();
    state.mark_completed("ex_01");

    let evaluated =
        evaluate_current_exercise_in(&exercises_path, &mut state, AccentMode::Forgiving);
    assert!(evaluated.is_ok());
    assert!(evaluated.unwrap());
}
