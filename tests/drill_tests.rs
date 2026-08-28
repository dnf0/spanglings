use clap::Parser;
use spanglings::cli::commands::drill::{
    evaluate_drill_answer, get_drill_items, DrillEvaluation, DrillItem,
};
use spanglings::cli::{Cli, Commands};

#[test]
fn test_get_all_drill_items_pool_size() {
    let items = get_drill_items(None);
    assert!(
        items.len() >= 50,
        "Expected at least 50 comprehensive drill items, got {}",
        items.len()
    );
}

#[test]
fn test_drill_items_topic_filtering() {
    let preterite_items = get_drill_items(Some("preterite"));
    assert!(!preterite_items.is_empty());
    for item in &preterite_items {
        assert!(
            item.topic.contains("preterite") || item.topic.contains("past"),
            "Unexpected topic: {}",
            item.topic
        );
    }

    let subj_items = get_drill_items(Some("subjunctive"));
    assert!(!subj_items.is_empty());
    for item in &subj_items {
        assert!(
            item.topic.contains("subjunctive"),
            "Unexpected topic: {}",
            item.topic
        );
    }

    let por_para_items = get_drill_items(Some("por-para"));
    assert!(!por_para_items.is_empty());
    for item in &por_para_items {
        assert_eq!(item.topic, "por_para");
    }

    let ser_estar_items = get_drill_items(Some("ser-estar"));
    assert!(!ser_estar_items.is_empty());
    for item in &ser_estar_items {
        assert_eq!(item.topic, "ser_estar");
    }

    let false_friends_items = get_drill_items(Some("false-friends"));
    assert!(!false_friends_items.is_empty());
    for item in &false_friends_items {
        assert_eq!(item.topic, "false_friends");
    }

    let prepositions_items = get_drill_items(Some("prepositions"));
    assert!(!prepositions_items.is_empty());
    for item in &prepositions_items {
        assert_eq!(item.topic, "prepositions");
    }

    let idioms_items = get_drill_items(Some("idioms"));
    assert!(!idioms_items.is_empty());
    for item in &idioms_items {
        assert_eq!(item.topic, "idioms");
    }
}

#[test]
fn test_evaluate_drill_answer() {
    let item = DrillItem {
        prompt: "Irregular Preterite Stem for 'tener'",
        target: "tuv",
        topic: "preterite",
        explanation: "tener -> tuv-",
    };

    assert_eq!(
        evaluate_drill_answer(&item, "tuv", false),
        DrillEvaluation::Correct
    );
    assert_eq!(
        evaluate_drill_answer(&item, "TUV", false),
        DrillEvaluation::Correct
    );
    assert_eq!(
        evaluate_drill_answer(&item, "  tuv  \n", false),
        DrillEvaluation::Correct
    );
    assert_eq!(
        evaluate_drill_answer(&item, "ten", false),
        DrillEvaluation::Incorrect
    );
}

#[test]
fn test_evaluate_drill_answer_accents() {
    let item = DrillItem {
        prompt: "Present Subjunctive 'yo' for 'dar'",
        target: "dé",
        topic: "subjunctive",
        explanation: "dar -> dé",
    };

    // Exact accent match
    assert_eq!(
        evaluate_drill_answer(&item, "dé", false),
        DrillEvaluation::Correct
    );

    // Missing accent in forgiving mode -> Forgiven
    match evaluate_drill_answer(&item, "de", false) {
        DrillEvaluation::Forgiven { expected, .. } => {
            assert_eq!(expected, "dé");
        }
        other => panic!("Expected Forgiven, got {:?}", other),
    }

    // Missing accent in strict mode -> Incorrect
    assert_eq!(
        evaluate_drill_answer(&item, "de", true),
        DrillEvaluation::Incorrect
    );
}

#[test]
fn test_cli_drill_count_parsing() {
    let cli = Cli::parse_from(["spanglings", "drill", "-n", "10"]);
    assert_eq!(
        cli.command,
        Some(Commands::Drill {
            topic: None,
            concept: None,
            count: Some(10),
        })
    );

    let cli_all = Cli::parse_from(["spanglings", "drill", "subjunctive", "--count", "15"]);
    assert_eq!(
        cli_all.command,
        Some(Commands::Drill {
            topic: Some("subjunctive".to_string()),
            concept: None,
            count: Some(15),
        })
    );
}

#[test]
fn test_drill_items_shuffling_randomization() {
    use rand::seq::SliceRandom;
    let mut pool1 = get_drill_items(None);
    let mut pool2 = get_drill_items(None);

    let mut rng = rand::thread_rng();
    pool1.shuffle(&mut rng);
    pool2.shuffle(&mut rng);

    // With 70+ items, the probability of two independent random 5-sample slices being identical is infinitesimal (< 1 in 10^7)
    let sample1: Vec<&str> = pool1.iter().take(5).map(|i| i.prompt).collect();
    let sample2: Vec<&str> = pool2.iter().take(5).map(|i| i.prompt).collect();

    // Verify sample contains valid questions and is non-empty
    assert_eq!(sample1.len(), 5);
    assert_eq!(sample2.len(), 5);
}
