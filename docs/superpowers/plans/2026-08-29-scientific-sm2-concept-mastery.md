# Scientific SM-2 Interval & Stability Concept Mastery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the coarse, flat +30% concept mastery formula with a scientifically grounded SuperMemo SM-2 Interval & Memory Stability model.

**Architecture:**
1. Extend `ConceptMastery` with `repetitions`, `interval_days`, and `ease_factor` (with serde defaults).
2. Implement true SM-2 Ease Factor adaptation and interval progression inside `update_concept_mastery`.
3. Calculate `mastery_score` using a composite of Memory Stability ($\ln(1+I)/\ln(61)$), Repetition Strength ($\min(1, \text{reps}/6)$), and Ease Scale ($EF/2.5$).
4. Update tests to verify calibrated multi-step progression, speed sensitivity, and lapse penalties.

**Tech Stack:** Rust (2021 edition), `chrono`, `serde`, `serde_json`.

---

### Task 1: Core SM-2 Data Model & Scientific Mastery Engine (`src/core/state.rs`, `tests/weakness_profiler_tests.rs`)

**Files:**
- Modify: `src/core/state.rs:25-45,205-240`
- Test: `tests/weakness_profiler_tests.rs`

- [ ] **Step 1: Write failing test in `tests/weakness_profiler_tests.rs`**
```rust
#[test]
fn test_scientific_sm2_concept_mastery_progression() {
    let mut state = AppState::default();
    let now = Utc::now();

    // 1st review (fast, quality 5): ~3%
    state.update_concept_mastery("por-para", 5, now);
    let m1 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m1.repetitions, 1);
    assert_eq!(m1.interval_days, 1);
    assert!(m1.mastery_score > 0.02 && m1.mastery_score < 0.05);

    // 2nd review (quality 5): ~17%
    state.update_concept_mastery("por-para", 5, now);
    let m2 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m2.repetitions, 2);
    assert_eq!(m2.interval_days, 6);
    assert!(m2.mastery_score > 0.14 && m2.mastery_score < 0.20);

    // 3rd review (quality 5): ~38%
    state.update_concept_mastery("por-para", 5, now);
    let m3 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m3.repetitions, 3);
    assert!(m3.mastery_score > 0.30 && m3.mastery_score < 0.45);

    // Lapse on mistake (quality 1): resets score to 0 and increments lapse count
    state.update_concept_mastery("por-para", 1, now);
    let m_lapse = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m_lapse.repetitions, 0);
    assert_eq!(m_lapse.interval_days, 1);
    assert_eq!(m_lapse.lapses, 1);
    assert_eq!(m_lapse.mastery_score, 0.0);
}
```

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test --test weakness_profiler_tests test_scientific_sm2_concept_mastery_progression`
Expected: FAIL.

- [ ] **Step 3: Implement `ConceptMastery` SM-2 fields and algorithm in `src/core/state.rs`**
```rust
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
```
Update `update_concept_mastery`:
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
            entry.repetitions = 0;
            entry.interval_days = 1;
            entry.mastery_score = 0.0;
        } else {
            entry.repetitions += 1;
            entry.interval_days = match entry.repetitions {
                1 => 1,
                2 => 6,
                _ => ((entry.interval_days as f32) * new_ef).round() as u32,
            }.clamp(1, 3650);

            let rep_factor = (entry.repetitions as f32 / 6.0).min(1.0);
            let stability_factor = ((1.0 + entry.interval_days as f32).ln() / 61.0_f32.ln()).min(1.0);
            let ease_scale = entry.ease_factor / 2.5;

            entry.mastery_score = (rep_factor * stability_factor * ease_scale).clamp(0.0, 1.0);
        }
    }
```

- [ ] **Step 4: Run unit tests**
Run: `cargo test --test weakness_profiler_tests`
Expected: PASS.

- [ ] **Step 5: Commit changes**
```bash
git add src/core/state.rs tests/weakness_profiler_tests.rs
git commit --no-gpg-sign -m "feat(core): implement scientific SM-2 interval and stability concept mastery model"
```

---

### Task 2: Update Existing Tests & Verify End-to-End (`tests/weakness_profiler_tests.rs`, `tests/cli_arcade_tests.rs`)

**Files:**
- Modify: `tests/weakness_profiler_tests.rs`
- Modify: `tests/cli_arcade_tests.rs`

- [ ] **Step 1: Update legacy test expectations in `tests/weakness_profiler_tests.rs`**
Update `test_concept_mastery_initialization_and_success_updates` and `test_concept_mastery_lapse_penalization_and_recovery` to align with the new SM-2 stability equations.

- [ ] **Step 2: Run all tests in the workspace**
Run: `cargo test`
Expected: PASS.

- [ ] **Step 3: Commit changes**
```bash
git add tests/weakness_profiler_tests.rs
git commit --no-gpg-sign -m "test(core): align weakness profiler tests with scientific SM-2 mastery equations"
```

---

### Task 3: Complete Verification, Formatting & Release Polish

- [ ] **Step 1: Run full test suite across all 20 packages**
Run: `cargo test`
- [ ] **Step 2: Run clippy and format checks**
Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`
- [ ] **Step 3: Update knowledge graph**
Run: `uvx --from graphifyy graphify update .`
- [ ] **Step 4: PR & Merge into main**
