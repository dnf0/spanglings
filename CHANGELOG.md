# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.3] - 2026-09-01

### Added
- **Universal Dual-Layer Explanations & Plain-English Mental Models**:
  - Transformed grammatical feedback across all Spanglings surfaces (CLI Arcade recaps, TUI Arena recap modals, pre-session cheat sheets, and interactive hints) into dual-layer cards pairing an intuitive communicative mental model with a precise structural rule:
    ```text
    💡 Meaning: Expresses requirement ("need you to know"); uncertainty triggers virtual mode.
    📐 Rule:    Present subjunctive (tú) of 'saber' uses irregular stem 'sep-' ➔ sepas.
    ```
  - Populated intuitive plain-English mental models for all 24 core grammar concepts in `src/core/reference.rs`, all 136 sentence frames in `src/core/generator.rs`, and all 262 showdown and specialized engine sentences in `src/core/arcade.rs`.
  - Added `🧠 PLAIN-ENGLISH MENTAL MODEL:` header sections to all 24 topic cheat sheets and reference cards (`spanglings explain <topic>`).
  - Added backward-compatible `#[serde(default)]` serialization and clean single-layer fallback formatting.

## [0.5.1] - 2026-08-29

### Added
- **Scientific SM-2 Concept Mastery Model**:
  - Replaced flat $+30\%$ increments with a mathematically grounded SuperMemo SM-2 interval expansion and log-scale memory stability curve:
    $$\text{Mastery Score} = \min\left(1.0, \frac{\text{reps}}{6}\right) \times \min\left(1.0, \frac{\ln(1 + \text{interval})}{\ln(61)}\right) \times \left(\frac{EF}{2.5}\right)$$
  - Extended `ConceptMastery` to track and persist `repetitions`, `interval_days`, and adaptive `ease_factor` with backward-compatible serde defaults.
- **Symmetric Leitner Step-Inversion Rollback on Lapses**:
  - Replaced the harsh 0% hard-reset on mistakes with a fair, intuitive 1-step rollback ladder ($R = R.\text{saturating\_sub}(1)$, interval $6\text{d} \to 1\text{d} \to 0\text{d}$), preserving prior learning while ensuring lapses decrement ease factor for adaptive targeting (`-w`).
- **Post-Workout Arcade Mistakes Breakdown with Explanations**:
  - Added dedicated `❌ Review Missed Questions` cards in both CLI (`print_arcade_summary`) and TUI recap modals, rendering sentence prompts, user answers, correct answers, and grammatical rule explanations.
  - Added celebratory perfect-run recognition for 100% accuracy runs.

## [0.5.0] - 2026-08-29

### Added
- **ADHD Rapid Single-Key Arcade Mode & Showdown Arena**:
  - Introduced zero-friction, single-key input (`1`..`4`, `j`/`k`, `←`/`→`) with zero `Enter` key requirement for ultra-fast response training (`spanglings arcade`).
  - Added real-time score multipliers ($1\times$ to $5\times$), response speed bonuses (up to +150 pts), combo rank evaluations (*Quick Focus*, *ON FIRE*, *UNSTOPPABLE*, *ULTRA INSTINCT*), and audio cues (macOS system audio / terminal bells).
  - Built a dedicated full-screen TUI Arcade Arena modal overlay (`[x]` / `[F8]`) with live showdown cycling (`[s]`, `[Tab]`, `[BackTab]`), instant recap screens, and quick restart (`[r]`).
