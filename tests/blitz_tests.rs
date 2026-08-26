use spanglings::cli::commands::blitz::{evaluate_blitz_answer, get_blitz_items, BlitzItem};

#[test]
fn test_get_all_blitz_items() {
    let items = get_blitz_items(None);
    assert!(items.len() >= 20, "Expected at least 20 blitz items, got {}", items.len());
}

#[test]
fn test_get_blitz_items_topic_filter() {
    let pret_items = get_blitz_items(Some("preterite"));
    assert!(!pret_items.is_empty());
    for item in pret_items {
        assert_eq!(item.topic, "preterite");
    }

    let subj_items = get_blitz_items(Some("subjunctive"));
    assert!(!subj_items.is_empty());
    for item in subj_items {
        assert_eq!(item.topic, "subjunctive");
    }
}

#[test]
fn test_evaluate_blitz_answer_case_and_whitespace() {
    let item = BlitzItem {
        prompt: "Preterite stem for tener",
        target: "tuv",
        topic: "preterite",
        explanation: "tener -> tuv",
    };

    assert!(evaluate_blitz_answer(&item, "tuv"));
    assert!(evaluate_blitz_answer(&item, "TUV"));
    assert!(evaluate_blitz_answer(&item, "  tuv  \n"));
    assert!(!evaluate_blitz_answer(&item, "ten"));
}
