# Spanglings Onboarding & Learner's Guide 🇪🇸 🦀

Welcome to **Spanglings**! Whether you are completely new to Spanish or an experienced developer looking to sharpen your grammar mechanics, aspectual precision, and professional C1 syntax, this guide will get you fully up to speed in minutes.

---

## 1. What is Spanglings?

Spanglings is an interactive, test-driven learning platform inspired by [Rustlings](https://github.com/rust-lang/rustlings) and [Raylings](https://github.com/dnf0/raylings). Rather than swiping through multiple-choice flashcards or memorizing isolated phrases, you learn Spanish by actively diagnosing, writing, and compiling syntactic transformations directly in your code editor and terminal.

### 🌟 Core Pedagogical Principles

```
  +-----------------------+      +-----------------------+      +-----------------------+
  |    Active Debugging   |      |  Sub-20ms Validation  |      |   Compiler Feedback   |
  |  Every exercise starts| ---> |  In-memory evaluator  | ---> |  Rustc-style colored  |
  |  with an active prompt|      |  checks answers on    |      |  diagnostics with     |
  |  and structural cues  |      |  save instantaneously |      |  E-codes & contrasts  |
  +-----------------------+      +-----------------------+      +-----------------------+
```

1. **Active Problem Solving with Zero Busywork**: Exercises evaluate purely on syntactic correctness. No magic comment deletion (`<!-- I AM NOT DONE -->`) required.
2. **Rustc-Style Grammar Compiler**: Grammatical mistakes trigger rich compiler diagnostics (`error[E0301]: Subjunctive Mood Required`), pointing out source tokens, underlying rules, prerequisite links, and contrast notes.
3. **81-Concept Linguistic Knowledge Graph (DAG)**: The curriculum maps every exercise to an ontological graph, dynamically evaluating your **learning frontier** and identifying conceptual weaknesses.
4. **SM-2 Spaced Repetition**: Combines active recall with SuperMemo-2 spaced intervals to prevent forgetting and reinforce tricky grammatical patterns.

---

## 2. The 6-Station Interactive Guided Tour

Spanglings includes a built-in terminal tour designed to walk you through the system in under two minutes:

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

### The 6 Interactive Stations:
1. **Philosophy & Core Loop**: The developer-first mental model, instantaneous evaluation, and zero-friction editing.
2. **Exercise Anatomy & Accent Handling**: Understanding inline `___` cloze blanks, UTF-8 Spanish punctuation, and forgiving accent matching.
3. **Rustc-Style Grammar Diagnostics**: Reading compiler-grade errors, dynamic carets, linguistic concept links, and contrast notes.
4. **Progressive Hints & Cheat Sheets**: Accessing 3-tier progressive hints (`[h]` / `F1`) and on-demand grammar cheat sheets (`[e]` / `F2`).
5. **Power Tools & Placement Fast-Track**: Using the in-TUI Verb Conjugator (`F3`), Reference Sheet Browser (`F4`), and Calibrated CEFR Placement Test (`F5` / `[p]`).
6. **Modern Watcher & Editor Workflows**: Running `spanglings watch` with non-blocking keys (`[n]`, `[p]`, `[r]`, `[q]`) and configuring Language Server Protocol (LSP) integrations.

---

## 3. The Three Ways to Learn

### Mode 1: Interactive Terminal UI (`spanglings` / `spanglings tui`)
A standalone, full-screen terminal application built with `ratatui`:
- Split-pane layout: Exercise instructions on the left, interactive editor on the right.
- Hotkey-driven modals for instant cheat sheets (`F2`), verb tables (`F3`), and curriculum search (`F4` or `/`).
- Ideal for distraction-free learning without configuring any external editors.

### Mode 2: Headless Watcher + IDE (`spanglings watch`)
Run Spanglings as a background daemon alongside your favorite editor (VS Code, Cursor, Neovim, Helix, Zed):
- Automatically triggers on file save in `< 20ms`.
- Interactive hotkeys directly in your watcher terminal (`n` next, `p` previous, `r` rerun, `h` hint, `q` quit).
- Real-time Language Server Protocol (`spanglings lsp`) provides hover tooltips, red squigglies, and autocomplete directly in your editor buffer.

### Mode 3: Spaced Repetition & Placement Testing (`spanglings test` & `review`)
- **Placement Assessment (`spanglings test`)**: Test your CEFR level from A1 through C1 and fast-track past concepts you already know.
- **SRS Review (`spanglings review`)**: Daily active-recall drills scheduled by the SM-2 algorithm.
- **Speed Blitz (`spanglings blitz`)**: 60-second rapid-fire conjugation challenges to build automatic muscle memory.

---

## 4. Solving Your First Exercise: A Step-by-Step Walkthrough

Every exercise is a clean Markdown file in `exercises/<track>/<id>.md`:

```markdown
<!-- exercises/03_subjunctive_weirdo/01_wishes_volition.md -->
# Wishes and Volition (Querer / Desear que)
- Level: B1
- Topic: Subjunctive Triggers (WEIRDO)
- Concepts: subjunctive_wishes_desires, present_subjunctive_regular
- Prerequisites: present_indicative_irregular
- Grammar Focus: "Main clause volition verb + que + subject change requires present subjunctive."

## Prompt
Fill in the correct present subjunctive form of the verb in parentheses.
Context: Expressing a desire for someone else's action.

Mis padres quieren que yo ___ (estudiar) ingeniería informática en la universidad.

## Hints
<!-- Tier 1: Conjunctions of volition and desire require the subjunctive mood when subjects differ. -->
<!-- Tier 2: The -ar present subjunctive endings are -e, -es, -e, -emos, -éis, -en. -->
<!-- Tier 3: estudie -->
```

### Step 1: Open and Inspect
Start the watcher:
```bash
spanglings watch
```

### Step 2: Triggering Diagnostic Output
If you provide an incorrect indicative form (e.g. `estudio`), Spanglings will emit a rich diagnostic:

```
error[E0301]: Subjunctive Mood Required
  --> exercises/03_subjunctive_weirdo/01_wishes_volition.md:14:15
   |
14 | Mis padres quieren que yo estudio ingeniería informática en la universidad.
   |                           ^^^^^^^ expected subjunctive form 'estudie', found indicative 'estudio'
   = note: Main clause expresses volition ('quieren que...'), triggering mood shift
   = note: Linked Concept: subjunctive_wishes_desires (Prerequisite: present_indicative_irregular)
   = note: Contrast: Indicative asserts a factual statement; Subjunctive expresses desired outcome
   = tip: For a deep conceptual breakdown, run 'spanglings explain E0301' or press [e] in TUI
```

### Step 3: Progressive Hints & Explanations
Need a hint? Press `h` in the watcher or run `spanglings hint`. Tier 1 gives a conceptual nudge; Tier 2 gives structural guidance; Tier 3 reveals the solution.

To read the complete grammar card:
```bash
spanglings explain E0301
# or
spanglings explain subjunctive
```

### Step 4: Advancing
Replace `___ (estudiar)` with `estudie` and save. Spanglings verifies the answer instantly! Press `n` or `Enter` to advance to the next exercise.

---

## 5. Diagnostic Error Codes Directory

Spanglings includes a comprehensive compiler catalog with 59 specialized diagnostic error codes:

| Error Code | Title | Linguistic Category |
| :--- | :--- | :--- |
| `E0001` | General Syntax Error | Orthography & General |
| `E0002` | Missing Accent Mark | Orthography & Stress Rules |
| `E0003` | Incorrect Verb Stem | Stem-Changing Morphology |
| `E0004` | Incorrect Verb Ending | Verb Conjugation Parity |
| `E0005` | Ser vs Estar Distinction Required | Permanent vs State Identity |
| `E0006` | Gender Agreement Mismatch | Noun-Adjective Agreement |
| `E0007` | Number Agreement Mismatch | Singular / Plural Concord |
| `E0011` | Preterite vs Imperfect Aspect Mismatch | Aspectual Past Completion vs Duration |
| `E0012` | Direct Object Pronoun Mismatch | Transitive Accusative Clitic |
| `E0013` | Indirect Object Pronoun Mismatch | Dative Clitic Placement |
| `E0014` | Double Object Clitic Ordering Error | *Se lo* Clitic Replacement |
| `E0015` | Accidental *Se* Construction Required | Involuntary Agentless Dative |
| `E0030` | Relative Pronoun Error (*que/quien/el cual*) | Relative Clause Binding |
| `E0031` | Conditional Tense Required | Hypothetical / Polite Formulations |
| `E0032` | Imperfect Subjunctive Required | Hypothetical / Past Subjunctive |
| `E0033` | Hypothetical *Si* Clause Mismatch | Mixed Conditional Sequence of Tenses |
| `E0034` | Passive *Se* Construction Required | Impersonal & Passive Formulations |
| `E0035` | Verbal Periphrasis Error (*llevar + gerundio*) | Aspectual Auxiliary Periphrases |
| `E0036` | False Cognate Trap Detected | False Friends & Semantic Traps |
| `E0037` | Technical / Engineering Vocabulary Mismatch | Software & Tech Collocations |
| `E0038` | Formal Business Register Required | Professional Correspondence |
| `E0048` | Epistemic Conjecture Marker Required | Future / Conditional of Probability |
| `E0049` | Redundant Clitic Doubling Required | Obligatory Topicalized Dative Clitic |
| `E0050` | Personal *A* Accusative Marker Missing | Animate Specific Direct Object |
| `E0051` | Gerund Restriction Violation | Adjectival / Temporal Gerund Ban |
| `E0052` | Adversative Marker Mismatch (*pero vs sino*) | Corrective Polarity & Exception |
| `E0053` | Archaic / Legal Subjunctive Required | Formulaic -ere Morphology |
| `E0054` | Verb of Becoming Inappropriate (*ponerse/hacerse/volverse*) | Involuntary vs Effortful Becoming |
| `E0055` | Epistemic Adverb Subjunctive Required (*quizás/tal vez*) | Adverbial Doubt & Epistemic Stance |
| `E0056` | Possessive Dative Construction Required | Inalienable Possession Dative |
| `E0057` | Corrective Polarity Mismatch (*sino que*) | Finite Verb Clause Correction |
| `E0058` | Absolute Participial Clause Error | Non-finite Temporal Clause |
| `E0059` | Scalar Concession Subjunctive Required (*por mucho que*) | Scalar Intensity Concessions |

---

## 6. Active Recall & Concept Mastery (SM-2)

Spanglings tracks your linguistic memory retention using the SuperMemo-2 spaced repetition algorithm:

- **Concept Mastery Scoring**: Every concept starts at baseline `0.0` and reaches mastery at `5.0`.
- **Lapse Penalties & Spaced Recovery**: Incorrect answers trigger an immediate lapse penalty (`-1.5` ease factor), prioritizing that exercise for upcoming review.
- **Dynamic Frontier Resolution**: Spanglings analyzes prerequisite relationships across the 81-concept DAG and recommends the highest-yield topics for your current level.

```bash
# Check mastery scores and weak areas
spanglings progress

# Run targeted drill on weakest concepts
spanglings drill
```

---

## 7. Power User Tools & FAQ

### QWERTY Smart Accent Mode
By default, Spanglings tolerates missing accents on QWERTY keyboards while providing a helpful tip notice:
```
✓ Correct! (Note: 'estudie' is accepted, but formal spelling uses 'estudié')
```
To strictly enforce acute accents (`á`, `é`, `í`, `ó`, `ú`) and tildes (`ñ`):
```bash
spanglings --strict-accents watch
```

### Git Practice Hooks
Stay sharp by answering a quick Spanish practice challenge before every commit or push:
```bash
# Install pre-commit practice hook
spanglings hook pre-commit

# Remove hook
spanglings hook --remove
```

### Anki & Obsidian Export
Export your entire curriculum progress and flashcard database for offline review:
```bash
# Export to Anki TSV format
spanglings export --format anki --output spanish_anki.tsv

# Export Markdown study guide for Obsidian
spanglings export --format markdown --output spanish_notes.md
```
