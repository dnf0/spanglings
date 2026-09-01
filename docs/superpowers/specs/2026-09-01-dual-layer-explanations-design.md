# Design Specification: Dual-Layer Explanations (Plain English + Grammar Rule)

**Date**: 2026-09-01  
**Status**: DRAFT / UNDER REVIEW  
**Objective**: Enhance feedback across Spanglings (Arcade recaps, Drill hints, Blitz reviews, and TUI modals) with dual-layer explanations that pair an intuitive plain-English mental model with a precise grammatical derivation rule.

---

## 1. Problem Statement & Motivation
Currently, explanation strings across Spanglings lean heavily on academic/linguistic terminology (e.g., `Present subjunctive (tú) of 'saber' uses stem 'sep-': sepas`). While grammatically accurate, learners benefit immensely from immediately understanding the **practical communicative meaning / mental model** (why someone is saying this in English) before digesting the morphological grammar rule.

---

## 2. Architecture & Data Model

### 2.1 Core Explanation Struct / Fields
In `src/core/arcade.rs` and `src/core/generator.rs`:
- Extend `ArcadeItem`, `ShowdownSentence`, `SpecializedEngineSentence`, `DrillItem`, and `SentenceFrame` to support dual-layer explanation components:
  - `plain_english`: An intuitive, plain-English summary of the sentence's contextual meaning or mental model (e.g., `"Expresses a requirement ('need you to know') rather than an established fact"`).
  - `explanation` / `grammar_rule`: The precise structural grammar rule (e.g., `"Present subjunctive (tú) of 'saber' uses irregular stem 'sep-' ➔ sepas"`).

### 2.2 Serde & Backward Compatibility
In `src/cli/commands/arcade.rs`:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArcadeMistake {
    pub topic: String,
    pub trigger_sentence: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub explanation: String,
    #[serde(default)]
    pub plain_english: String,
}
```
If `plain_english` is empty or missing, formatters fall back gracefully to the single `💡 Rule / Why: <explanation>` format.

---

## 3. UI & CLI Presentation

### 3.1 CLI Arcade Review Block (`spanglings arcade`)
```text
  1. [subjunctive] Es fundamental que tú ____ la contraseña maestra.
     ✗ Your answer:    sabes
     ✓ Correct answer: sepas
     💡 Context / Meaning: Expresses a requirement ("need you to know") rather than an established fact.
     📐 Grammar Rule:      Present subjunctive (tú) of 'saber' uses irregular stem 'sep-' ➔ sepas.
```

### 3.2 TUI Arcade Recap Modal (`[x]` / `[F8]`)
```text
  1. [subjunctive] Es fundamental que tú ____ la contraseña maestra.
     ✗ Your: sabes   |   ✓ Correct: sepas
     💡 Context: Expresses a requirement ("need you to know"); uncertainty triggers virtual mode.
     📐 Rule:    Present subjunctive (tú) of 'saber' uses irregular stem 'sep-' ➔ sepas.
```

### 3.3 Interactive Drill & Blitz Prompts (`spanglings drill`, `spanglings blitz`)
When typing `?` or `hint`, or on incorrect attempt:
```text
💡 Context: Expresses a requirement ("need you to know"); uncertainty triggers virtual mode.
📐 Rule:    Present subjunctive (tú) of 'saber' uses irregular stem 'sep-' ➔ sepas.
```

---

## 4. Content Population & Enrichment Strategy
1. **Showdowns (16 Pairs, 183 Sentences)**:
   - Provide plain-English conversational cues (e.g. *Por vs Para*: `"Indicates motivation/gratitude ('thanks for your help')"` vs `"Denotes definitive deadline ('due by Friday')"`).
2. **Specialized Engines (5 Engines, 81 Sentences)**:
   - Regimen: `"Fixed preposition pair for 'to dream about'"`
   - Irregulars: `"Irregular preterite stem for completed past action"`
   - False Friends: `"'Éxito' means success; 'salida' is physical exit"`
   - Se Matrix: `"Involuntary accidental occurrence ('it slipped from me')"`
   - Connectors: `"Introduces a counter-argument ('however / nevertheless')"`
3. **Core Grammar Concept Frames (138 Frames)**:
   - Provide plain-English triggers (`"Expresses subjective doubt"`, `"Hypothetical contrary-to-fact scenario"`, `"Origin and essence"`).

---

## 5. Verification & Testing
- Unit tests validating `ArcadeMistake` serde with and without `plain_english`.
- Unit tests verifying CLI and TUI formatters with dual-layer and single-layer fallback explanations.
- Full regression test across all CLI and TUI test suites (`cargo test`).
