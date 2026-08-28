use spanglings::core::curriculum::load_curriculum;
use spanglings::core::exercise::Exercise;

#[test]
fn test_curriculum_exercises_convert_to_drill_items() {
    let curriculum = load_curriculum().expect("Failed to load curriculum");
    let mut total_drills = 0;
    for exercise in &curriculum.exercises {
        let drills = exercise.to_drill_items();
        total_drills += drills.len();
    }
    assert!(
        total_drills >= 300,
        "Should extract at least 300 drill items from curriculum, found {}",
        total_drills
    );
}

#[test]
fn test_extracted_drill_items_have_valid_fields() {
    let curriculum = load_curriculum().expect("Failed to load curriculum");
    for exercise in &curriculum.exercises {
        for item in exercise.to_drill_items() {
            assert!(!item.topic.is_empty(), "Topic should not be empty");
            assert!(
                !item.trigger_sentence.is_empty(),
                "Trigger sentence should not be empty for exercise {}",
                exercise.id
            );
            assert!(
                !item.target.is_empty(),
                "Target should not be empty for exercise {}",
                exercise.id
            );
            assert!(
                !item.explanation.is_empty(),
                "Explanation should not be empty for exercise {}",
                exercise.id
            );
            let prompt = item.format_prompt(1, 10);
            assert!(
                !prompt.is_empty(),
                "format_prompt should produce non-empty string"
            );
            let hint = item.format_hint();
            assert!(
                !hint.is_empty(),
                "format_hint should produce non-empty string"
            );
        }
    }
}

#[test]
fn test_extracted_drill_item_from_custom_markdown() {
    let md = r#"# Baseline 01: Present Irregular 'Yo' Forms
<!-- id: b0_custom_test | level: Baseline | topic: baseline_present_stems | type: cloze | concepts: ["irregular_present_stems"] | prerequisites: [] | grammar_focus: "Irregular present indicative yo-form and vowel mutation." | contrast_note: "Contrast regular -er with stem mutation." -->

### Exercise
Yo no (caber) ___ en este coche tan pequeño.

<!-- SOLUTION
quepo
-->
"#;
    let exercise =
        Exercise::from_markdown("exercises/custom.md", md).expect("Failed to parse markdown");
    let items = exercise.to_drill_items();
    assert_eq!(items.len(), 1);
    let item = &items[0];
    assert_eq!(item.target, "quepo");
    assert_eq!(item.target_verb, "caber");
    assert_eq!(item.target_subject, "yo");
    assert!(item.trigger_sentence.contains("___"));
    assert_eq!(item.formula_cue, "Contrast regular -er with stem mutation.");
}
