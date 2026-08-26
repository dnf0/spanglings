use spanglings::cli::commands::search::search_exercises;

#[test]
fn test_search_by_topic_and_keyword() {
    let results = search_exercises("subjunctive").expect("Search failed");
    assert!(!results.is_empty());
    assert!(results.iter().all(|e| {
        e.topic.to_lowercase().contains("subjunctive")
            || e.title.to_lowercase().contains("subjunctive")
            || e.raw_content.to_lowercase().contains("subjunctive")
    }));
}

#[test]
fn test_search_by_level() {
    let results = search_exercises("C1").expect("Search failed");
    assert!(!results.is_empty());
    assert!(results.iter().any(|e| e.level.to_string() == "C1"));
}

#[test]
fn test_search_by_id() {
    let results = search_exercises("b1_subj_weirdo_wishes").expect("Search failed");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "b1_subj_weirdo_wishes");
}
