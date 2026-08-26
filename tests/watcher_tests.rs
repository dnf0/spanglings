use spanglings::core::state::AppState;
use spanglings::engine::accents::AccentMode;
use spanglings::watcher::runner::{evaluate_current_exercise, evaluate_current_exercise_in};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_watcher_evaluate_current_exercise_runs_cleanly() {
    // When no exercises exist, evaluate_current_exercise returns Ok(false)
    let result = evaluate_current_exercise(AccentMode::Forgiving);
    assert!(result.is_ok());
}

#[test]
fn test_watcher_detects_and_evaluates_uncompleted_exercise_in_dir() {
    let temp_dir = tempdir().unwrap();
    let exercises_path = temp_dir.path().join("exercises");
    fs::create_dir_all(&exercises_path).unwrap();

    let ex_file = exercises_path.join("ex01.md");
    let content = r#"<!-- I AM NOT DONE -->
# Test Watcher Exercise
<!-- id: b1_watch_test | level: B1 | topic: subjunctive | type: cloze -->

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
}

#[test]
fn test_watcher_detects_failing_submission() {
    let temp_dir = tempdir().unwrap();
    let exercises_path = temp_dir.path().join("exercises");
    fs::create_dir_all(&exercises_path).unwrap();

    let ex_file = exercises_path.join("ex01.md");
    let content = r#"<!-- I AM NOT DONE -->
# Test Watcher Exercise
<!-- id: b1_watch_failing | level: B1 | topic: subjunctive | type: cloze -->

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
}
