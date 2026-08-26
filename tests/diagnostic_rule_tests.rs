use spanglings::core::exercise::Exercise;
use spanglings::engine::accents::AccentMode;
use spanglings::engine::validator::{extract_user_answer, validate_submission, ValidationResult};

#[test]
fn test_validation_success() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let exercise =
        Exercise::from_markdown("exercises/03_subjunctive/b1_sub_01.md", content).unwrap();
    let result = validate_submission(&exercise, "vengas", AccentMode::Forgiving);
    assert!(result.is_success());
}

#[test]
fn test_validation_alternative_success() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->

<!-- ALTERNATIVES
vengas tú
-->
"#;
    let exercise =
        Exercise::from_markdown("exercises/03_subjunctive/b1_sub_01.md", content).unwrap();
    let result = validate_submission(&exercise, "vengas tú", AccentMode::Forgiving);
    assert!(result.is_success());
}

#[test]
fn test_validation_targeted_diagnostic_error() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->

<!-- DIAGNOSTIC_RULES
pattern: "viene" | code: "E0301" | message: "Expected Present Subjunctive ('vengas'), found Indicative ('viene')."
-->

<!-- HINTS
Tier 1: Check the main clause verb.
Tier 2: Use the irregular root 'veng-'.
Tier 3: Add '-as' to form 'vengas'.
-->
"#;
    let exercise =
        Exercise::from_markdown("exercises/03_subjunctive/b1_sub_01.md", content).unwrap();
    let result = validate_submission(&exercise, "viene", AccentMode::Forgiving);

    match result {
        ValidationResult::Failed { diagnostic, .. } => {
            assert_eq!(diagnostic.code, "E0301");
            assert!(diagnostic.message.contains("Expected Present Subjunctive"));
            assert_eq!(
                diagnostic.file_path,
                "exercises/03_subjunctive/b1_sub_01.md"
            );
            let formatted = diagnostic.format_terminal();
            assert!(formatted.contains("error[E0301]"));
            assert!(formatted.contains("Expected Present Subjunctive"));
        }
        _ => panic!("Expected failed validation with targeted diagnostic"),
    }
}

#[test]
fn test_validation_general_fallback_diagnostic() {
    let content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let exercise =
        Exercise::from_markdown("exercises/03_subjunctive/b1_sub_01.md", content).unwrap();
    let result = validate_submission(&exercise, "comas", AccentMode::Forgiving);

    match result {
        ValidationResult::Failed { diagnostic, .. } => {
            assert_eq!(diagnostic.code, "E0001");
            assert!(diagnostic.message.contains("vengas"));
        }
        _ => panic!("Expected general fallback diagnostic"),
    }
}

#[test]
fn test_extract_user_answer() {
    let exercise_content = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

> **Grammar Rule**: Subjunctive required.

### Context
English: "I want you to come."

### Exercise
Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let exercise = Exercise::from_markdown("test.md", exercise_content).unwrap();

    let user_edited = r#"<!-- I AM NOT DONE -->
# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

> **Grammar Rule**: Subjunctive required.

### Context
English: "I want you to come."

### Exercise
Quiero que tú (venir) vengas a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let answer = extract_user_answer(&exercise, user_edited);
    assert!(!answer.is_empty());
}
