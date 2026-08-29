use spanglings::cli::commands::arcade::{
    evaluate_arcade_choice, get_combo_rank, ArcadeSessionStats,
};
use spanglings::core::arcade::ArcadeItem;

#[test]
fn test_arcade_choice_evaluation_and_scoring() {
    let item = ArcadeItem {
        topic: "por-para".to_string(),
        trigger_sentence: "Estudio ____ aprender español.".to_string(),
        prompt_cue: "purpose/goal -> para".to_string(),
        options: vec!["por".to_string(), "para".to_string()],
        correct_index: 1,
        explanation: "Para indicates purpose or goal.".to_string(),
    };

    let mut stats = ArcadeSessionStats::default();

    // Correct selection with <800ms speed
    let result = evaluate_arcade_choice(&item, 1, 500, &mut stats);
    assert!(result.is_correct);
    assert_eq!(stats.current_streak, 1);
    assert_eq!(stats.best_streak, 1);
    assert!(stats.score >= 200);

    // Another correct selection to test combo multiplier
    let result2 = evaluate_arcade_choice(&item, 1, 600, &mut stats);
    assert!(result2.is_correct);
    assert_eq!(stats.current_streak, 2);

    // Incorrect selection resets streak
    let result3 = evaluate_arcade_choice(&item, 0, 1200, &mut stats);
    assert!(!result3.is_correct);
    assert_eq!(stats.current_streak, 0);
    assert_eq!(stats.best_streak, 2);
}

#[test]
fn test_arcade_choice_records_mistakes() {
    let item = ArcadeItem {
        topic: "por-para".to_string(),
        trigger_sentence: "Estudio ____ aprender español.".to_string(),
        prompt_cue: "purpose/goal -> para".to_string(),
        options: vec!["por".to_string(), "para".to_string()],
        correct_index: 1,
        explanation: "Para indicates purpose or goal.".to_string(),
    };

    let mut stats = ArcadeSessionStats::default();

    // Correct choice: no mistake recorded
    let result = evaluate_arcade_choice(&item, 1, 400, &mut stats);
    assert!(result.is_correct);
    assert!(stats.mistakes.is_empty());

    // Incorrect choice: mistake recorded with all details
    let result_fail = evaluate_arcade_choice(&item, 0, 700, &mut stats);
    assert!(!result_fail.is_correct);
    assert_eq!(stats.mistakes.len(), 1);
    let mistake = &stats.mistakes[0];
    assert_eq!(mistake.topic, "por-para");
    assert_eq!(mistake.trigger_sentence, "Estudio ____ aprender español.");
    assert_eq!(mistake.user_answer, "por");
    assert_eq!(mistake.correct_answer, "para");
    assert_eq!(mistake.prompt_cue, "purpose/goal -> para");
    assert_eq!(mistake.explanation, "Para indicates purpose or goal.");
}

#[test]
fn test_arcade_session_stats_json_serialization_and_deserialization_compatibility() {
    let stats = ArcadeSessionStats {
        total_answered: 2,
        correct_count: 1,
        incorrect_count: 1,
        current_streak: 0,
        best_streak: 1,
        score: 150,
        total_time_ms: 1200,
        mistakes: vec![spanglings::cli::commands::arcade::ArcadeMistake {
            topic: "se-matrix".to_string(),
            trigger_sentence: "Se ____ olvidaron las llaves.".to_string(),
            user_answer: "me".to_string(),
            correct_answer: "le".to_string(),
            prompt_cue: "involuntary dative -> le".to_string(),
            explanation: "Involuntary dative requires 'le' for 3rd person.".to_string(),
        }],
    };

    // Serialize to JSON and verify contents
    let json_str = serde_json::to_string(&stats).unwrap();
    assert!(json_str.contains("\"mistakes\""));
    assert!(json_str.contains("se-matrix"));
    assert!(json_str.contains("Involuntary dative requires 'le' for 3rd person."));

    // Deserialize back from JSON
    let deserialized: ArcadeSessionStats = serde_json::from_str(&json_str).unwrap();
    assert_eq!(deserialized, stats);
    assert_eq!(deserialized.mistakes.len(), 1);
    assert_eq!(deserialized.mistakes[0].correct_answer, "le");

    // Test backward compatibility: JSON without "mistakes" field deserializes with empty mistakes vec
    let old_json = r#"{
        "total_answered": 10,
        "correct_count": 8,
        "incorrect_count": 2,
        "current_streak": 3,
        "best_streak": 5,
        "score": 1200,
        "total_time_ms": 5000
    }"#;
    let old_stats: ArcadeSessionStats = serde_json::from_str(old_json).unwrap();
    assert_eq!(old_stats.total_answered, 10);
    assert_eq!(old_stats.correct_count, 8);
    assert_eq!(old_stats.incorrect_count, 2);
    assert!(old_stats.mistakes.is_empty());
}

