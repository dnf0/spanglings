use crate::core::exercise::Exercise;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{validate_submission, ValidationResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Editing,
}

pub struct App {
    pub exercises: Vec<Exercise>,
    pub current_index: usize,
    pub input_buffer: String,
    pub cursor_position: usize,
    pub state: AppState,
    pub strict_accents: bool,
    pub show_hint: bool,
    pub show_reference: bool,
    pub last_result: Option<ValidationResult>,
    pub status_message: Option<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new(exercises: Vec<Exercise>, strict_accents: bool) -> Self {
        Self {
            exercises,
            current_index: 0,
            input_buffer: String::new(),
            cursor_position: 0,
            state: AppState::Editing,
            strict_accents,
            show_hint: false,
            show_reference: false,
            last_result: None,
            status_message: None,
            should_quit: false,
        }
    }

    pub fn current_exercise(&self) -> Option<&Exercise> {
        self.exercises.get(self.current_index)
    }

    pub fn next_exercise(&mut self) {
        if self.exercises.is_empty() {
            return;
        }
        self.current_index = (self.current_index + 1) % self.exercises.len();
        self.reset_current_state();
    }

    pub fn prev_exercise(&mut self) {
        if self.exercises.is_empty() {
            return;
        }
        if self.current_index == 0 {
            self.current_index = self.exercises.len() - 1;
        } else {
            self.current_index -= 1;
        }
        self.reset_current_state();
    }

    pub fn insert_char(&mut self, c: char) {
        let byte_idx = self
            .input_buffer
            .char_indices()
            .nth(self.cursor_position)
            .map(|(idx, _)| idx)
            .unwrap_or(self.input_buffer.len());
        self.input_buffer.insert(byte_idx, c);
        self.cursor_position += 1;
    }

    pub fn delete_char_backwards(&mut self) {
        if self.cursor_position > 0 {
            let char_to_remove = self.cursor_position - 1;
            if let Some((byte_idx, _)) = self.input_buffer.char_indices().nth(char_to_remove) {
                self.input_buffer.remove(byte_idx);
                self.cursor_position -= 1;
            }
        }
    }

    pub fn delete_char_forwards(&mut self) {
        if let Some((byte_idx, _)) = self.input_buffer.char_indices().nth(self.cursor_position) {
            self.input_buffer.remove(byte_idx);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input_buffer.chars().count() {
            self.cursor_position += 1;
        }
    }

    pub fn set_cursor_end(&mut self) {
        self.cursor_position = self.input_buffer.chars().count();
    }

    pub fn submit_current_answer(&mut self) {
        let Some(ex) = self.current_exercise() else {
            return;
        };
        let accent_mode = if self.strict_accents {
            AccentMode::Strict
        } else {
            AccentMode::Forgiving
        };
        let result = validate_submission(ex, &self.input_buffer, accent_mode);

        if result.is_success() {
            // Mark exercise as done in our local clone
            if let Some(e) = self.exercises.get_mut(self.current_index) {
                e.is_done = true;
            }
            self.status_message = Some("Correct!".to_string());
        } else {
            self.status_message = Some("Incorrect. Check diagnostics.".to_string());
        }
        self.last_result = Some(result);
    }

    pub fn toggle_hint(&mut self) {
        self.show_hint = !self.show_hint;
        if self.show_hint {
            self.show_reference = false;
        }
    }

    pub fn toggle_reference(&mut self) {
        self.show_reference = !self.show_reference;
        if self.show_reference {
            self.show_hint = false;
        }
    }

    pub fn reset(&mut self) {
        if let Some(e) = self.exercises.get_mut(self.current_index) {
            e.is_done = false;
        }
        self.reset_current_state();
    }

    fn reset_current_state(&mut self) {
        self.input_buffer.clear();
        self.cursor_position = 0;
        self.show_hint = false;
        self.show_reference = false;
        self.last_result = None;
        self.status_message = None;
    }
}
