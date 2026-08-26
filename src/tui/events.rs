use crate::tui::app::App;
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

                match (key.code, key.modifiers) {
                    // Quit actions
                    (KeyCode::Esc, _) => {
                        app.should_quit = true;
                    }
                    (KeyCode::Char('c'), KeyModifiers::CONTROL)
                    | (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        app.should_quit = true;
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
                    (KeyCode::F(1), _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
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
                        app.cursor_position = app.input_buffer.len();
                    }
                    (KeyCode::Backspace, _) => {
                        app.delete_char_backwards();
                    }
                    (KeyCode::Delete, _) if app.cursor_position < app.input_buffer.len() => {
                        app.input_buffer.remove(app.cursor_position);
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
