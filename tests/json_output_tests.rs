use spanglings::cli::commands::list::get_exercises_json;
use spanglings::cli::commands::progress::get_progress_json;

#[test]
fn test_list_exercises_json_serialization() {
    let json_str = get_exercises_json().expect("Failed to serialize exercises to JSON");
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert!(parsed.is_array());
    assert!(parsed.as_array().unwrap().len() >= 116);
    let first = &parsed[0];
    assert!(first.get("id").is_some());
    assert!(first.get("title").is_some());
    assert!(first.get("level").is_some());
    assert!(first.get("topic").is_some());
}

#[test]
fn test_progress_json_serialization() {
    let json_str = get_progress_json().expect("Failed to serialize progress to JSON");
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert!(parsed.get("total").is_some());
    assert!(parsed.get("completed").is_some());
    assert!(parsed.get("due_reviews").is_some());
}
