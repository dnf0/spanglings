use spanglings::core::exercise::Exercise;
use spanglings::engine::accents::AccentMode;
use spanglings::engine::validator::{extract_user_answer, validate_submission, ValidationResult};

#[test]
fn test_validation_success() {
    let content = r#"# Subjunctive 01
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
    let content = r#"# Subjunctive 01
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
    let content = r#"# Subjunctive 01
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
    let content = r#"# Subjunctive 01
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
    let exercise_content = r#"# Subjunctive 01
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

    let user_edited = r#"# Subjunctive 01
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
    assert_eq!(answer, "vengas");
}

#[test]
fn test_extract_user_answer_unmodified_returns_empty() {
    let exercise_content = r#"# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

### Exercise
Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    let exercise = Exercise::from_markdown("test.md", exercise_content).unwrap();
    let answer = extract_user_answer(&exercise, exercise_content);
    assert_eq!(answer, "");
}

#[test]
fn test_dynamic_line_number_in_diagnostic() {
    use std::io::Write;
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    let content = r#"# Subjunctive 01
<!-- id: b1_sub_01 | level: B1 | topic: subjunctive | type: cloze -->

### Exercise
Quiero que tú (venir) ___ a mi fiesta.

<!-- SOLUTION
vengas
-->
"#;
    write!(temp_file, "{}", content).unwrap();

    let exercise = Exercise::from_markdown(temp_file.path(), content).unwrap();
    let result = validate_submission(&exercise, "viene", AccentMode::Forgiving);

    match result {
        ValidationResult::Failed { diagnostic, .. } => {
            assert_eq!(diagnostic.line_number, 5);
        }
        _ => panic!("Expected failed validation"),
    }
}

#[test]
fn test_concept_aware_diagnostic_compiler_fields() {
    let content = r#"# Flight Delays & Gate Changes
<!-- id: b1_sub_temp_01 | level: B1 | topic: subjunctive_temporal | type: cloze | concepts: ["subjunctive_temporal_future", "temporal_connectors"] | prerequisites: ["present_subjunctive_formation"] | grammar_focus: "Temporal conjunctions referring to future events require the subjunctive mood." | contrast_note: "Contrast with habitual past actions which use the indicative." -->

### Context
English: "As soon as we land, I will call you."

### Exercise
En cuanto nosotros (aterrizar) ___ , te llamaré.

<!-- SOLUTION
aterricemos
-->

<!-- DIAGNOSTIC_RULES
pattern: "aterrizamos" | code: "E0501" | message: "Expected Present Subjunctive ('aterricemos') for prospective event, found Indicative ('aterrizamos')."
-->

<!-- HINTS
Tier 1: 'En cuanto' triggers subjunctive when referring to future actions.
Tier 2: Root for aterrizar in subjunctive: 'aterric-'.
Tier 3: Conjugate for nosotros: 'aterricemos'.
-->
"#;

    let exercise = Exercise::from_markdown("exercises/42_travel/01_delays.md", content).unwrap();
    let result = validate_submission(&exercise, "aterrizamos", AccentMode::Forgiving);

    match result {
        ValidationResult::Failed { diagnostic, .. } => {
            assert_eq!(diagnostic.code, "E0501");
            assert_eq!(
                diagnostic.linked_concept,
                Some("subjunctive_temporal_future, temporal_connectors".to_string())
            );
            assert_eq!(
                diagnostic.prerequisite,
                Some("present_subjunctive_formation".to_string())
            );
            assert_eq!(
                diagnostic.grammar_focus,
                Some(
                    "Temporal conjunctions referring to future events require the subjunctive mood."
                        .to_string()
                )
            );
            assert_eq!(
                diagnostic.contrast_note,
                Some("Contrast with habitual past actions which use the indicative.".to_string())
            );

            let formatted = diagnostic.format_terminal();
            assert!(formatted.contains("error[E0501]"));
            assert!(formatted.contains("Grammar Focus: Temporal conjunctions referring to future events require the subjunctive mood."));
            assert!(formatted
                .contains("Linked Concept: subjunctive_temporal_future, temporal_connectors"));
            assert!(formatted.contains("Prerequisite: present_subjunctive_formation"));
            assert!(formatted.contains(
                "Contrast: Contrast with habitual past actions which use the indicative."
            ));
        }
        _ => panic!("Expected failed validation with concept-aware diagnostic"),
    }
}

