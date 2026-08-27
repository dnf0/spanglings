# Spanglings Product & Engineering Backlog

This backlog documents upcoming enhancements, planned curriculum tracks, diagnostic features, and developer ergonomics improvements for Spanglings.

---

## 🎯 Focus Area 1: Discoverability, Portability & Zero-Setup CLI (Completed)

- [x] **SPANG-001: Embedded Curriculum & `spanglings init`**
  - **Description**: Embed all 116+ curriculum markdown files directly into the compiled binary via `include_dir!`.
  - **CLI Command**: `spanglings init [target_dir] [--force]`
  - **Behavior**: Scaffolds exercises into `./exercises` (or custom directory) with initial configuration and starter guide, allowing anyone installing via `cargo install spanglings` to run anywhere immediately.
- [x] **SPANG-002: Automatic In-Memory Fallback & Guided Setup**
  - **Description**: If `spanglings` is launched in a directory without an `./exercises` folder, fallback automatically to reading embedded exercises in-memory.
- [x] **SPANG-003: JSON Machine-Readable Output Mode (`--json`)**
  - **Description**: Added `--json` flag to `spanglings list`, `spanglings progress`, and `spanglings search`.
  - **Use Case**: Enables integrations with external scripts, status bars (e.g., Starship, tmux), CI pipelines, and IDE extensions.
- [x] **SPANG-004: Fuzzy Topic & Full-Text Search (`spanglings search`)**
  - **Description**: `spanglings search <query>` to search across exercise titles, prompts, grammar topics, tags, and hints with ranked matching.
- [x] **SPANG-005: Shell Completions Generator (`spanglings completions`)**
  - **Description**: `spanglings completions <bash|zsh|fish|powershell|elvish>` using `clap_complete` for subcommands, topics, and exercise paths.

---

## 📚 Focus Area 2: Curriculum & Advanced Vocabulary Expansion (Completed)

- [x] **SPANG-010: Track 22 - Tech & Software Engineering Spanish (B2–C1)**
  - **Topics**: Git workflows (*desplegar*, *solicitud de extracción*), debugging (*depurar*, *subsanar*), backend infrastructure (*concurrencia*, *bloqueo mutuo*, *rendimiento*, *latencia*).
- [x] **SPANG-011: Track 23 - Formal Business & Diplomatic Correspondence (B2–C1)**
  - **Topics**: Email formulas (*Quedo a su entera disposición*, *En lo que atañe a*), negotiation rhetoric (*saldar*, *acordar*, *desestimar*).
- [x] **SPANG-012: Track 24 - False Friends & High-Frequency Trap Drills**
  - **Topics**: Cognate traps (*actualmente*, *eventualmente*, *pretender*, *realizar*, *soportar*, *sensato*).
- [x] **SPANG-013: Track 25 - Register Elevation & Formal Stylistics (C1)**
  - **Topics**: Elevating conversational verbs into literary/journalistic equivalents (*hacer* -> *acometer*, *provocar* -> *suscitar*, *dar lugar a*, *surtir efecto*, *albergar dudas*, *arrojar luz*).
- [x] **SPANG-014: Track 26 - Regional Contrasts & Voseo (Latin America vs Peninsular)**
  - **Topics**: Rioplatense *voseo* conjugations (*tenés*, *sabés*, *decime*, *sentate*), Pan-American *ustedes*, and dialectal vocabulary pairs (*manejar*, *computadora*).

---

## 🧠 Focus Area 3: Smart Diagnostics & Pedagogical Enhancements (Completed)

- [x] **SPANG-020: Weakness Profiler & Targeted Smart Drills**
  - **Description**: Analyze SRS failure rates and ease factors by grammatical category/topic and generate targeted diagnostic recommendations in `spanglings progress` (and `--json`).
- [x] **SPANG-021: Accentuation & Stress Reference Sheet (`spanglings explain accents`)**
  - **Description**: In-terminal cheat sheet detailing *Agudas*, *Llanas*, *Esdrújulas*, *Diptongos vs Hiatos*, and diacritical accents.
