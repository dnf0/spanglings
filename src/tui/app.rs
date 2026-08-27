use crate::core::exercise::Exercise;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{validate_submission, ValidationResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Editing,
    Searching,
}

pub struct App {
    pub exercises: Vec<Exercise>,
    pub current_index: usize,
    pub input_buffer: String,
    pub cursor_position: usize,
    pub mode: AppMode,
    pub strict_accents: bool,
    pub show_hint: bool,
    pub show_reference: bool,
    pub last_result: Option<ValidationResult>,
    pub status_message: Option<String>,
    pub should_quit: bool,

    // Search state
    pub search_query: String,
    pub search_cursor: usize,
    pub filtered_indices: Vec<usize>,
}

impl App {
    pub fn new(exercises: Vec<Exercise>, strict_accents: bool) -> Self {
        let count = exercises.len();
        let filtered_indices = (0..count).collect();
        Self {
            exercises,
            current_index: 0,
            input_buffer: String::new(),
            cursor_position: 0,
            mode: AppMode::Editing,
            strict_accents,
            show_hint: false,
            show_reference: false,
            last_result: None,
            status_message: None,
            should_quit: false,
            search_query: String::new(),
            search_cursor: 0,
            filtered_indices,
        }
    }

    pub fn current_exercise(&self) -> Option<&Exercise> {
        if self.mode == AppMode::Searching || !self.search_query.is_empty() {
            self.filtered_indices
                .get(self.current_index)
                .and_then(|&idx| self.exercises.get(idx))
        } else {
            self.exercises.get(self.current_index)
        }
    }

    pub fn current_exercise_mut(&mut self) -> Option<&mut Exercise> {
        if self.mode == AppMode::Searching || !self.search_query.is_empty() {
            if let Some(&actual_idx) = self.filtered_indices.get(self.current_index) {
                self.exercises.get_mut(actual_idx)
            } else {
                None
            }
        } else {
            self.exercises.get_mut(self.current_index)
        }
    }

    pub fn next_exercise(&mut self) {
        let len = self.active_list_len();
        if len == 0 {
            return;
        }
        self.current_index = (self.current_index + 1) % len;
        self.reset_current_state();
    }

    pub fn prev_exercise(&mut self) {
        let len = self.active_list_len();
        if len == 0 {
            return;
        }
        if self.current_index == 0 {
            self.current_index = len - 1;
        } else {
            self.current_index -= 1;
        }
        self.reset_current_state();
    }

    fn active_list_len(&self) -> usize {
        if self.mode == AppMode::Searching || !self.search_query.is_empty() {
            self.filtered_indices.len()
        } else {
            self.exercises.len()
        }
    }

    pub fn enter_search(&mut self) {
        self.mode = AppMode::Searching;
        self.search_query.clear();
        self.search_cursor = 0;
        self.update_search_filter();
    }

    pub fn exit_search(&mut self, confirm: bool) {
        if confirm {
            if let Some(&actual_idx) = self.filtered_indices.get(self.current_index) {
                self.current_index = actual_idx;
            }
        } else {
            self.search_query.clear();
            self.filtered_indices = (0..self.exercises.len()).collect();
        }
        self.mode = AppMode::Editing;
        self.reset_current_state();
    }

    pub fn update_search_filter(&mut self) {
        let q = self.search_query.trim().to_lowercase();
        if q.is_empty() {
            self.filtered_indices = (0..self.exercises.len()).collect();
        } else {
            self.filtered_indices = self
                .exercises
                .iter()
                .enumerate()
                .filter(|(_, ex)| {
                    ex.id.to_lowercase().contains(&q)
                        || ex.title.to_lowercase().contains(&q)
                        || ex.topic.to_lowercase().contains(&q)
                        || ex.level.to_string().to_lowercase().contains(&q)
                        || ex.raw_content.to_lowercase().contains(&q)
                })
                .map(|(idx, _)| idx)
                .collect();
        }
        self.current_index = 0;
    }

    pub fn insert_search_char(&mut self, c: char) {
        let byte_idx = self
            .search_query
            .char_indices()
            .nth(self.search_cursor)
            .map(|(idx, _)| idx)
            .unwrap_or(self.search_query.len());
        self.search_query.insert(byte_idx, c);
        self.search_cursor += 1;
        self.update_search_filter();
    }

    pub fn delete_search_char_backwards(&mut self) {
        if self.search_cursor > 0 {
            let char_to_remove = self.search_cursor - 1;
            if let Some((byte_idx, _)) = self.search_query.char_indices().nth(char_to_remove) {
                self.search_query.remove(byte_idx);
                self.search_cursor -= 1;
                self.update_search_filter();
            }
        }
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
            if let Some(e) = self.current_exercise_mut() {
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
        if let Some(e) = self.current_exercise_mut() {
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
