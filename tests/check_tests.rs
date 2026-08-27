use spanglings::cli::commands::check::{evaluate_exercise_for_check, run_check};
use spanglings::core::exercise::Exercise;
use spanglings::engine::accents::AccentMode;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_evaluate_exercise_for_check_passed() {
    let raw = r#"<!-- id: test_check_01 | level: B1 | topic: subjunctive | type: cloze -->
# Subjunctive Test

### Context
English: I hope you come

### Exercise
Espero que tú vengas.

<!-- SOLUTION
vengas
-->
"#;
    let temp = NamedTempFile::new().unwrap();
    fs::write(temp.path(), raw).unwrap();

    let ex = Exercise::from_markdown(temp.path(), raw).unwrap();
    let report = evaluate_exercise_for_check(&ex, raw, AccentMode::Forgiving);

    assert!(report.passed);
    assert_eq!(report.exercise_id, "test_check_01");
    assert!(report.diagnostics.is_empty());
}

#[test]
fn test_evaluate_exercise_for_check_failed_with_diagnostic() {
    let raw = r#"<!-- I AM NOT DONE -->
<!-- id: test_check_02 | level: B1 | topic: subjunctive | type: cloze -->
# Subjunctive Test

### Context
English: I hope you come

### Exercise
Espero que tú viene.

<!-- SOLUTION
vengas
-->

<!-- DIAGNOSTIC_RULES
pattern: "viene" | code: "E0301" | message: "Expected Subjunctive, found Indicative."
-->
"#;
    let temp = NamedTempFile::new().unwrap();
    fs::write(temp.path(), raw).unwrap();

    let ex = Exercise::from_markdown(temp.path(), raw).unwrap();
    let report = evaluate_exercise_for_check(&ex, raw, AccentMode::Forgiving);

    assert!(!report.passed);
    assert!(!report.is_done);
    assert_eq!(report.diagnostics.len(), 1);
    assert_eq!(report.diagnostics[0].code, "E0301");
    assert_eq!(report.diagnostics[0].severity, "error");
}

#[test]
fn test_run_check_on_exercise_file() {
    let raw = r#"<!-- id: test_check_03 | level: B1 | topic: subjunctive | type: cloze -->
# Subjunctive Test

### Context
English: I hope you come

### Exercise
Espero que tú vengas.

<!-- SOLUTION
vengas
-->
"#;
    let temp = NamedTempFile::new().unwrap();
    fs::write(temp.path(), raw).unwrap();

    let passed = run_check(Some(temp.path().to_str().unwrap()), true, false).unwrap();
    assert!(passed);
}
