# CLI & TUI Reference

Comprehensive command-line and keyboard reference for **Spanglings**.

---

## 💻 CLI Commands

### `spanglings tour`
Run the interactive 6-station guided onboarding tour.

```bash
spanglings tour [OPTIONS]
```

**Options:**
- `--skip-challenges`: Run tour in non-interactive batch mode (useful for CI or rapid reading).

---

### `spanglings watch`
Launch continuous file watching. Re-evaluates exercises upon saving.

```bash
spanglings watch [OPTIONS]
```

**Options:**
- `--strict-accents`: Require exact accents and inverted punctuation.

**Interactive Keystrokes in Watch Mode:**
- `n` / `Enter`: Advance to next exercise.
- `p`: Go to previous exercise.
- `h`: Show next progressive hint tier.
- `r`: Force re-evaluation of current exercise.
- `q`: Exit watch mode.

---

### `spanglings test`
Run calibrated CEFR placement diagnostic test battery.

```bash
spanglings test [OPTIONS]
```

**Options:**
- `--level <LEVEL>`: Test a specific CEFR tier (`baseline`, `b1`, `b2`, `c1`).
- `--fast-track`: Automatically mark passed levels as completed and seed SRS deck.
- `--json`: Output diagnostic evaluation in JSON format.

---

### `spanglings drill`
Start an active-recall flashcard drill session.

```bash
spanglings drill [OPTIONS]
```

**Options:**
- `--topic <TOPIC>`: Filter drill cards by topic (e.g., `subjunctive`, `irregular_preterite`).
- `--concept <CONCEPT>`: Filter drill cards by linguistic ontology concept ID.
- `--level <LEVEL>`: Filter by CEFR level (`baseline`, `b1`, `b2`, `c1`).

---

### `spanglings blitz`
Start a 60-second rapid-fire conjugation speed drill.

```bash
spanglings blitz [OPTIONS]
```

**Options:**
- `--seconds <SECS>`: Duration of blitz session (default: `60`).
- `--topic <TOPIC>`: Verb category or tense focus.

---

### `spanglings review`
Review exercises due for Spaced Repetition (SM-2 algorithm).

```bash
spanglings review
```

---

### `spanglings progress`
Display visual progress across CEFR levels and concept mastery scores.

```bash
spanglings progress [OPTIONS]
```

**Options:**
- `--json`: Stream JSON progress metrics, SM-2 intervals, and weakness recommendations.

---

### `spanglings list`
List all curriculum exercises and completion statuses.

```bash
spanglings list [OPTIONS]
```

**Options:**
- `--concept <CONCEPT>`: Filter exercises by ontology concept ID.
- `--topic <TOPIC>`: Filter by topic string.
- `--level <LEVEL>`: Filter by CEFR tier.
- `--json`: Output full curriculum catalogue in JSON format.

---

### `spanglings search`
Search exercises across topics, keywords, prompts, solutions, and ontology concepts.

```bash
spanglings search <QUERY>
```

---

### `spanglings check`
Check an exercise file or entire curriculum for syntax or validation errors.

```bash
spanglings check [EXERCISE_PATH] [OPTIONS]
```

**Options:**
- `--json`: Output editor-compatible diagnostic stream.

---

### `spanglings explain`
Display in-terminal grammar cheat sheet.

```bash
spanglings explain <TOPIC>
```

**Topics:** `ser-estar`, `past-tenses`, `subjunctive`, `por-para`, `pronouns`, `accidental-se`, `accents`, `tech`, `business`, `false-friends`, `voseo`, `prepositions`, `epistemic-conjecture`, `clitic-doubling`, `personal-a`, `gerund-rules`, `adversatives`, `legal-subjunctive`.

---

### `spanglings conjugate`
Look up full conjugation matrices for any Spanish verb.

```bash
spanglings conjugate <VERB> [TENSE] [OPTIONS]
```

**Options:**
- `--json`: Output full verb paradigm in JSON format.

---

### `spanglings export`
Export curriculum study materials to external tools.

```bash
spanglings export --format <anki|markdown|json> --out <FILE> [OPTIONS]
```

**Options:**
- `--format <anki|markdown|json>`: Export format.
- `--out <PATH>`: Output destination file path.
- `--only-due`: Export only cards currently due for SM-2 review.

---

### `spanglings sync`
Backup, restore, or merge learning state across machines.

```bash
spanglings sync --export <FILE>
spanglings sync --import <FILE> [--dry-run]
```

---

### `spanglings lsp`
Start Language Server Protocol (LSP) stdio server for IDEs.

```bash
spanglings lsp
```

---

### `spanglings hook`
Manage Git pre-commit / pre-push micro-drill hooks.

```bash
spanglings hook <install|uninstall>
```

---

## ⌨️ TUI Keyboard Reference

| Keybinding | Scope | Description |
| :--- | :--- | :--- |
| `Tab` / `Ctrl+N` / `Down` | Global | Move selection to next exercise |
| `BackTab` / `Ctrl+P` / `Up` | Global | Move selection to previous exercise |
| `Enter` | Editor | Submit cloze answer for validation |
| `Enter` | Search | Select filtered exercise and open editor |
| `Ctrl+H` / `F1` | Editor | Cycle progressive hint tier (1 → 2 → 3) |
| `Ctrl+E` / `F2` | Editor | Toggle contextual grammar reference card |
| `Ctrl+K` / `F3` | Global | Open in-TUI Verb Conjugator modal |
| `Ctrl+B` / `F4` | Global | Open in-TUI Reference Sheet Browser modal |
| `[p]` / `F5` / `Alt+P` | Global | Open Diagnostic Placement Test modal |
| `[T]` / `F6` / `Alt+T` | Global | Open Guided Onboarding Tour modal |
| `/` | List | Activate fuzzy search and filtering bar |
| `Ctrl+R` | Editor | Reset current exercise to blank state |
| `?` | Global | Open Help & Keybindings overlay |
| `Esc` / `Ctrl+C` | Global | Dismiss modal / Exit search / Quit |
