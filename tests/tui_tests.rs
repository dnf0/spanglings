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
            concept_tags: vec![],
            prerequisites: vec![],
            grammar_focus: None,
            contrast_note: None,
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
            concept_tags: vec![],
            prerequisites: vec![],
            grammar_focus: None,
            contrast_note: None,
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

    // 4. Draw Placement modal in testing state
    app.enter_placement_test();
    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();

    // 5. Submit all answers and draw Placement modal in results state
    while !app.placement_finished {
        for c in "hablo".chars() {
            app.insert_placement_char(c);
        }
        app.submit_placement_answer();
    }
    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();
}

#[test]
fn test_tui_placement_test_flow() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);

    app.enter_placement_test();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::PlacementTest);
    assert!(!app.placement_battery.is_empty());
    assert_eq!(app.placement_current_idx, 0);

    // Text editing
    app.insert_placement_char('h');
    app.insert_placement_char('o');
    app.insert_placement_char('l');
    app.insert_placement_char('a');
    assert_eq!(app.placement_input, "hola");
    assert_eq!(app.placement_cursor, 4);

    app.delete_placement_char_backwards();
    assert_eq!(app.placement_input, "hol");
    assert_eq!(app.placement_cursor, 3);

    // Answer questions to completion
    while !app.placement_finished {
        app.submit_placement_answer();
    }

    assert!(app.placement_result.is_some());
    assert!(app.placement_finished);

    // Fast track
    let marked = app.fast_track_placement_levels();
    assert_eq!(marked, 0); // None passed since answers were empty

    app.exit_placement_test();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Editing);
}

#[test]
fn test_app_first_run_welcome_state_initialization() {
    let exercises = create_sample_exercises();

    // 1. When state.tour_completed == false, show_tour_welcome is true
    let state1 = spanglings::core::state::AppState {
        tour_completed: false,
        ..Default::default()
    };
    let app1 = App::new_with_state(exercises.clone(), false, state1);
    assert!(app1.show_tour_welcome);
    assert!(!app1.show_tour_modal);

    // 2. When state.tour_completed == true, show_tour_welcome is false
    let state2 = spanglings::core::state::AppState {
        tour_completed: true,
        ..Default::default()
    };
    let app2 = App::new_with_state(exercises, false, state2);
    assert!(!app2.show_tour_welcome);
    assert!(!app2.show_tour_modal);
}

#[test]
fn test_app_tour_hotkey_toggle_in_normal_mode() {
    let exercises = create_sample_exercises();
    let state = spanglings::core::state::AppState {
        tour_completed: true,
        ..Default::default()
    };
    let mut app = App::new_with_state(exercises, false, state);
    assert!(!app.show_tour_modal);

    // Press 't' in editing mode opens tour modal
    app.on_key(crossterm::event::KeyCode::Char('t'));
    assert!(app.show_tour_modal);
    assert_eq!(app.tour_current_station, 0);

    // Dismiss with Esc
    app.on_key(crossterm::event::KeyCode::Esc);
    assert!(!app.show_tour_modal);

    // Press 'T' (uppercase) opens tour modal
    app.on_key(crossterm::event::KeyCode::Char('T'));
    assert!(app.show_tour_modal);
    assert_eq!(app.tour_current_station, 0);

    // Dismiss with 'q'
    app.on_key(crossterm::event::KeyCode::Char('q'));
    assert!(!app.show_tour_modal);

    // Reopen and dismiss with 'Q'
    app.on_key(crossterm::event::KeyCode::Char('t'));
    assert!(app.show_tour_modal);
    app.on_key(crossterm::event::KeyCode::Char('Q'));
    assert!(!app.show_tour_modal);
}

