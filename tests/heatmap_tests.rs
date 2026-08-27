use chrono::{Duration, Utc};
use spanglings::cli::commands::progress::{compute_activity_summary, render_activity_heatmap};
use spanglings::core::state::AppState;
use std::collections::HashMap;

#[test]
fn test_compute_activity_summary_streaks() {
    let mut state = AppState::default();
    let now = Utc::now();
    let today = now.date_naive();

    // Create 3 consecutive active days: today, yesterday, 2 days ago
    let d0 = today.format("%Y-%m-%d").to_string();
    let d1 = (today - Duration::days(1)).format("%Y-%m-%d").to_string();
    let d2 = (today - Duration::days(2)).format("%Y-%m-%d").to_string();

    state.record_activity(&d0);
    state.record_activity(&d1);
    state.record_activity(&d2);

    let summary = compute_activity_summary(&state, now);
    assert_eq!(summary.current_streak, 3);
    assert_eq!(summary.longest_streak, 3);
    assert_eq!(summary.total_active_days, 3);
}

#[test]
fn test_render_activity_heatmap_format() {
    let mut daily = HashMap::new();
    let now = Utc::now();
    let today_str = now.format("%Y-%m-%d").to_string();
    daily.insert(today_str, 5);

    let rows = render_activity_heatmap(&daily, now, 12);
    assert_eq!(rows.len(), 7, "Heatmap must produce 7 weekday rows");
    for row in rows {
        assert!(row.starts_with("  "), "Row must have indentation");
    }
}
