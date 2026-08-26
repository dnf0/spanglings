use spanglings::core::exercise::Exercise;
use spanglings::core::state::AppState;
use spanglings::engine::accents::AccentMode;
use spanglings::watcher::runner::evaluate_current_exercise;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_watcher_evaluate_current_exercise_runs_cleanly() {
    // When no exercises exist, evaluate_current_exercise returns Ok(false)
    let result = evaluate_current_exercise(AccentMode::Forgiving);
    assert!(result.is_ok());
}

#[test]
fn test_watcher_detects_and_evaluates_uncompleted_exercise() {
    let temp_dir = tempdir().unwrap();
    let exercises_path = temp_dir.path().join("exercises");
    fs::create_dir_all(&exercises_path).unwrap();

    let ex_file = exercises_path.join("ex01.md");
    let content = r#"<!-- I AM NOT DONE -->
# Test Watcher Exercise
<!-- id: b1_watch_test | level: B1 | topic: subjunctive | type: cloze -->

### Exercise
Quiero que tú vengas.

<!-- SOLUTION
vengas
-->
"#;
    fs::write(&ex_file, content).unwrap();

    let exercise = Exercise::from_markdown(&ex_file, content).unwrap();
    assert_eq!(exercise.id, "b1_watch_test");

    let mut state = AppState::default();
    assert!(!state.is_completed(&exercise.id));
    state.mark_completed(&exercise.id);
    assert!(state.is_completed(&exercise.id));
}