#[test]
fn test_fallback_diagnostic_populates_concept_links() {
    let content = r#"# Accidental Se Leaks
<!-- id: b1_acc_01 | level: B1 | topic: accidental_se | type: cloze | concepts: ["accidental_involuntary_se"] | prerequisites: ["indirect_object_pronouns"] | grammar_focus: "Accidental se with unexpected events." | contrast_note: "Contrast with direct reflexive." -->

### Exercise
Se nos (romper) ___ una tubería empotrada.

<!-- SOLUTION
rompió
-->
"#;

    let exercise = Exercise::from_markdown("exercises/45_repairs/01_leak.md", content).unwrap();
    let result = validate_submission(&exercise, "rompemos", AccentMode::Forgiving);

    match result {
        ValidationResult::Failed { diagnostic, .. } => {
            assert_eq!(diagnostic.code, "E0001");
            assert_eq!(
                diagnostic.linked_concept,
                Some("accidental_involuntary_se".to_string())
            );
            assert_eq!(
                diagnostic.prerequisite,
                Some("indirect_object_pronouns".to_string())
            );
            assert_eq!(
                diagnostic.grammar_focus,
                Some("Accidental se with unexpected events.".to_string())
            );
            assert_eq!(
                diagnostic.contrast_note,
                Some("Contrast with direct reflexive.".to_string())
            );

            let formatted = diagnostic.format_terminal();
            assert!(formatted.contains("Linked Concept: accidental_involuntary_se"));
            assert!(formatted.contains("Prerequisite: indirect_object_pronouns"));
            assert!(formatted.contains("Grammar Focus: Accidental se with unexpected events."));
            assert!(formatted.contains("Contrast: Contrast with direct reflexive."));
        }
        _ => panic!("Expected fallback diagnostic with populated concept metadata"),
    }
}

#[test]
fn test_language_completeness_diagnostic_rules_e0048_to_e0053() {
    let raw_md = r#"# Adversatives 04
<!-- id: b2_adversative_test | level: B2 | topic: adversatives | type: cloze | concepts: ["adversative_sino_que_clauses"] | prerequisites: ["adversative_pero_vs_sino"] | grammar_focus: "Finite clause substitution with 'sino que'." | contrast_note: "Finite clause substitution (*sino que rediseñó*) vs bare *sino* or *pero*." -->

### Exercise
El equipo no solo corrigió el error en producción, (sino que rediseñó / pero rediseñó) ___ todo el subsistema.

<!-- SOLUTION
sino que rediseñó
-->

<!-- DIAGNOSTIC_RULES
pattern: "pero rediseñó" | code: "E0052" | message: "Substituting with a conjugated verb requires 'sino que rediseñó'."
-->

<!-- HINTS
Tier 1: Use 'sino que' followed by 'rediseñó'.
Tier 2: Form: 'sino que rediseñó'.
Tier 3: Write 'sino que rediseñó'.
-->
"#;

    let exercise = Exercise::from_markdown("exercises/52_adversatives/04_test.md", raw_md).unwrap();
    let result = validate_submission(&exercise, "pero rediseñó", AccentMode::Forgiving);

    match result {
        ValidationResult::Failed { diagnostic, .. } => {
            assert_eq!(diagnostic.code, "E0052");
            assert_eq!(
                diagnostic.title,
                "adversative contrast (pero / sino / sino que)"
            );
            assert!(diagnostic
                .message
                .contains("Substituting with a conjugated verb requires 'sino que rediseñó'."));
            assert_eq!(
                diagnostic.linked_concept,
                Some("adversative_sino_que_clauses".to_string())
            );

            let terminal = diagnostic.format_terminal();
            assert!(terminal.contains("error[E0052]"));
            assert!(terminal.contains("adversative contrast (pero / sino / sino que)"));
            assert!(terminal.contains("Linked Concept: adversative_sino_que_clauses"));
        }
        _ => panic!("Expected E0052 diagnostic match"),
    }
}
