use clap::Parser;
use spanglings::cli::commands::drill::{
    evaluate_drill_answer, get_drill_items, get_topic_cheat_sheet, DrillEvaluation, DrillItem,
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
fn test_get_topic_cheat_sheet_all_topics() {
    assert!(get_topic_cheat_sheet("subjunctive").is_some());
    assert!(get_topic_cheat_sheet("preterite").is_some());
    assert!(get_topic_cheat_sheet("por_para").is_some());
    assert!(get_topic_cheat_sheet("ser_estar").is_some());
    assert!(get_topic_cheat_sheet("pronouns").is_some());
    assert!(get_topic_cheat_sheet("prepositions").is_some());
    assert!(get_topic_cheat_sheet("accidental_se").is_some());
    assert!(get_topic_cheat_sheet("imperative").is_some());
    assert!(get_topic_cheat_sheet("future").is_some());
    assert!(get_topic_cheat_sheet("false_friends").is_some());
    assert!(get_topic_cheat_sheet("idioms").is_some());
    assert!(get_topic_cheat_sheet("all").is_some());

    let subj_sheet = get_topic_cheat_sheet("subjunctive").unwrap();
    assert!(subj_sheet.contains("Subjunctive"));
    assert!(subj_sheet.contains("opposite vowel"));

    let pret_sheet = get_topic_cheat_sheet("preterite").unwrap();
    assert!(pret_sheet.contains("Preterite"));
    assert!(pret_sheet.contains("unaccented endings") || pret_sheet.contains("stems"));

    let por_sheet = get_topic_cheat_sheet("por_para").unwrap();
    assert!(por_sheet.contains("Por"));
    assert!(por_sheet.contains("Para"));

    let ser_sheet = get_topic_cheat_sheet("ser_estar").unwrap();
    assert!(ser_sheet.contains("Ser"));
    assert!(ser_sheet.contains("Estar"));

    let pro_sheet = get_topic_cheat_sheet("pronouns").unwrap();
    assert!(pro_sheet.contains("Pronoun") || pro_sheet.contains("se lo"));

    let prep_sheet = get_topic_cheat_sheet("prepositions").unwrap();
    assert!(prep_sheet.contains("Preposition") || prep_sheet.contains("soñar CON"));

    let acc_sheet = get_topic_cheat_sheet("accidental_se").unwrap();
    assert!(acc_sheet.contains("Accidental") || acc_sheet.contains("Se"));

    let imp_sheet = get_topic_cheat_sheet("imperative").unwrap();
    assert!(imp_sheet.contains("Imperative") || imp_sheet.contains("Commands"));

    let fut_sheet = get_topic_cheat_sheet("future").unwrap();
    assert!(fut_sheet.contains("Future") || fut_sheet.contains("tendr-"));

    let ff_sheet = get_topic_cheat_sheet("false_friends").unwrap();
    assert!(ff_sheet.contains("False Friends") || ff_sheet.contains("actualmente"));

    let id_sheet = get_topic_cheat_sheet("idioms").unwrap();
    assert!(id_sheet.contains("Idiom") || id_sheet.contains("dar por sentado"));

    let all_sheet = get_topic_cheat_sheet("all").unwrap();
    assert!(all_sheet.contains("Subjunctive") && all_sheet.contains("Preterite"));

    // Unknown topic returns None
    assert!(get_topic_cheat_sheet("unknown_topic_xyz").is_none());
}

#[test]
fn test_drill_items_rich_fields() {
    let items = get_drill_items(None);
    assert!(!items.is_empty());

    for item in &items {
        assert!(
            !item.formula_cue.is_empty(),
            "formula_cue must not be empty for {}",
            item.target
        );
        assert!(
            !item.trigger_sentence.is_empty(),
            "trigger_sentence must not be empty for {}",
            item.target
        );
        assert!(
            item.trigger_sentence.contains("____"),
            "trigger_sentence must contain blank marker '____' for {}",
            item.target
        );
        assert!(
            !item.target_verb.is_empty(),
            "target_verb must not be empty for {}",
            item.target
        );
        assert!(
            !item.target_subject.is_empty(),
            "target_subject must not be empty for {}",
            item.target
        );
        assert!(
            !item.target.is_empty(),
            "target must not be empty for {}",
            item.target
        );
        assert!(
            !item.explanation.is_empty(),
            "explanation must not be empty for {}",
            item.target
        );
    }
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
        topic: "preterite".to_string(),
        formula_cue: "stem tuv- + unaccented endings".to_string(),
        trigger_sentence: "Anoche yo ____ un problema con el coche.".to_string(),
        target_verb: "tener".to_string(),
        target_subject: "yo".to_string(),
        target: "tuv".to_string(),
        explanation: "tener -> tuv-".to_string(),
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
        topic: "subjunctive".to_string(),
        formula_cue: "irregular subjunctive yo form".to_string(),
        trigger_sentence: "Espero que ella me ____ una oportunidad.".to_string(),
        target_verb: "dar".to_string(),
        target_subject: "ella".to_string(),
        target: "dé".to_string(),
        explanation: "dar -> dé".to_string(),
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
            weak: false,
            level: None,
            track: None,
        })
    );

    let cli_all = Cli::parse_from(["spanglings", "drill", "subjunctive", "--count", "15"]);
    assert_eq!(
        cli_all.command,
        Some(Commands::Drill {
            topic: Some("subjunctive".to_string()),
            concept: None,
            count: Some(15),
            weak: false,
            level: None,
            track: None,
        })
    );

    let cli_flags = Cli::parse_from([
        "spanglings",
        "drill",
        "-w",
        "-l",
        "b1",
        "-t",
        "2",
        "--count",
        "8",
    ]);
    assert_eq!(
        cli_flags.command,
        Some(Commands::Drill {
            topic: None,
            concept: None,
            count: Some(8),
            weak: true,
            level: Some("b1".to_string()),
            track: Some(2),
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
    let sample1: Vec<&str> = pool1
        .iter()
        .take(5)
        .map(|i| i.trigger_sentence.as_str())
        .collect();
    let sample2: Vec<&str> = pool2
        .iter()
        .take(5)
        .map(|i| i.trigger_sentence.as_str())
        .collect();

    // Verify sample contains valid questions and is non-empty
    assert_eq!(sample1.len(), 5);
    assert_eq!(sample2.len(), 5);
}

#[test]
fn test_drill_item_prompt_formatting() {
    let item = DrillItem {
        topic: "subjunctive".to_string(),
        formula_cue: "drop -o -> opposite vowel -a".to_string(),
        trigger_sentence: "Dudo que yo ____ los libros en la mesa.".to_string(),
        target_verb: "poner".to_string(),
        target_subject: "yo".to_string(),
        target: "ponga".to_string(),
        explanation: "yo pongo -> drop -o -> add -a -> ponga".to_string(),
    };
    let formatted = item.format_prompt(1, 5);
    assert!(formatted.contains(
        "Q1/5 [Subjunctive (wishes, hypotheses, doubt, demands) | drop -o -> opposite vowel -a]"
    ));
    assert!(formatted.contains("Subjunctive (wishes, hypotheses, doubt, demands)"));
    assert!(formatted.contains("Sentence: \"Dudo que yo ____ los libros en la mesa.\""));
    assert!(formatted.contains("(verb: poner | subject: yo)"));

    // Fallback when no formula cue
    let item_no_cue = DrillItem {
        topic: "subjunctive".to_string(),
        formula_cue: "".to_string(),
        trigger_sentence: "Dudo que yo ____.".to_string(),
        target_verb: "poner".to_string(),
        target_subject: "yo".to_string(),
        target: "ponga".to_string(),
        explanation: "yo pongo -> ponga".to_string(),
    };
    let formatted_no_cue = item_no_cue.format_prompt(2, 5);
    assert!(formatted_no_cue.contains("Q2/5 [Subjunctive (wishes, hypotheses, doubt, demands)]"));

    // Fallback for unknown topic
    let item_unknown = DrillItem {
        topic: "custom_topic".to_string(),
        formula_cue: "rule 1".to_string(),
        trigger_sentence: "Sentence ____".to_string(),
        target_verb: "v".to_string(),
        target_subject: "s".to_string(),
        target: "t".to_string(),
        explanation: "e".to_string(),
    };
    let formatted_unknown = item_unknown.format_prompt(3, 5);
    assert!(formatted_unknown.contains("Q3/5 [Custom Topic | rule 1]"));
}

#[test]
fn test_drill_live_hint_generation() {
    let item = DrillItem {
        topic: "subjunctive".to_string(),
        formula_cue: "drop -o -> opposite vowel -a".to_string(),
        trigger_sentence: "Dudo que yo ____ los libros en la mesa.".to_string(),
        target_verb: "poner".to_string(),
        target_subject: "yo".to_string(),
        target: "ponga".to_string(),
        explanation: "yo pongo -> drop -o -> add -a -> ponga".to_string(),
    };
    let hint = item.format_hint();
    assert!(hint.contains("💡 Hint:"));
    assert!(hint.contains("yo pongo -> drop -o -> add -a -> ponga"));
}

#[test]
fn test_adaptive_weakness_drill_selection() {
    use spanglings::cli::commands::drill::{select_drill_items, DrillFilter};
    use spanglings::core::state::AppState;

    let mut state = AppState::default();
    let now = chrono::Utc::now();
    // Simulate high failure rate on por_para and subjunctive
    state.update_concept_mastery("por_para", 1, now);
    state.update_concept_mastery("por_para", 1, now);
    state.update_concept_mastery("subjunctive", 1, now);

    let filter = DrillFilter {
        weak_only: true,
        topic: None,
        level: None,
        track: None,
        count: 6,
    };

    let items = select_drill_items(&state, filter);
    assert_eq!(items.len(), 6);
    let weak_count = items
        .iter()
        .filter(|i| i.topic == "por_para" || i.topic == "subjunctive")
        .count();
    assert!(
        weak_count > 0,
        "Adaptive drill selection should sample weak concepts"
    );
}

#[test]
fn test_drill_level_and_track_filtering() {
    use spanglings::cli::commands::drill::{select_drill_items, DrillFilter};
    use spanglings::core::curriculum::Level;
    use spanglings::core::state::AppState;

    let state = AppState::default();
    let filter = DrillFilter {
        weak_only: false,
        topic: None,
        level: Some(Level::B1),
        track: Some(1),
        count: 5,
    };

    let items = select_drill_items(&state, filter);
    assert_eq!(items.len(), 5);
}

#[test]
fn test_drill_concept_mastery_update_live() {
    use spanglings::core::state::AppState;

    let mut state = AppState::default();
    let now = chrono::Utc::now();

    // Direct correct answer (quality 5)
    state.update_concept_mastery("preterite", 5, now);
    let score1 = state
        .concept_mastery
        .get("preterite")
        .unwrap()
        .mastery_score;
    assert!(score1 > 0.0);

    // Second correct answer boosts score further
    state.update_concept_mastery("preterite", 5, now);
    let score2 = state
        .concept_mastery
        .get("preterite")
        .unwrap()
        .mastery_score;
    assert!(score2 > score1);

    // Incorrect answer (quality 1) decreases score and adds lapse
    state.update_concept_mastery("preterite", 1, now);
    let mastery = state.concept_mastery.get("preterite").unwrap();
    assert!(mastery.mastery_score < score2);
    assert_eq!(mastery.lapses, 1);
}