#[test]
fn test_app_tour_navigation_and_completion() {
    let exercises = create_sample_exercises();
    let state = spanglings::core::state::AppState {
        tour_completed: false,
        ..Default::default()
    };
    let mut app = App::new_with_state(exercises, false, state);

    // Open tour modal directly
    app.show_tour_welcome = false;
    app.show_tour_modal = true;
    app.tour_current_station = 0;

    let total_stations = spanglings::cli::commands::tour::get_tour_stations().len();
    assert_eq!(total_stations, 6);

    // Advance with Right arrow
    app.on_key(crossterm::event::KeyCode::Right);
    assert_eq!(app.tour_current_station, 1);

    // Advance with 'n'
    app.on_key(crossterm::event::KeyCode::Char('n'));
    assert_eq!(app.tour_current_station, 2);

    // Advance with 'N'
    app.on_key(crossterm::event::KeyCode::Char('N'));
    assert_eq!(app.tour_current_station, 3);

    // Go back with Left arrow
    app.on_key(crossterm::event::KeyCode::Left);
    assert_eq!(app.tour_current_station, 2);

    // Go back with 'p'
    app.on_key(crossterm::event::KeyCode::Char('p'));
    assert_eq!(app.tour_current_station, 1);

    // Go back with 'P'
    app.on_key(crossterm::event::KeyCode::Char('P'));
    assert_eq!(app.tour_current_station, 0);

    // Saturating sub at station 0
    app.on_key(crossterm::event::KeyCode::Left);
    assert_eq!(app.tour_current_station, 0);

    // Advance to station 5 (last station)
    for _ in 0..5 {
        app.on_key(crossterm::event::KeyCode::Right);
    }
    assert_eq!(app.tour_current_station, 5);
    assert!(app.show_tour_modal);
    assert!(!app.state.tour_completed);

    // Finishing last station with Enter marks tour_completed = true in state
    app.on_key(crossterm::event::KeyCode::Enter);
    assert!(!app.show_tour_modal);
    assert!(app.state.tour_completed);
}

#[test]
fn test_app_welcome_modal_responses() {
    let exercises = create_sample_exercises();

    // 1. Respond with 'y' -> opens tour modal
    let mut app_y = App::new(exercises.clone(), false);
    app_y.show_tour_welcome = true;
    app_y.on_key(crossterm::event::KeyCode::Char('y'));
    assert!(!app_y.show_tour_welcome);
    assert!(app_y.show_tour_modal);
    assert_eq!(app_y.tour_current_station, 0);

    // 2. Respond with 'Y' -> opens tour modal
    let mut app_cap_y = App::new(exercises.clone(), false);
    app_cap_y.show_tour_welcome = true;
    app_cap_y.on_key(crossterm::event::KeyCode::Char('Y'));
    assert!(!app_cap_y.show_tour_welcome);
    assert!(app_cap_y.show_tour_modal);

    // 3. Respond with 'n' -> dismisses welcome modal
    let mut app_n = App::new(exercises.clone(), false);
    app_n.show_tour_welcome = true;
    app_n.on_key(crossterm::event::KeyCode::Char('n'));
    assert!(!app_n.show_tour_welcome);
    assert!(!app_n.show_tour_modal);

    // 4. Respond with 'N' -> dismisses welcome modal
    let mut app_cap_n = App::new(exercises.clone(), false);
    app_cap_n.show_tour_welcome = true;
    app_cap_n.on_key(crossterm::event::KeyCode::Char('N'));
    assert!(!app_cap_n.show_tour_welcome);
    assert!(!app_cap_n.show_tour_modal);

    // 5. Respond with Esc -> dismisses welcome modal
    let mut app_esc = App::new(exercises, false);
    app_esc.show_tour_welcome = true;
    app_esc.on_key(crossterm::event::KeyCode::Esc);
    assert!(!app_esc.show_tour_welcome);
    assert!(!app_esc.show_tour_modal);
}

#[test]
fn test_tui_draw_tour_modals_without_panicking() {
    let exercises = create_sample_exercises();
    let mut app = App::new(exercises, false);
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    // 1. Draw Welcome Modal
    app.show_tour_welcome = true;
    app.show_tour_modal = false;
    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();

    // 2. Draw each station of Tour Modal
    app.show_tour_welcome = false;
    app.show_tour_modal = true;
    let total_stations = spanglings::cli::commands::tour::get_tour_stations().len();
    for i in 0..total_stations {
        app.tour_current_station = i;
        terminal.draw(|frame| draw_ui(frame, &app)).unwrap();
    }

    // 3. Draw on small/narrow terminal
    let narrow_backend = TestBackend::new(70, 25);
    let mut narrow_terminal = Terminal::new(narrow_backend).unwrap();
    app.show_tour_welcome = true;
    narrow_terminal.draw(|frame| draw_ui(frame, &app)).unwrap();
    app.show_tour_welcome = false;
    narrow_terminal.draw(|frame| draw_ui(frame, &app)).unwrap();
}