- [x] **SPANG-022: 60-Second Rapid-Fire Blitz Mode (`spanglings blitz`)**
  - **Description**: Fast-paced terminal time-trial drill testing irregular roots and clitics under time constraints with streaks, WPM, and accuracy scoring.

---

## 📊 Focus Area 4: Interactive TUI & IDE Integrations (Completed)

- [x] **SPANG-030: ANSI Activity Heatmap in TUI & Progress CLI**
  - **Description**: GitHub-style calendar heatmap displaying exercise completions and daily reviews over 12 weeks with active streak tracking.
- [x] **SPANG-031: Live Fuzzy Search in Interactive TUI**
  - **Description**: Press `/` in TUI to filter exercises instantly by topic, keyword, or level with interactive search bar and live keyboard navigation.
- [x] **SPANG-032: Editor Diagnostic Checker & JSON Streamer (`spanglings check`)**
  - **Description**: Real-time compiler-style and JSON diagnostic streamer (`spanglings check <file> [--json]`) for seamless integration with VS Code, Zed, Neovim, and CI pipelines.

---

## ⚡ Focus Area 5: Terminal Conjugation Engine & Developer Workflow Hooks (Completed)

- [x] **SPANG-040: Automated Release Pipeline & Crates.io Publishing**
  - **Description**: Multi-platform release pipeline publishing signed binary bundles and automated `cargo publish` execution on tag pushes with `CARGO_REGISTRY_TOKEN`.
- [x] **SPANG-041: High-Precision Verb Conjugator (`spanglings conjugate <verb> [tense]`)**
  - **Description**: Terminal lookup engine rendering complete color-coded conjugation grids (Present, Preterite, Imperfect, Subjunctive, Imperative, Participle, Gerund) with irregular stem highlights.
- [x] **SPANG-042: Git Pre-Commit / Pre-Push Spanish Practice Hook (`spanglings hook`)**
  - **Description**: Installable Git hook prompting developer with 1 rapid active-recall conjugation/grammar flashcard before code commits.
- [x] **SPANG-043: Custom Curriculum Pack Scaffolder & Validator (`spanglings pack`)**
  - **Description**: Tools to scaffold custom community/industry tracks (`spanglings pack create`) and validate exercise markdown solvability (`spanglings pack validate`).

---

## 🖥️ Focus Area 6: In-TUI Power Tools & Modal Modifiers (Completed)

- [x] **SPANG-050: In-TUI Verb Conjugator Modal (`F3` / `Ctrl+K`)**
  - **Description**: Open a non-disruptive popup modal to search and conjugate any verb across all tenses without leaving the exercise workspace.
- [x] **SPANG-051: In-TUI Grammar Reference Card Browser Modal (`F4` / `Ctrl+B`)**
  - **Description**: Open an interactive modal with full-text search across all grammar cheat sheets (Subjunctive, Por vs Para, Clitics, Accents, etc.).
- [x] **SPANG-052: In-TUI Drill & 60-Second Blitz Launcher (`spanglings blitz` / `spanglings drill`)**
  - **Description**: Rapid-fire drill trials and 60-second Blitz with live scoreboard and keyboard response.
- [x] **SPANG-053: In-TUI Keybinding Overlay & Help Modal (`?` / `F1`)**
  - **Description**: Searchable help modal listing all hotkeys, navigation shortcuts, and workflow commands.

---

## 📚 Focus Area 7: Advanced C1 Technical, RFC & Professional Track Expansion (Completed)

- [x] **SPANG-060: Track 27 - System Architecture RFCs, Post-Mortems & Incident Response (B2–C1)**
  - **Topics**: Outage post-mortems (*caída del servicio*, *análisis de causa raíz*, *remediación*), architectural RFCs (*desacoplar componentes*, *tolerancia a fallos*, *rendimiento sostenido*).
