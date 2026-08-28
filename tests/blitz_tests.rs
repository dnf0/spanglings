use spanglings::cli::commands::blitz::{evaluate_blitz_answer, get_blitz_items, BlitzItem};

#[test]
fn test_get_all_blitz_items() {
    let items = get_blitz_items(None);
    assert!(
        items.len() >= 20,
        "Expected at least 20 blitz items, got {}",
        items.len()
    );
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
fn test_blitz_items_rich_fields() {
    let items = get_blitz_items(None);
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
fn test_blitz_prompt_formatting() {
    let item = BlitzItem {
        topic: "subjunctive".to_string(),
        formula_cue: "drop -o -> opposite vowel -a".to_string(),
        trigger_sentence: "Dudo que yo ____ los libros en la mesa.".to_string(),
        target_verb: "poner".to_string(),
        target_subject: "yo".to_string(),
        target: "ponga".to_string(),
        explanation: "yo pongo -> drop -o -> add -a -> ponga".to_string(),
    };
    let formatted = item.format_prompt(45, 3);
    assert!(formatted
        .contains("[45s remaining | Streak: 3] [Subjunctive (wishes, hypotheses, doubt, demands) | drop -o -> opposite vowel -a]"));
    assert!(formatted.contains("Subjunctive (wishes, hypotheses, doubt, demands)"));
    assert!(formatted.contains("Sentence: \"Dudo que yo ____ los libros en la mesa.\""));
    assert!(formatted.contains("(verb: poner | subject: yo) > "));

    // Fallback when no formula cue
    let item_no_cue = BlitzItem {
        topic: "subjunctive".to_string(),
        formula_cue: "".to_string(),
        trigger_sentence: "Dudo que yo ____.".to_string(),
        target_verb: "poner".to_string(),
        target_subject: "yo".to_string(),
        target: "ponga".to_string(),
        explanation: "yo pongo -> ponga".to_string(),
    };
    let formatted_no_cue = item_no_cue.format_prompt(30, 0);
    assert!(formatted_no_cue.contains(
        "[30s remaining | Streak: 0] [Subjunctive (wishes, hypotheses, doubt, demands)]"
    ));

    // Fallback for unknown topic
    let item_unknown = BlitzItem {
        topic: "custom_topic".to_string(),
        formula_cue: "rule 1".to_string(),
        trigger_sentence: "Sentence ____".to_string(),
        target_verb: "v".to_string(),
        target_subject: "s".to_string(),
        target: "t".to_string(),
        explanation: "e".to_string(),
    };
    let formatted_unknown = item_unknown.format_prompt(15, 1);
    assert!(formatted_unknown.contains("[15s remaining | Streak: 1] [Custom Topic | rule 1]"));
}

#[test]
fn test_evaluate_blitz_answer_case_and_whitespace() {
    let item = BlitzItem {
        topic: "preterite".to_string(),
        formula_cue: "stem tuv- + unaccented endings".to_string(),
        trigger_sentence: "Anoche yo ____ una reunión urgente.".to_string(),
        target_verb: "tener".to_string(),
        target_subject: "yo".to_string(),
        target: "tuv".to_string(),
        explanation: "tener -> tuv".to_string(),
    };

    assert!(evaluate_blitz_answer(&item, "tuv"));
    assert!(evaluate_blitz_answer(&item, "TUV"));
    assert!(evaluate_blitz_answer(&item, "  tuv  \n"));
    assert!(!evaluate_blitz_answer(&item, "ten"));
}

#[test]
fn test_cli_blitz_flag_parsing() {
    use clap::Parser;
    use spanglings::cli::{Cli, Commands};

    let cli = Cli::parse_from([
        "spanglings",
        "blitz",
        "-t",
        "1",
        "-w",
        "-l",
        "b1",
        "-s",
        "45",
    ]);
    assert_eq!(
        cli.command,
        Some(Commands::Blitz {
            seconds: Some(45),
            topic: None,
            weak: true,
            level: Some("b1".to_string()),
            track: Some(1),
        })
    );
}
