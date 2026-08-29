use crossterm::event::KeyCode;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use spanglings::core::state::AppState;
use spanglings::tui::app::App;

#[test]
fn test_tui_arcade_modal_lifecycle_and_single_key_navigation() {
    let mut app = App::new_with_state(vec![], false, AppState::default());

    // Enter arcade mode
    app.enter_arcade_mode(None);
    assert!(app.show_arcade_modal);
    assert!(!app.arcade_items.is_empty());
    assert_eq!(app.arcade_item_idx, 0);

    // Answer with key
    let target_idx = app.arcade_items[0].correct_index;
    let key_char = match target_idx {
        0 => '1',
        1 => '2',
        2 => '3',
        _ => '4',
    };
    let initial_score = app.arcade_stats.score;
    app.handle_arcade_key_char(key_char);

    assert!(app.arcade_stats.score > initial_score);
    assert_eq!(app.arcade_stats.current_streak, 1);

    // Exit arcade mode with 'q'
    app.handle_arcade_key_code(KeyCode::Char('q'));
    assert!(!app.show_arcade_modal);
}

#[test]
fn test_tui_arcade_showdown_mode_navigation() {
    use spanglings::core::arcade::ShowdownPair;
    let mut app = App::new_with_state(vec![], false, AppState::default());

    app.enter_arcade_mode(Some(ShowdownPair::PorPara));
    assert!(app.show_arcade_modal);
    assert_eq!(app.arcade_items[0].options.len(), 2);

    // Answer with 'j' (index 0) or 'k' (index 1)
    app.handle_arcade_key_char('j');
    assert_eq!(app.arcade_stats.total_answered, 1);
}

#[test]
fn test_tui_all_16_showdown_pairs_lifecycle_and_rendering() {
    use spanglings::core::arcade::list_showdown_pairs;

    let pairs = list_showdown_pairs();
    assert_eq!(pairs.len(), 16);

    let mut app = App::new_with_state(vec![], false, AppState::default());
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    for pair in pairs {
        // 1. Enter showdown
        app.enter_arcade_mode(Some(pair));
        assert!(app.show_arcade_modal);
        assert_eq!(app.arcade_selected_showdown, Some(pair));
        assert_eq!(app.arcade_items.len(), 10);
        assert_eq!(app.arcade_item_idx, 0);

        for item in &app.arcade_items {
            assert_eq!(item.options.len(), 2);
            assert_eq!(item.topic, pair.slug());
            assert!(!item.trigger_sentence.is_empty());
            assert!(!item.explanation.is_empty());
        }

        // 2. Render active question screen
        terminal
            .draw(|f| spanglings::tui::ui::draw_ui(f, &app))
            .unwrap();

        // 3. Answer first question with 'j' (index 0) or 'k' (index 1)
        let correct_idx = app.arcade_items[0].correct_index;
        let answer_key = if correct_idx == 0 { 'j' } else { 'k' };
        app.handle_arcade_key_char(answer_key);

        assert_eq!(app.arcade_item_idx, 1);
        assert_eq!(app.arcade_stats.total_answered, 1);
        assert_eq!(app.arcade_stats.correct_count, 1);

        // 4. Answer remaining 9 questions to complete session
        while app.arcade_item_idx < app.arcade_items.len() {
            let idx = app.arcade_items[app.arcade_item_idx].correct_index;
            let key = if idx == 0 { 'j' } else { 'k' };
            app.handle_arcade_key_char(key);
        }

        // 5. Verify completion recap screen
        assert_eq!(app.arcade_item_idx, app.arcade_items.len());
        assert_eq!(app.arcade_stats.total_answered, 10);
        assert_eq!(app.arcade_stats.correct_count, 10);

        // Render recap screen
        terminal
            .draw(|f| spanglings::tui::ui::draw_ui(f, &app))
            .unwrap();

        // 6. Test restart with 'r'
        app.handle_arcade_key_char('r');
        assert_eq!(app.arcade_item_idx, 0);
        assert_eq!(app.arcade_selected_showdown, Some(pair));
        assert_eq!(app.arcade_stats.total_answered, 0);

        // Exit before next iteration
        app.exit_arcade_mode();
        assert!(!app.show_arcade_modal);
    }
}

#[test]
fn test_tui_arcade_showdown_cycling_navigation() {
    use spanglings::core::arcade::{list_showdown_pairs, ShowdownPair};

    let pairs = list_showdown_pairs();
    let mut app = App::new_with_state(vec![], false, AppState::default());

    // Start in mixed 4-choice mode
    app.enter_arcade_mode(None);
    assert_eq!(app.arcade_selected_showdown, None);

    // Tab cycles to first showdown pair (PorPara)
    app.handle_arcade_key_code(KeyCode::Tab);
    assert_eq!(app.arcade_selected_showdown, Some(ShowdownPair::PorPara));

    // 's' cycles forward through all 16 pairs
    for expected_pair in &pairs[1..] {
        app.handle_arcade_key_char('s');
        assert_eq!(app.arcade_selected_showdown, Some(*expected_pair));
    }

    // Cycling after the last pair (BienBueno) wraps around to the first (PorPara)
    app.handle_arcade_key_char('s');
    assert_eq!(app.arcade_selected_showdown, Some(ShowdownPair::PorPara));

    // BackTab cycles backward to BienBueno
    app.handle_arcade_key_code(KeyCode::BackTab);
    assert_eq!(app.arcade_selected_showdown, Some(ShowdownPair::BienBueno));

    // Complete session and verify cycling from recap screen
    while app.arcade_item_idx < app.arcade_items.len() {
        app.handle_arcade_key_char('j');
    }
    assert!(app.arcade_item_idx >= app.arcade_items.len());

    // Cycle from recap screen with Tab
    app.handle_arcade_key_code(KeyCode::Tab);
    assert_eq!(app.arcade_selected_showdown, Some(ShowdownPair::PorPara));
    assert_eq!(app.arcade_item_idx, 0);
}

