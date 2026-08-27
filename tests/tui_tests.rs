use ratatui::backend::TestBackend;
use ratatui::Terminal;
use spanglings::core::curriculum::Level;
use spanglings::core::exercise::{DiagnosticRule, Exercise, ExerciseType};
use spanglings::tui::app::App;
use spanglings::tui::ui::draw_ui;
use std::path::PathBuf;

fn create_sample_exercises() -> Vec<Exercise> {
    vec![
        Exercise {
            path: PathBuf::from("exercises/01_subjunctive.md"),
            id: "b1_subj_01".to_string(),
            level: Level::B1,
            topic: "subjunctive_weirdo".to_string(),
            exercise_type: ExerciseType::Cloze,
            is_done: false,
            title: "Subjunctive Verbs of Influence".to_string(),
            solution: "vengas".to_string(),
            alternatives: vec!["vengas tú".to_string()],
            diagnostic_rules: vec![DiagnosticRule {
                pattern: "viene".to_string(),
                code: "E0301".to_string(),
                message: "Expected Subjunctive, found Indicative.".to_string(),
            }],
            hints: vec![
                "Tier 1: Look at the main verb.".to_string(),
                "Tier 2: Root is 'veng-'.".to_string(),
                "Tier 3: Add '-as'.".to_string(),
            ],
            raw_content:
                "### Context\nEnglish: I want you to come\n### Exercise\nQuiero que tú vengas\n"
                    .to_string(),
        },
        Exercise {
            path: PathBuf::from("exercises/02_por_para.md"),
            id: "b1_por_para_01".to_string(),
            level: Level::B1,
            topic: "por_vs_para".to_string(),
            exercise_type: ExerciseType::Cloze,
            is_done: false,
            title: "Por vs Para Purpose".to_string(),
            solution: "para".to_string(),
            alternatives: vec![],
            diagnostic_rules: vec![DiagnosticRule {
                pattern: "por".to_string(),
                code: "E0701".to_string(),
                message: "Expected 'para' for purpose.".to_string(),
            }],
            hints: vec!["Think about destination vs motive.".to_string()],
            raw_content: "### Context\nEnglish: For you\n### Exercise\nEsto es para ti\n"
                .to_string(),
        },
    ]
}

#[test]
fn test_app_initialization_and_navigation() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    assert_eq!(app.current_index, 0);
    assert_eq!(app.current_exercise().unwrap().id, "b1_subj_01");

    app.next_exercise();
    assert_eq!(app.current_index, 1);
    assert_eq!(app.current_exercise().unwrap().id, "b1_por_para_01");

    // Wrap around
    app.next_exercise();
    assert_eq!(app.current_index, 0);

    // Prev wrap around
    app.prev_exercise();
    assert_eq!(app.current_index, 1);
}

#[test]
fn test_app_input_editing() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    app.insert_char('h');
    app.insert_char('o');
    app.insert_char('l');
    app.insert_char('a');
    assert_eq!(app.input_buffer, "hola");
    assert_eq!(app.cursor_position, 4);

    app.move_cursor_left();
    assert_eq!(app.cursor_position, 3);

    app.delete_char_backwards();
    assert_eq!(app.input_buffer, "hoa");
    assert_eq!(app.cursor_position, 2);

    app.move_cursor_right();
    assert_eq!(app.cursor_position, 3);

    app.insert_char('!');
    assert_eq!(app.input_buffer, "hoa!");
}

#[test]
fn test_app_submission_evaluation_passed_and_failed() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    // Submit wrong answer
    app.input_buffer = "viene".to_string();
    app.submit_current_answer();
    assert!(app.last_result.is_some());
    assert!(!app.last_result.as_ref().unwrap().is_success());
    assert!(!app.current_exercise().unwrap().is_done);

    // Submit correct answer
    app.input_buffer = "vengas".to_string();
    app.submit_current_answer();
    assert!(app.last_result.is_some());
    assert!(app.last_result.as_ref().unwrap().is_success());
    assert!(app.current_exercise().unwrap().is_done);
}

#[test]
fn test_app_toggles_hints_and_reference() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    assert!(!app.show_hint);
    assert!(!app.show_reference);

    app.toggle_hint();
    assert!(app.show_hint);
    assert!(!app.show_reference);

    app.toggle_reference();
    assert!(!app.show_hint);
    assert!(app.show_reference);

    app.toggle_reference();
    assert!(!app.show_reference);
}

#[test]
fn test_app_reset() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    app.input_buffer = "vengas".to_string();
    app.submit_current_answer();
    assert!(app.current_exercise().unwrap().is_done);

    app.reset();
    assert!(!app.current_exercise().unwrap().is_done);
    assert!(app.input_buffer.is_empty());
    assert!(app.last_result.is_none());
}

#[test]
fn test_app_draw_ui_renders_without_panicking() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);
    app.input_buffer = "viene".to_string();
    app.submit_current_answer();

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();

    // Also test narrow width
    let narrow_backend = TestBackend::new(80, 30);
    let mut narrow_terminal = Terminal::new(narrow_backend).unwrap();
    narrow_terminal.draw(|frame| draw_ui(frame, &app)).unwrap();
}

