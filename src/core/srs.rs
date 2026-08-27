use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrsItem {
    pub repetitions: u32,
    pub interval_days: u32,
    pub ease_factor: f32,
    pub next_review_due: DateTime<Utc>,
    pub last_reviewed: Option<DateTime<Utc>>,
}

impl SrsItem {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            repetitions: 0,
            interval_days: 0,
            ease_factor: 2.5,
            next_review_due: now,
            last_reviewed: None,
        }
    }
}

impl Default for SrsItem {
    fn default() -> Self {
        Self {
            repetitions: 0,
            interval_days: 0,
            ease_factor: 2.5,
            next_review_due: DateTime::UNIX_EPOCH,
            last_reviewed: None,
        }
    }
}

pub fn calculate_sm2_review(item: &SrsItem, quality: u8, now: DateTime<Utc>) -> SrsItem {
    let q = quality.clamp(0, 5);
    let q_f32 = q as f32;
    let mut new_ef = item.ease_factor + (0.1 - (5.0 - q_f32) * (0.08 + (5.0 - q_f32) * 0.02));
    if new_ef < 1.3 {
        new_ef = 1.3;
    }

    let (new_reps, new_interval) = if q < 3 {
        (0, 1)
    } else {
        match item.repetitions {
            0 => (1, 1),
            1 => (2, 6),
            n => {
                let interval = ((item.interval_days as f32) * new_ef).round() as u32;
                (n + 1, interval.clamp(1, 3650))
            }
        }
    };

    let next_due = now
        .checked_add_signed(Duration::days(new_interval as i64))
        .unwrap_or(DateTime::<Utc>::MAX_UTC);

    SrsItem {
        repetitions: new_reps,
        interval_days: new_interval,
        ease_factor: new_ef,
        next_review_due: next_due,
        last_reviewed: Some(now),
    }
}