- [x] **SPANG-061: Track 28 - Complex Subjunctive Triggers & Concessive Clauses (C1)**
  - **Topics**: Concessive expressions (*por mucho que*, *aun a riesgo de que*, *así sea*, *pase lo que pase*, *a pesar de que* + subjuntivo vs indicativo).
- [x] **SPANG-062: Track 29 - Advanced Verbal Periphrases & Nuanced Aspect (B2–C1)**
  - **Topics**: Modal and aspectual periphrases (*dar por sentado*, *echar a perder*, *ponerse a*, *llevar sin*, *venir a decir*, *quedar en*).
- [x] **SPANG-063: Track 30 - Executive Debate & Open Source Leadership Collocations (C1)**
  - **Topics**: High-register idiomatic leadership phrases (*hacer hincapié*, *reorientación estratégica*, *indicadores clave de rendimiento*, *reasignación de plantilla*, *exposición regulatoria*, *aprobar por unanimidad*).

---

## 📦 Focus Area 8: Portable Study Pack & Anki/Markdown Exporter (Completed)

- [x] **SPANG-070: Anki Flashcards Deck Exporter (`spanglings export --format anki`)**
  - **Description**: Export full SRS review decks, high-difficulty items, or custom tracks to Anki-compatible TSV format with tiered hints and diagnostic tags.
- [x] **SPANG-071: Markdown / Obsidian Study Guide Generator (`spanglings export --format markdown`)**
  - **Description**: Generate formatted markdown study notes summarizing learned grammar patterns, frequent pitfalls, and personal review metrics.
- [x] **SPANG-072: Portable State Backup & Cross-Machine Sync (`spanglings sync`)**
  - **Description**: Single-command portable JSON backup export (`spanglings sync --export <file>`) and merge import (`spanglings sync --import <file>`) to easily sync exercise completion and SRS states across workstations.

---

## 🔌 Focus Area 9: Language Server Protocol (LSP) Engine (`spanglings lsp`) (Completed)

- [x] **SPANG-080: `spanglings lsp` Standard JSON-RPC Server**
  - **Description**: Native stdio LSP server powering real-time editor integration for VS Code, Neovim, Zed, and Helix.
- [x] **SPANG-081: Live In-Editor Diagnostics & Hover Grammar Tooltips**
  - **Description**: Publishes live diagnostic squigglies on exercise markdown files as the developer types, with rich hover tooltips explaining grammar rules and conjugations.
- [x] **SPANG-082: Grammar Autocompletion & Code Action Quick Fixes**
  - **Description**: Intelligent autocomplete for Spanish technical vocabulary and conjugations, plus one-click Quick Fixes (*"Mark exercise as done (remove <!-- I AM NOT DONE -->)"*).
---

## 💻 Focus Area 10: VS Code & Cursor IDE Extension (`spanglings-vscode`) (Completed)

- [x] **SPANG-090: Extension Manifest & Bundling Pipeline (`editors/vscode`)**
  - **Description**: Package manifest, TypeScript configuration, and esbuild pipeline for `spanglings-vscode`.
- [x] **SPANG-091: Native LSP Client Lifecycle & Auto-Connecting Server**
  - **Description**: Uses `vscode-languageclient` to manage the `spanglings lsp` subprocess over stdio with concurrency guards and shell injection protection.
- [x] **SPANG-092: Activity Bar Sidebar Curriculum Explorer Tree View**
  - **Description**: Visual curriculum tree grouping tracks, displaying completion checkmarks (`✅` / `⬜`), CEFR badges, and opening exercise files on click.
- [x] **SPANG-093: Streak & Due Review Status Bar Widget**
  - **Description**: Real-time status bar widget showing active learning streaks 🔥 and due spaced repetition reviews with interactive markdown tooltips.
- [x] **SPANG-094: Command Palette Spanish Tools**
  - **Description**: Integrated commands for verb conjugation lookups (`Spanglings: Look Up Verb Conjugator`), grammar cheat sheets (`Spanglings: Browse Grammar Cheat Sheets`), and opening the next due exercise.

