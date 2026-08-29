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
    let _ = summary.concept_mastery;
    let _ = summary.weakest_concepts;
}

#[test]
fn test_concept_mastery_initialization_and_success_updates() {
    let mut state = AppState::default();
    let now = Utc::now();

    assert!(state.concept_mastery.is_empty());

    // 1st success review
    state.update_concept_mastery("subjunctive_temporal_future", 5, now);
    let mastery = state
        .concept_mastery
        .get("subjunctive_temporal_future")
        .expect("Concept mastery should be initialized");

    assert_eq!(mastery.concept_id, "subjunctive_temporal_future");
    assert_eq!(mastery.total_reviews, 1);
    assert_eq!(mastery.repetitions, 1);
    assert_eq!(mastery.interval_days, 1);
    assert_eq!(mastery.lapses, 0);
    assert!(mastery.mastery_score > 0.02 && mastery.mastery_score < 0.05);
    assert_eq!(mastery.last_practiced, Some(now));

    // 2nd success review (quality 4 >= 3)
    let t2 = now + Duration::days(1);
    state.update_concept_mastery("subjunctive_temporal_future", 4, t2);
    let mastery2 = state
        .concept_mastery
        .get("subjunctive_temporal_future")
        .unwrap();
    assert_eq!(mastery2.total_reviews, 2);
    assert_eq!(mastery2.repetitions, 2);
    assert_eq!(mastery2.interval_days, 6);
    assert_eq!(mastery2.lapses, 0);
    assert!(mastery2.mastery_score > 0.14 && mastery2.mastery_score < 0.20);
    assert_eq!(mastery2.last_practiced, Some(t2));

    // Multiple successes should asymptotically approach and cap at 1.0
    for _ in 0..50 {
        state.update_concept_mastery("subjunctive_temporal_future", 5, t2);
    }
    let mastery_capped = state
        .concept_mastery
        .get("subjunctive_temporal_future")
        .unwrap();
    assert!((mastery_capped.mastery_score - 1.0).abs() < 1e-4);
    assert!(mastery_capped.mastery_score <= 1.0);
}

#[test]
fn test_concept_mastery_lapse_penalization_and_recovery() {
    let mut state = AppState::default();
    let now = Utc::now();

    // Establish initial mastery (2 successful reviews)
    state.update_concept_mastery("accidental_se", 5, now);
    state.update_concept_mastery("accidental_se", 5, now + Duration::days(1));
    let score_before = state
        .concept_mastery
        .get("accidental_se")
        .unwrap()
        .mastery_score;
    assert!(score_before > 0.14 && score_before < 0.20);

    // Lapse (quality 2 < 3)
    let lapse_time = now + Duration::days(2);
    state.update_concept_mastery("accidental_se", 2, lapse_time);
    let lapsed = state.concept_mastery.get("accidental_se").unwrap();
    assert_eq!(lapsed.lapses, 1);
    assert_eq!(lapsed.repetitions, 0);
    assert_eq!(lapsed.interval_days, 1);
    assert_eq!(lapsed.total_reviews, 3);
    assert_eq!(lapsed.mastery_score, 0.0);
    assert_eq!(lapsed.last_practiced, Some(lapse_time));

    // Subsequent lapse (quality 1 < 3)
    state.update_concept_mastery("accidental_se", 1, lapse_time + Duration::hours(1));
    let lapsed2 = state.concept_mastery.get("accidental_se").unwrap();
    assert_eq!(lapsed2.lapses, 2);
    assert_eq!(lapsed2.repetitions, 0);
    assert_eq!(lapsed2.interval_days, 1);
    assert_eq!(lapsed2.mastery_score, 0.0);

    // Recovery on success (quality 5 >= 3)
    state.update_concept_mastery("accidental_se", 5, lapse_time + Duration::days(1));
    let recovered = state.concept_mastery.get("accidental_se").unwrap();
    assert_eq!(recovered.lapses, 2);
    assert_eq!(recovered.repetitions, 1);
    assert_eq!(recovered.total_reviews, 5);
    assert!(recovered.mastery_score > 0.02 && recovered.mastery_score < 0.05);
}

#[test]
fn test_get_weakest_concepts_sorting_and_limiting() {
    let mut state = AppState::default();
    let now = Utc::now();

    state.update_concept_mastery("concept_a", 5, now);
    state.update_concept_mastery("concept_b", 5, now);
    state.update_concept_mastery("concept_b", 5, now);
    state.update_concept_mastery("concept_c", 1, now);

    let weakest_2 = state.get_weakest_concepts(2);
    assert_eq!(weakest_2.len(), 2);
    assert_eq!(weakest_2[0].0, "concept_c");
    assert_eq!(weakest_2[1].0, "concept_a");

    let all_weakest = state.get_weakest_concepts(10);
    assert_eq!(all_weakest.len(), 3);
    assert_eq!(all_weakest[0].0, "concept_c");
    assert_eq!(all_weakest[1].0, "concept_a");
    assert_eq!(all_weakest[2].0, "concept_b");
}

#[test]
fn test_get_concept_mastery_scores() {
    let mut state = AppState::default();
    let now = Utc::now();

    state.update_concept_mastery("por_vs_para_foundations", 5, now);
    state.update_concept_mastery("irregular_preterite_stems", 5, now);
    state.update_concept_mastery("irregular_preterite_stems", 5, now);

    let scores = state.get_concept_mastery_scores();
    assert_eq!(scores.len(), 2);
    let por_para_score = *scores.get("por_vs_para_foundations").unwrap();
    let pret_score = *scores.get("irregular_preterite_stems").unwrap();
    assert!(por_para_score > 0.02 && por_para_score < 0.05);
    assert!(pret_score > 0.14 && pret_score < 0.20);
}

#[test]
fn test_state_backwards_compatibility_without_concept_mastery() {
    let json_legacy = r#"{
        "version": 1,
        "completed_exercises": ["ex01"],
        "current_exercise": null,
        "accent_mode": "Forgiving",
        "srs": {},
        "stats": {}
    }"#;

    let state: AppState =
        serde_json::from_str(json_legacy).expect("Should deserialize legacy state");
    assert!(state.concept_mastery.is_empty());
    assert!(state.get_weakest_concepts(5).is_empty());
    assert!(state.get_concept_mastery_scores().is_empty());
}

#[test]
fn test_scientific_sm2_concept_mastery_progression() {
    let mut state = AppState::default();
    let now = Utc::now();

    // 1st review (fast, quality 5): ~3%
    state.update_concept_mastery("por-para", 5, now);
    let m1 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m1.repetitions, 1);
    assert_eq!(m1.interval_days, 1);
    assert!(m1.mastery_score > 0.02 && m1.mastery_score < 0.05);

    // 2nd review (quality 5): ~17%
    state.update_concept_mastery("por-para", 5, now);
    let m2 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m2.repetitions, 2);
    assert_eq!(m2.interval_days, 6);
    assert!(m2.mastery_score > 0.14 && m2.mastery_score < 0.20);

    // 3rd review (quality 5): ~38%
    state.update_concept_mastery("por-para", 5, now);
    let m3 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m3.repetitions, 3);
    assert!(m3.mastery_score > 0.30 && m3.mastery_score < 0.45);

    // Lapse on mistake (quality 1): resets score to 0 and increments lapse count
    state.update_concept_mastery("por-para", 1, now);
    let m_lapse = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m_lapse.repetitions, 0);
    assert_eq!(m_lapse.interval_days, 1);
    assert_eq!(m_lapse.lapses, 1);
    assert_eq!(m_lapse.mastery_score, 0.0);
}
