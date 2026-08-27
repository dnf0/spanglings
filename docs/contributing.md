# Contributing to Spanglings

We welcome contributions to Spanglings! Whether you're adding new Spanish curriculum tracks, refining diagnostics, improving the interactive TUI, or optimizing the LSP engine, here is everything you need to know.

---

## 🛠️ Development Setup

### Prerequisites
- **Rust toolchain** (1.75+): Install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Build & Run Tests
```bash
# Clone the repository
git clone https://github.com/dnf0/spanglings.git
cd spanglings

# Build in debug mode
cargo build

# Run complete test suite (26+ test files)
cargo test

# Run linter checks
cargo clippy --all-targets -- -D warnings

# Check code formatting
cargo fmt --check
```

---

## 🏗️ Codebase Architecture

```
src/
├── core/             # Core domain models, state persistence, SRS, DAG ontology
│   ├── exercise.rs   # Exercise markdown parser and metadata extractor
│   ├── graph.rs      # Linguistic Knowledge Graph (66-concept DAG)
│   ├── placement.rs  # Calibrated CEFR placement battery & evaluation
│   ├── reference.rs  # Grammar reference cards & cheat sheets
│   └── state.rs      # User progress persistence, SM-2 cards, concept mastery
├── engine/           # Evaluation, normalizer, and diagnostic compiler
│   ├── diagnostics.rs# Rustc-style terminal compiler diagnostics
│   ├── normalizer.rs # Smart accent matching and whitespace stripping
│   └── validator.rs  # Submission evaluation against expected solutions
├── tui/              # Interactive Ratatui Terminal UI
│   ├── app.rs        # State machine, key handlers, and modal managers
│   ├── events.rs     # Crossterm terminal event pump
│   └── ui.rs         # Responsive layout renderers for all modals and panes
├── watcher/          # Headless file watcher and runner engine
├── lsp/              # Language Server Protocol stdio implementation
└── cli/              # Subcommand handlers and argument routing
```

---

## ✍️ Authoring Curriculum Exercises

Exercises are structured Markdown files residing under `exercises/<track_name>/<exercise_num>_<title>.md`.

### Exercise Format Specification

```markdown
# Verb Aspect in Past Narratives
- Level: B2
- Topic: Past Tenses & Aspectual Shifts
- Concepts: preterite_vs_imperfect_aspect, aspectual_shift_verbs
- Prerequisites: irregular_preterite_stems, imperfect_habitual_markers
- Grammar Focus: "Conocer in the preterite denotes first encounter or meeting."
- Contrast Note: "Conocí (I met for the first time) vs Conocía (I knew for years)."

## Prompt
Fill in the correct form of the verb in parentheses.
Context: Recounting when you first met a collaborator.

Ayer ___ (conocer) a la nueva ingeniera de sistemas en el standup.

## Hints
<!-- Tier 1: This event describes a punctual first encounter. -->
<!-- Tier 2: First-person singular preterite of conocer (regular -í ending). -->
<!-- Tier 3: conocí -->
```

### Exercise Requirements Checklist
- [x] **No Legacy Markers**: Never include `<!-- I AM NOT DONE -->` comments.
- [x] **Ontology Tags**: Every `Concepts` and `Prerequisites` identifier must exist in `src/core/graph.rs`.
- [x] **Progressive Hints**: Must contain exactly 3 tiers (`Tier 1: conceptual`, `Tier 2: morphological`, `Tier 3: solution`).
- [x] **Accents**: Specify precise Spanish orthography and accents in Tier 3 solutions.

---

## 📋 Git Commit Standards

We enforce [Conventional Commits](https://www.conventionalcommits.org/):

- `feat(scope): ...` for new features
- `fix(scope): ...` for bug fixes
- `docs(scope): ...` for documentation updates
- `test(scope): ...` for test additions
- `refactor(scope): ...` for architectural improvements
