# Spanglings 🇪🇸 🦀

[![CI](https://github.com/dnf0/spanglings/actions/workflows/ci.yml/badge.svg)](https://github.com/dnf0/spanglings/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/dnf0/spanglings/blob/main/LICENSE-MIT)
[![Rust: 1.75+](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Curriculum: 60 Tracks • 339 Exercises](https://img.shields.io/badge/Curriculum-60%20Tracks%20%E2%80%A2%20339%20Exercises-emerald.svg)](syllabus.md)
[![CEFR: A1 to C1](https://img.shields.io/badge/CEFR-A1%20%E2%86%92%20C1%20Mastery-gold.svg)](syllabus.md)

> **Spanglings builds the syntax compiler; real-world usage supplies the data.**  
> A developer-grade CLI, interactive TUI, and web platform for mastering Spanish grammar, aspectual mechanics, and nuanced syntactic architecture.

---

<div class="grid cards" markdown>

-   :material-book-open-page-variant:{ .lg .middle } __Spanish Language Manual__

    ---

    Explore **24 comprehensive chapters** with dual-layer explanations: cognitive communicative mental models and compiler-style grammar decision matrices.

    [:octicons-arrow-right-24: Read Spanish Language Manual](manual.md)

-   :material-lightning-bolt:{ .lg .middle } __Interactive Web Playground__

    ---

    Practice in your browser with zero installation. Experience the **24-Topic Curriculum Syntax Studio** and real-time **Rapid Showdown Duel Arena**.

    [:octicons-arrow-right-24: Launch Web Playground](playground/index.html)

-   :material-format-list-bulleted-type:{ .lg .middle } __Curriculum Syllabus & Graph__

    ---

    Explore **60 tracks**, **339 exercises**, and the **81-concept Directed Acyclic Graph (DAG)** powering adaptive weakness profiling.

    [:octicons-arrow-right-24: View Curriculum Syllabus](syllabus.md)

</div>

---

<p align="center">
  <img src="assets/spanglings-demo.svg" alt="Spanglings Terminal Watcher Demo" width="850">
</p>

Inspired by [Rustlings](https://github.com/rust-lang/rustlings) and [Raylings](https://github.com/dnf0/raylings), **Spanglings** provides a terminal-native and browser-accessible hands-on learning environment for engineers, developers, and power users who want to master authentic Spanish syntax, aspectual contrasts (*pretérito vs imperfecto*), subjunctive triggers, accidental *se*, and professional collocations without childish gamification.

---

## 🏛️ Language Architecture: 24 Topics across 3 CEFR Tiers

Spanglings categorizes the entire Spanish grammar continuum into 24 core pedagogical domains across 3 CEFR competency tiers:

### 🟢 Tier 1: Foundations & Aspectual Geometry (A1–A2)
Fundamental distinctions and syntactic ordering:
- **Ser vs. Estar** (`#ser-estar`): Essence & identity vs. transient state, posture, and location.
- **Por vs. Para** (`#por-para`): Backward motive & trajectory vs. forward goal, recipient & deadline.
- **Past Tenses & Aspect** (`#past-tenses`): Completed bounded events (preterite) vs. narrative background (imperfect).
- **Pronoun Stacking** (`#pronouns`), **Verbs like Gustar** (`#gustar`), **Reflexive Verbs** (`#reflexive`), **Stem-Changing Verbs** (`#stem-changing`), and **Prepositional Regimen** (`#prepositions`).

### 🟡 Tier 2: Mood, Triggers & Pragmatic Voice (B1–B2)
Modality, hypothetical conditions, and non-agentive structures:
- **Present Subjunctive** (`#subjunctive`): WEIRDO volition, emotion, doubt, and non-existent antecedents.
- **Imperfect Subjunctive** (`#imperfect-subjunctive`): Counterfactual *si*-clauses and polite hypothetical requests.
- **Imperative Mood** (`#imperative`): Affirmative and negative commands with direct clitic attachment.
- **Accidental "Se"** (`#accidental-se`): De-agentified involuntary actions (*se me cayó*).
- **Passive vs. Impersonal "Se"** (`#passive-impersonal-se`), **Possessive Datives** (`#possessive-datives`), **Relative Pronouns** (`#relative-pronouns`), and **Gerund Syntax Rules** (`#gerund-rules`).

### 🟣 Tier 3: Advanced Nuance, Registers & Edge Mechanics (B2–C1)
Subtle semantic shifts, pragmatic discourse markers, and specialized domains:
- **Verbs of Becoming** (`#verbs-of-becoming`): Nuanced transformations (*ponerse*, *quedarse*, *hacerse*, *volverse*, *convertirse en*).
- **Scalar Concession** (`#scalar-concession`): Intensive concessive polarity (*por mucho que*, *aun a riesgo de que*).
- **Epistemic Conjecture** (`#epistemic-conjecture`): Future and conditional of probability (*serán las 4*, *estaría cansado*).
- **Adversatives & Rectification** (`#adversatives`): Restrictive *pero* vs. exclusive corrective *sino* / *sino que*.
- **False Friends** (`#false-friends`), **Voseo Conjugation** (`#voseo`), **Software & Tech Spanish** (`#tech`), and **Legal & Statutory Spanish** (`#legal`).

👉 Read the in-depth rules and mental models in the [📘 Spanish Language Manual](manual.md) or practice them in the [⚡ Interactive Playground](playground/index.html).


---

## Pedagogical Philosophy: The Syntax Compiler Model

Learning a language through passive flashcards or multiple-choice apps is frustrating for technical minds because it hides the underlying structural mental model. **Spanglings** treats Spanish grammar like a compiled language with strict morphological transformations, scope rules, and pragmatic contracts:

1. ⚡ **Active Debugging & Iteration**: Every exercise starts in an incomplete state with clear instructions. Modify the file in your favorite editor; Spanglings validates your submission on save in < 20ms with zero comment-deletion busywork.
2. 🔍 **Rustc-Style Grammar Diagnostics**: When you encounter a grammatical pitfall, Spanglings generates colored, compiler-grade error diagnostics (`error[E0301]: Subjunctive Mood Required`), pinpointing the exact offending token, grammatical rationale, linked concept, and contrast notes.
3. 🌐 **81-Concept Ontological Knowledge Graph (DAG)**: All 339 exercises are anchored to a Directed Acyclic Graph. The engine dynamically computes your **learning frontier**, prerequisite dependencies, and concept decay rates over time.
4. 🧠 **SM-2 Spaced Repetition & Weakness Profiling**: Integrates SuperMemo-2 active recall scheduling. The scheduler prioritizes exercises based on ease factors, error frequency, and linguistic topic lapses.
5. 🔀 **Dual-Interface Synergy**: Seamlessly alternate between a full-screen terminal app (`spanglings` / `ratatui` TUI) and a headless watcher + native IDE Language Server Protocol engine (`spanglings lsp`) in VS Code, Cursor, Neovim, Helix, or Zed.

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

## Quickstart

=== "Interactive Web Playground (Zero Install)"
    Practice directly in your browser with zero installation:  
    👉 **[Open Spanglings Interactive Playground](playground/index.html)**

=== "Cargo (Recommended)"
    ```bash
    # Install globally
    cargo install spanglings

    # Initialize exercise catalog in current directory
    spanglings init

    # Start interactive TUI
    spanglings
    ```

=== "Pre-Built Binaries"
    Download pre-compiled binaries for Linux, macOS (Apple Silicon & Intel), and Windows from the [GitHub Releases](https://github.com/dnf0/spanglings/releases) page.

=== "Build from Source"
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

### 2. ⚡ Modern Headless Watcher (`spanglings watch`)
Work directly in your favorite editor (VS Code, Cursor, Neovim, Helix, Zed). Spanglings continuously validates files on save with interactive non-blocking terminal keystrokes:
```bash
spanglings watch
```
> **Interactive Hotkeys**: `n` / `Enter` (Next), `p` (Previous), `r` (Rerun), `h` (Hint), `c` (Conjugate), `q` (Quit).

### 3. 🎯 Diagnostic CEFR Placement Assessment (`spanglings test`)
Evaluate your Spanish baseline across CEFR tiers (A1 to C1) with a calibrated multi-tier test battery and automatic fast-tracking:
```bash
spanglings test
spanglings test --level b1 --fast-track
```

### 4. 📖 In-Terminal Grammar Explainers (`spanglings explain`)
Query conceptual cheat sheets directly by topic name, concept identifier, or compiler error code:
```bash
spanglings explain subjunctive
spanglings explain E0301
spanglings explain por-para
```

---

## Editor Integration (LSP) 🔌

Configure your editor to run `spanglings lsp` for live diagnostics, red squigglies, code completions, and rich mouseover hover cheat sheets:

=== "VS Code / Cursor"
    ```json
    // .vscode/settings.json
    {
      "spanglings.serverPath": "spanglings",
      "spanglings.enableHover": true,
      "spanglings.enableDiagnostics": true
    }
    ```

=== "Neovim"
    ```lua
    -- init.lua / nvim-lspconfig
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

=== "Helix"
    ```toml
    # ~/.config/helix/languages.toml
    [[language]]
    name = "markdown"
    language-servers = [ "spanglings-lsp" ]

    [language-server.spanglings-lsp]
    command = "spanglings"
    args = ["lsp"]
    ```

---

## 🌐 The *lings Ecosystem

If you enjoy hands-on, terminal-driven mastery, check out our companion platforms:

- ☸️ [**Kubelings**](https://github.com/dnf0/kubelings) – Hands-on interactive CLI learning environment for Kubernetes.
- 🏗️ [**Terralings**](https://github.com/dnf0/terralings) – Master Terraform and OpenTofu through interactive infrastructure-as-code exercises.
- ⚡ [**Raylings**](https://github.com/dnf0/raylings) – Learn distributed AI, Ray Core actors, and scalable clusters through hands-on Python exercises.
- 🦀 [**Rustlings**](https://github.com/rust-lang/rustlings) – Small exercises to get you used to reading and writing Rust code.
