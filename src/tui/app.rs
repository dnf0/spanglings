use crate::core::conjugator::{conjugate_verb, VerbTable};
use crate::core::exercise::Exercise;
use crate::core::reference::list_reference_topics;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{validate_submission, ValidationResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Editing,
    Searching,
    Conjugating,
    BrowsingReference,
    Help,
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

    // Conjugator modal state
    pub conjugator_query: String,
    pub conjugator_cursor: usize,
    pub conjugator_table: Option<VerbTable>,
    pub conjugator_scroll: usize,

    // Reference browser modal state
    pub ref_query: String,
    pub ref_cursor: usize,
    pub ref_topics: Vec<&'static str>,
    pub ref_filtered_topics: Vec<&'static str>,
    pub ref_selected_idx: usize,
    pub ref_scroll: usize,
}

impl App {
    pub fn new(exercises: Vec<Exercise>, strict_accents: bool) -> Self {
        let count = exercises.len();
        let filtered_indices = (0..count).collect();
        let ref_topics: Vec<&'static str> = list_reference_topics().to_vec();
        let ref_filtered_topics = ref_topics.clone();
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
            conjugator_query: String::new(),
            conjugator_cursor: 0,
            conjugator_table: None,
            conjugator_scroll: 0,
            ref_query: String::new(),
            ref_cursor: 0,
            ref_topics,
            ref_filtered_topics,
            ref_selected_idx: 0,
            ref_scroll: 0,
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

    // --- Conjugator Modal Methods ---

    pub fn enter_conjugator(&mut self) {
        self.mode = AppMode::Conjugating;
        self.conjugator_query.clear();
        self.conjugator_cursor = 0;
        self.conjugator_table = None;
        self.conjugator_scroll = 0;
    }

    pub fn exit_conjugator(&mut self) {
        self.mode = AppMode::Editing;
    }

    pub fn insert_conjugator_char(&mut self, c: char) {
        let byte_idx = self
            .conjugator_query
            .char_indices()
            .nth(self.conjugator_cursor)
            .map(|(idx, _)| idx)
            .unwrap_or(self.conjugator_query.len());
        self.conjugator_query.insert(byte_idx, c);
        self.conjugator_cursor += 1;
        self.conjugator_table = conjugate_verb(&self.conjugator_query);
    }

    pub fn delete_conjugator_char_backwards(&mut self) {
        if self.conjugator_cursor > 0 {
            let char_to_remove = self.conjugator_cursor - 1;
            if let Some((byte_idx, _)) = self.conjugator_query.char_indices().nth(char_to_remove) {
                self.conjugator_query.remove(byte_idx);
                self.conjugator_cursor -= 1;
                self.conjugator_table = if self.conjugator_query.trim().is_empty() {
                    None
                } else {
                    conjugate_verb(&self.conjugator_query)
                };
            }
        }
    }

    pub fn submit_conjugation(&mut self) {
        self.conjugator_table = conjugate_verb(&self.conjugator_query);
    }

    pub fn scroll_conjugator_up(&mut self) {
        if self.conjugator_scroll > 0 {
            self.conjugator_scroll -= 1;
        }
    }

    pub fn scroll_conjugator_down(&mut self) {
        self.conjugator_scroll = self.conjugator_scroll.saturating_add(1);
    }

    // --- Reference Browser Modal Methods ---

    pub fn enter_reference_browser(&mut self) {
        self.mode = AppMode::BrowsingReference;
        self.ref_query.clear();
        self.ref_cursor = 0;
        self.ref_filtered_topics = self.ref_topics.clone();
        self.ref_selected_idx = 0;
        self.ref_scroll = 0;
    }

    pub fn exit_reference_browser(&mut self) {
        self.mode = AppMode::Editing;
    }

    pub fn insert_ref_search_char(&mut self, c: char) {
        let byte_idx = self
            .ref_query
            .char_indices()
            .nth(self.ref_cursor)
            .map(|(idx, _)| idx)
            .unwrap_or(self.ref_query.len());
        self.ref_query.insert(byte_idx, c);
        self.ref_cursor += 1;
        self.update_ref_filter();
    }

    pub fn delete_ref_search_char_backwards(&mut self) {
        if self.ref_cursor > 0 {
            let char_to_remove = self.ref_cursor - 1;
            if let Some((byte_idx, _)) = self.ref_query.char_indices().nth(char_to_remove) {
                self.ref_query.remove(byte_idx);
                self.ref_cursor -= 1;
                self.update_ref_filter();
            }
        }
    }

    pub fn update_ref_filter(&mut self) {
        let q = self.ref_query.trim().to_lowercase();
        if q.is_empty() {
            self.ref_filtered_topics = self.ref_topics.clone();
        } else {
            self.ref_filtered_topics = self
                .ref_topics
                .iter()
                .filter(|topic| topic.to_lowercase().contains(&q))
                .copied()
                .collect();
        }
        self.ref_selected_idx = 0;
        self.ref_scroll = 0;
    }

    pub fn next_ref_topic(&mut self) {
        if self.ref_filtered_topics.is_empty() {
            return;
        }
        self.ref_selected_idx = (self.ref_selected_idx + 1) % self.ref_filtered_topics.len();
        self.ref_scroll = 0;
    }

    pub fn prev_ref_topic(&mut self) {
        if self.ref_filtered_topics.is_empty() {
            return;
        }
        if self.ref_selected_idx == 0 {
            self.ref_selected_idx = self.ref_filtered_topics.len() - 1;
        } else {
            self.ref_selected_idx -= 1;
        }
        self.ref_scroll = 0;
    }

    pub fn scroll_ref_up(&mut self) {
        if self.ref_scroll > 0 {
            self.ref_scroll -= 1;
        }
    }

    pub fn scroll_ref_down(&mut self) {
        self.ref_scroll = self.ref_scroll.saturating_add(1);
    }

    // --- Help Modal Methods ---

    pub fn enter_help(&mut self) {
        self.mode = AppMode::Help;
    }

    pub fn exit_help(&mut self) {
        self.mode = AppMode::Editing;
    }
}
