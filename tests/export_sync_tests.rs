use spanglings::cli::commands::export::{
    generate_anki_tsv, generate_json_export, generate_markdown_notes,
};
use spanglings::cli::commands::sync::{export_state_json, import_state_json};
use spanglings::core::curriculum::Level;
use spanglings::core::exercise::{Exercise, ExerciseType};
use spanglings::core::state::AppState;
use std::path::PathBuf;

fn sample_exercises() -> Vec<Exercise> {
    vec![
        Exercise {
            path: PathBuf::from("exercises/01_subj.md"),
            id: "b1_subj_01".to_string(),
            level: Level::B1,
            topic: "subjunctive_weirdo".to_string(),
            exercise_type: ExerciseType::Cloze,
            is_done: false,
            title: "Subjunctive Verbs of Influence".to_string(),
            solution: "vengas".to_string(),
            alternatives: vec!["vengas tú".to_string()],
            diagnostic_rules: vec![],
            hints: vec!["Root is veng-".to_string()],
            raw_content: "### Context\nEnglish: Come\n### Exercise\nQuiero que tú vengas"
                .to_string(),
            concept_tags: vec![],
            prerequisites: vec![],
            grammar_focus: None,
            contrast_note: None,
        },
        Exercise {
            path: PathBuf::from("exercises/02_por.md"),
            id: "b1_por_para_01".to_string(),
            level: Level::B1,
            topic: "por_vs_para".to_string(),
            exercise_type: ExerciseType::Cloze,
            is_done: false,
            title: "Por vs Para Purpose".to_string(),
            solution: "para".to_string(),
            alternatives: vec![],
            diagnostic_rules: vec![],
            hints: vec!["Destination".to_string()],
            raw_content: "### Context\nEnglish: For you\n### Exercise\nEsto es para ti".to_string(),
            concept_tags: vec![],
            prerequisites: vec![],
            grammar_focus: None,
            contrast_note: None,
        },
    ]
}

#[test]
fn test_anki_tsv_export_generation() {
    let exercises = sample_exercises();
    let state = AppState::default();

    let tsv = generate_anki_tsv(&exercises, &state, false, None, None);
    assert!(tsv.contains("#separator:tab"));
    assert!(tsv.contains("#html:true"));
    assert!(tsv.contains("#tags column:3"));
    assert!(tsv.contains("vengas"));
    assert!(tsv.contains("spanglings B1 subjunctive_weirdo"));
    assert!(tsv.contains("para"));

    // Test with level filter
    let b1_tsv = generate_anki_tsv(&exercises, &state, false, Some("B1"), None);
    assert!(b1_tsv.contains("vengas"));

    // Test with nonexistent level
    let c1_tsv = generate_anki_tsv(&exercises, &state, false, Some("C1"), None);
    assert!(!c1_tsv.contains("vengas"));
}

#[test]
fn test_markdown_study_guide_export() {
    let exercises = sample_exercises();
    let state = AppState::default();

    let md = generate_markdown_notes(&exercises, &state, None, None);
    assert!(md.contains("# Spanglings Study Notes & Curriculum Guide"));
    assert!(md.contains("## Topic: subjunctive_weirdo [B1]"));
    assert!(md.contains("`b1_subj_01`"));
    assert!(md.contains("`vengas`"));
    assert!(md.contains("Root is veng-"));
}

#[test]
fn test_json_export_generation() {
    let exercises = sample_exercises();
    let mut state = AppState::default();
    state.mark_completed("b1_subj_01");

    let json_str = generate_json_export(&exercises, &state, None, None).unwrap();
    assert!(json_str.contains("b1_subj_01"));
    assert!(json_str.contains("\"is_completed\": true"));
    assert!(json_str.contains("b1_por_para_01"));
}

#[test]
fn test_state_sync_export_and_import() {
    let mut state = AppState::default();
    state.mark_completed("b1_subj_01");
    let now = chrono::Utc::now();
    state.update_srs("b1_subj_01", 5, now);

    let exported = export_state_json(&state).unwrap();
    assert!(exported.contains("b1_subj_01"));

    let mut new_state = AppState::default();
    assert!(!new_state.is_completed("b1_subj_01"));

    let merged_count = import_state_json(&exported, &mut new_state).unwrap();
    assert!(merged_count >= 1);
    assert!(new_state.is_completed("b1_subj_01"));
    assert_eq!(new_state.srs.get("b1_subj_01").unwrap().repetitions, 1);
}
