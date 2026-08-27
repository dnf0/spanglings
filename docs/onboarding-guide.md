# Onboarding & Learner's Guide

Welcome to **Spanglings**! This guide takes you on a comprehensive tour of the platform's architecture, learning loops, compiler diagnostics, and power tools.

---

## 🧭 The 6-Station Guided Tour

Spanglings includes an interactive guided tour designed to introduce core workflows in under 2 minutes:

```bash
spanglings tour
```

```
════════════════════════════════════════════════════════════════
 ✦ [Station 1/6] The Spanglings Philosophy & Workflow
════════════════════════════════════════════════════════════════
Spanglings is designed for developers learning Spanish syntax
through real problem-solving, immediate compiler-style feedback,
and spaced repetition.

  • Work in your real editor (VS Code, Neovim, Zed) alongside 'spanglings watch'
  • No busywork: Exercises are evaluated on pure correctness
  • Zero comment-deletion mechanics: Fix the prompt and save!
```

### The 6 Interactive Stations

1. **Philosophy & Core Loop**: The developer-first mental model, instantaneous validation, and zero comment-deletion friction.
2. **Exercise Anatomy & Accent Handling**: Understanding inline `___` blanks, UTF-8 Spanish punctuation, and forgiving accent matching.
3. **Rustc-Style Grammar Diagnostics**: Reading compiler-grade errors, dynamic carets, linguistic concept links, and contrast notes.
4. **Progressive Hints & Cheat Sheets**: Accessing 3-tier progressive hints (`[h]` / `F1`) and on-demand grammar cheat sheets (`[e]` / `F2`).
5. **Power Tools & Placement Fast-Track**: Using the in-TUI Verb Conjugator (`F3`), Reference Sheet Browser (`F4`), and Calibrated CEFR Placement Test (`F5` / `[p]`).
6. **Modern Watcher & Editor Workflows**: Running `spanglings watch` with non-blocking keys (`[n]`, `[p]`, `[r]`, `[q]`) and configuring Language Server Protocol (LSP) integrations.

---

## 📝 Exercise Anatomy & Cloze Mechanics

Every exercise in Spanglings is a structured Markdown file located in `exercises/<track_name>/<exercise_id>.md`:

```markdown
# Adverbial Conjunctions of Time (En cuanto / Tan pronto como)
- Level: B1
- Topic: Subjunctive Conjunctions
- Concepts: subjunctive_temporal_conjunctions, present_subjunctive_regular
- Prerequisites: present_indicative_regular

## Prompt
Fill in the correct form of the verb in parentheses.
Context: Future action dependent on a temporal conjunction.

Te llamaré tan pronto como yo ___ (llegar) a la oficina.

## Hints
<!-- Tier 1: Conjunctions of anticipated future time require subjunctive. -->
<!-- Tier 2: First-person singular present subjunctive of llegar (g -> gu). -->
<!-- Tier 3: llegue -->
```

### How to Solve an Exercise
1. Read the **Context** and **Prompt**.
2. Replace `___ (llegar)` with your conjugated Spanish answer: `llegue`.
3. Save the file. Spanglings will evaluate your submission immediately.

---

## 🧠 Smart Forgiving Accent Matching

Spanish requires written accent marks (*tildes*) for stress differentiation and grammatical distinctions (e.g., *hablo* vs *habló*, *si* vs *sí*, *esta* vs *está*).

### Flexible Default Mode
By default, Spanglings is forgiving to accommodate standard QWERTY keyboards:
- Entering `llego` when `llegó` is expected will pass with a helpful pedagogical note:
  > `[Notice]: Good job! Remember to include the written accent: llegó`
- Inverted punctuation (`¿`, `¡`) is optional by default.

### Strict Accent Enforcement
When preparing for formal examinations or practicing exact orthography, pass the `--strict-accents` flag:

```bash
spanglings watch --strict-accents
spanglings --strict-accents
```

---

## 🔍 Rustc-Style Compiler Diagnostics

When an exercise fails or contains a common grammatical trap, Spanglings generates colored, rustc-style diagnostics:

```
error[E0301]: Subjunctive Mood Required
  --> exercises/03_subjunctive_weirdo/01_wishes_volition.md:8:23
   |
 8 | Mis padres quieren que yo estudio informática.
   |                           ^^^^^^^ expected subjunctive 'estudie', found present indicative 'estudio'
   |
   = note: Verbs of volition (querer, desear, esperar) require the subjunctive in subordinate clauses when subjects differ.
   = concept: subjunctive_wishes_desires (prerequisite: present_indicative_irregular)
   = contrast: 'Quiero estudiar' (same subject -> infinitive) vs 'Quiero que estudies' (different subjects -> subjunctive)
```

### Anatomy of a Diagnostic
- **Error Code**: Structured error classification (e.g., `E0101` Ser vs Estar, `E0201` Aspectual Preterite Shift, `E0301` Subjunctive Trigger).
- **Source Context**: Precise line numbers and dynamic `^^^^` caret highlighting.
- **Linguistic Concept**: Linked concept in the 53-node DAG ontology and foundational prerequisites.
- **Contrast Note**: Actionable comparison clarifying frequent misconceptions.

---

## 💡 3-Tier Progressive Hint System

Never get stuck on an exercise. Spanglings provides progressive hints on demand:

```bash
# Reveal progressive hints via CLI
spanglings hint exercises/03_subjunctive_weirdo/01_wishes_volition.md
```

- **Tier 1 (Conceptual Clue)**: Explains the underlying grammatical rule without hinting at the verb root.
- **Tier 2 (Morphological Clue)**: Identifies stem shifts, orthographic changes (e.g., *c → qu*, *z → c*, *g → gu*), or pronoun placement rules.
- **Tier 3 (Solution Reveal)**: Full solution with grammatical breakdown.

In watch mode, press `h` to reveal hints sequentially. In the interactive TUI, press `Ctrl+H` or `F1`.

---

## 🛠️ Power Modals & In-Terminal Reference Tools

The interactive TUI features several non-disruptive popups accessible via global hotkeys:

### 1. In-TUI Verb Conjugator (`F3` / `Ctrl+K`)
Look up full conjugation matrices across all indicative and subjunctive tenses without leaving the exercise workspace.

### 2. In-TUI Grammar Reference Sheet Browser (`F4` / `Ctrl+B`)
Full-text searchable access to all 12 grammar reference cards (Subjunctive, Por vs Para, Clitics, Accents, Prepositional Regimes, False Friends, Voseo, etc.).

### 3. Diagnostic Placement & Level Fast-Track (`F5` / `[p]`)
Interactive 15-question diagnostic test modal evaluating CEFR proficiency and automatically fast-tracking mastered tiers with `[F]`.

### 4. Interactive Guided Tour (`F6` / `[T]`)
Launch the 6-station onboarding tour at any time directly within the TUI.
