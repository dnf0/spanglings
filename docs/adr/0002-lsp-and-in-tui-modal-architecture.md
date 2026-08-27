# ADR-0002: Language Server Protocol (LSP) Engine & In-TUI Modal Architecture

## Status
Accepted

## Context
As Spanglings expanded to support 30 comprehensive curriculum tracks (covering B1-C1 grammar, distributed system architecture RFCs, and executive leadership collocations), developers needed seamless, non-disruptive access to reference material and instant feedback both inside the terminal and inside their primary code editors (VS Code, Neovim, Helix, Zed).

Prior to this architecture:
1. Reviewing verb conjugations or reference cards required exiting or opening a secondary terminal window to execute CLI commands (`spanglings explain`, `spanglings conjugate`).
2. External editor integrations relied on polling or CLI check executions (`spanglings check --json`).
3. Flashcard learners who use Anki or Obsidian could not easily export their SRS progression or study guides.

## Decisions

### 1. In-TUI Non-Disruptive Popup Modals
- Extended `ratatui` rendering pipeline with a centralized `centered_rect` layer using `Clear` widgets.
- Added dedicated modal states in `AppMode`:
  - `AppMode::Conjugating`: Real-time verb search and colorized 7-tense pronoun grid with irregular stem highlights.
  - `AppMode::BrowsingReference`: Split-pane topic selector and markdown reference cheat sheet viewer with real-time filtering.
  - `AppMode::Help`: Overlay detailing all shortcuts, power tools, and workflow bindings.
- Bound modal activations to global hotkeys (`F1`/`F3`/`F4`, `Ctrl+K`, `Ctrl+B`, `?`) and `Esc` for instant dismissal back to editing mode.

### 2. Native Stdio Language Server Protocol (LSP) Engine (`spanglings lsp`)
- Implemented a lightweight, zero-dependency JSON-RPC 2.0 stdio server supporting:
  - `initialize` / `shutdown` protocol handshake.
  - `textDocument/didOpen`, `textDocument/didChange`: Real-time incremental/full diagnostic streaming with warnings for missing accents and errors for grammatical/calque mistakes.
  - `textDocument/hover`: Instant Markdown popups rendering complete conjugation tables for verbs under cursor and grammar reference cheat sheets for grammatical topics.
  - `textDocument/completion`: Authentic technical Spanish completions (e.g. *conmutación por error*, *desplegar*).
  - `textDocument/codeAction`: One-click QuickFix action to remove `<!-- I AM NOT DONE -->`.

### 3. Portable Study Notes & Spaced Repetition Exporter
- Created `spanglings export`:
  - `--format anki`: Emits tab-separated values (TSV) with HTML formatting, hints, and tagging compatible with Anki deck imports.
  - `--format markdown`: Emits clean, topic-indexed study guides suitable for Obsidian or GitHub wikis.
  - `--format json`: Emits machine-readable curriculum and review performance metadata.
- Created `spanglings sync`:
  - `--export <file>`: Exports portable learning state and SRS history with version metadata.
  - `--import <file>`: Merges external state with local history, maintaining highest repetition intervals and latest review timestamps.

## Consequences
- **Positive**: Zero context switching for learners: reference cheat sheets and verb tables are accessible within 1 keystroke inside both the interactive TUI and code editors.
- **Positive**: Complete portability: study progress can be synchronized across machines or exported to external flashcard ecosystems.
- **Positive**: High testability: LSP protocol messages and modal state machines are 100% covered by deterministic integration tests.
