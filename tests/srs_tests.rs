use chrono::Utc;
use spanglings::core::srs::{calculate_sm2_review, SrsItem};
use spanglings::core::state::AppState;
use tempfile::NamedTempFile;

#[test]
fn test_sm2_repetition_intervals() {
    let mut item = SrsItem::default();

    // First successful review (quality 5) -> interval 1 day
    item = calculate_sm2_review(&item, 5, Utc::now());
    assert_eq!(item.repetitions, 1);
    assert_eq!(item.interval_days, 1);

    // Second review (quality 5) -> interval 6 days
    item = calculate_sm2_review(&item, 5, Utc::now());
    assert_eq!(item.repetitions, 2);
    assert_eq!(item.interval_days, 6);

    // Third review (quality 5) -> interval ~15-16 days (6 * 2.6 = 15.6 -> 16)
    item = calculate_sm2_review(&item, 5, Utc::now());
    assert_eq!(item.repetitions, 3);
    assert!(item.interval_days >= 15);

    // Failed review (quality 1) -> resets repetitions, interval back to 1
    item = calculate_sm2_review(&item, 1, Utc::now());
    assert_eq!(item.repetitions, 0);
    assert_eq!(item.interval_days, 1);
}

#[test]
fn test_state_save_and_load() {
    let tmp = NamedTempFile::new().unwrap();
    let path = tmp.path().to_path_buf();

    let mut state = AppState::default();
    state.completed_exercises.insert("b1_sub_01".to_string());
    state.save_to_path(&path).unwrap();

    let loaded = AppState::load_from_path(&path).unwrap();
    assert!(loaded.completed_exercises.contains("b1_sub_01"));
}

#[test]
fn test_state_mark_completed() {
    let mut state = AppState::default();
    state.mark_completed("b1_sub_01");
    assert!(state.completed_exercises.contains("b1_sub_01"));
    assert!(state.stats.contains_key("b1_sub_01"));
    assert!(state.stats.get("b1_sub_01").unwrap().completed_at.is_some());
}
