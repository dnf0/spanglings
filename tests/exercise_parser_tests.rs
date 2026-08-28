use spanglings::core::curriculum::Level;
use spanglings::core::exercise::{Exercise, ExerciseType};

#[test]
fn test_parse_valid_markdown_exercise() {
    let content = r#"# Subjunctive 01: Verbs of Influence
<!-- id: b1_subjunctive_01 | level: B1 | topic: subjunctive_weirdo | type: cloze -->

> **Grammar Rule**: Verbs of wishing/influence require subjunctive with subject change.

### Context
English: "I want you to come."

### Exercise
Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->

<!-- ALTERNATIVES
vengas tú
-->

<!-- DIAGNOSTIC_RULES
pattern: "viene" | code: "E0301" | message: "Expected Subjunctive ('vengas'), found Indicative ('viene')."
-->

<!-- HINTS
Tier 1: Check the main verb.
Tier 2: Irregular root 'veng-'.
Tier 3: Add -as -> 'vengas'.
-->
"#;

    let exercise = Exercise::from_markdown("exercises/03_subjunctive/subjunctive_01.md", content)
        .expect("Failed to parse exercise");

    assert_eq!(exercise.id, "b1_subjunctive_01");
    assert_eq!(exercise.level, Level::B1);
    assert_eq!(exercise.exercise_type, ExerciseType::Cloze);
    assert!(!exercise.is_done); // Has unfilled blank ___
    assert_eq!(exercise.solution, "vengas");
    assert_eq!(exercise.alternatives, vec!["vengas tú"]);
    assert_eq!(exercise.hints.len(), 3);
    assert_eq!(exercise.diagnostic_rules.len(), 1);
    assert_eq!(exercise.diagnostic_rules[0].code, "E0301");

    // Completed version without blank placeholder
    let completed_content = content.replace("___", "vengas");
    let completed_ex = Exercise::from_markdown(
        "exercises/03_subjunctive/subjunctive_01.md",
        &completed_content,
    )
    .expect("Failed to parse completed exercise");
    assert!(completed_ex.is_done);
}

#[test]
fn test_parse_missing_id_error() {
    let content = r#"
# Test Exercise
<!-- level: B1 | topic: subjunctive | type: cloze -->
"#;
    let result = Exercise::from_markdown("exercises/test.md", content);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_level_error() {
    let content = r#"
# Test Exercise
<!-- id: test_01 | level: XYZ | topic: subjunctive | type: cloze -->
"#;
    let result = Exercise::from_markdown("exercises/test.md", content);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_type_error() {
    let content = r#"
# Test Exercise
<!-- id: test_01 | level: B1 | topic: subjunctive | type: invalid_type -->
"#;
    let result = Exercise::from_markdown("exercises/test.md", content);
    assert!(result.is_err());
}

#[test]
fn test_parse_exercise_with_concepts_and_prerequisites() {
    let content = r#"<!--
id: test_concepts_01
level: B1
type: cloze
title: Test Concept Linking
topic: travel_logistics_and_borders
concepts: ["subjunctive_temporal_future", "impersonal_se"]
prerequisites: ["05_subjunctive_conjunctions"]
grammar_focus: "Subjunctive required for prospective time clauses."
contrast_note: "Contrast with indicative for habitual actions."
-->

### Context
Test context

### Exercise
En cuanto <!-- ANSWER -->, saldremos.
"#;
    let ex = Exercise::from_markdown("exercises/test.md", content).expect("Failed to parse");
    assert_eq!(
        ex.concept_tags,
        vec!["subjunctive_temporal_future", "impersonal_se"]
    );
    assert_eq!(ex.prerequisites, vec!["05_subjunctive_conjunctions"]);
    assert_eq!(
        ex.grammar_focus.as_deref(),
        Some("Subjunctive required for prospective time clauses.")
    );
    assert_eq!(
        ex.contrast_note.as_deref(),
        Some("Contrast with indicative for habitual actions.")
    );
    assert_eq!(ex.title, "Test Concept Linking");
    assert!(!ex.is_done);
}

#[test]
fn test_parse_exercise_with_concept_tags_and_comma_separated() {
    let content = r#"# Header Title
<!-- id: test_concepts_02 | level: B2 | topic: banking | type: transformation | concept_tags: concept_a, concept_b | prerequisites: prereq_1, prereq_2 | grammar_focus: Unquoted grammar focus | contrast_note: Unquoted contrast note -->

### Context
Test context
"#;
    let ex = Exercise::from_markdown("exercises/test2.md", content).expect("Failed to parse");
    assert_eq!(ex.concept_tags, vec!["concept_a", "concept_b"]);
    assert_eq!(ex.prerequisites, vec!["prereq_1", "prereq_2"]);
    assert_eq!(ex.grammar_focus.as_deref(), Some("Unquoted grammar focus"));
    assert_eq!(ex.contrast_note.as_deref(), Some("Unquoted contrast note"));
    assert_eq!(ex.title, "Header Title");
    assert!(ex.is_done);
}
