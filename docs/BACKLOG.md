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
