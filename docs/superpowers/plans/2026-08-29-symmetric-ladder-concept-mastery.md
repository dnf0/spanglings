# Symmetric Step-Inversion Ladder Concept Mastery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the symmetric 1-step rollback ladder on lapses in `update_concept_mastery`, replacing the 0% hard-reset with smooth, fair Leitner-style step degradation.

**Architecture:**
- On success ($q \ge 3$): $R = R + 1$, expand interval, recalculate stability score.
- On lapse ($q < 3$): $R = R.\text{saturating\_sub}(1)$, shrink interval, adjust $EF$, recalculate stability score.
- If $R=0$, score is $0.0\%$.

**Tech Stack:** Rust (2021 edition), `chrono`, `serde`.

---

### Task 1: Implement Symmetric Ladder in `src/core/state.rs` & Tests (`tests/weakness_profiler_tests.rs`)

**Files:**
- Modify: `src/core/state.rs:220-270`
- Test: `tests/weakness_profiler_tests.rs`

- [ ] **Step 1: Write failing test in `tests/weakness_profiler_tests.rs`**
```rust
#[test]
fn test_symmetric_ladder_progression_and_step_rollback() {
    let mut state = AppState::default();
    let now = Utc::now();

    // 1st review (q=5): Step 1 (~3%)
    state.update_concept_mastery("por-para", 5, now);
    let m1 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m1.repetitions, 1);
    assert_eq!(m1.interval_days, 1);
    assert!(m1.mastery_score > 0.02 && m1.mastery_score < 0.05);

    // 2nd review (q=5): Step 2 (~17%)
    state.update_concept_mastery("por-para", 5, now);
    let m2 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m2.repetitions, 2);
    assert_eq!(m2.interval_days, 6);
    assert!(m2.mastery_score > 0.14 && m2.mastery_score < 0.20);

    // 3rd review (q=5): Step 3 (~38%)
    state.update_concept_mastery("por-para", 5, now);
    let m3 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m3.repetitions, 3);
    assert!(m3.mastery_score > 0.30 && m3.mastery_score < 0.45);

    // 1st lapse (q=1): Steps back to Step 2 (~17%), NOT 0%
    state.update_concept_mastery("por-para", 1, now);
    let m_lapse1 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m_lapse1.repetitions, 2);
    assert_eq!(m_lapse1.lapses, 1);
    assert!(m_lapse1.mastery_score > 0.10 && m_lapse1.mastery_score < 0.20);

    // 2nd lapse (q=1): Steps back to Step 1 (~3%)
    state.update_concept_mastery("por-para", 1, now);
    let m_lapse2 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m_lapse2.repetitions, 1);
    assert_eq!(m_lapse2.lapses, 2);
    assert!(m_lapse2.mastery_score > 0.01 && m_lapse2.mastery_score < 0.05);

    // 3rd lapse (q=1): Steps back to Step 0 (0%)
    state.update_concept_mastery("por-para", 1, now);
    let m_lapse3 = state.concept_mastery.get("por-para").unwrap();
    assert_eq!(m_lapse3.repetitions, 0);
    assert_eq!(m_lapse3.lapses, 3);
    assert_eq!(m_lapse3.mastery_score, 0.0);
}
```

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test --test weakness_profiler_tests test_symmetric_ladder_progression_and_step_rollback`

- [ ] **Step 3: Implement symmetric 1-step rollback in `src/core/state.rs`**
Update `update_concept_mastery`:
```rust
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
```

- [ ] **Step 4: Run unit tests**
Run: `cargo test --test weakness_profiler_tests`

- [ ] **Step 5: Commit changes**
```bash
git add src/core/state.rs tests/weakness_profiler_tests.rs
git commit --no-gpg-sign -m "feat(core): implement symmetric step ladder progression and rollback on lapses"
```

---

### Task 2: Full Workspace Verification, Graphify & PR Integration

- [ ] **Step 1: Run full test suite**
Run: `cargo test`
- [ ] **Step 2: Run clippy and format checks**
Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`
- [ ] **Step 3: Update knowledge graph**
Run: `uvx --from graphifyy graphify update .`
- [ ] **Step 4: PR & Merge into main**
