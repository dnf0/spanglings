# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
