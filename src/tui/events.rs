use crate::tui::app::{App, AppMode};
use crate::tui::ui::draw_ui;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::time::Duration;

pub fn run_tui_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|frame| draw_ui(frame, &app))?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // --- Arcade Arena Modal Event Handling ---
                if app.show_arcade_modal {
                    if app.arcade_item_idx >= app.arcade_items.len() {
                        match (key.code, key.modifiers) {
                            (KeyCode::Char('s'), _)
                            | (KeyCode::Char('S'), _)
                            | (KeyCode::Tab, _) => {
                                app.cycle_arcade_showdown(true);
                            }
                            (KeyCode::BackTab, _) => {
                                app.cycle_arcade_showdown(false);
                            }
                            (KeyCode::Char('r'), _) | (KeyCode::Char('R'), _) => {
                                if let Some(ref topic) = app.arcade_selected_topic.clone() {
                                    app.enter_arcade_with_topic(topic);
                                } else {
                                    let showdown = app.arcade_selected_showdown;
                                    app.enter_arcade_mode(showdown);
                                }
                            }
                            (KeyCode::Esc, _)
                            | (KeyCode::Char('q'), _)
                            | (KeyCode::Char('Q'), _) => {
                                app.exit_arcade_mode();
                            }
                            _ => {
                                app.on_key(key);
                            }
                        }
                    } else {
                        app.on_key(key);
                    }
                    continue;
                }

                // --- Welcome & Tour Modal Event Handling ---
                if app.show_tour_welcome || app.show_tour_modal {
                    app.on_key(key);
                    continue;
                }

                // --- Concept Mastery & Weakness Dashboard Modal Event Handling ---
                if app.show_mastery_dashboard {
                    app.on_key(key);
                    continue;
                }

                // --- Searching Mode Event Handling ---
                if app.mode == AppMode::Searching {
                    match (key.code, key.modifiers) {
                        (KeyCode::Esc, _) => {
                            app.exit_search(false);
                        }
                        (KeyCode::Enter, _) => {
                            app.exit_search(true);
                        }
                        (KeyCode::Down, _) | (KeyCode::Tab, _) => {
                            app.next_exercise();
                        }
                        (KeyCode::Up, _) | (KeyCode::BackTab, _) => {
                            app.prev_exercise();
                        }
                        (KeyCode::Backspace, _) => {
                            app.delete_search_char_backwards();
                        }
                        (KeyCode::Char(c), _) => {
                            app.insert_search_char(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                // --- Conjugating Modal Event Handling ---
                if app.mode == AppMode::Conjugating {
                    match (key.code, key.modifiers) {
                        (KeyCode::Esc, _)
                        | (KeyCode::F(3), _)
                        | (KeyCode::Char('c'), KeyModifiers::CONTROL)
                        | (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                            app.exit_conjugator();
                        }
                        (KeyCode::Enter, _) => {
                            app.submit_conjugation();
                        }
                        (KeyCode::Down, _) => {
                            app.scroll_conjugator_down();
                        }
                        (KeyCode::Up, _) => {
                            app.scroll_conjugator_up();
                        }
                        (KeyCode::Backspace, _) => {
                            app.delete_conjugator_char_backwards();
                        }
                        (KeyCode::Char(c), KeyModifiers::NONE)
                        | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                            app.insert_conjugator_char(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                // --- Browsing Reference Modal Event Handling ---
                if app.mode == AppMode::BrowsingReference {
                    match (key.code, key.modifiers) {
                        (KeyCode::Esc, _)
                        | (KeyCode::F(4), _)
                        | (KeyCode::Char('c'), KeyModifiers::CONTROL)
                        | (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                            app.exit_reference_browser();
                        }
                        (KeyCode::Down, _) | (KeyCode::Tab, _) => {
                            app.next_ref_topic();
                        }
                        (KeyCode::Up, _) | (KeyCode::BackTab, _) => {
                            app.prev_ref_topic();
                        }
                        (KeyCode::PageDown, _) => {
                            for _ in 0..5 {
                                app.scroll_ref_down();
                            }
                        }
                        (KeyCode::PageUp, _) => {
                            for _ in 0..5 {
                                app.scroll_ref_up();
                            }
                        }
                        (KeyCode::Backspace, _) => {
                            app.delete_ref_search_char_backwards();
                        }
                        (KeyCode::Char(c), KeyModifiers::NONE)
                        | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                            app.insert_ref_search_char(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                // --- Help Modal Event Handling ---
                if app.mode == AppMode::Help {
                    match (key.code, key.modifiers) {
                        (KeyCode::Char('t'), _) | (KeyCode::Char('T'), _) => {
                            app.exit_help();
                            app.show_tour_modal = true;
                            app.tour_current_station = 0;
                        }
                        (KeyCode::Esc, _)
                        | (KeyCode::Enter, _)
                        | (KeyCode::F(1), _)
                        | (KeyCode::Char('?'), _) => {
                            app.exit_help();
                        }
                        _ => {}
                    }
                    continue;
                }

                // --- Placement Test Modal Event Handling ---
                if app.mode == AppMode::PlacementTest {
                    match (key.code, key.modifiers) {
                        (KeyCode::Esc, _) => {
                            app.exit_placement_test();
                        }
                        (KeyCode::Enter, _) => {
                            if app.placement_finished {
                                app.exit_placement_test();
                            } else {
                                app.submit_placement_answer();
                            }
                        }
                        (KeyCode::Char('f'), KeyModifiers::NONE) | (KeyCode::Char('F'), _) => {
                            if app.placement_finished {
                                app.fast_track_placement_levels();
                            } else {
                                app.insert_placement_char('f');
                            }
                        }
                        (KeyCode::Backspace, _) => {
                            app.delete_placement_char_backwards();
                        }
                        (KeyCode::Char(c), KeyModifiers::NONE)
                        | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                            app.insert_placement_char(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                // --- Editing Mode Event Handling ---
                match (key.code, key.modifiers) {
                    // Quit actions
                    (KeyCode::Esc, _) => {
                        app.should_quit = true;
                    }
                    (KeyCode::Char('c'), KeyModifiers::CONTROL)
                    | (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        app.should_quit = true;
                    }

                    // Open Modals & Search
                    (KeyCode::Char('/'), KeyModifiers::NONE)
                    | (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                        app.enter_search();
                    }
                    (KeyCode::F(3), _)
                    | (KeyCode::Char('k'), KeyModifiers::CONTROL)
                    | (KeyCode::Char('c'), KeyModifiers::ALT) => {
                        app.enter_conjugator();
                    }
                    (KeyCode::F(4), _)
                    | (KeyCode::Char('b'), KeyModifiers::CONTROL)
                    | (KeyCode::Char('r'), KeyModifiers::ALT) => {
                        app.enter_reference_browser();
                    }
                    (KeyCode::F(5), _) | (KeyCode::Char('p'), KeyModifiers::ALT) => {
                        app.enter_placement_test();
                    }
                    (KeyCode::F(6), _)
                    | (KeyCode::Char('t'), KeyModifiers::ALT)
                    | (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                        app.show_tour_modal = true;
                        app.tour_current_station = 0;
                    }
                    (KeyCode::F(7), _)
                    | (KeyCode::Char('m'), KeyModifiers::ALT)
                    | (KeyCode::Char('m'), KeyModifiers::CONTROL)
                    | (KeyCode::Char('m'), KeyModifiers::NONE)
                    | (KeyCode::Char('M'), _) => {
                        app.toggle_mastery_dashboard();
                    }
                    (KeyCode::F(8), _)
                    | (KeyCode::Char('x'), KeyModifiers::ALT)
                    | (KeyCode::Char('x'), KeyModifiers::CONTROL)
                    | (KeyCode::Char('x'), KeyModifiers::NONE)
                    | (KeyCode::Char('X'), _)
                    | (KeyCode::Char('d'), KeyModifiers::ALT)
                    | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        app.enter_arcade_mode(None);
                    }
                    (KeyCode::F(1), _) | (KeyCode::Char('h'), KeyModifiers::ALT) => {
                        app.enter_help();
                    }

                    // Navigation actions
                    (KeyCode::Tab, _) | (KeyCode::Down, _) => {
                        app.next_exercise();
                    }
                    (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                        app.next_exercise();
                    }
                    (KeyCode::BackTab, _) | (KeyCode::Up, _) => {
                        app.prev_exercise();
                    }
                    (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                        app.prev_exercise();
                    }

                    // Hint & Reference shortcuts
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                        app.toggle_hint();
                    }
                    (KeyCode::F(2), _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                        app.toggle_reference();
                    }

                    // Reset
                    (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                        app.reset();
                    }

                    // Submit
                    (KeyCode::Enter, _) => {
                        app.submit_current_answer();
                    }

                    // Text Editing
                    (KeyCode::Left, _) => {
                        app.move_cursor_left();
                    }
                    (KeyCode::Right, _) => {
                        app.move_cursor_right();
                    }
                    (KeyCode::Home, _) => {
                        app.cursor_position = 0;
                    }
                    (KeyCode::End, _) => {
                        app.set_cursor_end();
                    }
                    (KeyCode::Backspace, _) => {
                        app.delete_char_backwards();
                    }
                    (KeyCode::Delete, _) => {
                        app.delete_char_forwards();
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE)
                    | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                        app.insert_char(c);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