#[test]
fn test_arcade_combo_rank_titles() {
    assert_eq!(get_combo_rank(1), "✨ Good");
    assert!(get_combo_rank(3).contains("Quick"));
    assert!(get_combo_rank(5).contains("ON FIRE"));
    assert!(get_combo_rank(10).contains("UNSTOPPABLE"));
    assert!(get_combo_rank(20).contains("ULTRA INSTINCT"));
}

#[test]
fn test_select_arcade_items_modes() {
    use spanglings::cli::commands::arcade::select_arcade_items;
    use spanglings::core::state::AppState;

    let state = AppState::default();

    // Specific Showdown pair
    let showdown_items = select_arcade_items(Some("por-para"), None, false, 8, &state);
    assert_eq!(showdown_items.len(), 8);
    for it in showdown_items {
        assert_eq!(it.options.len(), 2);
        assert_eq!(it.topic, "por-para");
    }

    // Specific Concept
    let concept_items = select_arcade_items(None, Some("subjunctive"), false, 6, &state);
    assert_eq!(concept_items.len(), 6);
    for it in concept_items {
        assert_eq!(it.options.len(), 4);
    }

    // Weakness mode fallback (no state weaknesses recorded yet)
    let weak_items = select_arcade_items(None, None, true, 5, &state);
    assert_eq!(weak_items.len(), 5);

    // Mixed mode default
    let mixed_items = select_arcade_items(None, None, false, 10, &state);
    assert_eq!(mixed_items.len(), 10);
}

#[test]
fn test_arcade_summary_and_sound_helpers() {
    use spanglings::cli::commands::arcade::{
        play_arcade_sound, print_arcade_summary, ArcadeMistake,
    };
    use spanglings::core::state::AppState;
    use std::collections::HashMap;

    // Disabled sound should return without spawning
    play_arcade_sound(true, false);
    play_arcade_sound(false, false);

    let stats_with_mistakes = ArcadeSessionStats {
        total_answered: 5,
        correct_count: 4,
        incorrect_count: 1,
        current_streak: 3,
        best_streak: 4,
        score: 1250,
        total_time_ms: 2500,
        mistakes: vec![ArcadeMistake {
            topic: "por-para".to_string(),
            trigger_sentence: "Trabajo ____ ganar dinero.".to_string(),
            user_answer: "por".to_string(),
            correct_answer: "para".to_string(),
            prompt_cue: "purpose/goal -> para".to_string(),
            explanation: "Para indicates purpose or goal.".to_string(),
        }],
    };

    let mut initial = HashMap::new();
    initial.insert("por-para".to_string(), 0.2f32);
    let mut state = AppState::default();
    state.update_concept_mastery("por-para", 5, chrono::Utc::now());

    // Verify summary formatter runs cleanly with mistakes review
    print_arcade_summary(&stats_with_mistakes, &initial, &state);

    // Verify summary formatter runs cleanly on a perfect run
    let perfect_stats = ArcadeSessionStats {
        total_answered: 5,
        correct_count: 5,
        incorrect_count: 0,
        current_streak: 5,
        best_streak: 5,
        score: 2500,
        total_time_ms: 2000,
        mistakes: Vec::new(),
    };
    print_arcade_summary(&perfect_stats, &initial, &state);

    // Verify summary formatter runs cleanly on empty session
    let empty_stats = ArcadeSessionStats::default();
    print_arcade_summary(&empty_stats, &initial, &state);
}

