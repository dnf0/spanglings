use crate::core::conjugator::{conjugate_verb, VerbTable};
use crate::core::exercise::Exercise;
use crate::core::placement::{
    evaluate_placement_test, get_placement_battery, PlacementQuestion, PlacementResult,
};
use crate::core::reference::list_reference_topics;
use crate::core::state::{AppState, EvaluatedLevel, ExerciseStat};
use crate::engine::accents::AccentMode;
use crate::engine::validator::{validate_submission, ValidationResult};
use chrono::Utc;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Editing,
    Searching,
    Conjugating,
    BrowsingReference,
    PlacementTest,
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
    pub state: AppState,

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

    // Placement test modal state
    pub placement_battery: Vec<PlacementQuestion>,
    pub placement_answers: Vec<String>,
    pub placement_current_idx: usize,
    pub placement_input: String,
    pub placement_cursor: usize,
    pub placement_result: Option<PlacementResult>,
    pub placement_finished: bool,
    pub placement_fast_tracked: bool,

    // Tour state
    pub show_tour_welcome: bool,
    pub show_tour_modal: bool,
    pub tour_current_station: usize,
}

impl App {
    pub fn new(exercises: Vec<Exercise>, strict_accents: bool) -> Self {
        let state = AppState::load().unwrap_or_default();
        Self::new_with_state(exercises, strict_accents, state)
    }

    pub fn new_with_state(
        mut exercises: Vec<Exercise>,
        strict_accents: bool,
        state: AppState,
    ) -> Self {
        for ex in &mut exercises {
            if state.is_completed(&ex.id) {
                ex.is_done = true;
            }
        }
        let count = exercises.len();
        let filtered_indices = (0..count).collect();
        let ref_topics: Vec<&'static str> = list_reference_topics().to_vec();
        let ref_filtered_topics = ref_topics.clone();
        let show_tour_welcome = !state.tour_completed;

        let start_index = if let Some(ref curr_id) = state.current_exercise {
            exercises
                .iter()
                .position(|e| &e.id == curr_id)
                .unwrap_or_else(|| exercises.iter().position(|e| !e.is_done).unwrap_or(0))
        } else {
            exercises.iter().position(|e| !e.is_done).unwrap_or(0)
        };

        Self {
            exercises,
            current_index: start_index,
            input_buffer: String::new(),
            cursor_position: 0,
            mode: AppMode::Editing,
            strict_accents,
            show_hint: false,
            show_reference: false,
            last_result: None,
            status_message: None,
            should_quit: false,
            state,
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
            placement_battery: Vec::new(),
            placement_answers: Vec::new(),
            placement_current_idx: 0,
            placement_input: String::new(),
            placement_cursor: 0,
            placement_result: None,
            placement_finished: false,
            placement_fast_tracked: false,
            show_tour_welcome,
            show_tour_modal: false,
            tour_current_station: 0,
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
        let ex_id = self.current_exercise().map(|e| e.id.clone());
        if let Some(id) = ex_id {
            self.state.current_exercise = Some(id);
            let _ = self.state.save();
        }
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
        let ex_id = self.current_exercise().map(|e| e.id.clone());
        if let Some(id) = ex_id {
            self.state.current_exercise = Some(id);
            let _ = self.state.save();
        }
    }

    fn active_list_len(&self) -> usize {
        if self.mode == AppMode::Searching || !self.search_query.is_empty() {
            self.filtered_indices.len()
        } else {
            self.exercises.len()
        }
    }

    pub fn enter_search(&mut self) {
        self.show_tour_welcome = false;
        self.show_tour_modal = false;
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
        let ex_id = self.current_exercise().map(|e| e.id.clone());
        if let Some(id) = ex_id {
            self.state.current_exercise = Some(id);
            let _ = self.state.save();
        }
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
        let exercise_id = ex.id.clone();
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
            let quality = if self.show_hint { 4 } else { 5 };
            self.state.mark_completed(&exercise_id);
            self.state.update_srs(&exercise_id, quality, Utc::now());
            self.state.current_exercise = Some(exercise_id);
            let _ = self.state.save();
            self.status_message = Some("Correct!".to_string());
        } else {
            let stat = self
                .state
                .stats
                .entry(exercise_id)
                .or_insert_with(|| ExerciseStat {
                    attempts: 0,
                    completed_at: None,
                    hints_used: 0,
                });
            stat.attempts += 1;
            let _ = self.state.save();
            self.status_message = Some("Incorrect. Check diagnostics.".to_string());
        }
        self.last_result = Some(result);
    }

