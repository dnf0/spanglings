# Getting Started

> 📖 **Looking for a deep-dive walkthrough?** See the [**Complete Onboarding & Learner's Guide**](onboarding-guide.md) for an illustrated tour covering installation, the 6-station CLI tour, solving your first exercise, and TUI power tools.

---

## Installation

### 1. Install via Cargo (Recommended)

Spanglings is written in Rust and can be installed via Cargo:

```bash
# Install globally from crates.io
cargo install spanglings
```

### 2. Build from Source

```bash
# Clone the repository
git clone https://github.com/dnf0/spanglings.git
cd spanglings

# Build in release mode
cargo build --release

# Run binary directly or link into PATH
./target/release/spanglings --version
```

---

## Quickstart in 3 Steps

### Step 1: Take the Onboarding Tour

Take the interactive 6-station walkthrough to learn the philosophy, accent handling, compiler diagnostics, and editor shortcuts:

```bash
spanglings tour
```

### Step 2: Initialize Your Workspace

Scaffold all 267 curriculum exercises into your current directory or target path with zero git-cloning needed:

```bash
spanglings init
```

### Step 3: Launch Watch Mode or TUI

Choose your preferred learning workflow:

=== "Headless Watch Mode (Recommended for IDEs)"
    ```bash
    spanglings watch
    ```
    Open the indicated exercise file (e.g., `exercises/00_baseline/01_irregular_preterite.md`) in VS Code, Neovim, or Zed. When you edit and save, Spanglings re-evaluates in milliseconds. Press `n` or `Enter` to advance.

=== "Interactive Dual-Pane TUI"
    ```bash
    spanglings
    ```
    Full interactive terminal user interface with live syntax highlighting, progress heatmaps, and in-TUI verb conjugator and reference cheat sheets.

---

## Calibrated Placement Diagnostic (`spanglings test`)

Already know some Spanish? Skip baseline drilling by taking the 15-question calibrated CEFR diagnostic placement test:

```bash
# Run full diagnostic placement test
spanglings test

# Run and automatically fast-track mastered levels
spanglings test --fast-track
```

Passing tiers will automatically mark foundational exercises as completed and seed your SM-2 spaced repetition deck with optimal ease factors.

---

## Core Keyboard Shortcuts

### Watch Mode Shortcuts
While `spanglings watch` is running in your terminal:

| Key | Action | Description |
| :--- | :--- | :--- |
| `n` / `Enter` | **Next** | Advance to the next incomplete or subsequent exercise. |
| `p` | **Previous** | Go back to review or modify the previous exercise. |
| `h` | **Hint** | Reveal the next progressive hint tier for current exercise. |
| `r` | **Rerun** | Force immediate re-evaluation of current file. |
| `q` | **Quit** | Gracefully exit watch mode. |

### Interactive TUI Shortcuts
While `spanglings` TUI is active:

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