#[test]
fn test_app_utf8_spanish_characters_editing_and_rendering() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    // Type Spanish accented characters and ñ
    app.insert_char('e');
    app.insert_char('s');
    app.insert_char('p');
    app.insert_char('a');
    app.insert_char('ñ');
    app.insert_char('o');
    app.insert_char('l');
    assert_eq!(app.input_buffer, "español");
    assert_eq!(app.cursor_position, 7);

    // Navigate cursor inside UTF-8 multi-byte sequence
    app.move_cursor_left(); // at 'l'
    app.move_cursor_left(); // at 'o'
    app.move_cursor_left(); // at 'ñ'
    assert_eq!(app.cursor_position, 4);

    // Delete 'a' before 'ñ'
    app.delete_char_backwards();
    assert_eq!(app.input_buffer, "espñol");
    assert_eq!(app.cursor_position, 3);

    // Delete 'ñ' with forward delete
    app.delete_char_forwards();
    assert_eq!(app.input_buffer, "espol");
    assert_eq!(app.cursor_position, 3);

    // Insert 'á' (2-byte character)
    app.insert_char('á');
    assert_eq!(app.input_buffer, "espáol");
    assert_eq!(app.cursor_position, 4);

    // Render with cursor at accented position
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();
}

#[test]
fn test_app_search_filtering_and_navigation() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    assert_eq!(app.exercises.len(), 2);

    // Enter search mode
    app.enter_search();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Searching);
    assert_eq!(app.filtered_indices.len(), 2);

    // Type "para"
    app.insert_search_char('p');
    app.insert_search_char('a');
    app.insert_search_char('r');
    app.insert_search_char('a');
    assert_eq!(app.search_query, "para");
    assert_eq!(app.filtered_indices.len(), 1);
    assert_eq!(app.current_exercise().unwrap().id, "b1_por_para_01");

    // Delete characters back
    app.delete_search_char_backwards(); // "par"
    app.delete_search_char_backwards(); // "pa"
    app.delete_search_char_backwards(); // "p"
    app.delete_search_char_backwards(); // ""
    assert_eq!(app.filtered_indices.len(), 2);

    // Filter by "subj"
    app.insert_search_char('s');
    app.insert_search_char('u');
    app.insert_search_char('b');
    app.insert_search_char('j');
    assert_eq!(app.filtered_indices.len(), 1);
    assert_eq!(app.current_exercise().unwrap().id, "b1_subj_01");

    // Confirm search selection
    app.exit_search(true);
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Editing);
    assert_eq!(app.current_exercise().unwrap().id, "b1_subj_01");
}

#[test]
fn test_app_search_cancel_restores_state() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    app.enter_search();
    app.insert_search_char('p');
    app.insert_search_char('a');
    app.insert_search_char('r');
    app.insert_search_char('a');
    assert_eq!(app.filtered_indices.len(), 1);

    // Cancel search
    app.exit_search(false);
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Editing);
    assert_eq!(app.exercises.len(), 2);
    assert!(app.search_query.is_empty());
}

#[test]
fn test_app_draw_ui_in_search_mode() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    app.enter_search();
    app.insert_search_char('p');
    app.insert_search_char('o');
    app.insert_search_char('r');

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();
}

#[test]
fn test_tui_conjugator_modal_navigation_and_lookup() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Editing);

    // Open conjugator modal
    app.enter_conjugator();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Conjugating);
    assert_eq!(app.conjugator_query, "");

    // Type "tener"
    for c in "tener".chars() {
        app.insert_conjugator_char(c);
    }
    assert_eq!(app.conjugator_query, "tener");
    assert!(app.conjugator_table.is_some());
    let table = app.conjugator_table.as_ref().unwrap();
    assert_eq!(table.infinitive, "tener");
    assert_eq!(table.present.yo, "tengo");
    assert_eq!(table.preterite.yo, "tuve");
    assert_eq!(table.present_subjunctive.yo, "tenga");

    // Scroll
    app.scroll_conjugator_down();
    assert_eq!(app.conjugator_scroll, 1);
    app.scroll_conjugator_up();
    assert_eq!(app.conjugator_scroll, 0);

    // Backspace
    app.delete_conjugator_char_backwards();
    assert_eq!(app.conjugator_query, "tene");

    // Exit conjugator modal
    app.exit_conjugator();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Editing);
}

#[test]
fn test_tui_reference_browser_modal() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    app.enter_reference_browser();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::BrowsingReference);
    assert!(!app.ref_topics.is_empty());
    assert_eq!(app.ref_filtered_topics.len(), app.ref_topics.len());

    // Filter by "accents"
    for c in "accents".chars() {
        app.insert_ref_search_char(c);
    }
    assert_eq!(app.ref_query, "accents");
    assert_eq!(app.ref_filtered_topics.len(), 1);
    assert_eq!(app.ref_filtered_topics[0], "accents");

    // Clear filter
    for _ in 0..7 {
        app.delete_ref_search_char_backwards();
    }
    assert_eq!(app.ref_query, "");
    assert_eq!(app.ref_filtered_topics.len(), app.ref_topics.len());

    // Navigation and scrolling
    app.next_ref_topic();
    assert_eq!(app.ref_selected_idx, 1);
    app.prev_ref_topic();
    assert_eq!(app.ref_selected_idx, 0);

    app.scroll_ref_down();
    assert_eq!(app.ref_scroll, 1);
    app.scroll_ref_up();
    assert_eq!(app.ref_scroll, 0);

    // Exit reference browser
    app.exit_reference_browser();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Editing);
}

#[test]
fn test_tui_help_modal() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    app.enter_help();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Help);

    app.exit_help();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Editing);
}

#[test]
fn test_tui_draw_all_modals_without_panicking() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    // 1. Draw Conjugating modal with active table
    app.enter_conjugator();
    for c in "hacer".chars() {
        app.insert_conjugator_char(c);
    }
    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();

    // 2. Draw Reference Browser modal
    app.enter_reference_browser();
    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();

    // 3. Draw Help modal
    app.enter_help();
    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();
}