    pub fn toggle_hint(&mut self) {
        self.show_hint = !self.show_hint;
        if self.show_hint {
            self.show_reference = false;
            let ex_id = self.current_exercise().map(|e| e.id.clone());
            if let Some(id) = ex_id {
                let stat = self.state.stats.entry(id).or_insert_with(|| ExerciseStat {
                    attempts: 0,
                    completed_at: None,
                    hints_used: 0,
                });
                stat.hints_used += 1;
                let _ = self.state.save();
            }
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
        let ex_id = self.current_exercise().map(|e| e.id.clone());
        if let Some(id) = ex_id {
            self.state.unmark_completed(&id);
            let _ = self.state.save();
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
        self.show_tour_welcome = false;
        self.show_tour_modal = false;
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
        self.show_tour_welcome = false;
        self.show_tour_modal = false;
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
        self.show_tour_welcome = false;
        self.show_tour_modal = false;
        self.mode = AppMode::Help;
    }

    pub fn exit_help(&mut self) {
        self.mode = AppMode::Editing;
    }

    // --- Placement Test Modal Methods ---

    pub fn enter_placement_test(&mut self) {
        self.show_tour_welcome = false;
        self.show_tour_modal = false;
        self.mode = AppMode::PlacementTest;
        self.placement_battery = get_placement_battery(None);
        self.placement_answers = Vec::new();
        self.placement_current_idx = 0;
        self.placement_input.clear();
        self.placement_cursor = 0;
        self.placement_finished = false;
        self.placement_result = None;
        self.placement_fast_tracked = false;
    }

    pub fn exit_placement_test(&mut self) {
        self.mode = AppMode::Editing;
    }

    pub fn submit_placement_answer(&mut self) {
        if self.placement_finished {
            return;
        }

        self.placement_answers
            .push(self.placement_input.trim().to_string());
        self.placement_input.clear();
        self.placement_cursor = 0;
        self.placement_current_idx += 1;

        if self.placement_current_idx >= self.placement_battery.len() {
            let accent_mode = if self.strict_accents {
                AccentMode::Strict
            } else {
                AccentMode::Forgiving
            };
            let res = evaluate_placement_test(
                &self.placement_battery,
                &self.placement_answers,
                accent_mode,
            );

            // Update state
            self.state.evaluated_level = Some(EvaluatedLevel {
                level: res.assessed_level,
                score_percent: res.percentage,
                evaluated_at: Utc::now(),
            });
            let _ = self.state.save();

            self.placement_result = Some(res);
            self.placement_finished = true;
        }
    }

    pub fn fast_track_placement_levels(&mut self) -> usize {
        if let Some(res) = &self.placement_result {
            let mut total_marked = 0;
            for &lvl in &res.passed_levels {
                total_marked += self.state.fast_track_level(lvl, &self.exercises);
            }
            for ex in &mut self.exercises {
                if self.state.is_completed(&ex.id) {
                    ex.is_done = true;
                }
            }
            let _ = self.state.save();
            self.placement_fast_tracked = true;
            self.status_message = Some(format!(
                "✨ Fast-tracked {} exercises across passed levels!",
                total_marked
            ));
            return total_marked;
        }
        0
    }

    pub fn insert_placement_char(&mut self, c: char) {
        if self.placement_finished {
            return;
        }
        let byte_idx = self
            .placement_input
            .char_indices()
            .nth(self.placement_cursor)
            .map(|(i, _)| i)
            .unwrap_or(self.placement_input.len());
        self.placement_input.insert(byte_idx, c);
        self.placement_cursor += 1;
    }

    pub fn delete_placement_char_backwards(&mut self) {
        if self.placement_finished || self.placement_cursor == 0 {
            return;
        }
        let char_to_remove = self.placement_cursor - 1;
        if let Some((byte_idx, _)) = self.placement_input.char_indices().nth(char_to_remove) {
            self.placement_input.remove(byte_idx);
            self.placement_cursor -= 1;
        }
    }

    // --- Key Event Handling ---

    pub fn on_key(&mut self, key: impl Into<KeyEvent>) {
        let key = key.into();
        if self.show_tour_welcome {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.show_tour_welcome = false;
                    self.show_tour_modal = true;
                    self.tour_current_station = 0;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.show_tour_welcome = false;
                }
                _ => {}
            }
            return;
        }

        if self.show_tour_modal {
            match key.code {
                KeyCode::Right | KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Enter => {
                    let total_stations = crate::cli::commands::tour::get_tour_stations().len();
                    if self.tour_current_station + 1 < total_stations {
                        self.tour_current_station += 1;
                    } else {
                        self.show_tour_modal = false;
                        self.state.mark_tour_completed();
                        if let Err(e) = self.state.save() {
                            self.status_message =
                                Some(format!("Warning: Failed to save state: {}", e));
                        }
                    }
                }
                KeyCode::Left | KeyCode::Char('p') | KeyCode::Char('P') => {
                    self.tour_current_station = self.tour_current_station.saturating_sub(1);
                }
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.show_tour_modal = false;
                }
                _ => {}
            }
            return;
        }

        if self.mode == AppMode::Editing {
            match key.code {
                KeyCode::Char('t') | KeyCode::Char('T') => {
                    self.show_tour_modal = true;
                    self.tour_current_station = 0;
                }
                _ => {}
            }
        }
    }
}