---

## 🌎 Focus Area 11: Latin American Spanish High-Impact Curriculum Expansion (Completed)

- [x] **SPANG-100: Track 31 - Mexican Tech, Startup & Venture Capital Spanish (B2–C1)**
  - **Topics**: *levantamiento de capital*, *ronda semilla*, *valuación pre-capital*, *tasa de abandono (churn)*, *ajuste producto-mercado*, *tasa de quema*, *dilución accionaria*.
- [x] **SPANG-101: Track 32 - Colombian & Andean Professional Nuances (B2–C1)**
  - **Topics**: *hacer una vuelta*, *estar pendiente de*, *caer en cuenta de que*, *echar reversa a*, *poner la lupa sobre*, *dar papaya*.
- [x] **SPANG-102: Track 33 - Rioplatense Production Voseo & Engineering (B2–C1)**
  - **Topics**: *sacar a producción*, *bancarse la carga*, *ponete las pilas*, *dar de baja la instancia*, *hacer un laburo fino*, *no te quedés atrás*.
- [x] **SPANG-103: Track 34 - LatAm Technical Anglicism Elimination (B2–C1)**
  - **Topics**: *rastrear* vs *trackear*, *personalizar* vs *customizar*, *rendir* vs *performar*, *desaconsejar/descontinuar* vs *deprecar*, *redirigir* vs *forwardear*, *restablecer* vs *resetear*.
- [x] **SPANG-104: Track 35 - Latin American Enterprise SLA & Risk Governance (C1)**
  - **Topics**: *acuerdos de confidencialidad*, *lucro cesante y daño emergente*, *cláusulas de rescisión por incumplimiento*, *planes de mitigación de riesgos*, *mantener indemne*, *penalizaciones económicas y créditos de servicio*.
---

## 🗣️ Focus Area 12: Standard Spanish, Everyday Life & General Conversational Expansion (Completed)

- [x] **SPANG-110: Track 36 - Everyday Life, Housing & Practical Bureaucracy (B1)**
  - **Topics**: Lease agreements & deposits (*contrato de arrendamiento*, *fianza*), setting up utilities (*dar de alta el suministro*), bank transfers & fees (*transferencia bancaria*, *comisión*), refunds (*solicitar un reembolso*), post office (*recoger un paquete*), transit transfers (*hacer transbordo*).
- [x] **SPANG-111: Track 37 - Healthcare, Medical Encounters & Symptoms (B1–B2)**
  - **Topics**: Physical symptoms & discomfort (*punzadas agudas*, *mareos*), prescriptions & dosages (*recetar antibióticos*, *dosis cada ocho horas*), OTC drugs & leaflets (*medicamento de venta libre*, *prospecto*), emergency room & discharge (*urgencias*, *alta médica*), specialist checkups (*cita previa*, *revisión anual*), allergies (*alérgico a frutos secos*, *intolerancia a la lactosa*).
- [x] **SPANG-112: Track 38 - Dining, Socializing, Small Talk & Nightlife (B1–B2)**
  - **Topics**: Split checks (*pedir la cuenta por separado*), meeting up & plans (*quedar a las ocho*, *cancelar a última hora*), compliments & reassurances (*lo bien que le quedaba*, *no te preocupes*), discourse softeners (*por cierto*, *la verdad es que*), banter (*armar un plan*, *pasarla muy bien*), friendly goodbyes (*estamos en contacto*, *avísame en cuanto llegues*).
- [x] **SPANG-113: Track 39 - Nuanced Prepositions & Spatial/Temporal Locutions (B1–C1)**
  - **Topics**: *Hacia* vs *Hasta* (direction vs endpoint), *Tras*, *Según* & *Bajo* (formal sequence & condition), compound locutions (*a base de*, *a expensas de*), elapsed past vs future spans (*al cabo de* vs *dentro de*), spatial extension (*a lo largo de*, *alrededor de*), origin & channels (*a raíz de*, *por medio de*).
