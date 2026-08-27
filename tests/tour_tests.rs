use spanglings::core::state::AppState;
use tempfile::NamedTempFile;

#[test]
fn test_tour_state_defaults_and_toggle() {
    let mut state = AppState::default();
    assert!(!state.tour_completed);
    state.mark_tour_completed();
    assert!(state.tour_completed);
}

#[test]
fn test_tour_state_serialization_backwards_compatibility() {
    // Old state JSON without tour_completed
    let json = r#"{"completed":{},"srs_items":{},"concept_mastery":{},"version":1}"#;
    let state: AppState = serde_json::from_str(json).expect("Failed to deserialize old state");
    assert!(!state.tour_completed);
}

#[test]
fn test_tour_state_persistence_roundtrip() {
    let tmp = NamedTempFile::new().expect("Failed to create temp file");
    let mut state = AppState::default();
    state.mark_tour_completed();
    state
        .save_to_path(tmp.path())
        .expect("Failed to save state");

    let loaded = AppState::load_from_path(tmp.path()).expect("Failed to load state");
    assert!(loaded.tour_completed);
}
