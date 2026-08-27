# Getting Started 🚀

Welcome to **Spanglings**! This guide walks you through installation, initializing your exercise workspace, taking the placement test, and setting up your favorite code editor.

> 📖 **Looking for an in-depth tour?** Check out the [**Complete Onboarding & Learner's Guide**](onboarding-guide.md) for a visual step-by-step tutorial covering compiler diagnostics, diagnostic error codes, and mental models.

---

## Installation

=== "Cargo (Recommended)"
    ```bash
    # Install globally from crates.io
    cargo install spanglings
    ```

=== "Pre-Built Binaries"
    Download the latest binary for your operating system from the [GitHub Releases](https://github.com/dnf0/spanglings/releases) page:
    - **macOS Apple Silicon (M1/M2/M3/M4)**: `spanglings-aarch64-apple-darwin.tar.gz`
    - **macOS Intel**: `spanglings-x86_64-apple-darwin.tar.gz`
    - **Linux (x86_64)**: `spanglings-x86_64-unknown-linux-gnu.tar.gz`
    - **Windows**: `spanglings-x86_64-pc-windows-msvc.zip`

=== "Build from Source"
    ```bash
    git clone https://github.com/dnf0/spanglings.git
    cd spanglings
    cargo build --release
    ./target/release/spanglings --version
    ```

---

## Quickstart in 3 Steps

### Step 1: Initialize Your Workspace

Scaffold all **339 curriculum exercises across 60 tracks** directly into your current folder with zero git-cloning required:

```bash
spanglings init
```

### Step 2: Take the Placement Assessment (Optional)

If you already have previous Spanish exposure, take the calibrated CEFR diagnostic test to fast-track past foundational concepts you have already mastered:

```bash
# Run full placement diagnostic with automatic level fast-tracking
spanglings test --fast-track
```

### Step 3: Choose Your Learning Mode

=== "Headless Watch Mode (Recommended for IDEs)"
    ```bash
    spanglings watch
    ```
    Open the indicated exercise file (e.g. `exercises/00_baseline/01_irregular_preterite.md`) in VS Code, Cursor, Neovim, Helix, or Zed. When you edit and save, Spanglings re-evaluates in &lt; 20ms. Press `n` or `Enter` to advance.

=== "Interactive Dual-Pane TUI"
    ```bash
    spanglings
    ```
    Full interactive terminal user interface with live syntax highlighting, progress heatmaps, and in-TUI verb conjugator (`F3`) and reference cheat sheets (`F2` / `F4`).

---

## Shell Autocompletions

Generate native autocompletions for your shell:

=== "Zsh"
    ```bash
    spanglings completions zsh > ~/.zfunc/_spanglings
    # Ensure ~/.zfunc is in your $fpath in ~/.zshrc
    ```

=== "Bash"
    ```bash
    spanglings completions bash > ~/.local/share/bash-completion/completions/spanglings
    ```

=== "Fish"
    ```bash
    spanglings completions fish > ~/.config/fish/completions/spanglings.fish
    ```

=== "PowerShell"
    ```powershell
    spanglings completions powershell | Out-String | Invoke-Expression
    ```

---

## Core Keyboard Shortcuts

### Watch Mode Shortcuts (`spanglings watch`)
| Key | Action | Description |
| :--- | :--- | :--- |
| `n` / `Enter` | **Next** | Advance to the next incomplete or subsequent exercise. |
| `p` | **Previous** | Go back to review or modify the previous exercise. |
| `h` | **Hint** | Reveal the next progressive hint tier for current exercise. |
| `r` | **Rerun** | Force immediate re-evaluation of current file. |
| `c` | **Conjugate** | Prompt for instant verb conjugation table. |
| `q` | **Quit** | Gracefully exit watch mode. |

### Interactive TUI Shortcuts (`spanglings`)
| Key | Action | Description |
| :--- | :--- | :--- |
| `Tab` / `Ctrl+N` / `Down` | **Next Exercise** | Navigate down the exercise list. |
| `BackTab` / `Ctrl+P` / `Up` | **Previous Exercise** | Navigate up the exercise list. |
| `Enter` | **Submit / Select** | Submit current answer or pick search result. |
| `Ctrl+H` / `F1` | **Progressive Hint** | Cycle Tier 1 → Tier 2 → Tier 3 hints. |
| `Ctrl+E` / `F2` | **Reference Card** | Toggle contextual grammar cheat sheet. |
| `Ctrl+K` / `F3` | **Verb Conjugator** | Open in-TUI verb conjugation popup modal. |
| `Ctrl+B` / `F4` | **Reference Browser** | Open in-TUI grammar cheat sheet browser. |
| `[p]` / `F5` / `Alt+P` | **Placement Test** | Open in-TUI diagnostic placement test. |
| `[T]` / `F6` / `Alt+T` | **Guided Tour** | Open in-TUI 6-station guided onboarding tour. |
| `/` | **Fuzzy Search** | Filter exercises by topic, keyword, or level. |
| `?` | **Help Overlay** | View complete keybinding reference. |
| `Esc` / `Ctrl+C` | **Dismiss / Quit** | Close modal, cancel search, or exit TUI. |
