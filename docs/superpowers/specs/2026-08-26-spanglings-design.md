# Spanglings Design Specification

**Date:** 2026-08-26  
**Status:** Approved  
**Author:** Pair Programming (Antigravity & User)

---

## 1. Executive Summary & Philosophy

### 1.1 Problem Statement
Mainstream language learning tools (like Duolingo) are agonizingly slow, repetitive, infantile, and cluttered with gamification traps (animations, ads, gems, hearts). For intermediate-to-advanced learners (B1, B2, C1), these platforms fail to deliver high-density deliberate practice on complex syntactic and grammatical structures where learners actually plateau:
- Subjunctive triggers, nuances, and sequence of tenses
- Preterite vs. imperfect aspectual differences
- Object pronoun stacking with reflexives and accent shifts
- *Por* vs. *para* distinctions
- Prepositional verb regimes (*soñar con*, *acordarse de*)
- Involuntary/accidental *se* constructions
- Advanced connectors and subjunctive idioms (*hagas lo que hagas*, *de ahí que*)

### 1.2 The Solution: Spanglings
Inspired by [Rustlings](https://github.com/rust-lang/rustlings), **Spanglings** brings developer-grade learning to Spanish:
- **Zero Fluff / High Density:** No cartoon animations or wait timers. Just pure language exercises, rules, and instant terminal feedback.
- **Sub-millisecond Feedback Loop:** Instant evaluation when you edit an exercise or submit an answer.
- **Dual Execution Modes:**
  1. **Interactive TUI:** Full-screen terminal UI (`ratatui`) for rapid-fire drills, inline editing, and split-pane explanations.
  2. **File Watcher Mode:** Background watcher (`spanglings watch`) that checks your edits in your preferred editor (VS Code, Neovim, Emacs) upon file save.
- **Compiler-Grade Diagnostics:** Pinpoints exact linguistic errors (mood mismatch, wrong aspect, incorrect preposition, clitic ordering) with code snippets, carets, and explanations.
- **Keyboard-Friendly & Smart Accents:** Seamlessly usable on standard QWERTY keyboards. Normalizes inverted punctuation (`¿`, `¡`) and provides a *Forgiving with Warnings* accent system by default.

---

## 2. Architecture & System Design

```
┌────────────────────────────────────────────────────────────────────────┐
│                         spanglings CLI (Rust)                          │
├───────────────────────────────────┬────────────────────────────────────┤
│      Mode A: Interactive TUI      │        Mode B: File Watcher        │
│       (ratatui + crossterm)       │          (notify + stdout)         │
│  - Split-pane drill UI            │  - Watches exercises/ dir          │
│  - In-terminal editor & prompt    │  - Evaluates on file save          │
│  - Keyboard shortcuts (H/N/P/D)   │  - Prints rustc-style diagnostics  │
│  - Rapid-fire drill mode          │  - Auto-advances to next exercise  │
└─────────────────┬─────────────────┴──────────────────┬─────────────────┘
                  │                                    │
                  ▼                                    ▼
       ┌──────────────────────────────────────────────────────┐
       │                Core Engine & Runtime                 │
       │  - Exercise Loader (Filesystem & Embedded assets)    │
       │  - Diagnostic & Linguistic Rule Evaluator            │
       │  - Accent & Normalization Engine                     │
       │  - Progress Tracker (~/.config/spanglings/state.json)│
       └──────────────────────────────────────────────────────┘
```

### 2.1 CLI Interface & Commands

| Command | Description |
|---|---|
| `spanglings` | Launches the interactive TUI starting at the next incomplete exercise. |
| `spanglings watch` | Starts the headless file-watcher mode, monitoring `exercises/`. |
| `spanglings run <name>` | Runs validation on a specific exercise (e.g. `spanglings run subjunctive_01`). |
| `spanglings hint <name?>` | Displays progressive grammatical hints (Tiers 1, 2, or 3). |
| `spanglings drill [topic]` | Launches rapid-fire irregular conjugation / vocab baseline drills. |
| `spanglings list` | Displays all curriculum modules, exercises, and completion status. |
| `spanglings progress` | Summarizes CEFR level mastery (A1-Baseline, B1, B2, C1). |
| `spanglings reset <name>` | Resets an exercise back to its initial prompt state. |

### 2.2 Configuration & State Persistence
- Progress is stored in JSON format at `~/.config/spanglings/state.json` (with local fallback `.spanglings_state.json`).
- Schema:
  ```json
  {
    "version": 1,
    "completed_exercises": ["b1_ser_estar_01", "b1_subjunctive_01"],
    "current_exercise": "b1_subjunctive_02",
    "accent_mode": "forgiving",
    "exercise_stats": {
      "b1_subjunctive_01": {
        "attempts": 2,
        "completed_at": "2026-08-26T14:30:00Z",
        "hints_used": 1
      }
    }
  }
  ```

---

## 3. Linguistic & Diagnostic Engine

### 3.1 Accent & Punctuation Handling

To eliminate friction for developers using US/UK QWERTY keyboards while still teaching correct orthography:

1. **Punctuation Normalization:**
   - Leading inverted punctuation (`¿`, `¡`) and trailing punctuation (`.`, `?`, `!`, `,`) are optional.
   - Example: `¿Cómo estás?` matches `como estas`, `cómo estás`, or `¿Cómo estás?`.

2. **Smart Accent Modes:**
   - **`forgiving` (Default):** The engine strips diacritics for core grammatical correctness. If the word/conjugation is correct but missing an accent (e.g., `gustaria` instead of `gustaría`), the exercise **PASSES**, but the terminal outputs a helpful warning notice:
     ```text
     💡 Notice: Passed! Accent Tip: 'gustaría' carries an accent on the 'í' (hiatus: weak + strong vowel).
     ```
   - **`strict` (Optional CLI flag `--accent-mode=strict`):** Requires exact diacritics (`á, é, í, ó, ú, ü, ñ`) to pass.
   - **Ambiguity Detection:** If an accent distinguishes two different words/tenses (e.g. `hablo` [I speak, present] vs `habló` [he/she spoke, preterite]), the diagnostic highlights the tense ambiguity if the sentence context requires the other.

### 3.2 Compiler-Style Diagnostic Output
When an answer fails, `spanglings` emits a structured diagnostic with code locations, error codes, notes, and rule hints:

```text
error[E0301]: incorrect verb mood in subordinate clause
  --> exercises/03_subjunctive_weirdo/subjunctive_01.md:14:15
   |
14 | Quiero que tú viene a mi fiesta esta noche.
   |               ^^^^^ expected Present Subjunctive ('vengas'), found Present Indicative ('viene')
   |
   = note: The main clause verb 'querer' expresses a wish/influence with a change of subject ('yo' -> 'tú').
   = help: In Spanish, verbs of wishing/influence require the subordinate clause verb in the Subjunctive mood.
   = hint: Form the Present Subjunctive of 'venir': yo vengo -> drop -o -> stem 'veng-' + '-as' = 'vengas'.
```

### 3.3 Diagnostic Error Taxonomy

| Error Code | Category | Description |
|---|---|---|
| `E0101` | *Ser* vs *Estar* | Used *ser* instead of *estar* for temporary state or condition. |
| `E0102` | *Ser* vs *Estar* | Misinterpreted adjective meaning shift (*listo, atento, rico, verde*). |
| `E0201` | Past Aspect | Used Imperfect for completed, bounded event (expected Preterite). |
| `E0202` | Past Aspect | Used Preterite for habitual background action or state of mind (expected Imperfect). |
| `E0203` | Past Aspect | Meaning-changing preterite (*supe* vs *sabía*, *quise* vs *quería*, *pude* vs *podía*). |
| `E0301` | Subjunctive | Used Indicative when WEIRDO trigger requires Subjunctive. |
| `E0401` | Subjunctive | Used Indicative for non-existent or indefinite antecedent in relative clause. |
| `E0501` | Subjunctive | Failed Subjunctive trigger with temporal/concessive conjunction (*en cuanto, a menos que*). |
| `E0601` | Conditionals | Used Present or Conditional in *si* clause (expected Imperfect Subjunctive *tuviera*). |
| `E0701` | *Por* vs *Para* | Used *por* instead of *para* (purpose/goal/deadline vs cause/motive/exchange). |
| `E0801` | Clitics | Incorrect pronoun stacking order (expected Reflexive/Indirect + Direct: *se lo*). |
| `E0802` | Clitics / Accent | Missing written accent on verb with attached enclitics (*explicándomelo*, *dímelo*). |
| `E0901` | Prepositions | Incorrect prepositional regime (*darse cuenta de*, *pensar en*, *soñar con*). |
| `E1001` | *Se* Construction | Incorrect agreement with accidental *se* (*se me olvidaron las llaves*). |
| `E1101` | B2/C1 Advanced | Pluperfect Subjunctive in counterfactual 3rd conditional (*hubiera sabido*). |
| `E1301` | B2/C1 Advanced | Advanced concessive clause with subjunctive (*hagas lo que hagas*, *por mucho que*). |

---

## 4. Exercise File Format & Authoring

Each exercise is a standalone Markdown file with standardized frontmatter and comment markers:

```markdown
<!-- I AM NOT DONE -->
# Subjunctive 01: Verbs of Influence
<!-- id: b1_subjunctive_01 | level: B1 | topic: subjunctive_weirdo -->

> **Grammar Rule**: When the subject of the main clause differs from the subordinate clause, 
> verbs of influence/wishes (*querer, sugerir, exigir, aconsejar*) require the Present Subjunctive.

### Context
English: "I want you to come to my party tonight."

### Exercise
Replace `___` with the correct conjugated form of the verb in parentheses.

Quiero que tú (venir) ___ a mi fiesta esta noche.

<!-- SOLUTION
vengas
-->

<!-- ALTERNATIVES
vengas tú
-->

<!-- DIAGNOSTIC_RULES
pattern: "viene" | code: "E0301" | message: "Expected Present Subjunctive ('vengas'), but found Indicative ('viene')."
pattern: "vienes" | code: "E0301" | message: "Expected Present Subjunctive ('vengas'), but found Indicative ('vienes')."
pattern: "ven" | code: "E0301" | message: "Expected Subjunctive ('vengas'), but found Imperative ('ven')."
-->

<!-- HINTS
Tier 1: Look at the main verb 'quiero que'. Does the subject change?
Tier 2: The verb 'venir' is irregular in the present subjunctive: start with 'yo vengo'.
Tier 3: Drop the '-o' and add the '-er/-ir' subjunctive 2nd person ending: 'vengas'.
-->
```

---

## 5. Curriculum Map & Tracks

### Track 00: Baseline Reflex Drills (A1-Pre-B1 Irregular Stems & High-Yield Vocab)
- `00_irregular_preterite_stems`: Fast drills on *anduv-, tuv-, sup-, quis-, pud-, pus-, conduj-, dij-, traj-*.
- `00_irregular_future_conditional`: Fast drills on *habr-, sabr-, podr-, pondr-, saldr-, tendr-, valdr-, dir-, har-*.
- `00_irregular_subjunctive_roots`: Fast drills on *quepa, haya, sepa, valga, nazca, luzca, conozca*.
- `00_false_cognates_and_precision`: Precision vocabulary drills (*soportar* vs *apoyar*, *realizar* vs *darse cuenta*, *atender* vs *asistir*, *discutir* vs *argumentar*).

### Track 01: B1 (Core Mastery & Subjunctive Launchpad)
- `01_ser_vs_estar_nuances`: Meaning shifts with adjectives (*listo, atento, rico, orgulloso, verde, seguro, comprometido*).
- `02_preterite_vs_imperfect_aspects`: Aspectual contrasts (*supe/sabía*, *quise/no quise/quería*, *pude/podía*, *conocí/conocía*).
- `03_subjunctive_present_weirdo`: Wishes, emotions, impersonal expressions, recommendations, doubts, ojalá.
- `04_subjunctive_relative_clauses`: Indefinite / non-existent antecedents (*Busco a alguien que sepa...* vs *Conozco a alguien que sabe...*).
- `05_subjunctive_conjunctions`: Temporal & purpose clauses (*en cuanto*, *tan pronto como*, *para que*, *antes de que*, *con tal de que*).
- `06_imperfect_subjunctive_conditionals`: Second conditionals (*Si tuviera dinero, viajaría*), courteous requests (*quisiera*).
- `07_por_vs_para_advanced`: Causes, motives, deadlines, destinations, recipients, and idiomatic locutions.
- `08_pronoun_stacking_and_accents`: Double clitics (*se lo dije*) + reflexive combinations and enclitic accentuation (*explicándomelo*).
- `09_prepositional_regimes`: Verbs bound to specific prepositions (*darse cuenta de*, *pensar en*, *insistir en*, *tardar en*, *soñar con*).
- `10_accidental_se_and_passives`: Involuntary occurrences (*se me cayó*, *se nos olvidó*) and passive/impersonal *se*.

### Track 02: B2 (Advanced Fluency & Complex Structures)
- `11_pluperfect_subjunctive_3rd_conditionals`: Counterfactuals (*Si me hubieras dicho, te habría acompañado*).
- `12_aspectual_and_modal_periphrases`: Nuanced verbal phrases (*llevar + gerundio*, *acabar de / por*, *dejar de*, *volver a*, *darle por*, *ponerse a*, *estar a punto de*).
- `13_advanced_concessives`: Reduplicative idioms & concessions (*hagas lo que hagas*, *por más que insistas*, *a pesar de que* + subj vs ind).
- `14_causal_consecutive_connectors`: High-register connectors (*de ahí que + subj*, *dado que*, *en vista de que*, *por lo tanto*).
- `15_indirect_speech_and_tense_sequencing`: Reported speech harmonization across tenses (*me pidió que fuera*).
- `16_idiomatic_expressions_in_context`: High-frequency authentic locutions (*tomar el pelo*, *dar la lata*, *meter la pata*, *hacer la vista gorda*).

### Track 03: C1 (Stylistic Nuance, Precision & Native Collocations)
- `17_verbs_of_perception_and_opinion_negated`: Mood shifts under negation (*no digo que sea...*, *no es que no quiera...*).
- `18_emphatic_structures_and_clefting`: Cleft sentences & focalization (*Fue entonces cuando...*, *Lo que pasa es que...*).
- `19_stylistic_inversion_and_nominalization`: High-register writing (*Habiendo considerado...*, *El hecho de que + subj*).
- `20_advanced_passive_refleja_nuances`: Impersonal *se* with animate direct objects (*Se busca a los culpables*) and *quedar por + inf*.
- `21_nuanced_collocations_and_registers`: Cultured lexicon, subtle synonyms, and register switching.

---

## 6. Rust Project Layout

```
spanglings/
├── Cargo.toml
├── src/
│   ├── main.rs                  # CLI entrypoint & dispatcher
│   ├── cli/
│   │   ├── mod.rs               # Clap CLI args
│   │   └── commands/            # Handlers for watch, run, hint, drill, list, progress, reset
│   ├── core/
│   │   ├── mod.rs
│   │   ├── exercise.rs          # Exercise struct & Markdown parser
│   │   ├── curriculum.rs        # Tracks, topics, level categorization
│   │   └── state.rs             # JSON persistence & progress tracking
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── validator.rs         # Multi-step evaluation pipeline
│   │   ├── normalizer.rs        # Punctuation & unicode normalization
│   │   ├── accents.rs           # Smart accent checking & warning generator
│   │   ├── diagnostics.rs       # Error code mapping & ANSI compiler formatter
│   │   └── rules.rs             # Grammar rule patterns
│   ├── watcher/
│   │   ├── mod.rs               # notify-based file watcher with debouncing
│   │   └── runner.rs            # Terminal evaluation loop on file save
│   └── tui/
│       ├── mod.rs               # Ratatui application state
│       ├── ui.rs                # Split-pane terminal views
│       └── events.rs            # Keyboard shortcuts (H=hint, N=next, P=prev, D=drill, Q=quit)
├── exercises/                   # Complete curriculum files (.md)
│   ├── 00_baseline_drills/
│   ├── 01_ser_vs_estar/
│   ├── 02_past_aspects/
│   ├── 03_subjunctive_weirdo/
│   ├── 04_subjunctive_relative/
│   ├── 05_subjunctive_conjunctions/
│   ├── ... (06 to 21)
└── tests/
    ├── exercise_validity_tests.rs  # Asserts 100% of exercises have valid passing solutions
    ├── diagnostic_rule_tests.rs    # Asserts diagnostic codes trigger on common errors
    └── normalizer_tests.rs         # Asserts punctuation and accent normalization rules
```

---

## 7. Testing & Quality Assurance

1. **Curriculum Integrity Suite:** CI automated tests load all exercise files and test their canonical solutions and alternative accepted forms, ensuring zero broken exercises.
2. **Diagnostic Precision Suite:** Unit tests for every diagnostic error code (E0101 through E2101) to verify correct pinpointing and explanations.
3. **Punctuation & Accent Invariance Tests:** Verifies that US/UK keyboard input without inverted punctuation or with missing accents gracefully passes in forgiving mode with the appropriate notice.
4. **Watcher & TUI State Tests:** Verifies state transitions, progress saving, and persistence across sessions.
