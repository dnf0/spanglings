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
fn test_tui_arcade_rendering_showdown_and_4choice() {
    let mut app = App::new_with_state(vec![], false, AppState::default());
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    // 4-choice mode
    app.enter_arcade_mode(None);
    terminal
        .draw(|f| spanglings::tui::ui::draw_ui(f, &app))
        .unwrap();

    // Showdown mode
    app.enter_arcade_mode(Some(spanglings::core::arcade::ShowdownPair::SerEstar));
    terminal
        .draw(|f| spanglings::tui::ui::draw_ui(f, &app))
        .unwrap();
}