#[test]
fn test_select_arcade_items_expanded_showdowns() {
    use spanglings::cli::commands::arcade::select_arcade_items;
    use spanglings::core::state::AppState;

    let state = AppState::default();

    let test_cases = [
        // (input_slug_or_alias, expected_topic)
        ("tener-haber", "tener-haber"),
        ("have", "tener-haber"),
        ("tener", "tener-haber"),
        ("haber", "tener-haber"),
        ("saber-conocer", "saber-conocer"),
        ("know", "saber-conocer"),
        ("saber", "saber-conocer"),
        ("conocer", "saber-conocer"),
        ("muy-mucho", "muy-mucho"),
        ("very", "muy-mucho"),
        ("much", "muy-mucho"),
        ("pedir-preguntar", "pedir-preguntar"),
        ("ask", "pedir-preguntar"),
        ("pedir", "pedir-preguntar"),
        ("llevar-traer", "llevar-traer"),
        ("take-bring", "llevar-traer"),
        ("llevar", "llevar-traer"),
        ("haber-estar", "haber-estar"),
        ("exist-locate", "haber-estar"),
        ("hay-esta", "haber-estar"),
        ("ir-irse", "ir-irse"),
        ("go-leave", "ir-irse"),
        ("ir", "ir-irse"),
        ("bien-bueno", "bien-bueno"),
        ("well-good", "bien-bueno"),
        ("buen", "bien-bueno"),
    ];

    for (input, expected_topic) in test_cases {
        let items = select_arcade_items(Some(input), None, false, 6, &state);
        assert_eq!(
            items.len(),
            6,
            "Expected 6 items for input '{}', got {}",
            input,
            items.len()
        );
        for item in &items {
            assert_eq!(
                item.options.len(),
                2,
                "Expected 2 options for binary showdown '{}', got {}",
                input,
                item.options.len()
            );
            assert_eq!(
                item.topic, expected_topic,
                "Expected topic '{}' for input '{}', got '{}'",
                expected_topic, input, item.topic
            );
            assert!(
                item.correct_index < item.options.len(),
                "Correct index {} out of bounds for input '{}'",
                item.correct_index,
                input
            );
            assert!(
                !item.trigger_sentence.is_empty(),
                "Empty sentence for input '{}'",
                input
            );
            assert!(
                !item.explanation.is_empty(),
                "Empty explanation for input '{}'",
                input
            );
        }
    }
}

#[test]
fn test_arcade_cli_argument_parsing() {
    use clap::Parser;
    use spanglings::cli::{Cli, Commands};

    // Default arcade command
    let cli = Cli::parse_from(["spanglings", "arcade"]);
    assert_eq!(
        cli.command,
        Some(Commands::Arcade {
            topic: None,
            showdown: None,
            concept: None,
            weak: false,
            count: None,
            sound: false,
        })
    );

    // Positional showdown
    let cli_pos = Cli::parse_from(["spanglings", "arcade", "ser-estar", "-n", "12", "-w", "-s"]);
    assert_eq!(
        cli_pos.command,
        Some(Commands::Arcade {
            topic: Some("ser-estar".to_string()),
            showdown: None,
            concept: None,
            weak: true,
            count: Some(12),
            sound: true,
        })
    );

    // Positional new showdown pairs & aliases
    let pairs_to_test = [
        ("tener-haber", "tener-haber"),
        ("have", "have"),
        ("saber-conocer", "saber-conocer"),
        ("know", "know"),
        ("muy-mucho", "muy-mucho"),
        ("very", "very"),
        ("pedir-preguntar", "pedir-preguntar"),
        ("ask", "ask"),
        ("llevar-traer", "llevar-traer"),
        ("take-bring", "take-bring"),
        ("haber-estar", "haber-estar"),
        ("ir-irse", "ir-irse"),
        ("bien-bueno", "bien-bueno"),
    ];

    for (arg, expected_str) in pairs_to_test {
        let parsed = Cli::parse_from(["spanglings", "arcade", arg]);
        assert_eq!(
            parsed.command,
            Some(Commands::Arcade {
                topic: Some(expected_str.to_string()),
                showdown: None,
                concept: None,
                weak: false,
                count: None,
                sound: false,
            })
        );
    }

    // Explicit showdown flag
    let cli_flag = Cli::parse_from([
        "spanglings",
        "arcade",
        "--showdown",
        "subj-ind",
        "--count",
        "20",
    ]);
    assert_eq!(
        cli_flag.command,
        Some(Commands::Arcade {
            topic: None,
            showdown: Some("subj-ind".to_string()),
            concept: None,
            weak: false,
            count: Some(20),
            sound: false,
        })
    );

    // Explicit showdown flag with new pairs & aliases
    let cli_flag_have = Cli::parse_from(["spanglings", "arcade", "--showdown", "have"]);
    assert_eq!(
        cli_flag_have.command,
        Some(Commands::Arcade {
            topic: None,
            showdown: Some("have".to_string()),
            concept: None,
            weak: false,
            count: None,
            sound: false,
        })
    );

    let cli_flag_tener_haber =
        Cli::parse_from(["spanglings", "arcade", "--showdown", "tener-haber"]);
    assert_eq!(
        cli_flag_tener_haber.command,
        Some(Commands::Arcade {
            topic: None,
            showdown: Some("tener-haber".to_string()),
            concept: None,
            weak: false,
            count: None,
            sound: false,
        })
    );

    // Positional specialized engine topics & aliases
    let engines_to_test = [
        ("regimen", "regimen"),
        ("prepositions", "prepositions"),
        ("irregulars", "irregulars"),
        ("verbs", "verbs"),
        ("false-friends", "false-friends"),
        ("cognates", "cognates"),
        ("se-matrix", "se-matrix"),
        ("se", "se"),
        ("connectors", "connectors"),
        ("discourse", "discourse"),
    ];

    for (arg, expected_str) in engines_to_test {
        let parsed = Cli::parse_from(["spanglings", "arcade", arg]);
        assert_eq!(
            parsed.command,
            Some(Commands::Arcade {
                topic: Some(expected_str.to_string()),
                showdown: None,
                concept: None,
                weak: false,
                count: None,
                sound: false,
            })
        );
    }

    // Explicit concept flag with specialized engines
    for (arg, expected_str) in engines_to_test {
        let parsed = Cli::parse_from(["spanglings", "arcade", "--concept", arg]);
        assert_eq!(
            parsed.command,
            Some(Commands::Arcade {
                topic: None,
                showdown: None,
                concept: Some(expected_str.to_string()),
                weak: false,
                count: None,
                sound: false,
            })
        );
    }
}

