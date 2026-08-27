use chrono::{Duration, Utc};
use spanglings::cli::commands::progress::{
    compute_weakness_profile, get_progress_json, ProgressSummary,
};
use spanglings::core::curriculum::find_all_exercises_or_embedded;
use spanglings::core::srs::SrsItem;
use spanglings::core::state::{AppState, ExerciseStat};

#[test]
fn test_weakness_profiler_detects_low_ease_and_lapses() {
    let exercises = find_all_exercises_or_embedded("exercises").expect("find exercises");
    let mut state = AppState::default();
    let now = Utc::now();

    // Add a failing/struggling SRS item in 'subjunctive'
    let mut subj_item = SrsItem::new(now - Duration::days(5));
    subj_item.ease_factor = 1.7;
    subj_item.repetitions = 0;
    subj_item.interval_days = 1;
    subj_item.next_review_due = now - Duration::hours(2);

    state
        .srs
        .insert("b1_subj_weirdo_wishes".to_string(), subj_item);
    state.stats.insert(
        "b1_subj_weirdo_wishes".to_string(),
        ExerciseStat {
            attempts: 4,
            completed_at: None,
            hints_used: 2,
        },
    );

    let profile = compute_weakness_profile(&exercises, &state, now);
    assert!(
        !profile.is_empty(),
        "Weakness profile should detect weak topics"
    );

    let top = &profile[0];
    assert!(
        top.topic.contains("subjunctive"),
        "Top weak topic should be subjunctive, got {}",
        top.topic
    );
    assert!(top.lapses > 0);
    assert!(top.avg_ease_factor < 2.3);
    assert!(
        top.recommendation.contains("spanglings explain")
            || top.recommendation.contains("spanglings drill")
    );
}

#[test]
fn test_progress_json_includes_weak_topics_and_recommendations() {
    let json_str = get_progress_json().expect("get_progress_json should succeed");
    let summary: ProgressSummary = serde_json::from_str(&json_str).expect("Valid JSON summary");
    assert!(summary.total > 100);
    // weak_topics and recommendations fields must be valid arrays
    let _ = summary.weak_topics;
    let _ = summary.recommendations;
}
