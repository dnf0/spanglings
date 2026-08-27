use spanglings::cli::commands::test::run_test_with_io;
use std::io::Cursor;
use std::sync::Mutex;
use tempfile::tempdir;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_cli_test_command_interactive_fast_track() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let temp = tempdir().unwrap();
    let state_file = temp.path().join("state_interactive.json");
    std::env::set_var("SPANGLINGS_STATE_PATH", &state_file);

    // Provide 4 correct answers for B1, then "y" to confirm fast-track
    let input = "había revisado\nactualice\npor, para\nse le cayeron\ny\n";
    let mut reader = Cursor::new(input.as_bytes());
    let mut writer = Vec::new();

    let res = run_test_with_io(
        Some("b1".to_string()),
        false, // fast_track_flag = false, test interactive prompt
        false, // json_mode
        false, // strict_accents
        &mut reader,
        &mut writer,
    );
    assert!(res.is_ok(), "run_test_with_io failed: {:?}", res);
    assert!(state_file.exists());

    let state = spanglings::core::state::AppState::load_from_path(&state_file).unwrap();
    assert!(state.evaluated_level.is_some());
    assert!(
        state.completed_exercises.len() >= 60,
        "B1 exercises should have been fast-tracked, found {}",
        state.completed_exercises.len()
    );
    std::env::remove_var("SPANGLINGS_STATE_PATH");
}

#[test]
fn test_cli_test_command_json_mode() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let temp = tempdir().unwrap();
    let state_file = temp.path().join("state_json.json");
    std::env::set_var("SPANGLINGS_STATE_PATH", &state_file);

    let input = "había revisado\nactualice\npor, para\nse le cayeron\n";
    let mut reader = Cursor::new(input.as_bytes());
    let mut writer = Vec::new();

    let res = run_test_with_io(
        Some("b1".to_string()),
        true,
        true, // json_mode
        false,
        &mut reader,
        &mut writer,
    );
    assert!(res.is_ok());
    let output_str = String::from_utf8(writer).unwrap();
    assert!(output_str.contains("\"assessed_level\""));
    assert!(output_str.contains("\"percentage\""));
    std::env::remove_var("SPANGLINGS_STATE_PATH");
}