#[test]
fn test_select_arcade_items_specialized_engines() {
    use spanglings::cli::commands::arcade::select_arcade_items;
    use spanglings::core::arcade::{get_engine_title, list_specialized_engines};
    use spanglings::core::state::AppState;

    let state = AppState::default();

    // Verify list_specialized_engines
    let engines = list_specialized_engines();
    assert_eq!(
        engines,
        &[
            "regimen",
            "irregulars",
            "false-friends",
            "se-matrix",
            "connectors"
        ]
    );

    // Verify get_engine_title for canonical and aliases
    assert_eq!(
        get_engine_title("regimen"),
        Some("Prepositional Regimen Engine (Verbos con Régimen)")
    );
    assert_eq!(
        get_engine_title("prepositions"),
        Some("Prepositional Regimen Engine (Verbos con Régimen)")
    );
    assert_eq!(
        get_engine_title("irregulars"),
        Some("Irregular Verb Speed Gun (Conjugación Irregular)")
    );
    assert_eq!(
        get_engine_title("verbs"),
        Some("Irregular Verb Speed Gun (Conjugación Irregular)")
    );
    assert_eq!(
        get_engine_title("false-friends"),
        Some("False Friends Trap Detector (Falsos Amigos)")
    );
    assert_eq!(
        get_engine_title("cognates"),
        Some("False Friends Trap Detector (Falsos Amigos)")
    );
    assert_eq!(
        get_engine_title("se-matrix"),
        Some("The \"Se\" Matrix (Las 5 Caras del Se)")
    );
    assert_eq!(
        get_engine_title("se"),
        Some("The \"Se\" Matrix (Las 5 Caras del Se)")
    );
    assert_eq!(
        get_engine_title("connectors"),
        Some("Discourse Connectors & Flow (Conectores B2/C1)")
    );
    assert_eq!(
        get_engine_title("discourse"),
        Some("Discourse Connectors & Flow (Conectores B2/C1)")
    );
    assert_eq!(get_engine_title("unknown-engine"), None);

    let test_cases = [
        ("regimen", "regimen"),
        ("prepositions", "regimen"),
        ("irregulars", "irregulars"),
        ("verbs", "irregulars"),
        ("false-friends", "false-friends"),
        ("cognates", "false-friends"),
        ("se-matrix", "se-matrix"),
        ("se", "se-matrix"),
        ("connectors", "connectors"),
        ("discourse", "connectors"),
    ];

    for (input, expected_topic) in test_cases {
        // Test selection via concept argument
        let items_via_concept = select_arcade_items(None, Some(input), false, 8, &state);
        assert_eq!(
            items_via_concept.len(),
            8,
            "Expected 8 items for concept '{}', got {}",
            input,
            items_via_concept.len()
        );
        for item in &items_via_concept {
            assert_eq!(
                item.options.len(),
                4,
                "Expected 4 options for specialized engine '{}', got {}",
                input,
                item.options.len()
            );
            assert_eq!(
                item.topic, expected_topic,
                "Expected topic '{}' for input '{}', got '{}'",
                expected_topic, input, item.topic
            );
            assert!(
                item.correct_index < 4,
                "Correct index out of bounds for input '{}'",
                input
            );
            assert!(
                !item.trigger_sentence.is_empty(),
                "Empty sentence for input '{}'",
                input
            );
            assert!(
                !item.explanation.is_empty(),
                "Empty explanation for input '{}'",
                input
            );
            assert!(
                !item.prompt_cue.is_empty(),
                "Empty prompt cue for input '{}'",
                input
            );
        }

        // Test selection via showdown argument (fallback/alias support)
        let items_via_showdown = select_arcade_items(Some(input), None, false, 5, &state);
        assert_eq!(
            items_via_showdown.len(),
            5,
            "Expected 5 items for showdown '{}', got {}",
            input,
            items_via_showdown.len()
        );
        for item in &items_via_showdown {
            assert_eq!(item.options.len(), 4);
            assert_eq!(item.topic, expected_topic);
        }
    }
}
