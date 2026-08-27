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

## 🖥️ Focus Area 6: In-TUI Power Tools & Modal Modifiers

- [ ] **SPANG-050: In-TUI Verb Conjugator Modal (`c` key)**
  - **Description**: Press `c` inside the TUI to open a non-disruptive popup modal to search and conjugate any verb across all tenses without leaving the exercise workspace.
- [ ] **SPANG-051: In-TUI Grammar Reference Card Browser Modal (`r` key)**
  - **Description**: Press `r` inside the TUI to open an interactive modal with full-text search across all grammar cheat sheets (Subjunctive, Por vs Para, Clitics, Accents, etc.).
- [ ] **SPANG-052: In-TUI Drill & 60-Second Blitz Launcher (`d` and `b` keys)**
  - **Description**: Launch rapid-fire drill trials and the 60-second Blitz directly inside the TUI with instant live scoreboard and keyboard response.
- [ ] **SPANG-053: In-TUI Keybinding Overlay & Quick Command Palette (`?` / `F1`)**
  - **Description**: Comprehensive searchable help modal listing all hotkeys, navigation shortcuts, and workflow commands.

---

## 📚 Focus Area 7: Advanced C1 Technical, RFC & Professional Track Expansion

- [ ] **SPANG-060: Track 27 - System Architecture RFCs, Post-Mortems & Incident Response (B2–C1)**
  - **Topics**: Outage post-mortems (*caída del servicio*, *análisis de causa raíz*, *remediación*), architectural RFCs (*desacoplar componentes*, *tolerancia a fallos*, *rendimiento sostenido*).
- [ ] **SPANG-061: Track 28 - Complex Subjunctive Triggers & Concessive Clauses (C1)**
  - **Topics**: Concessive expressions (*por mucho que*, *comoquiera que*, *quiera o no*, *a condición de que*, *tan pronto como* + subjuntivo vs indicativo).
- [ ] **SPANG-062: Track 29 - Advanced Verbal Periphrases & Nuanced Aspect (B2–C1)**
  - **Topics**: Modal and aspectual periphrases (*llevar + gerundio*, *andar + gerundio*, *darle a uno por*, *quedar en*, *dejar de*, *acabar por*).
- [ ] **SPANG-063: Track 30 - Executive Debate & Open Source Leadership Collocations (C1)**
  - **Topics**: High-register idiomatic debate phrases (*hacer hincapié*, *poner en tela de juicio*, *dar el visto bueno*, *llevar la voz cantante*, *zanjar la disputa*).

---

## 📦 Focus Area 8: Portable Study Pack & Anki/Markdown Exporter

- [ ] **SPANG-070: Anki Flashcards Deck Exporter (`spanglings export anki`)**
  - **Description**: Export full SRS review decks, high-difficulty items, or custom tracks to Anki-compatible TSV/APKG format with tiered hints and diagnostic tags.
- [ ] **SPANG-071: Markdown / Obsidian Study Guide Generator (`spanglings export markdown`)**
  - **Description**: Generate formatted markdown study notes summarizing learned grammar patterns, frequent pitfalls, and personal review metrics.
- [ ] **SPANG-072: Encrypted / Portable State Backup & Cross-Machine Sync (`spanglings sync`)**
  - **Description**: Single-command encrypted export (`spanglings sync export`) and import (`spanglings sync import`) to easily sync exercise completion and SRS states across workstations.

---

## 🔌 Focus Area 9: Language Server Protocol (LSP) Engine (`spanglings lsp`)

- [ ] **SPANG-080: `spanglings lsp` Standard JSON-RPC Server**
  - **Description**: Native stdio LSP server powering real-time editor integration for VS Code, Neovim, Zed, and Helix.
- [ ] **SPANG-081: Live In-Editor Diagnostics & Hover Grammar Tooltips**
  - **Description**: Publishes live diagnostic squigglies on exercise markdown files as the developer types, with rich hover tooltips explaining grammar rules and conjugations.
- [ ] **SPANG-082: Grammar Autocompletion & Code Action Quick Fixes**
  - **Description**: Intelligent autocomplete for Spanish verb forms, accents, and triggers, plus one-click Quick Fixes (*"Fix accents"*, *"Mark exercise as completed"*).