- **16 High-Stakes Spanish Binary Contrast Showdown Duels**:
  - Expanded the binary showdown duel library to 16 pairs with rich alias matching (`spanglings arcade <slug>`):
    - `tener-haber` (`have`): Tener (possession, age, feelings, obligation) vs Haber (auxiliary, existential, impersonal obligation).
    - `saber-conocer` (`know`): Saber (facts, knowledge, skills) vs Conocer (people, places, acquaintance).
    - `muy-mucho` (`very-much`): Muy (adverb modifying adj/adv) vs Mucho (quantifier/adjective or modifying verbs).
    - `pedir-preguntar` (`ask`): Pedir (requesting objects/services) vs Preguntar (asking questions/inquiries).
    - `llevar-traer` (`take-bring`): Llevar (away from speaker) vs Traer (toward speaker).
    - `haber-estar` (`hay-esta`): Haber (indefinite existence) vs Estar (specific known location).
    - `ir-irse` (`go-leave`): Ir (movement to destination) vs Irse (departure / leaving a place).
    - `bien-bueno` (`well-good`): Bien (adverb) vs Bueno/Buen (adjective).
    - `por-para`: Por (cause, means, duration, exchange) vs Para (destination, recipient, deadline, purpose).
    - `ser-estar`: Ser (identity, essence) vs Estar (states, conditions, locations).
    - `subj-ind`: Subjuntivo (doubt, wishes, uncertainty) vs Indicativo (facts, certainty).
    - `pret-imp`: Pretérito (completed events) vs Imperfecto (habitual past, descriptions).
    - `tu-usted`: Tú (informal address) vs Usted (formal address).
    - `lo-le`: Direct object (lo/la) vs Indirect object (le/les).
    - `sino-pero`: Sino (replacement after negation) vs Pero (contrast).
    - `para-que-porque`: Para que (+subj) vs Porque (+ind).
- **5 High-Yield Specialized Advanced Drill Engines**:
  - `regimen` (`prepositions`): Fixed verb-preposition bonds (*soñar con*, *pensar en*, *acordarse de*, *fijarse en*, *depender de*, *enamorarse de*, *darse cuenta de*, *insistir en*, *consistir en*, *aprender a*, *acostumbrarse a*, *negarse a*, *preocuparse por*, *quejarse de*, *atreverse a*, *tardar en*).
  - `irregulars` (`verbs`): High-friction irregular stem retrieval for preterites (*puse*, *quiso*, *cupo*, *trajiste*, *condujo*, *anduvimos*, *supe*, *hubo*), presents (*quepo*, *sé*), subjunctives (*sepas*, *quepa*, *haya*), and futures (*pondré*, *harás*, *saldremos*).
  - `false-friends` (`cognates`): Real-time override training for deceptive English-Spanish false cognates (*actual* vs *real*, *embarazada* vs *avergonzado*, *éxito* vs *salida*, *atender* vs *asistir*, *sensible* vs *sensato*, *largo* vs *grande*, *carpeta* vs *alfombra*, *soportar* vs *apoyar*, *pretender* vs *fingir*, *realizar* vs *darse cuenta*, *constipado* vs *estreñido*, *recordar* vs *grabar*).
  - `se-matrix` (`se`): The 5 functional faces of Spanish *Se* (Accidental Involuntary Dative *se me cayó*, Impersonal *se vive bien*, Passive Reflexive *se alquilan*, Reciprocal *se saludaron*, and Aspectual/Telic *se comió*).
  - `connectors` (`discourse`): B2/C1 sentence transition markers and discourse connectors (*sin embargo*, *no obstante*, *en cambio*, *por lo tanto*, *por consiguiente*, *de ahí que*, *dado que*, *debido a*, *a pesar de que*, *es decir*, *por ende*).
- **Combinatorial Question Engine & Adaptive Weakness Profiler**:
  - Built a 130+ template combinatorial question engine generating over 4,000 unique questions across all 24 CEFR grammar topics.
  - Integrated SM-2 spaced repetition concept mastery profiling with `--weak` targeting in `drill`, `blitz`, and `arcade`.
  - Added an interactive TUI Concept Mastery Dashboard modal (`[m]`).
- **Intelligent Mixed Arcade Blending**:
  - Default `spanglings arcade` intelligently blends 45% Showdowns, 30% Specialized Drill Engines, and 25% Grammar Concept clozes for comprehensive Spanish mastery workouts.

## [0.4.5] - 2026-08-28

### Added
- **Rich Context Drill & Blitz Prompts**:
  - Upgraded question formatting in `spanglings drill` and `spanglings blitz` with authentic sentence context containing `____` blanks, target base infinitive verb, and grammatical subject cues.
