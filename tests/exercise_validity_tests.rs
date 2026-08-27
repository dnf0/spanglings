use spanglings::core::curriculum::find_all_exercises;
use spanglings::core::exercise::Exercise;
use spanglings::engine::accents::AccentMode;
use spanglings::engine::validator::{validate_submission, ValidationResult};
use std::fs;
use std::path::{Path, PathBuf};

fn collect_md_paths(dir: &Path, paths: &mut Vec<PathBuf>) {
    if dir.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .expect("read_dir failed")
            .filter_map(|e| e.ok())
            .collect();
        entries.sort_by_key(|e| e.path());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                collect_md_paths(&path, paths);
            } else if path.extension().is_some_and(|ext| ext == "md") {
                paths.push(path);
            }
        }
    }
}

#[test]
fn test_all_curriculum_exercises_are_valid_and_solvable() {
    let exercises_dir = Path::new("exercises");
    assert!(
        exercises_dir.exists(),
        "exercises directory must exist at workspace root"
    );

    let mut md_paths = Vec::new();
    collect_md_paths(exercises_dir, &mut md_paths);

    assert_eq!(
        md_paths.len(),
        339,
        "Expected exactly 339 exercises across all 60 tracks, found {}",
        md_paths.len()
    );

    // Also verify find_all_exercises finds all of them
    let discovered_exercises =
        find_all_exercises(exercises_dir).expect("find_all_exercises should succeed");
    assert_eq!(
        discovered_exercises.len(),
        339,
        "find_all_exercises should return all 339 discovered exercises"
    );

    let graph = spanglings::core::graph::get_default_linguistic_graph();
    assert!(
        graph.validate_no_cycles().is_ok(),
        "Default linguistic ontology graph must be a valid DAG without cycles"
    );

    for path in &md_paths {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("Failed to read exercise file at {:?}: {}", path, e));

        let exercise = Exercise::from_markdown(path, &content)
            .unwrap_or_else(|e| panic!("Failed to parse exercise at {:?}: {}", path, e));

        assert!(
            !exercise.id.is_empty(),
            "Exercise at {:?} has empty id",
            path
        );
        assert!(
            !exercise.title.is_empty(),
            "Exercise at {:?} has empty title",
            path
        );
        assert!(
            !exercise.solution.is_empty(),
            "Exercise at {:?} has empty solution",
            path
        );
        assert!(
            !exercise.concept_tags.is_empty(),
            "Exercise '{}' at {:?} must have at least one concept tag",
            exercise.id,
            path
        );
        assert!(
            exercise.grammar_focus.is_some(),
            "Exercise '{}' at {:?} must have a grammar_focus note",
            exercise.id,
            path
        );

        // Verify that all concept_tags and prerequisites exist in the default ontology graph
        for tag in &exercise.concept_tags {
            let cid = spanglings::core::graph::ConceptId::from(tag.as_str());
            assert!(
                graph.nodes.contains_key(&cid),
                "Exercise '{}' ({:?}) references unknown concept '{}' not in ontology graph",
                exercise.id,
                path,
                tag
            );
        }
        for prereq in &exercise.prerequisites {
            let cid = spanglings::core::graph::ConceptId::from(prereq.as_str());
            assert!(
                graph.nodes.contains_key(&cid),
                "Exercise '{}' ({:?}) references unknown prerequisite concept '{}' not in ontology graph",
                exercise.id,
                path,
                prereq
            );
        }

        // 1. Primary solution validates with AccentMode::Forgiving
        let forgiving_res =
            validate_submission(&exercise, &exercise.solution, AccentMode::Forgiving);
        assert!(
            forgiving_res.is_success(),
            "Exercise '{}' ({:?}) primary solution '{}' failed forgiving validation: {:?}",
            exercise.id,
            path,
            exercise.solution,
            forgiving_res
        );

        // 2. Primary solution validates with AccentMode::Strict
        let strict_res = validate_submission(&exercise, &exercise.solution, AccentMode::Strict);
        assert!(
            strict_res.is_success(),
            "Exercise '{}' ({:?}) primary solution '{}' failed strict validation: {:?}",
            exercise.id,
            path,
            exercise.solution,
            strict_res
        );

        // 3. Every alternative validates with AccentMode::Forgiving
        for alt in &exercise.alternatives {
            let alt_res = validate_submission(&exercise, alt, AccentMode::Forgiving);
            assert!(
                alt_res.is_success(),
                "Exercise '{}' ({:?}) alternative '{}' failed validation: {:?}",
                exercise.id,
                path,
                alt,
                alt_res
            );
        }

        // 4. Every diagnostic rule triggers the expected diagnostic failure
        for rule in &exercise.diagnostic_rules {
            let diag_res = validate_submission(&exercise, &rule.pattern, AccentMode::Forgiving);
            match diag_res {
                ValidationResult::Failed {
                    diagnostic,
                    user_input,
                } => {
                    assert_eq!(
                        diagnostic.code, rule.code,
                        "Exercise '{}' ({:?}): submitting diagnostic pattern '{}' triggered code '{}', expected '{}'",
                        exercise.id, path, rule.pattern, diagnostic.code, rule.code
                    );
                    assert!(
                        !diagnostic.message.is_empty(),
                        "Exercise '{}' ({:?}): diagnostic message should not be empty for code '{}'",
                        exercise.id, path, rule.code
                    );
                    assert_eq!(
                        user_input, rule.pattern,
                        "Exercise '{}' ({:?}): diagnostic user_input should match rule pattern",
                        exercise.id, path
                    );
                    assert!(
                        !diagnostic.user_snippet.is_empty(),
                        "Exercise '{}' ({:?}): diagnostic user_snippet should not be empty",
                        exercise.id,
                        path
                    );
                }
                ValidationResult::Passed { .. } => {
                    panic!(
                        "Exercise '{}' ({:?}): submitting diagnostic pattern '{}' unexpectedly passed!",
                        exercise.id, path, rule.pattern
                    );
                }
            }
        }
    }
}
