# Adaptive Weakness-Driven Drills, Dynamic Question Bank & TUI Concept Mastery Dashboard Design

**Author:** Antigravity  
**Date:** 2026-08-28  
**Status:** Approved  
**Branch:** `feat/adaptive-drills-and-mastery-dashboard`

---

## 1. Executive Summary

This design addresses the pedagogical limitation of small, static drill question pools by introducing:
1. **Curriculum-Wide Drill Pool & Expanded Bank (500+ Questions)**: Combining an expanded, hand-crafted 150+ rapid-fire question bank (6–8 per concept across all 24 `GrammarConcept`s) with dynamic exercise-to-drill extraction from all 354 exercises.
2. **Adaptive Weakness-Driven Drill Engine (`spanglings drill --weak` / `spanglings blitz --weak`)**: Intelligently querying `AppState::concept_mastery`, sampling questions using inverse-mastery weights, updating mastery scores live after every attempt, and presenting delta summaries.
3. **Flexible CLI Filtering**: Support for `--weak`, `--topic <slug|intent>`, `--level <cefr>`, `--track <1-59>`, and `--count <n>` flags.
4. **Interactive TUI Mastery & Weakness Dashboard (`[m]`)**: A dedicated modal displaying real-time mastery percentage bars across all 24 grammar concepts, lapse counters, and instant hotkey actions to launch targeted drills or view cheat sheets.

---

## 2. Architecture & Data Structures

### 2.1 Expanded `DrillItem` & Question Bank
In `src/core/reference.rs` / `src/cli/commands/drill.rs`:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillItem {
    pub topic: &'static str,
    pub formula_cue: &'static str,
    pub trigger_sentence: &'static str,
    pub target_verb: &'static str,
    pub target_subject: &'static str,
    pub target: &'static str,
    pub explanation: &'static str,
}
```

### 2.2 Question Bank Expansion
- Provide dedicated arrays of 6–8 curated questions for **every single one of the 24 `GrammarConcept`s** (160+ static rapid-fire items):
  - `subjunctive` (wishes, emotions, doubt, impersonal, ojalá, conjunctions)
  - `por-para` (duration, cause, destination, recipient, deadline, comparison)
  - `ser-estar` (identity, origin, profession, temporary states, locations, conditions)
  - `past` (completed preterite vs ongoing imperfect, interruptions, habitual past)
  - `pronouns` (direct/indirect stacking, se lo substitution, imperative enclisis)
  - `prepositions` (régimen: *soñar con, depender de, fijarse en, insistir en, tardar en*)
  - `accidental-se` (*caer, olvidar, romper, perder, acabar*)
  - `tech-software` (desplegar, compilar, alojar, refactorizar, depurar)
  - `business` (acordar, negociar, aplazar, remitir, formal address)
  - `false-friends` (*actualmente, realizar, atender, pretender, constipado*)
  - `voseo` (*vos tenés, vos sos, vos podés, vos querés, vos sabés*)
  - `accents` (agudas, llanas, esdrújulas, diacritical: *dé/de, él/el, más/mas, sí/si*)
  - `epistemic-conjecture` (future of probability: *serán las tres, tendrá hambre*)
  - `clitic-doubling` (*le di el libro a Juan, a ella le gusta*)
  - `personal-a` (human direct objects vs inanimate objects)
  - `gerund-rules` (simultaneous actions vs forbidden adjectival gerunds)
  - `adversatives` (*pero* vs *sino* vs *sino que*)
  - `legal-subjunctive` (statutory formulas & future subjunctive)
  - `verbs-of-becoming` (*hacerse, volverse, ponerse, quedarse, convertirse en*)
  - `epistemic-adverbs` (mood selection with *quizás, tal vez, probablemente*)
  - `possessive-datives` (inalienable possession: *me lavo las manos, se le cayó el pelo*)
  - `corrective-polarity` (*no es que... sino que...*)
  - `participial-absolutes` (absolute constructions: *terminada la reunión...*)
  - `scalar-concession` (*por más que, aun cuando, ni siquiera*)

### 2.3 Dynamic Exercise-to-Drill Transformer
In `src/core/exercise.rs`:
```rust
impl Exercise {
    pub fn to_drill_items(&self) -> Vec<DrillItem>;
}
```
Extracts the target sentence, prompt context, and expected solution token directly from the exercise's markdown tests and instructions, tagging them with the exercise's CEFR level and track number.

---

## 3. Adaptive Weakness-Driven Engine (`--weak`)

### 3.1 Adaptive Question Selection
When `--weak` is passed:
1. Load `AppState`.
2. Inspect `state.concept_mastery` for all 24 concepts. Unpracticed concepts default to `0.0` mastery.
3. Filter drill items matching the weakest concepts (mastery < 0.75, or top 5 lowest scores/highest lapses).
4. Shuffle and take `count` items (default: 5).

### 3.2 Real-time Live State Updating
As the learner proceeds through each drill question:
- If answered correctly on attempt 1 without hints: `quality = 5`.
- If answered with hints: `quality = 3`.
- If failed or revealed solution: `quality = 1`.
- Call `state.update_concept_mastery(item.topic, quality, now)` and save `state.json`.

### 3.3 Post-Session Mastery Summary
Displays session deltas:
```
📊 Mastery Progress:
  • Subjunctive:            40% ➔ 65% (+25%)
  • Por vs. Para:           70% ➔ 85% (+15%)
```

---

## 4. TUI Concept Mastery & Weakness Dashboard Modal (`[m]`)

### 4.1 UI Layout
- **Modal Header**: `Spanglings Concept Mastery & Weakness Profiler`
- **Curriculum Overview**: Overall Mastery percentage bar and breakdown of Mastered / In Progress / Needs Review topics.
- **24-Concept Table / List**:
  - Selection cursor: ` ▶ `
  - Concept Title + Communicative Gloss: `Subjunctive (wishes, hypotheses, doubt, demands)`
  - Progress Bar: `[████████░░░░░░] 55%` (Color-coded: Green >75%, Yellow 40-75%, Red <40%)
  - Reviews count (`rev: 12`), Lapses count (`lapses: 2`), and Last Practiced relative time (`2h ago`).
- **Footer Keybindings**:
  - `[↑/↓] [j/k]`: Navigate concepts
  - `[Enter] / [r]`: Open Concept Reference Cheat Sheet
  - `[d]`: Launch instant 5-question micro-drill for selected topic
  - `[w]`: Launch adaptive Weakness Drill (top 5 weakest concepts)
  - `[Esc] / [q]`: Close dashboard

---

## 5. Testing & Verification Strategy

1. **Unit Tests**:
   - `test_all_24_concepts_have_comprehensive_drill_bank`: asserts each of the 24 concepts has ≥6 drill items in the bank.
   - `test_exercise_to_drill_items_conversion`: verifies exercise conversion logic.
   - `test_adaptive_weakness_selection_prioritizes_low_scores`: verifies weak topics are sampled with highest priority.
   - `test_drill_updates_concept_mastery_live`: verifies state updates and persistence.
2. **TUI Tests**:
   - `test_tui_mastery_modal_navigation_and_rendering`: tests `[m]` keybinding, layout rendering, and event dispatch.
3. **CLI Integration Tests**:
   - `test_cli_drill_weak_flag_filtering`
   - `test_cli_drill_level_and_topic_filtering`
