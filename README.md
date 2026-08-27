# Spanglings 🇪🇸 🦀

[![CI](https://github.com/dnf0/spanglings/actions/workflows/ci.yml/badge.svg)](https://github.com/dnf0/spanglings/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust: 1.75+](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Docs](https://img.shields.io/badge/docs-gh--pages-blue)](https://dnf0.github.io/spanglings/)
[![Curriculum: 60 Tracks • 339 Exercises](https://img.shields.io/badge/Curriculum-60%20Tracks%20%E2%80%A2%20339%20Exercises-emerald.svg)](https://dnf0.github.io/spanglings/syllabus/)
[![CEFR: A1 to C1](https://img.shields.io/badge/CEFR-A1%20%E2%86%92%20C1%20Mastery-gold.svg)](https://dnf0.github.io/spanglings/syllabus/)

> **Spanglings builds the syntax compiler; real-world usage supplies the data.**
> A developer-grade CLI & interactive TUI for mastering Spanish grammar, verb mechanics, and nuanced syntactic architecture, inspired by [Rustlings](https://github.com/rust-lang/rustlings) and [Raylings](https://github.com/dnf0/raylings).

<p align="center">
  <img src="docs/assets/spanglings-demo.svg" alt="Spanglings Terminal Watcher Demo" width="850">
</p>

---

## Pedagogical Philosophy: The Syntax Compiler Model

Most language learning platforms rely on gamified flashcards and repetitive multiple-choice matching. They fail to teach the underlying **generative mental models** that developers use every day. 

Natural languages are structural systems with strict morphological transformations, scope rules, and pragmatic contracts. **Spanglings** approaches Spanish acquisition through a **compiler-driven pedagogical architecture**:

1. **Active Debugging with Zero Busywork**: Every exercise starts in an incomplete state. Modify code in your editor; the watcher validates your submissions on save in < 20ms with zero comment-deletion busywork.
2. **Rustc-Style Grammar Diagnostics**: When you make a grammatical error (such as selecting indicative where subjunctive is mandated), Spanglings produces rich compiler-grade error diagnostics (`error[E0301]: Subjunctive Mood Required`), pinpointing the exact offending token, underlying grammar focus, linked DAG concept, and contrast notes.
3. **81-Concept Ontological Knowledge Graph (DAG)**: All 339 exercises are mapped onto an 81-concept Directed Acyclic Graph. The engine dynamically computes your **learning frontier**, prerequisite dependencies, and concept decay rates over time.
4. **SM-2 Spaced Repetition & Weakness Profiling**: Integrates SuperMemo-2 active recall scheduling. Exercises adapt based on ease factors, error frequency, and linguistic topic lapses.
5. **Dual-Interface Synergy**: Seamlessly alternate between a full-screen terminal app (`spanglings` / `ratatui` TUI) and a headless watcher + native IDE Language Server Protocol engine (`spanglings lsp`) in VS Code, Cursor, Neovim, Helix, or Zed.

---

## Architecture

```
                                  +-----------------------+
                                  |     User Terminal     |
                                  | (VS Code / Neovim/Zed)|
                                  +-----------+-----------+
                                              |
                                              v
                                  +-----------------------+
                                  |    Spanglings CLI     |
                                  +-----------+-----------+
                                              |
                     +------------------------+------------------------+
                     |                                                 |
                     v                                                 v
         +-----------------------+                         +-----------------------+
         |  File Watcher Engine  |                         | Ratatui Terminal TUI  |
         | (notify / hotkeys)    |                         |  (Dual-Pane Editor)   |
         +-----------+-----------+                         +-----------------------+
                     |                                                 |
                     +------------------------+------------------------+
                                              |
                                              v
                                  +-----------------------+
                                  |  Diagnostic Compiler  |
                                  | (Rustc-Style E-Codes) |
                                  +-----------+-----------+
                                              |
                     +------------------------+------------------------+
                     |                                                 |
                     v                                                 v
         +-----------------------+                         +-----------------------+
         | 81-Concept DAG Graph  |                         |  SM-2 Spaced Recall   |
         |  (Learning Frontiers) |                         |   & Weakness Profiler |
         +-----------+-----------+                         +-----------------------+
                     |                                                 |
                     +------------------------+------------------------+
                                              |
                                              v
                                  +-----------------------+
                                  |  Curriculum Catalog   |
                                  | 60 Tracks / 339 Exs.  |
                                  | (Embedded in Binary)  |
                                  +-----------------------+
```

---

## Quickstart & Installation

### Option 1: Install via Cargo (Recommended)

```bash
# Install globally
cargo install spanglings

# Initialize the curriculum workspace in any directory
spanglings init

# Start learning immediately with the interactive TUI
spanglings
```

### Option 2: Pre-Built Binary Releases

Download pre-compiled binaries for Linux (x86_64 / aarch64), macOS (Apple Silicon / Intel), and Windows from the [GitHub Releases](https://github.com/dnf0/spanglings/releases) page.

### Option 3: Build from Source

```bash
git clone https://github.com/dnf0/spanglings.git
cd spanglings
cargo build --release
./target/release/spanglings init
./target/release/spanglings
```

---

## Interactive Learning Modes

### 1. 🚀 Interactive Terminal UI (`spanglings` / `spanglings tui`)

Full-screen, distraction-free terminal learning environment with dual-pane code viewing, instant syntax validation, live status counters, and integrated pop-up modals:

```bash
spanglings
```

| Keybinding | Action |
| :--- | :--- |
| `Enter` | Submit current exercise solution |
| `/` | Open live fuzzy search and filter curriculum |
| `Ctrl+H` / `F1` | Cycle progressive hints (Tier 1: Clue $\rightarrow$ Tier 2: Structure $\rightarrow$ Tier 3: Solution) |
| `Ctrl+E` / `F2` | Toggle instant Grammar Reference Cheat Sheet for active exercise |
| `Ctrl+K` / `F3` | Open real-time Verb Conjugator modal |
| `Ctrl+B` / `F4` | Open Searchable Grammar Reference Browser (24 cheat sheets) |
| `[p]` / `F5` | Launch CEFR Placement Assessment & Fast-Track dialog |
| `[T]` / `F6` | Open 6-Station Interactive Onboarding Tour |
| `Tab` / `Ctrl+N` | Navigate to next exercise |
| `BackTab` / `Ctrl+P` | Navigate to previous exercise |
| `Ctrl+R` | Reset exercise to initial template |
| `Esc` / `Ctrl+C` | Close modal / cancel search / exit |

---

### 2. ⚡ Modern Headless Watcher (`spanglings watch`)

Work directly inside your favorite text editor (VS Code, Cursor, Neovim, Helix, Zed). Spanglings continuously monitors exercise files, re-evaluating on save with interactive keyboard navigation:

```bash
spanglings watch
```

> **Interactive Hotkeys**:
> - `n` / `Enter` : Advance to next exercise
> - `p` : Jump to previous exercise
> - `r` : Force re-run validation
> - `h` : Reveal next hint tier
> - `c` : Prompt for instant verb conjugation
> - `q` : Exit watcher cleanly

---

### 3. 🎯 Diagnostic CEFR Placement Assessment (`spanglings test`)

Evaluate your Spanish baseline across CEFR tiers (A1 to C1) with a calibrated multi-tier test battery. Automatically fast-tracks mastered levels:

```bash
# Run full calibrated 15-question placement assessment
spanglings test

# Assess specific level and fast-track upon passing
spanglings test --level b1 --fast-track

# Output machine-readable JSON evaluation
spanglings test --json
```

---

### 4. 📖 In-Terminal Grammar Explainers (`spanglings explain`)

Query conceptual cheat sheets directly by topic name, concept identifier, or compiler error code:

```bash
# Query by topic name
spanglings explain subjunctive
spanglings explain por-para
spanglings explain accidental-se

# Query directly by compiler error code
spanglings explain E0301
spanglings explain E0701
spanglings explain E0054
```

---

### 5. 🔄 Spaced Repetition Review (`spanglings review` & `spanglings drill`)

Reinforce active recall using the SuperMemo-2 (SM-2) algorithm. Spanglings schedules due exercises and prioritizes conceptual weak spots:

```bash
# Review items currently due for recall
spanglings review

# Targeted drill focusing on a specific grammar concept
spanglings drill --concept subjunctive_volition_influence

# 60-second rapid-fire conjugation speed drill
spanglings blitz
```

---

### 6. 🔌 Native Language Server Protocol (LSP) Integration

Spanglings ships with a built-in LSP server (`spanglings lsp`) providing hover tooltips, diagnostics, completions, and code actions in any editor:

#### VS Code / Cursor Integration
Add to `.vscode/settings.json`:
```json
{
  "spanglings.serverPath": "spanglings",
  "spanglings.enableHover": true,
  "spanglings.enableDiagnostics": true
}
```

#### Neovim (`nvim-lspconfig`)
```lua
local lspconfig = require("lspconfig")
local configs = require("lspconfig.configs")

if not configs.spanglings then
  configs.spanglings = {
    default_config = {
      cmd = { "spanglings", "lsp" },
      filetypes = { "markdown" },
      root_dir = lspconfig.util.root_pattern(".git", "spanglings.toml"),
    },
  }
end
lspconfig.spanglings.setup({})
```

#### Helix (`languages.toml`)
```toml
[[language]]
name = "markdown"
language-servers = [ "spanglings-lsp" ]

[language-server.spanglings-lsp]
command = "spanglings"
args = ["lsp"]
```

---

## 🗺️ Comprehensive Curriculum Map (60 Tracks • 339 Exercises)

| CEFR Tier | Tracks | Exercise Count | Core Grammatical & Practical Scope |
| :--- | :---: | :---: | :--- |
| **A1 — Survival & Foundations** | `00`–`02` | 18 exercises | Baseline irregulars, *ser vs estar* state/identity, regular present paradigms |
| **A2 — Daily Routine & Aspect** | `03`–`11` | 54 exercises | Preterite vs Imperfect aspectual shifts, stem-changers, direct/indirect pronouns |
| **B1 — The Independent Threshold** | `12`–`27` | 96 exercises | Subjunctive WEIRDO triggers, *por vs para*, clitic stacking (*se lo*), accidental *se*, relative clauses, conditional hypothesis |
| **B2 — Professional & Technical Fluency** | `28`–`41` | 84 exercises | Software engineering collocations, business correspondence, hypothetical *si* clauses, passive *se*, verbal periphrases, false cognates |
| **C1 — Pragmatics & Advanced Discourse** | `42`–`59` | 87 exercises | Travel logistics & disputes, banking & taxation, consumer rights, housing maintenance, healthcare emergencies, academic debate, epistemic conjecture, clitic doubling, personal *a*, gerund restrictions, adversatives (*pero/sino/sino que*), archaic/legal subjunctives, verbs of becoming, epistemic adverbs, possessive datives, corrective polarity, participial absolutes, and scalar concessions (*por mucho que*) |

---

## Complete CLI Reference

```
Usage: spanglings [OPTIONS] [COMMAND]

Commands:
  watch        Watch exercise files and re-evaluate on file save
  init         Initialize exercises in the current directory or target path
  tour         Take interactive guided onboarding tour of Spanglings philosophy & tools
  run          Run and validate a specific exercise by path or ID
  hint         Show progressive hints for an exercise (Tier 1 to 3)
  list         List all curriculum exercises and completion statuses (supports --concept)
  progress     Display learning progress across CEFR levels and concept mastery
  search       Search exercises by topic, keyword, or grammar concept
  check        Check exercise file for errors or stream JSON editor diagnostics
  test         Run calibrated CEFR placement diagnostic test & level fast-track
  drill        Start an active-recall flashcard drill session (supports --concept)
  blitz        Start 60-second rapid-fire conjugation speed drill
  review       Review exercises due for Spaced Repetition (SM-2)
  explain      Display in-terminal grammar cheat sheet for a topic or error code
  conjugate    Look up full conjugation tables and tenses for any Spanish verb
  export       Export study materials to Anki TSV, Markdown guide, or JSON
  sync         Backup, restore, or merge learning state and review history
  lsp          Start Language Server Protocol (LSP) stdio server for editor integrations
  hook         Manage Git pre-commit / pre-push Spanish practice hooks
  pack         Manage, scaffold, and validate custom curriculum exercise packs
  completions  Generate shell auto-completions (bash, zsh, fish, powershell)
  reset        Reset an exercise to its initial prompt
  tui          Launch the interactive terminal UI
  help         Print this message or the help of the given subcommand(s)

Options:
      --strict-accents  Require exact accent marks and tildes
      --json            Output results in JSON format
  -h, --help            Print help
  -V, --version         Print version
```

---

## 🌐 The *lings Ecosystem

If you enjoy hands-on, terminal-driven technical mastery, explore our companion platforms:

- ☸️ [**Kubelings**](https://github.com/dnf0/kubelings) — Hands-on interactive CLI learning environment for Kubernetes.
- 🏗️ [**Terralings**](https://github.com/dnf0/terralings) — Master Terraform and OpenTofu through interactive infrastructure-as-code exercises.
- ⚡ [**Raylings**](https://github.com/dnf0/raylings) — Learn distributed AI, Ray Core actors, and scalable clusters through hands-on Python exercises.
- 🦀 [**Rustlings**](https://github.com/rust-lang/rustlings) — Small exercises to get you used to reading and writing Rust code.

---

## Documentation

Full documentation is available at **[https://dnf0.github.io/spanglings/](https://dnf0.github.io/spanglings/)**:

- 🚀 [Getting Started](https://dnf0.github.io/spanglings/getting-started/) — Prerequisites, installation methods, and your first 5 minutes.
- 🧭 [Onboarding & Learner's Guide](https://dnf0.github.io/spanglings/onboarding-guide/) — In-depth guide to the tour, watcher shortcuts, and compiler mental models.
- 📚 [Curriculum Syllabus](https://dnf0.github.io/spanglings/syllabus/) — Complete 60-track syllabus map spanning all 339 exercises.
- 📖 [Grammar Cheat Sheets & Diagnostics](https://dnf0.github.io/spanglings/grammar-reference/) — 24 reference cards with error code indexes.
- ⌨️ [CLI & TUI Reference Manual](https://dnf0.github.io/spanglings/cli-reference/) — Comprehensive reference for all CLI subcommands and JSON schemas.
- 🤝 [Contributing Guide](https://dnf0.github.io/spanglings/contributing/) — Authoring exercises, ontological DAG extensions, and test suites.

---

## License

This project is licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
