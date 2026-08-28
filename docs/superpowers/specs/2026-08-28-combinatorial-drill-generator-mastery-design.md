# Combinatorial Grammar Question Engine, Adaptive Weakness Drills & TUI Mastery Dashboard Design

**Author:** Antigravity  
**Date:** 2026-08-28  
**Status:** Approved  
**Branch:** `feat/adaptive-drills-and-mastery-dashboard`

---

## 1. Executive Summary

To provide Duolingo/Kwiziq-scale grammar practice (4,000+ unique questions) across all 24 `GrammarConcept`s in `spanglings`, this design introduces:
1. **Combinatorial Grammar Generator (`src/core/generator.rs`)**: A declarative, zero-error sentence frame generator with substitution slots for triggers, subjects, irregular verb stems, contrastive pairs, and regional addresses across all 24 grammar topics.
2. **Curriculum Exercise-to-Drill Extractor (`src/core/exercise.rs`)**: Extracts rapid-fire test assertions from all 354 exercises in the curriculum.
3. **Adaptive Weakness-Driven Drill Engine (`spanglings drill --weak` / `spanglings blitz --weak`)**: Prioritizes lowest-mastery concepts from `AppState::concept_mastery`, updates mastery scores live upon every user attempt, and prints post-session delta summaries.
4. **Interactive TUI Concept Mastery & Weakness Dashboard (`[m]`)**: Modal displaying real-time mastery bars (0–100%) for all 24 grammar concepts, with instant hotkeys to launch targeted micro-drills (`[d]`), weak-concept drills (`[w]`), or view cheat sheets (`[r]`).

---

## 2. Architecture & Data Structures

### 2.1 Combinatorial Question Generation (`src/core/generator.rs`)

```rust
pub struct SentenceFrame {
    pub topic: &'static str,
    pub formula_cue: &'static str,
    pub template: &'static str, // e.g. "{trigger} que {subject} ____ {complement}."
    pub target_verb: &'static str,
    pub conjugation_slot: &'static [(&'static str, &'static str, &'static str)], // (subject, target_form, explanation)
    pub triggers: &'static [&'static str],
    pub complements: &'static [&'static str],
}

impl SentenceFrame {
    pub fn generate(&self, rng: &mut impl rand::Rng) -> DrillItem { ... }
}
```

### 2.2 Coverage across all 24 `GrammarConcept`s
Each concept contains 6–12 parameterized frames yielding 150–250+ grammatical variations:
- `subjunctive`: WEIRDO triggers $\times$ 30 irregular/stem-changing verbs $\times$ 5 subjects = 1,500+ items.
- `por-para`: 20 cause/motive frames $\times$ 20 destination/deadline frames.
- `ser-estar`: 25 permanent identity/profession/characteristics vs temporary states/locations.
- `past`: completed preterite triggers (*ayer, de repente*) vs imperfect habit/background (*siempre, mientras*).
- `pronouns`: direct + indirect double clitic placement (*se lo di, dámelo, va a decírmelo*).
- `prepositions`: 30 verb régime frames (*soñar con, depender de, fijarse en, insistir en*).
- `accidental-se`: involuntary loss/slips (*se me cayó, se nos olvidó, se te perdieron*).
- `false-friends`: 20 deceptive cognate pairs (*actualmente, realizar, atender, pretender*).
- `voseo`: 20 Rioplatense present indicative & imperative forms (*vos tenés, vos sos, hacé*).
- `epistemic-conjecture`: future of probability frames (*serán las diez, tendrá hambre*).
- `clitic-doubling`: redundant object focus frames (*a Juan le entregué la carta*).
- `personal-a`: animate human direct objects vs inanimate accusatives.
- `gerund-rules`: adverbial simultaneous actions vs non-permitted adjectival gerunds.
- `adversatives`: *pero* vs *sino* vs *sino que* contrastive frames.
- `legal-subjunctive`: statutory formulas & archaic future subjunctive (*fuere, hubiere*).
- `verbs-of-becoming`: *hacerse, volverse, ponerse, quedarse, convertirse en*.
- `epistemic-adverbs`: mood choice with *quizás, tal vez, probablemente, acaso*.
- `possessive-datives`: inalienable possession with dative pronouns (*me lavo las manos*).
- `corrective-polarity`: negated main clause mood selection (*no digo que sea malo*).
- `participial-absolutes`: absolute participial clauses (*terminado el informe...*).
- `scalar-concession`: intensive concessives (*por más que, aun cuando, ni siquiera*).
- `tech-software`: dev workflows (*desplegar, compilar, refactorizar, depurar, alojar*).
- `business`: executive communication (*remitir, aplazar, acordar, encabezar*).
- `accents`: agudas, llanas, esdrújulas, and diacritical pairs (*él/el, tú/tu, té/te*).

---

## 3. Adaptive Weakness-Driven Engine (`--weak`)

### 3.1 Adaptive Question Sampling
```rust
pub fn sample_drill_items(
    count: usize,
    topic: Option<&str>,
    level: Option<Level>,
    track: Option<usize>,
    weak_only: bool,
    state: &AppState,
) -> Vec<DrillItem>
```
1. If `weak_only` is true:
   - Identify topics with `mastery_score < 0.75` or highest `lapses`.
   - Sample questions with weight $\propto (1.0 - \text{mastery\_score} + 0.1)$.
2. If `topic` is specified:
   - Filter questions exclusively from that `GrammarConcept` (by slug or intent).
3. If `level` or `track` is specified:
   - Filter questions extracted from matching curriculum exercises.

### 3.2 Real-time Live State Updates
During a drill session:
- First attempt correct (no hints) ➔ `quality = 5` (mastery increases by +0.3).
- Hints used ➔ `quality = 3` (mastery increases moderately).
- Incorrect answer / solution shown ➔ `quality = 1` (mastery reduced, lapse incremented).
- Live write to `state.json`.

---

## 4. TUI Concept Mastery & Weakness Dashboard (`[m]`)

### 4.1 Modal View & Navigation
- Keybinding `[m]` in normal mode opens `Concept Mastery & Weakness Profiler`.
- Overall score: `Mastery: 72% | 18 Mastered | 4 In Progress | 2 Needs Review`
- 24-Concept Table:
  - ` ▶ Subjunctive (wishes, hypotheses, doubt, demands)` | `[████████░░░░] 60%` | `rev: 14` | `lapses: 2`
  - Color coded: Green (>75%), Yellow (40–75%), Red (<40%).
- Actions:
  - `[d]`: Launch 5-question micro-drill for highlighted concept.
  - `[w]`: Launch adaptive weakness drill across weakest concepts.
  - `[Enter]` / `[r]`: View Reference Cheat Sheet.
  - `[Esc]` / `[q]`: Return to exercises.

---

## 5. Testing & Verification Plan

1. `tests/generator_tests.rs`:
   - Assert all 24 concepts have registered generation frames.
   - Assert generation produces non-empty sentences, target tokens, subjects, and explanations without template formatting artifacts (`{` or `}`).
   - Verify generation of 1,000 items without error or panic.
2. `tests/drill_tests.rs` & `tests/blitz_tests.rs`:
   - Test `--weak`, `--topic`, `--level`, `--track`, and `--count` flags.
   - Test live `concept_mastery` updates and post-session delta printing.
3. `tests/tui_tests.rs`:
   - Test `[m]` modal opening, list scrolling, mastery bar rendering, and action triggers.
4. `cargo test`, `cargo clippy`, `cargo fmt`.