#[test]
fn test_tui_tour_events_delegation() {
    let exercises = create_sample_exercises();
    let state = spanglings::core::state::AppState {
        tour_completed: false,
        ..Default::default()
    };
    let mut app = App::new_with_state(exercises, false, state);

    // Initial state: welcome modal is shown for new state
    assert!(app.show_tour_welcome);

    // Press 'y' -> opens tour modal at station 0
    app.on_key(crossterm::event::KeyCode::Char('y'));
    assert!(!app.show_tour_welcome);
    assert!(app.show_tour_modal);
    assert_eq!(app.tour_current_station, 0);

    // Advance station with Right
    app.on_key(crossterm::event::KeyCode::Right);
    assert_eq!(app.tour_current_station, 1);

    // Advance station with 'n'
    app.on_key(crossterm::event::KeyCode::Char('n'));
    assert_eq!(app.tour_current_station, 2);

    // Go back with 'p'
    app.on_key(crossterm::event::KeyCode::Char('p'));
    assert_eq!(app.tour_current_station, 1);

    // Close with 'q'
    app.on_key(crossterm::event::KeyCode::Char('q'));
    assert!(!app.show_tour_modal);

    // In editing mode, open help modal then press 't'
    app.enter_help();
    assert_eq!(app.mode, spanglings::tui::app::AppMode::Help);
    app.exit_help();
    app.show_tour_modal = true;
    app.tour_current_station = 0;
    assert!(app.show_tour_modal);
}

#[test]
fn test_tui_renders_exercise_instructions_block() {
    let exercise = Exercise {
        path: PathBuf::from("exercises/sample.md"),
        id: "sample_01".to_string(),
        level: Level::B1,
        topic: "ser_vs_estar".to_string(),
        exercise_type: ExerciseType::Cloze,
        is_done: false,
        title: "Ser vs Estar Test".to_string(),
        solution: "estamos".to_string(),
        alternatives: vec![],
        diagnostic_rules: vec![],
        hints: vec![],
        raw_content: r#"> **Grammar Rule**: 'Estar' is used for readiness.

### Context
English: "We are ready."

### Instructions
**TODO**: Conjugate the verb (estar) in 1st person plural.
**Why**: Readiness expresses a temporary resultant state.

### Exercise
<!-- TODO: Fill in the correct form of estar -->
Nosotros (estar) ___ listos.
"#
        .to_string(),
        concept_tags: vec![],
        prerequisites: vec![],
        grammar_focus: None,
        contrast_note: None,
    };

    let mut app = App::new(vec![exercise], true);
    app.show_tour_welcome = false;
    app.show_tour_modal = false;
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| draw_ui(frame, &app)).unwrap();
    let buffer = terminal.backend().buffer();
    let rendered_text = buffer
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    assert!(
        rendered_text.contains("Instructions (TODO & Why):"),
        "Buffer should render Instructions section header"
    );
    assert!(
        rendered_text.contains("TODO:"),
        "Buffer should render TODO label"
    );
    assert!(
        rendered_text.contains("Why:"),
        "Buffer should render Why label"
    );
    assert!(
        rendered_text.contains("Sentence:"),
        "Buffer should render Sentence section"
    );
}

#[test]
fn test_app_initialization_with_embedded_fallback() {
    let embedded = spanglings::core::curriculum::find_all_exercises_or_embedded(PathBuf::from(
        "nonexistent_directory_for_test",
    ))
    .expect("Embedded exercises should load");
    assert_eq!(embedded.len(), 339);
    let app = App::new(embedded, false);
    assert_eq!(app.exercises.len(), 339);
    assert_eq!(app.filtered_indices.len(), 339);
    assert_eq!(app.current_index, 0);
}