- [x] **SPANG-114: Track 40 - Middle-Voice Shifts & Reflexive Nuances (B1–C1)**
  - **Topics**: *Ir* vs *Irse* (destination vs departure), *Dormir* vs *Dormirse* (state vs falling asleep), *Comer* vs *Comerse* (atelic vs total consumption), *Llevar* vs *Llevarse* (transport vs getting along / taking away), *Quedar* vs *Quedarse* (meeting/fit vs staying), *Volver* vs *Volverse* (return vs permanent character transformation).
- [x] **SPANG-115: Track 41 - Advanced Temporal, Manner & Concessive Adverbial Clauses (B2–C1)**
  - **Topics**: *A medida que* / *Conforme* (proportional change), *De modo que* (purpose vs consequence), *En tanto que* / *Mientras* (duration condition), *Tan pronto como* / *Apenas* (immediate succession), *Salvo que* / *A menos que* (exceptive subjunctive), *Siempre y cuando* / *A practical condition*.

---

## 🎯 Focus Area 13: Diagnostic Placement Testing & Level Fast-Tracking (Completed)

- [x] **SPANG-120: Calibrated CEFR Placement Battery & Evaluation Engine (`src/core/placement.rs`)**
  - **Description**: 15-question calibrated diagnostic battery spanning Baseline (A1–A2), B1, B2, and C1 evaluating verb inflection, clitic positioning, subjunctive triggers, discourse connectors, and advanced nuance with tiered scoring algorithms.
- [x] **SPANG-121: CLI Adaptive Diagnostic Test Subcommand (`spanglings test`)**
  - **Description**: Terminal test runner (`spanglings test [--level <LEVEL>] [--fast-track] [--json]`) assessing CEFR proficiency, generating diagnostic breakdown scorecards, and saving `EvaluatedLevel` in user state.
- [x] **SPANG-122: Automatic Fast-Track & SM-2 SRS Seeding (`AppState::fast_track_level`)**
  - **Description**: Automatically marks all exercises in mastered CEFR tiers as completed and initializes spaced repetition cards with optimal ease factors, skipping tedious baseline drilling for intermediate/advanced learners.
- [x] **SPANG-123: Interactive TUI Diagnostic Assessment Modal (`AppMode::PlacementTest`)**
  - **Description**: Full-featured in-TUI diagnostic test overlay accessible via `[t]` or `[F5]`, featuring question-by-question cloze inputs, live answer validation, CEFR breakdown tables, and one-key `[F]` fast-track action.
- [x] **SPANG-124: Verified CEFR Level Badge in Dashboard & LSP Status**
  - **Description**: Displays verified CEFR placement badges and diagnostic accuracy in `spanglings progress`, JSON metrics, and VS Code / LSP status bar.

---

## 🌐 Focus Area 14: Linguistic Knowledge Graph & Practical Everyday Expansion (Completed)

- [x] **SPANG-130: Directed Acyclic Graph (DAG) Linguistic Knowledge Graph (`src/core/graph.rs`)**
  - **Description**: In-memory static ontology DAG modeling 53 prerequisite relationships and conceptual primitives across grammar rules, aspectual shifts, and situational domains with cycle detection and learning frontier resolution.
- [x] **SPANG-131: Concept-Aware Exercise Model & Markdown Parser (`src/core/exercise.rs`)**
  - **Description**: Extended `Exercise` struct with `concept_tags`, `prerequisites`, `grammar_focus`, and `contrast_note` metadata with support for YAML frontmatter and single-line syntax.
- [x] **SPANG-132: Concept-Level SRS Mastery & Weakness Profiler (`src/core/state.rs`, `src/cli/commands/progress.rs`)**
  - **Description**: Aggregates SRS review performance into `ConceptMastery` scores, pinpoints foundational gaps across tracks, and recommends personalized learning frontiers in CLI and JSON output.