#[test]
fn test_tui_all_5_specialized_engines_lifecycle_and_rendering() {
    use spanglings::core::arcade::get_engine_title;

    let engines = [
        "regimen",
        "irregulars",
        "false-friends",
        "se-matrix",
        "connectors",
    ];

    let mut app = App::new_with_state(vec![], false, AppState::default());
    let backend = TestBackend::new(140, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    for engine in engines {
        // 1. Enter specialized drill engine
        app.enter_arcade_with_topic(engine);
        assert!(app.show_arcade_modal);
        assert_eq!(app.arcade_selected_topic.as_deref(), Some(engine));
        assert_eq!(app.arcade_selected_showdown, None);
        assert_eq!(app.arcade_items.len(), 15);
        assert_eq!(app.arcade_item_idx, 0);

        for item in &app.arcade_items {
            assert_eq!(item.options.len(), 4);
            assert_eq!(item.topic, engine);
            assert!(!item.trigger_sentence.is_empty());
            assert!(!item.explanation.is_empty());
            assert!(item.correct_index < 4);
        }

        // 2. Render active question screen and verify header title
        terminal
            .draw(|f| spanglings::tui::ui::draw_ui(f, &app))
            .unwrap();

        let buffer = terminal.backend().buffer();
        let rendered_text: String = buffer
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<Vec<_>>()
            .join("");

        let expected_title = get_engine_title(engine).expect("Engine title must exist");
        assert!(
            rendered_text.contains(expected_title),
            "Modal title must contain specialized engine title '{}', found buffer: {}",
            expected_title,
            rendered_text
        );

        // 3. Answer first question with single key ('1', '2', '3', '4')
        let correct_idx = app.arcade_items[0].correct_index;
        let answer_key = match correct_idx {
            0 => '1',
            1 => '2',
            2 => '3',
            _ => '4',
        };
        app.handle_arcade_key_char(answer_key);

        assert_eq!(app.arcade_item_idx, 1);
        assert_eq!(app.arcade_stats.total_answered, 1);
        assert_eq!(app.arcade_stats.correct_count, 1);

        // 4. Answer remaining 14 questions to complete session
        while app.arcade_item_idx < app.arcade_items.len() {
            let idx = app.arcade_items[app.arcade_item_idx].correct_index;
            let key = match idx {
                0 => '1',
                1 => '2',
                2 => '3',
                _ => '4',
            };
            app.handle_arcade_key_char(key);
        }

        // 5. Verify completion recap screen
        assert_eq!(app.arcade_item_idx, 15);
        assert_eq!(app.arcade_stats.total_answered, 15);
        assert_eq!(app.arcade_stats.correct_count, 15);

        terminal
            .draw(|f| spanglings::tui::ui::draw_ui(f, &app))
            .unwrap();

        // 6. Test restart with 'r'
        app.handle_arcade_key_char('r');
        assert_eq!(app.arcade_item_idx, 0);
        assert_eq!(app.arcade_selected_topic.as_deref(), Some(engine));
        assert_eq!(app.arcade_stats.total_answered, 0);

        // Exit before next iteration
        app.exit_arcade_mode();
        assert!(!app.show_arcade_modal);
    }
}

#[test]
fn test_tui_enter_arcade_with_topic_aliases_and_showdowns() {
    use spanglings::core::arcade::ShowdownPair;

    let mut app = App::new_with_state(vec![], false, AppState::default());

    // Matching a showdown pair
    app.enter_arcade_with_topic("por-para");
    assert!(app.show_arcade_modal);
    assert_eq!(app.arcade_selected_showdown, Some(ShowdownPair::PorPara));
    assert_eq!(app.arcade_selected_topic, None);
    assert_eq!(app.arcade_items.len(), 10);
    app.exit_arcade_mode();

    // Matching aliases of specialized engines
    let aliases = [
        ("prepositions", "regimen"),
        ("irregular-verbs", "irregulars"),
        ("cognates", "false-friends"),
        ("valores-de-se", "se-matrix"),
        ("transitions", "connectors"),
    ];

    for (alias, canonical) in aliases {
        app.enter_arcade_with_topic(alias);
        assert!(app.show_arcade_modal);
        assert_eq!(app.arcade_selected_showdown, None);
        assert_eq!(app.arcade_selected_topic.as_deref(), Some(canonical));
        assert_eq!(app.arcade_items.len(), 15);
        app.exit_arcade_mode();
    }
}
