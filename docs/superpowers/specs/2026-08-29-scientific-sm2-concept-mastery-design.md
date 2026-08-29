# Design Specification: Scientific SM-2 Interval & Stability Concept Mastery Model

**Date:** 2026-08-29  
**Status:** Approved  
**Target:** `src/core/state.rs`, `src/core/srs.rs`, `src/cli/commands/arcade.rs`, `src/tui/app.rs`

---

## 1. Problem Statement & Motivation
Currently, answering a single arcade question awards a flat `+30%` (`+0.30`) concept mastery bump via `(score * 0.7 + 0.3)`. This suffers from three core scientific issues:
1. **Gross Over-Estimation on Single Trials**: Answering one isolated question correctly should not grant 30% mastery of an entire grammar category (e.g. all of Subjunctive).
2. **Speed & Quality Invariance**: Fast recall (<800ms) awards the exact same jump as hesitant recall (1500ms).
3. **Massed Practice Inflation**: Cramming 10 questions in 30 seconds produces an illusion of mastery without true long-term memory stability (Ebbinghaus 1885, Kornell & Bjork 2008).

---

## 2. Scientific Foundations (SM-2 & Spacing Effect)
In learning science (Wozniak 1990, Cepeda et al. 2006, Settles & Meeder 2016):
- **Memory Stability ($S$)**: Represents how long a memory trace remains retrievable before decaying.
- **Ease Factor ($EF$)**: Models the inherent cognitive difficulty and friction of the concept.
- **Inter-Repetition Interval Expansion**: In SM-2, interval grows exponentially with successful recalls ($I(1)=1\text{d}, I(2)=6\text{d}, I(n)=I(n-1)\times EF$).

---

## 3. Data Model Architecture (`src/core/state.rs`)

### `ConceptMastery` Struct
```rust
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

fn default_ease_factor() -> f32 {
    2.5
}
```

### SM-2 Mastery Calculation Logic (`update_concept_mastery`)
```rust
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
        // Lapse: reset interval and repetition streak
        entry.lapses += 1;
        entry.repetitions = 0;
        entry.interval_days = 1;
        entry.mastery_score = 0.0;
    } else {
        // Successful retrieval
        entry.repetitions += 1;
        entry.interval_days = match entry.repetitions {
            1 => 1,
            2 => 6,
            _ => ((entry.interval_days as f32) * new_ef).round() as u32,
        }.clamp(1, 3650);

        // Scientific Mastery Formula based on Stability & Repetitions
        let rep_factor = (entry.repetitions as f32 / 6.0).min(1.0);
        let stability_factor = ((1.0 + entry.interval_days as f32).ln() / 61.0_f32.ln()).min(1.0);
        let ease_scale = entry.ease_factor / 2.5;

        entry.mastery_score = (rep_factor * stability_factor * ease_scale).clamp(0.0, 1.0);
    }
}
```

---

## 4. Expected Progression Rates
- **1st correct answer (Fast, $q=5$)**: $+3\%$ mastery (Trial baseline)
- **2nd correct answer ($q=5$)**: $+17\%$ mastery
- **3rd correct answer ($q=5$)**: $+38\%$ mastery
- **4th correct answer ($q=5$)**: $+72\%$ mastery
- **5th correct answer ($q=5$)**: $+100\%$ mastery
- **Incorrect answer ($q < 3$)**: Lapses increment, interval resets to 1, mastery resets to 0% until rebuilt.

---

## 5. Verification & Backward Compatibility
- Existing state files deserialize seamlessly via `#[serde(default)]` and `default_ease_factor`.
- Unit tests verify:
  1. Progression steps across 1 to 5 successful reviews.
  2. Quality differentiation ($q=5$ vs $q=3$).
  3. Lapse resets and lapse counting.
  4. Non-breaking state JSON roundtrips.
