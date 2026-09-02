use crate::core::curriculum::Level;
use crate::core::exercise::Exercise;
use crate::core::srs::{calculate_sm2_review, SrsItem};
use crate::engine::accents::AccentMode;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluatedLevel {
    pub level: Level,
    pub score_percent: f64,
    pub evaluated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseStat {
    pub attempts: u32,
    pub completed_at: Option<DateTime<Utc>>,
    pub hints_used: u32,
}

fn default_ease_factor() -> f32 {
    2.5
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConceptMastery {
    pub concept_id: String,
    pub mastery_score: f32, // 0.0 to 1.0
    #[serde(default)]
    pub repetitions: u32,
    #[serde(default)]
    pub interval_days: u32,
    #[serde(default = "default_ease_factor")]
    pub ease_factor: f32,
    pub total_reviews: u32,
    pub lapses: u32,
    pub last_practiced: Option<DateTime<Utc>>,
}

impl ConceptMastery {
    pub fn new(concept_id: impl Into<String>) -> Self {
        Self {
            concept_id: concept_id.into(),
            mastery_score: 0.0,
            repetitions: 0,
            interval_days: 0,
            ease_factor: 2.5,
            total_reviews: 0,
            lapses: 0,
            last_practiced: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppState {
    pub version: u32,
    pub completed_exercises: HashSet<String>,
    pub current_exercise: Option<String>,
    pub accent_mode: AccentMode,
    pub srs: HashMap<String, SrsItem>,
    pub stats: HashMap<String, ExerciseStat>,
    pub activity_history: HashMap<String, u32>,
    pub evaluated_level: Option<EvaluatedLevel>,
    pub concept_mastery: HashMap<String, ConceptMastery>,
    pub tour_completed: bool,
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
            evaluated_level: None,
            concept_mastery: HashMap::new(),
            tour_completed: false,
        }
    }
}

impl AppState {
    pub fn default_path() -> PathBuf {
        if let Some(override_path) = std::env::var_os("SPANGLINGS_STATE_PATH") {
            return PathBuf::from(override_path);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            dirs::config_dir()
                .map(|p| p.join("spanglings").join("state.json"))
                .unwrap_or_else(|| PathBuf::from(".spanglings_state.json"))
        }
        #[cfg(target_arch = "wasm32")]
        {
            PathBuf::from(".spanglings_state.json")
        }
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
        *self
            .activity_history
            .entry(date_str.to_string())
            .or_insert(0) += 1;
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

    pub fn mark_tour_completed(&mut self) {
        self.tour_completed = true;
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

    pub fn fast_track_level(&mut self, level: Level, exercises: &[Exercise]) -> usize {
        let now = Utc::now();
        let mut count = 0;
        for ex in exercises {
            if ex.level == level {
                self.completed_exercises.insert(ex.id.clone());
                let stat = self.stats.entry(ex.id.clone()).or_insert(ExerciseStat {
                    attempts: 1,
                    completed_at: Some(now),
                    hints_used: 0,
                });
                stat.completed_at = Some(now);

                let srs_item = SrsItem {
                    interval_days: 14,
                    ease_factor: 2.6,
                    repetitions: 2,
                    last_reviewed: Some(now),
                    next_review_due: now + chrono::Duration::days(14),
                };
                self.srs.insert(ex.id.clone(), srs_item);
                count += 1;
            }
        }
        let today = now.format("%Y-%m-%d").to_string();
        self.record_activity(&today);
        count
    }

    pub fn update_concept_mastery(&mut self, concept_id: &str, quality: u8, now: DateTime<Utc>) {
        let entry = self
            .concept_mastery
            .entry(concept_id.to_string())
            .or_insert_with(|| ConceptMastery::new(concept_id));

        entry.last_practiced = Some(now);
        entry.total_reviews += 1;

        let q = quality.clamp(0, 5);
        let q_f32 = q as f32;
        let mut new_ef = entry.ease_factor + (0.1 - (5.0 - q_f32) * (0.08 + (5.0 - q_f32) * 0.02));
        if new_ef < 1.3 {
            new_ef = 1.3;
        }
        entry.ease_factor = new_ef;

        if q < 3 {
            entry.lapses += 1;
            entry.repetitions = entry.repetitions.saturating_sub(1);
            entry.interval_days = match entry.repetitions {
                0 => 0,
                1 => 1,
                2 => 6,
                _ => ((entry.interval_days as f32) / entry.ease_factor).round() as u32,
            }
            .clamp(0, 3650);
        } else {
            entry.repetitions += 1;
            entry.interval_days = match entry.repetitions {
                1 => 1,
                2 => 6,
                _ => ((entry.interval_days as f32) * new_ef).round() as u32,
            }
            .clamp(1, 3650);
        }

        if entry.repetitions == 0 || entry.interval_days == 0 {
            entry.mastery_score = 0.0;
        } else {
            const MAX_STABILITY_LN: f32 = 4.110874; // ln(61.0)
            let rep_factor = (entry.repetitions as f32 / 6.0).min(1.0);
            let stability_factor =
                ((1.0 + entry.interval_days as f32).ln() / MAX_STABILITY_LN).min(1.0);
            let ease_scale = entry.ease_factor / 2.5;

            entry.mastery_score = (rep_factor * stability_factor * ease_scale).clamp(0.0, 1.0);
        }
    }

    pub fn get_weakest_concepts(&self, limit: usize) -> Vec<(&String, &ConceptMastery)> {
        let mut list: Vec<(&String, &ConceptMastery)> = self.concept_mastery.iter().collect();
        list.sort_by(|a, b| {
            a.1.mastery_score
                .partial_cmp(&b.1.mastery_score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.1.lapses.cmp(&a.1.lapses))
                .then_with(|| a.0.cmp(b.0))
        });
        list.into_iter().take(limit).collect()
    }

    pub fn get_concept_mastery_scores(&self) -> HashMap<String, f32> {
        self.concept_mastery
            .iter()
            .map(|(k, v)| (k.clone(), v.mastery_score))
            .collect()
    }
}