- [x] **SPANG-133: Concept-Aware Compiler Diagnostics & Modern Watcher Experience (`src/engine/diagnostics.rs`, `src/watcher/runner.rs`)**
  - **Description**: Concept-aware diagnostic error reporting cross-linking failed exercises to prerequisite foundation tracks with actionable remediation notes and contrast guidance. Modernized watcher with non-blocking raw keystrokes (`[n]`, `[p]`, `[r]`, `[q]`) eliminating all legacy comment busywork.
- [x] **SPANG-134: Curriculum Expansion: Tracks 42 to 47 (36 New Exercises, 267 Total)**
  - **Description**: Handcrafted situational synthesis tracks for Travel Logistics, Banking/Taxes, Consumer Complaints, Home Maintenance, News/Media, and Conversational Nuance.
- [x] **SPANG-135: Retroactive Conceptual Tagging of Tracks 00–41 (231 Exercises)**
  - **Description**: Tag all 231 existing exercises with concepts, prerequisites, and grammar focus notes, completely eliminating legacy `<!-- I AM NOT DONE -->` markers.
- [x] **SPANG-136: Concept CLI Filtering (`spanglings list --concept`, `spanglings drill --concept`)**
  - **Description**: Filter exercises and targeted conjugation drills by linguistic concept ID.

---

## 🧭 Focus Area 15: Interactive Onboarding Guided Tour (`spanglings tour`) (Completed)

- [x] **SPANG-140: First-Run Interactive Guided Onboarding Tour (`spanglings tour`)**
  - **Description**: Dedicated `spanglings tour` subcommand and first-run wizard in TUI introducing the developer workflow step-by-step:
    1. Navigation and split-pane interface (`Tab`, `Down`, `/` fuzzy search).
    2. Cloze input mechanics, UTF-8 accent support, and instant evaluation (`Enter`).
    3. Progressive 3-tier hints (`Ctrl+H` / `F1`) and reference cheat sheets (`Ctrl+E` / `F2`).
    4. Verb conjugator modal (`Ctrl+K` / `F3`) and cheat sheet browser (`Ctrl+B` / `F4`).
    5. Headless watch mode workflow (`spanglings watch`) with external editors (VS Code / Neovim / Zed).
    6. Taking diagnostic placement test (`spanglings test` / `[t]`) to fast-track known levels.
  - **Completed Components**:
    - [x] **State Persistence**: Added `AppState::tour_completed` flag with backward compatibility and `mark_tour_completed()`.
    - [x] **Station Engine**: Implemented 6 interactive guided stations with inline active-recall micro-challenges in `src/cli/commands/tour.rs`.
    - [x] **CLI Subcommand**: Added `spanglings tour [--skip-challenges]` CLI command with CI batch mode fallback.
    - [x] **Interactive TUI Integration**: First-run welcome modal dialog, in-TUI station modals, footer `[T]` hint, Help menu integration, and keyboard routing in `src/tui/events.rs`.
    - [x] **Test Battery**: Full integration test coverage in `tests/tour_tests.rs`, `tests/cli_tests.rs`, and `tests/tui_tests.rs`.

---

## 💎 Focus Area 16: Linguistic Completeness & Advanced Grammatical Subtleties (Completed)

- [x] **SPANG-150: Linguistic Knowledge Graph Expansion to 66 Concepts (`src/core/graph.rs`)**
  - **Description**: Expanded default linguistic ontology from 53 to 66 concepts covering epistemic conjecture, mandatory dative clitic doubling, fronted accusative left-dislocation, personal *a* animacy/semantic shifts, gerund restrictions/anglicism elimination, adversative coordinate systems (*pero* vs *sino* vs *sino que*), and optative/legal future subjunctives.
- [x] **SPANG-151: Six New Grammar Reference Cheat Sheets (`src/core/reference.rs`, `docs/grammar-reference.md`)**
  - **Topics**: `epistemic-conjecture`, `clitic-doubling`, `personal-a`, `gerund-rules`, `adversatives`, `legal-subjunctive` accessible via `spanglings explain <topic>` and the TUI cheat sheet browser.