- **Pre-Session Grammar Topic Cheat Sheets**:
  - Added fast rule summary cheat sheets displayed before starting drills across 12 grammatical domains (`subjunctive`, `preterite`, `por_para`, `ser_estar`, `pronouns`, `prepositions`, `accidental_se`, `imperative`, `future`, `false_friends`, `idioms`, and mixed).
- **In-Drill & In-Blitz Interactive Live Hints**:
  - Added on-demand step-by-step derivation assistance by typing `?` or `hint` during interactive prompt loops without penalizing streaks or scores.

### Fixed
- **TUI First-Run Tour Persistence**:
  - Fixed issue where the onboarding welcome tour modal was shown repeatedly on subsequent launches by persisting `tour_completed = true` immediately upon presentation and handling modal dismissals cleanly.

## [0.4.4] - 2026-08-28

### Fixed
- **Drill Randomization & Comprehensive Question Variety**:
  - Fixed deterministic drill question sequencing in `spanglings drill` and `spanglings blitz` by shuffling candidate question pools with `rand::seq::SliceRandom` at the start of every session.
  - Expanded the drill question bank from 10 hardcoded items to **70+ comprehensive drill questions** covering 12 linguistic topics: irregular preterite stems, present & imperfect subjunctive, irregular future/conditional stems, affirmative/negative imperatives, por vs para, ser vs estar, clitic cacophony (`se lo`), prepositional verbs (*régimen preposicional*), accidental *se*, false friends / cognates, and native idioms.
  - Added `-n` / `--count` flag to `spanglings drill` allowing users to configure question batch sizes (e.g., `spanglings drill -n 10` or `spanglings drill preterite --count 15`).
  - Integrated smart forgiving accent evaluation with explanatory feedback tips into the drilling engine.

## [0.4.3] - 2026-08-28

### Fixed
- **TUI Progress Persistence & State Synchronization**:
  - Connected `spanglings tui` to `AppState` persistence engine (`state.json`), ensuring exercise completions, attempts, hints used, and SM-2 spaced repetition (SRS) schedules are immediately saved on answer evaluation.
  - Automatically restores completed exercise indicators (`✓ DONE`) across all views and resumes the cursor at the last active or next uncompleted exercise on startup.
  - Synchronized interactive placement test fast-tracking so passed levels instantly update both persistent state and active in-memory exercise models.
  - Added full test coverage for TUI session restore and persistence roundtrips.

## [0.4.2] - 2026-08-28

### Fixed
- **Seamless Global TUI & CLI Execution**:
  - Enhanced `spanglings tui`, `spanglings run`, `spanglings hint`, `spanglings review`, and `spanglings export` to automatically fallback to the embedded 339-exercise curriculum when run outside the repo or before running `spanglings init`.
  - Added helpful tip to `spanglings watch` pointing users to `spanglings init` when launched in an uninitialized directory.

## [0.4.1] - 2026-08-28

### Fixed
- **Initial Exercise Completion State Evaluation**:
  - Corrected `is_done` detection in exercise parser and evaluation engine to recognize unfilled blanks (`___`, `<!-- ANSWER -->`) as incomplete (`is_done = false`).
  - Fixed `spanglings list` and `spanglings progress` reporting `0/339 (0.0%)` on clean setups rather than falsely marking 100% completed out of the box.
- **VS Code Extension & Path Resolution**:
  - Added resilient workspace root detection preventing read-only file system errors (`os error 30`) when no workspace folder is open.
  - Implemented multi-depth exercise path resolution across root, subfolders, and parent directories.

## [0.4.0] - 2026-08-28

### Added
- **Informative & Non-Spoiling Exercise Instructions Architecture**:
  - Enriched all **339 exercises** across all **60 tracks** with a standardized `### Instructions` block containing explicit `**TODO**:` prompts and grammatical `**Why**:` explanations.
  - Added in-editor `<!-- TODO: ... -->` comments directly under `### Exercise` in every markdown file for frictionless external editor watch mode.
  - Implemented strict anti-leak policy ensuring zero solution words or inflection spoilers are revealed in instruction prompts.
- **TUI Instructions & Prompt Card Rendering**:
  - Enhanced the interactive TUI prompt card (`src/tui/ui.rs`) to parse and display structured instructions (`Instructions (TODO & Why):`) with custom terminal styling (Yellow bold `TODO:` and Cyan bold `Why:`).
