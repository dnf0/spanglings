use crate::core::srs::{calculate_sm2_review, SrsItem};
use crate::engine::accents::AccentMode;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseStat {
    pub attempts: u32,
    pub completed_at: Option<DateTime<Utc>>,
    pub hints_used: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    pub version: u32,
    pub completed_exercises: HashSet<String>,
    pub current_exercise: Option<String>,
    pub accent_mode: AccentMode,
    pub srs: HashMap<String, SrsItem>,
    pub stats: HashMap<String, ExerciseStat>,
    #[serde(default)]
    pub activity_history: HashMap<String, u32>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            version: 1,
            completed_exercises: HashSet::new(),
            current_exercise: None,
            accent_mode: AccentMode::Forgiving,
            srs: HashMap::new(),
            stats: HashMap::new(),
            activity_history: HashMap::new(),
        }
    }
}

impl AppState {
    pub fn default_path() -> PathBuf {
        if let Some(override_path) = std::env::var_os("SPANGLINGS_STATE_PATH") {
            return PathBuf::from(override_path);
        }
        dirs::config_dir()
            .map(|p| p.join("spanglings").join("state.json"))
            .unwrap_or_else(|| PathBuf::from(".spanglings_state.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::default_path();
        Self::load_from_path(&path)
    }

    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            Ok(Self::default())
        } else {
            let data = fs::read_to_string(path)
                .with_context(|| format!("Failed to read state file at {:?}", path))?;
            let state: Self = serde_json::from_str(&data)
                .with_context(|| format!("Failed to deserialize state JSON at {:?}", path))?;
            Ok(state)
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::default_path();
        self.save_to_path(&path)
    }

    pub fn save_to_path(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn is_completed(&self, exercise_id: &str) -> bool {
        self.completed_exercises.contains(exercise_id)
    }

    pub fn record_activity(&mut self, date_str: &str) {
        *self.activity_history.entry(date_str.to_string()).or_insert(0) += 1;
    }

    pub fn mark_completed(&mut self, exercise_id: &str) {
        self.completed_exercises.insert(exercise_id.to_string());
        let stat = self
            .stats
            .entry(exercise_id.to_string())
            .or_insert(ExerciseStat {
                attempts: 0,
                completed_at: None,
                hints_used: 0,
            });
        let now = Utc::now();
        stat.completed_at = Some(now);
        let today = now.format("%Y-%m-%d").to_string();
        self.record_activity(&today);
    }

    pub fn unmark_completed(&mut self, exercise_id: &str) {
        self.completed_exercises.remove(exercise_id);
        if let Some(stat) = self.stats.get_mut(exercise_id) {
            stat.completed_at = None;
        }
    }

    pub fn is_due_for_review(&self, exercise_id: &str, now: DateTime<Utc>) -> bool {
        if let Some(item) = self.srs.get(exercise_id) {
            item.next_review_due <= now
        } else {
            false
        }
    }

    pub fn update_srs(&mut self, exercise_id: &str, quality: u8, now: DateTime<Utc>) {
        let current = self
            .srs
            .get(exercise_id)
            .cloned()
            .unwrap_or_else(|| SrsItem::new(now));
        let updated = calculate_sm2_review(&current, quality, now);
        self.srs.insert(exercise_id.to_string(), updated);
        let today = now.format("%Y-%m-%d").to_string();
        self.record_activity(&today);
    }
}
