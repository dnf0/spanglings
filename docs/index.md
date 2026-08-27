# Spanglings 🇪🇸 🦀

**Spanglings builds the syntax compiler; real-world usage supplies the data.**

*A developer-grade CLI & interactive TUI for mastering Spanish grammar, verb mechanics, and nuanced syntactic architecture.*

Inspired by [Rustlings](https://github.com/rust-lang/rustlings) and [Raylings](https://github.com/dnf0/raylings), **Spanglings** provides a terminal-native, hands-on learning environment for engineers, developers, and power users who want to master real Spanish syntax, aspectual contrasts, and professional collocations without childish gamification.

---

## Why Spanglings?

Language learning apps are often too slow, repetitive, and child-oriented. They fail to teach the structural mental models engineers use every day. **Spanglings** approaches Spanish like a compiled language:

- 🌐 **Linguistic Knowledge Graph (DAG Ontology)**: 81-concept ontological graph mapping prerequisite relationships, morphological shifts, and situational domains with learning frontier resolution.
- 🚀 **Interactive Terminal UI (`ratatui`)**: Real-time dual-pane editor with live validation, syntax styling, progress tracking, and interactive drill modes.
- ⚡ **Modern Headless Watch Mode (`spanglings watch`)**: Modify exercise files in your favorite editor (VS Code, Neovim, Zed) while Spanglings continuously validates submissions with non-blocking keystrokes (`[n]`, `[p]`, `[r]`, `[q]`) and zero comment-deletion busywork.
- 🔍 **Rustc-Style Grammar Diagnostics**: Rich, colored compiler-style error diagnostics (`error[E0301]: Subjunctive Mood Required`) with source code context, dynamic line markers (`^^^^`), grammatical explanations, linked concepts, and contrast notes.
- 💡 **Progressive 3-Tier Hint System**: Get hints on demand (Tier 1: conceptual clue, Tier 2: morphological/structural clue, Tier 3: solution reveal).
- 🧠 **Forgiving Smart Accent Matching**: Designed for QWERTY keyboards. Accents and inverted punctuation (`¿`, `¡`) are forgiven by default with helpful tip notices, or strictly enforced with `--strict-accents`.
- 🔄 **SM-2 Spaced Repetition (SRS)**: Active recall review scheduler using the SuperMemo-2 algorithm (`spanglings review` / `spanglings drill`).
- 🎯 **Diagnostic Placement & CEFR Assessment (`spanglings test`)**: Calibrated multi-tier diagnostic test battery assessing CEFR proficiency with automatic level fast-tracking.
- 🔌 **Native Language Server Protocol (LSP) Engine (`spanglings lsp`)**: Real-time stdio JSON-RPC server with live diagnostics, rich hover popups, and autocompletions for VS Code, Neovim, Helix, and Zed.
- 📦 **Anki & Markdown Study Pack Exporter (`spanglings export`)**: Export full decks to Anki TSV format, generate Markdown study guides for Obsidian, or export JSON progress metrics.
- 📚 **339 Handcrafted Exercises across 60 Tracks**: Complete coverage from baseline irregular drills through Latin American engineering, everyday conversational mastery, practical logistics, formal C1 collocations, and nuanced linguistic edge cases.

---

## Quick Example

```markdown
<!-- exercises/03_subjunctive_weirdo/01_wishes_volition.md -->
# Wishes and Volition (Querer / Desear que)
- Level: B1
- Topic: Subjunctive Triggers (WEIRDO)
- Concepts: subjunctive_wishes_desires, present_subjunctive_regular
- Prerequisites: present_indicative_irregular

## Prompt
Fill in the correct present subjunctive form of the verb in parentheses.
Context: Expressing a desire for someone else's action.

Mis padres quieren que yo ___ (estudiar) ingeniería informática en la universidad.
```

Save the file and watch Spanglings validate your answer in milliseconds. When correct, press `n` or `Enter` to advance to the next challenge!

---

## Editor Integration (LSP) 🔌

Spanglings includes a built-in Language Server Protocol engine (`spanglings lsp`). Configure your favorite editor for live in-editor diagnostics, hover grammar cards, and verb conjugation tables:

=== "VS Code"
    ```json
    {
      "languageserver": {
        "spanglings": {
          "command": "spanglings",
          "args": ["lsp"],
          "filetypes": ["markdown"]
        }
      }
    }
    ```

=== "Neovim (nvim-lspconfig)"
    ```lua
    vim.api.nvim_create_autocmd("FileType", {
      pattern = "markdown",
      callback = function()
        vim.lsp.start({
          name = "spanglings-lsp",
          cmd = { "spanglings", "lsp" },
          root_dir = vim.fs.dirname(vim.fs.find({'Cargo.toml', '.git'}, { upward = true })[1]),
        })
      end,
    })
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

If you enjoy hands-on, terminal-driven mastery, check out our other interactive platforms:

- ☸️ [**Kubelings**](https://github.com/dnf0/kubelings) – Hands-on interactive CLI learning environment for Kubernetes.
- 🏗️ [**Terralings**](https://github.com/dnf0/terralings) – Master Terraform and OpenTofu through interactive infrastructure-as-code exercises.
- ⚡ [**Raylings**](https://github.com/dnf0/raylings) – Learn distributed AI, Ray Core actors, and scalable clusters through hands-on Python exercises.
