use spanglings::core::curriculum::Level;
use spanglings::core::placement::{
    calculate_cefr_level, evaluate_placement_test, get_placement_battery,
};
use spanglings::engine::accents::AccentMode;
use std::collections::HashMap;

#[test]
fn test_placement_battery_generation_and_filtering() {
    let full_battery = get_placement_battery(None);
    assert_eq!(full_battery.len(), 15, "Expected 15 calibrated placement questions in full battery");
    assert!(full_battery.iter().any(|q| q.level == Level::Baseline));
    assert!(full_battery.iter().any(|q| q.level == Level::B1));
    assert!(full_battery.iter().any(|q| q.level == Level::B2));
    assert!(full_battery.iter().any(|q| q.level == Level::C1));

    let b1_only = get_placement_battery(Some(Level::B1));
    assert_eq!(b1_only.len(), 4);
    assert!(b1_only.iter().all(|q| q.level == Level::B1));
}

#[test]
fn test_placement_evaluation_accuracy_and_scoring() {
    let battery = get_placement_battery(Some(Level::B1));
    let answers = vec![
        "había revisado".to_string(),
        "actualice".to_string(),
        "por, para".to_string(),
        "se le cayeron".to_string(),
    ];

    let result = evaluate_placement_test(&battery, &answers, AccentMode::Forgiving);
    assert_eq!(result.total_questions, 4);
    assert_eq!(result.total_correct, 4);
    assert_eq!(result.percentage, 100.0);
    assert!(result.passed_levels.contains(&Level::B1));
}

#[test]
fn test_placement_cefr_level_calculation() {
    let mut scores = HashMap::new();
    scores.insert(Level::Baseline, (3, 3)); // 100%
    scores.insert(Level::B1, (4, 4));       // 100%
    scores.insert(Level::B2, (3, 4));       // 75%
    scores.insert(Level::C1, (1, 4));       // 25%

    let (level, score) = calculate_cefr_level(&scores);
    assert_eq!(level, Level::B2);
    assert!(score >= 70.0);
}
