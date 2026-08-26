use spanglings::core::curriculum::Level;
use spanglings::core::exercise::{Exercise, ExerciseType};

#[test]
fn test_parse_valid_markdown_exercise() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01: Verbs of Influence
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
    assert!(!exercise.is_done);
    assert_eq!(exercise.solution, "vengas");
    assert_eq!(exercise.alternatives, vec!["vengas tú"]);
    assert_eq!(exercise.hints.len(), 3);
    assert_eq!(exercise.diagnostic_rules.len(), 1);
    assert_eq!(exercise.diagnostic_rules[0].code, "E0301");
}
