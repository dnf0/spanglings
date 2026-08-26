# Spanglings 🇪🇸 🦀

> A developer-grade CLI & interactive TUI for learning intermediate-to-advanced Spanish (B1, B2, C1 + baseline irregular drills), inspired by [Rustlings](https://github.com/rust-lang/rustlings) and [Raylings](https://github.com/ray-project/raylings).

---

## Overview

Duolingo is often too slow, repetitive, and child-oriented. **Spanglings** provides a developer-first, terminal-native environment for mastering the nuances of Spanish syntax, subjunctive triggers, aspectual contrasts (*pretérito vs imperfecto*), clitic pronoun stacking, accidental *se*, and formal C1 collocations.

### Key Features
- 🚀 **Interactive Terminal UI (`ratatui`)**: Real-time dual-pane editor with live validation, syntax styling, progress tracking, and interactive drill modes.
- ⚡ **Headless Watch Mode (`spanglings watch`)**: Modify exercise files in your favorite editor (VS Code, Neovim, Zed) while Spanglings continuously validates submissions via debounced filesystem events.
- 🔍 **Rustc-Style Grammar Diagnostics**: Rich, colored compiler-style error diagnostics (`error[E0301]: Subjunctive Mood Required`) with source code context, dynamic line markers (`^^^^`), and grammatical explanations.
- 💡 **Progressive 3-Tier Hint System**: Get hints on demand (Tier 1: conceptual clue, Tier 2: morphological/structural clue, Tier 3: solution reveal).
- 🧠 **Forgiving Smart Accent Matching**: Designed for QWERTY keyboards. Accents and inverted punctuation (`¿`, `¡`) are forgiven by default with helpful tip notices, or enforced with `--strict-accents`.
- 🔄 **SM-2 Spaced Repetition (SRS)**: Active recall review scheduler using the SuperMemo-2 algorithm (`spanglings review` / `spanglings drill`).
- 📖 **In-Terminal Cheat Sheets (`spanglings explain <topic>`)**: Reference cards for *ser vs estar*, past aspectual shifts, subjunctive triggers (WEIRDO), *por vs para*, prepositional regimes, pronoun stacking, and accidental *se*.
- 📚 **116 Handcrafted Exercises across 22 Tracks**: Complete coverage from baseline drills through advanced C1 collocations.

---

## Installation & Setup

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (edition 2021)

### Build & Run
```bash
git clone https://github.com/your-username/spanglings.git
cd spanglings

# Build in release mode
cargo build --release

# Run interactive TUI
cargo run --

# Or run with strict accent enforcement
cargo run -- --strict-accents
```

---

## Command Line Interface (CLI)

```
Usage: spanglings [OPTIONS] [COMMAND]

Commands:
  watch     Watch exercise files and re-evaluate on file changes
  run       Run and validate a specific exercise by path or ID
  hint      Show progressive hints for an exercise (Tier 1 to 3)
  list      List all curriculum exercises and completion statuses
  progress  Display current learning progress across CEFR levels
  drill     Start an active-recall flashcard drill session
  review    Review exercises due for Spaced Repetition (SM-2)
  explain   Display in-terminal grammar cheat sheet for a topic
  help      Print this message or the help of the given subcommand(s)

Options:
  -s, --strict-accents  Require exact accent marks and tildes
  -h, --help            Print help
  -V, --version         Print version
```

### Examples

#### 1. Interactive TUI Mode
```bash
spanglings
```
- `Enter` / `Ctrl+S`: Submit answer
- `Ctrl+H`: Cycle progressive hints (Tier 1 → 2 → 3)
- `Ctrl+E`: Toggle grammar reference cheat sheet
- `Ctrl+N` / `Ctrl+P`: Next / Previous exercise
- `Ctrl+R`: Reset current exercise
- `Esc` / `Ctrl+C`: Quit

#### 2. Headless Watcher Mode
```bash
spanglings watch
```
Open any `.md` file under `exercises/` in your editor, remove `<!-- I AM NOT DONE -->`, fill in the `___`, and save. Spanglings will automatically validate and report results.

#### 3. Targeted Run & Explanation
```bash
# Validate a single exercise
spanglings run exercises/03_subjunctive_weirdo/01_wishes_quiero_que.md

# Get hints
spanglings hint b1_subj_weirdo_wishes --tier 2

# Read a grammar cheat sheet
spanglings explain subjunctive
spanglings explain past_tenses
spanglings explain accidental_se
```

#### 4. Spaced Repetition Drills
```bash
# Review due exercises
spanglings review

# Drill a specific topic or level
spanglings drill --topic ser_vs_estar
spanglings drill --level B2 --count 10
```

---

## Curriculum Tracks (116 Exercises)

| Track | Topic | CEFR Level | Exercises |
|---|---|---|---|
| `00_baseline` | Irregular preterite/subjunctive stems & false friends | Baseline | 5 |
| `01_ser_vs_estar` | Essential contrasts & adjective meaning shifts (*listo*, *rico*, *atento*) | B1 | 5 |
| `02_past_aspects` | Pretérito vs Imperfecto (*supe/sabía*, *conocí/conocía*, *quise/quería*) | B1 | 6 |
| `03_subjunctive_weirdo` | WEIRDO triggers (Wishes, Emotions, Impersonal, Recommendations, Doubt, Ojalá) | B1 | 6 |
| `04_subjunctive_relative` | Relative clauses with indefinite or non-existent antecedents | B1 | 5 |
| `05_subjunctive_conjunctions` | Temporal & conditional conjunctions (*en cuanto*, *sin que*, *con tal de que*) | B1 | 6 |
| `06_imperfect_subjunctive_conditionals` | Second conditionals & hypothetical comparisons (*si tuviera*, *como si fuera*) | B2 | 6 |
| `07_por_vs_para` | Cause vs Purpose vs Deadline vs Exchange vs Recipient | B1 | 6 |
| `08_pronoun_stacking` | Double clitic placement & written accentuation (*se lo dije*, *¡Dímela!*) | B1 | 5 |
| `09_prepositional_regimes` | Verbs with fixed prepositions (*acordarse de*, *soñar con*, *pensar en*) | B2 | 6 |
| `10_accidental_se` | Involuntary events & number agreement (*se me cayó*, *se nos rompieron*) | B2 | 5 |
| `11_pluperfect_subjunctive` | Third conditionals (*si hubiera sabido*) & literary *de haber + participio* | B2 / C1 | 5 |
| `12_verbal_periphrases` | Aspectual periphrases (*llevar + gerundio*, *acabar de*, *dejar de*, *ponerse a*) | B2 | 5 |
| `13_advanced_concessives` | Concessive locutions (*por más que*, *aun a riesgo de que*, *hagas lo que hagas*) | C1 | 5 |
| `14_connectors` | Formal discourse connectors (*de ahí que*, *dado que*, *a no ser que*) | C1 | 5 |
| `15_indirect_speech` | Reported speech tense shifts & deictic anchors (*ayer* → *el día anterior*) | B2 / C1 | 5 |
| `16_idioms` | Fixed conversational idioms (*tomar el pelo*, *dar la lata*, *meter la pata*) | B1 / B2 | 5 |
| `17_negated_perception` | Negated cognition & sensory perception verbs (*no creo que*, *no veo que*) | C1 | 5 |
| `18_cleft_sentences` | Focus & cleft constructions (*fue entonces cuando*, *lo que pasa es que*) | C1 | 5 |
| `19_formal_inversion` | Absolute participle clauses (*habiendo considerado*, *dada la situación*) | C1 | 5 |
| `20_passive_refleja` | Pasiva refleja agreement vs impersonal *se* with personal *a* | C1 | 5 |
| `21_nuanced_collocations` | Formal register collocations (*entablar conversación*, *acatar la ley*, *surtir efecto*) | C1 | 5 |

---

## Development & Testing

Run the automated test suite and golden curriculum validator:

```bash
# Run all unit, integration, and golden tests
cargo test --all-targets

# Run linter
cargo clippy --all-targets -- -D warnings

# Check formatting
cargo fmt --check
```

---

## License

MIT OR Apache-2.0
