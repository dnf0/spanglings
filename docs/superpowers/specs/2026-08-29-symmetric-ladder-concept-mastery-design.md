# Design Specification: Symmetric Step-Inversion Ladder Concept Mastery

**Date:** 2026-08-29  
**Status:** Approved  
**Target:** `src/core/state.rs`, `tests/weakness_profiler_tests.rs`

---

## 1. Concept Summary
Replaces the harsh 0% lapse hard-reset with a symmetric 1-step rollback ladder inspired by the modern Leitner Box model:
- **Correct answer ($q \ge 3$)**: Advance +1 step on the SM-2 mastery ladder ($R = R + 1$).
- **Incorrect answer / lapse ($q < 3$)**: Step back -1 step on the SM-2 mastery ladder ($R = R.\text{saturating\_sub}(1)$).
- **Ease Factor adjustment**: On lapses, $EF$ decreases smoothly ($EF = \max(1.3, EF - 0.2)$) to capture cognitive friction and prioritize the concept for `spanglings arcade -w`.

---

## 2. Mathematical Progression Ladder

| Ladder Step ($R$) | Interval ($I$) | Mastery Formula ($\text{rep} \times \text{stability} \times \text{ease}$) | Mastery Score |
| :--- | :--- | :--- | :--- |
| **0** | 0 days | $0.0$ | **0.0%** |
| **1** | 1 day | $(1/6) \times (\ln(2)/\ln(61)) \times (EF/2.5)$ | **~3.0%** |
| **2** | 6 days | $(2/6) \times (\ln(7)/\ln(61)) \times (EF/2.5)$ | **~17.0%** |
| **3** | 16 days | $(3/6) \times (\ln(17)/\ln(61)) \times (EF/2.5)$ | **~38.0%** |
| **4** | 45 days | $(4/6) \times (\ln(46)/\ln(61)) \times (EF/2.5)$ | **~72.0%** |
| **5+** | 120+ days | $(5/6) \times (\ln(121)/\ln(61)) \times (EF/2.5)$ | **~100.0%** |

---

## 3. Algorithm Details (`src/core/state.rs`)

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
        entry.lapses += 1;
        // Symmetric 1-step ladder rollback
        entry.repetitions = entry.repetitions.saturating_sub(1);
        entry.interval_days = match entry.repetitions {
            0 => 0,
            1 => 1,
            2 => 6,
            _ => ((entry.interval_days as f32) / entry.ease_factor).round() as u32,
        }.clamp(0, 3650);
    } else {
        // Advance +1 step
        entry.repetitions += 1;
        entry.interval_days = match entry.repetitions {
            1 => 1,
            2 => 6,
            _ => ((entry.interval_days as f32) * new_ef).round() as u32,
        }.clamp(1, 3650);
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
```

---

## 4. Verification Requirements
- `tests/weakness_profiler_tests.rs`:
  - Advance to Step 3 (~38%).
  - On 1 mistake, assert rollback to Step 2 (~17%) with lapse counter incremented to 1.
  - On 2nd mistake, assert rollback to Step 1 (~3%) with lapse counter incremented to 2.
  - On 3rd mistake, assert rollback to Step 0 (0.0%) with lapse counter incremented to 3.
  - On subsequent correct answer, assert advance back to Step 1 (~3%).
