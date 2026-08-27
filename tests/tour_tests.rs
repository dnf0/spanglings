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

use spanglings::cli::commands::tour::{get_tour_stations, run_tour};

#[test]
fn test_get_tour_stations_contains_all_six_stations() {
    let stations = get_tour_stations();
    assert_eq!(stations.len(), 6);
    assert_eq!(stations[0].id, "philosophy");
    assert_eq!(stations[1].id, "anatomy_accents");
    assert_eq!(stations[2].id, "diagnostics");
    assert_eq!(stations[3].id, "hints_reference");
    assert_eq!(stations[4].id, "tools_placement");
    assert_eq!(stations[5].id, "workflows");

    for station in stations {
        assert!(!station.title.is_empty());
        assert!(!station.description.is_empty());
        assert!(!station.bullet_points.is_empty());
    }
}

#[test]
fn test_run_tour_non_interactive_skip_challenges() {
    // In test environment (!is_terminal or skip_challenges = true), it executes and marks state completed
    let res = run_tour(true);
    assert!(res.is_ok());
}
