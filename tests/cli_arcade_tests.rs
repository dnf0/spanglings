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
    use spanglings::cli::commands::arcade::{play_arcade_sound, print_arcade_summary};
    use spanglings::core::state::AppState;
    use std::collections::HashMap;

    // Disabled sound should return without spawning
    play_arcade_sound(true, false);
    play_arcade_sound(false, false);

    let stats = ArcadeSessionStats {
        total_answered: 5,
        correct_count: 4,
        incorrect_count: 1,
        current_streak: 3,
        best_streak: 4,
        score: 1250,
        total_time_ms: 2500,
    };

    let mut initial = HashMap::new();
    initial.insert("por-para".to_string(), 0.2f32);
    let mut state = AppState::default();
    state.update_concept_mastery("por-para", 5, chrono::Utc::now());

    // Verify summary formatter runs cleanly
    print_arcade_summary(&stats, &initial, &state);
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
}