- [x] **SPANG-152: Curriculum Expansion: Tracks 48 to 53 (36 New Exercises, 303 Total)**
  - **Description**: Authored 36 handcrafted exercises with 3-tier progressive hints and targeted diagnostic rules across:
    - Track 48: `48_epistemic_conjecture_and_probability` (B2–C1)
    - Track 49: `49_clitic_doubling_and_left_dislocation` (B1–B2)
    - Track 50: `50_personal_a_and_animacy_shifts` (B1–B2)
    - Track 51: `51_gerund_restrictions_and_anglicisms` (B2–C1)
    - Track 52: `52_adversative_pero_sino_sino_que` (B1–B2)
    - Track 53: `53_independent_subjunctives_and_legal_tenses` (C1)
- [x] **SPANG-153: Targeted Diagnostic Compiler Rules (E0048–E0053)**
  - **Description**: Added targeted diagnostic error codes and explanatory feedback in `src/engine/rules.rs` and `src/engine/validator.rs` for common native speaker traps.
- [x] **SPANG-154: Architectural Decision Record ADR-0003**
  - **Description**: Documented architecture and rationale in `docs/adr/0003-language-completeness-and-expanded-ontology.md`.

---

## 🏆 Focus Area 17: Full-Spectrum C1–C2 Linguistic Completeness (Completed)

- [x] **SPANG-160: Linguistic Knowledge Graph Expansion to 81 Concepts (`src/core/graph.rs`)**
  - **Description**: Expanded default ontology from 66 to 81 concepts covering verbs of becoming/transformation, epistemic adverbs with mood selection constraints, possessive and ethic datives, corrective and concessive polarities, participial absolute constructions, and scalar concession / intensive connectors.
- [x] **SPANG-161: Six New Grammar Reference Cheat Sheets (`src/core/reference.rs`, `docs/grammar-reference.md`)**
  - **Topics**: `verbs-of-becoming`, `epistemic-adverbs`, `possessive-datives`, `corrective-polarity`, `participial-absolutes`, `scalar-concession` (24 reference cards total).
- [x] **SPANG-162: Curriculum Expansion: Tracks 54 to 59 (36 New Exercises, 339 Total across 60 Tracks)**
  - **Description**: Authored 36 handcrafted exercises with 3-tier progressive hints and targeted diagnostic rules across:
    - Track 54: `54_verbs_of_becoming_and_transformation` (B2–C1)
    - Track 55: `55_epistemic_adverbs_and_mood_selection` (B1–C1)
    - Track 56: `56_datives_of_possession_and_ethic_datives` (B1–C1)
    - Track 57: `57_corrective_and_concessive_polarities` (B2–C1)
    - Track 58: `58_participial_absolute_constructions` (C1–C2)
    - Track 59: `59_scalar_concession_and_intensive_connectors` (C1–C2)
- [x] **SPANG-163: Targeted Diagnostic Compiler Rules (E0054–E0059)**
  - **Description**: Added targeted diagnostic error codes in `src/engine/rules.rs` and `src/engine/validator.rs`.
- [x] **SPANG-164: Architectural Decision Record ADR-0004**
  - **Description**: Documented architecture and rationale in `docs/adr/0004-full-spectrum-c1-c2-linguistic-completeness.md`.

---

## 🔮 Future Horizons & Stretch Ideas (Optional / Community)

- [ ] **Homebrew Formula / Tap (`brew install spanglings`)**:
  - Package binary for macOS/Linux users who don't have the Rust toolchain installed.
- [ ] **WebAssembly In-Browser Playground (`wasm-pack`)**:
  - Run the Spanglings validator and interactive exercises directly on the documentation website via WebAssembly.
- [ ] **Audio & Pronunciation Synthesis (Optional CLI flag `--audio`)**:
  - Optional integration with native text-to-speech engines (`say` on macOS, `espeak-ng` on Linux) to speak prompts and correct solutions aloud during watch mode and TUI exercises.
