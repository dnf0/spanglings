use spanglings::core::curriculum::{find_all_exercises_or_embedded, Level};
use spanglings::core::embedded::{get_embedded_exercises, init_exercises_dir};
use tempfile::tempdir;

#[test]
fn test_get_embedded_exercises_loads_full_catalog() {
    let exercises = get_embedded_exercises().expect("Failed to load embedded exercises");
    assert_eq!(
        exercises.len(),
        339,
        "Expected exactly 339 embedded exercises, found {}",
        exercises.len()
    );
    assert!(exercises.iter().any(|e| e.level == Level::B1));
    assert!(exercises.iter().any(|e| e.level == Level::B2));
    assert!(exercises.iter().any(|e| e.level == Level::C1));
}

#[test]
fn test_init_exercises_dir_writes_files() {
    let temp = tempdir().unwrap();
    let target = temp.path().join("exercises");
    let count = init_exercises_dir(&target, false).expect("Failed to init exercises");
    assert_eq!(count, 339);
    assert!(target.join("00_baseline").exists());
    assert!(target.join("03_subjunctive_weirdo").exists());
    assert!(target.join("21_nuanced_collocations").exists());
    assert!(target.join("47_conversational_markers_and_nuance").exists());
    assert!(target
        .join("48_epistemic_conjecture_and_probability")
        .exists());
    assert!(target
        .join("53_independent_subjunctives_and_legal_tenses")
        .exists());
    assert!(target
        .join("54_verbs_of_becoming_and_transformation")
        .exists());
    assert!(target
        .join("59_scalar_concession_and_intensive_connectors")
        .exists());
}

#[test]
fn test_find_all_exercises_or_embedded_fallback() {
    let temp = tempdir().unwrap();
    let non_existent = temp.path().join("empty_dir/exercises");
    let exercises = find_all_exercises_or_embedded(&non_existent).expect("Fallback failed");
    assert_eq!(exercises.len(), 339);
}