- **Automated Anti-Spoiler & Structural Test Suite**:
  - Added `tests/exercise_todo_tests.rs` with automated validation of markdown instruction structures, minimum character thresholds, and a tokenized diacritic-normalized zero-leakage detector across all 339 exercises.
- **Engine Validator Hardening**:
  - Hardened fallback answer extraction in `src/engine/validator.rs` to ignore `**TODO**:` and `**Why**:` lines, preventing instruction text from being parsed as student submissions.

### Changed
- Refactored exercise parser and watcher stream to preserve and format inline comments and instruction blocks cleanly.
- Updated ontological knowledge graph to 2,910 nodes, 3,219 edges, and 433 communities.

---

## [0.3.0] - 2026-08-27

### Added
- **Full-Spectrum C1 & Practical Curriculum Expansion**: Added Tracks 27 through 59, bringing the total curriculum catalog to **60 Tracks and 339 handcrafted exercises** across CEFR levels A1 through C1.
- **81-Concept Linguistic Knowledge Graph (DAG)**: Built a cycle-free Directed Acyclic Graph modeling dependencies between 81 linguistic concepts with automatic learning frontier computation and weakness root-cause tracing.
- **Compiler Diagnostic System & 59 Error Codes**: Implemented rustc-style diagnostics with dynamic carets (`^^^^`), contrast notes, linked concepts, and actionable tips for compiler error codes `E0001` through `E0059`.
- **In-Terminal Error Code Resolution**: Enabled direct query of compiler diagnostics via `spanglings explain <ERROR_CODE>` (e.g. `spanglings explain E0301`).
- **Interactive 6-Station Guided Onboarding Tour**: Added `spanglings tour` and an interactive first-run onboarding popup dialog in the TUI.
- **Native Language Server Protocol (LSP)**: Integrated `spanglings lsp` providing real-time diagnostics, autocompletions, and hover tooltips for VS Code, Neovim, Helix, and Zed.
- **Calibrated CEFR Diagnostic Placement Assessment**: Added `spanglings test` for multi-tier level evaluation and automatic level fast-tracking.
- **Anki & Markdown Study Exporter**: Added `spanglings export` supporting Anki TSV decks, Obsidian Markdown notes, and JSON progress exports.
- **Multi-Machine Progress Sync Engine**: Added `spanglings sync` for portable backup, restore, and progress merging.
- **Kubelings-Grade Documentation Site**: Launched live documentation site at https://dnf0.github.io/spanglings/ with an animated 4-frame CSS keyframe terminal demo.

### Changed
- **Modern Watcher Experience**: Replaced legacy comment-deletion markers (`<!-- I AM NOT DONE -->`) with non-blocking keybindings (`[n]`, `[p]`, `[r]`, `[h]`, `[c]`, `[q]`) and sub-20ms evaluation on pure correctness.
- **Retroactive Exercise Tagging**: Tagged all 339 exercises with `concepts: [...]`, `prerequisites: [...]`, and `grammar_focus: "..."` metadata.

---

## [0.2.0] - 2026-08-27

### Added
- **Full Interactive Terminal UI (`ratatui`)**: Dual-pane editor, exercise browser, and live validation.
- **In-TUI Verb Conjugator & Reference Browser**: Real-time verb table lookups and 24 searchable grammar cheat sheets.
- **Git Practice Hooks**: Pre-commit and pre-push Spanish micro-drill hooks (`spanglings hook`).
- **Custom Curriculum Packs**: Pack scaffolding, validation, and installation engine (`spanglings pack`).
- **Rapid-Fire Blitz Drills**: 60-second conjugation speed challenge (`spanglings blitz`).

---

## [0.1.1] - 2026-08-27

### Fixed
- Fixed headless file watcher event loop and terminal raw mode teardown.
- Corrected UTF-8 Spanish diacritic handling for accented characters (`á`, `é`, `í`, `ó`, `ú`, `ñ`, `ü`).

---

## [0.1.0] - 2026-08-27

### Added
- Initial release of Spanglings: core exercise validator, CLI runner, and baseline A1-B1 curriculum tracks.
