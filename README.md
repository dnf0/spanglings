# Spanglings 🇪🇸 🦀

> **Spanglings builds the syntax compiler; real-world usage supplies the data.**
> A developer-grade CLI & interactive TUI for mastering Spanish grammar, verb mechanics, and nuanced syntactic architecture, inspired by [Rustlings](https://github.com/rust-lang/rustlings) and [Raylings](https://github.com/dnf0/raylings).

---

## Overview

Duolingo is often too slow, repetitive, and child-oriented. **Spanglings** provides a developer-first, terminal-native environment for mastering the nuances of Spanish syntax, subjunctive triggers, aspectual contrasts (*pretérito vs imperfecto*), clitic pronoun stacking, accidental *se*, and formal C1 collocations.

### Key Features
- 🌐 **Linguistic Knowledge Graph (DAG Ontology)**: 81-concept ontological graph mapping prerequisite relationships, morphological shifts, and situational domains with learning frontier resolution.
- 🚀 **Interactive Terminal UI (`ratatui`)**: Real-time dual-pane editor with live validation, syntax styling, progress tracking, and interactive drill modes.
- ⚡ **Modern Headless Watch Mode (`spanglings watch`)**: Modify exercise files in your favorite editor (VS Code, Neovim, Zed) while Spanglings continuously validates submissions. Features interactive non-blocking terminal keystrokes (`[n]` next, `[p]` previous, `[r]` reset, `[q]` quit) with zero comment-deletion busywork.
- 📦 **Turnkey Zero-Setup Scaffolding (`spanglings init`)**: Embedded exercise catalog compiled directly into the binary—run `spanglings init` in any directory with zero git-cloning needed.
- 🔍 **Rustc-Style Grammar Diagnostics**: Rich, colored compiler-style error diagnostics (`error[E0301]: Subjunctive Mood Required`) with source code context, dynamic line markers (`^^^^`), grammatical explanations, linked concepts, and contrast notes.
- 💡 **Progressive 3-Tier Hint System**: Get hints on demand (Tier 1: conceptual clue, Tier 2: morphological/structural clue, Tier 3: solution reveal).
- 🧠 **Forgiving Smart Accent Matching**: Designed for QWERTY keyboards. Accents and inverted punctuation (`¿`, `¡`) are forgiven by default with helpful tip notices, or enforced with `--strict-accents`.
- 🔄 **SM-2 Spaced Repetition (SRS)**: Active recall review scheduler using the SuperMemo-2 algorithm (`spanglings review` / `spanglings drill`).
- 🔎 **Full-Text, Concept & Topic Search (`spanglings search`, `spanglings list --concept`)**: Instant matching across grammar topics, CEFR levels, exercise titles, prompts, solutions, and 81 linguistic ontology concepts.
- 🤖 **Machine-Readable Output (`--json`)**: Streamlined JSON serialization for external scripts, status bars (Starship, tmux), and IDE integrations.
- 🐚 **Shell Auto-Completions (`spanglings completions`)**: Native autocompletions for Bash, Zsh, Fish, PowerShell, and Elvish.
- 📖 **In-Terminal Cheat Sheets (`spanglings explain <topic>`)**: Reference cards for *ser vs estar*, past aspectual shifts, subjunctive triggers (WEIRDO), *por vs para*, prepositional regimes, pronoun stacking, accidental *se*, tech Spanish, business correspondence, false friends, *voseo*, accents, epistemic conjecture, clitic doubling, personal *a*, gerund restrictions, adversatives (*pero/sino/sino que*), and legal subjunctives.
- 🧭 **Interactive Guided Onboarding Tour (`spanglings tour`)**: 6-station interactive walkthrough with active-recall micro-challenges, architecture overviews, and developer shortcuts for first-time learners.
- 🎯 **Diagnostic Placement & CEFR Assessment (`spanglings test`)**: Calibrated multi-tier diagnostic test battery assessing CEFR proficiency (Baseline through C1). Includes one-click automatic fast-tracking to mark mastered tiers and seed SM-2 spaced repetition cards.
- 🔌 **Native Language Server Protocol (LSP) Engine (`spanglings lsp`)**: Real-time stdio JSON-RPC server with live diagnostics, rich hover popups (conjugations and grammar sheets), and autocompletion for VS Code, Neovim, Helix, and Zed.
- 📦 **Anki & Markdown Study Pack Exporter (`spanglings export`)**: Export full decks to Anki TSV format, generate Markdown study guides for Obsidian, or export JSON progress metrics.
- 🔄 **Portable State Sync (`spanglings sync`)**: Export and merge learning history, streaks, and SRS mastery across workstations.
- 📚 **339 Handcrafted Exercises across 60 Tracks**: Complete coverage from baseline irregular drills through Latin American engineering, everyday conversational mastery, practical logistics, formal C1 collocations, and nuanced linguistic edge cases.

---

## Installation & Setup

### 1. Install via Cargo (Recommended)
```bash
# Install globally from crates.io
cargo install spanglings

# Initialize exercises in your current workspace
spanglings init

# Launch interactive TUI
spanglings
```

### 2. Build from Source
```bash
git clone https://github.com/dnf0/spanglings.git
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
  explain      Display in-terminal grammar cheat sheet for a topic
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

### Examples

#### 1. Interactive TUI Mode
```bash
spanglings
```
- `/`: Live fuzzy search and filter exercises
- `Enter`: Submit answer (or select search result)
- `Ctrl+H` / `F1`: Cycle progressive hints (Tier 1 → 2 → 3)
- `Ctrl+E` / `F2`: Toggle grammar reference cheat sheet
- `Ctrl+K` / `F3`: Open in-TUI Verb Conjugator modal
- `Ctrl+B` / `F4`: Open in-TUI Reference Sheet Browser modal
- `[T]` / `F6` / `Alt+T`: Open in-TUI Guided Onboarding Tour
- `[p]` / `F5` / `Alt+P`: Launch in-TUI Diagnostic Placement Test & Fast-Track
- `?`: Open Help & Keybindings overlay
- `Tab` / `Ctrl+N` / `Down`: Next exercise
- `BackTab` / `Ctrl+P` / `Up`: Previous exercise
- `Ctrl+R`: Reset current exercise
- `Esc` / `Ctrl+C`: Dismiss modal / Cancel search / Quit

#### 2. Interactive Guided Onboarding Tour
```bash
# Take the full 6-station interactive onboarding tour with micro-challenges
spanglings tour

# Run tour in automated overview mode (skipping interactive challenges)
spanglings tour --skip-challenges
```

#### 3. Diagnostic Placement Assessment & Level Fast-Tracking
```bash
# Take full 15-question calibrated CEFR diagnostic test
spanglings test

# Test specific CEFR tier with automatic fast-track on pass
spanglings test --level b1 --fast-track

# Machine-readable JSON diagnostic evaluation
spanglings test --json
```

#### 4. Language Server Protocol (LSP) Editor Integration
Configure your editor to run `spanglings lsp` for live diagnostics and hover grammar docs.

**VS Code / Neovim / Helix configuration:**
```bash
spanglings lsp
```

#### 5. Export to Anki & Markdown Study Notes
```bash
# Export full curriculum to Anki TSV format
spanglings export --format anki --out anki_spanish_deck.txt

# Export only exercises due for SRS review
spanglings export --format anki --only-due --out due_cards.txt

# Export comprehensive Markdown study guide
spanglings export --format markdown --out SPANGLINGS_GUIDE.md

# Backup and sync state between machines
spanglings sync --export backup.json
spanglings sync --import backup.json
```

#### 6. Headless Watcher Mode & Editor Integration
```bash
# Continuous file watcher with interactive [n]ext / [p]rev / [r]eset controls
spanglings watch

# Single exercise or curriculum diagnostic check (supports editor problem matchers & JSON)
spanglings check exercises/03_subjunctive_weirdo/01_wishes_volition.md
spanglings check --json
```

#### 7. Search & Concept Filtering
```bash
# Search for subjunctive exercises
spanglings search subjunctive

# Filter curriculum by linguistic concept
spanglings list --concept subjunctive_wishes_desires

# Search for accidental 'se' exercises
spanglings search "se me"

# Search for C1 level exercises
spanglings search C1
```

#### 8. JSON Output & Activity Heatmap
```bash
# Terminal progress with 12-week ANSI contribution heatmap and weakness diagnostics
spanglings progress

# Machine-readable learning progress with SM-2 intervals & activity history
spanglings progress --json

# Machine-readable exercise listing
spanglings list --json
```

#### 9. Spaced Repetition & Rapid-Fire Drills
```bash
# Review due exercises with SM-2 spaced repetition
spanglings review

# Quick-fire irregular stem conjugation drills
spanglings drill
spanglings drill --topic subjunctive
spanglings drill --concept subjunctive_wishes_desires

# 60-Second Rapid-Fire Blitz Mode (speed drills with streaks & WPM)
spanglings blitz
spanglings blitz --seconds 30 --topic preterite
```

#### 10. Verb Conjugator & Tense Matrix
```bash
# Full colorized conjugation table (Indicative & Subjunctive)
spanglings conjugate ser
spanglings conjugate haber

# Specific tense lookup (Subjunctive or Imperatives)
spanglings conjugate hablar subjuntivo
spanglings conjugate poner imperativo

# Machine-readable verb paradigm JSON
spanglings conjugate tener --json
```

#### 11. Git Pre-Commit / Pre-Push Micro-Drill Hook
```bash
# Install pre-commit Spanish practice hook
spanglings hook install

# Remove hook safely
spanglings hook uninstall
```

---

## Curriculum Tracks (267 Exercises across 48 Tracks)

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
| `22_tech_software` | Software engineering, Git workflows, debugging, concurrency & architecture | B2 / C1 | 5 |
| `23_business_diplomatic` | Executive correspondence formulas, contracts, debt settlement & negotiations | B2 / C1 | 5 |
| `24_false_friends` | High-frequency cognate traps (*actualmente*, *eventualmente*, *pretender*, *sensato*) | B1 / B2 | 5 |
| `25_register_elevation` | Elevating light verbs to literary C1 equivalents (*acometer*, *suscitar*, *albergar*) | C1 | 5 |
| `26_regional_contrasts` | Rioplatense *voseo* (*tenés*, *decime*, *sentate*), Pan-American *ustedes* & lexical pairs | B1 / B2 | 5 |
| `27_system_design` | Distributed failover, 2PC transactions, circuit breakers, cache invalidation, database sharding | C1 | 6 |
| `28_advanced_subjunctive_clauses` | Reduplicative subjunctive, *por mucho que*, *aun a riesgo de que*, *así sea*, *pase lo que pase* | C1 | 6 |
| `29_advanced_verbal_periphrases` | High-register periphrases (*dar por sentado*, *echar a perder*, *ponerse a*, *llevar sin*, *venir a decir*) | B2 / C1 | 6 |
| `30_executive_leadership` | Stakeholder diplomacy, strategic pivots, KPI reporting, headcount reallocation, board resolutions | C1 | 6 |
| `31_mexican_tech_and_startups` | Capital raises (*levantamiento de capital*), seed rounds, churn (*tasa de abandono*), burn rate (*tasa de quema*), PMF | B2 / C1 | 6 |
| `32_colombian_professional_nuances` | Administrative errands (*hacer una vuelta*), monitoring (*estar pendiente*), realizing (*caer en cuenta*), rollback (*echar reversa*) | B2 / C1 | 6 |
| `33_rioplatense_production_voseo` | Production deploys (*sacar a producción*), load endurance (*bancarse la carga*), on-call (*ponete las pilas*), *voseo* subjunctives | B2 / C1 | 6 |
| `34_latam_anglicism_elimination` | Replacing Spanglish calques (*rastrear* vs *trackear*, *personalizar* vs *customizar*, *rendir* vs *performar*, *descontinuar* vs *deprecar*) | B2 / C1 | 6 |
| `35_latam_enterprise_risk_and_sla` | Non-disclosure agreements (*acuerdos de confidencialidad*), lost profits (*lucro cesante*), breach termination, indemnification | C1 | 6 |
| `36_everyday_life_and_housing` | Leases (*contrato de arrendamiento*, *fianza*), utilities setup (*dar de alta*), bank fees, refunds, package pickups, transit | B1 | 6 |
| `37_healthcare_and_symptoms` | Describing symptoms (*punzadas agudas*, *mareos*), prescriptions (*recetar*), OTC drugs & leaflets (*prospecto*), ER discharge, allergies | B1 / B2 | 6 |
| `38_dining_and_social_conversation` | Split checks (*cuenta por separado*), plans (*quedar a las ocho*), compliments, conversational softeners (*por cierto*), banter, goodbyes | B1 / B2 | 6 |
| `39_nuanced_prepositions_and_locutions` | *Hacia* vs *Hasta*, *Tras*, *Según*, *Bajo*, *a base de*, *a expensas de*, *al cabo de* vs *dentro de*, *a lo largo de*, *a raíz de* | B1 / C1 | 6 |
| `40_middle_voice_and_reflexive_shifts` | Meaning shifts: *ir/irse*, *dormir/dormirse*, *comer/comerse*, *llevar/llevarse*, *quedar/quedarse*, *volver/volverse* | B1 / C1 | 6 |
| `41_adverbial_clauses_and_conjunctions` | *A medida que*, *Conforme*, *De modo que*, *En tanto que*, *Tan pronto como*, *A menos que*, *Siempre y cuando* | B2 / C1 | 6 |
| `42_travel_logistics_and_borders` | Flight delays, missed layovers, lost baggage claims, border declarations, transit permits | B1 / C1 | 6 |
| `43_banking_taxes_and_finances` | VAT withholding, tax returns (*declaración de la renta*), tax certificates, fee appeals, IBAN transfers | B2 / C1 | 6 |
| `44_consumer_complaints_and_rights` | Official complaint forms (*hoja de reclamaciones*), defective product warranty, billing disputes, chargebacks | B2 / C1 | 6 |
| `45_home_maintenance_and_repairs` | Pipe leaks (*fugas*), short circuits (*cortocircuito*), structural dampness, landlord repair obligations, quote requests | B1 / B2 | 6 |
| `46_news_media_and_civic_debate` | Public policy debates, investigative reporting (*arrojar luz*), parliamentary scrutiny, consensus building | C1 | 6 |
| `47_conversational_markers_and_nuance` | Pragmatic fillers (*a ver*, *o sea*), emphatic assertions (*faltaría más*), ironic concessions, polite hedging (*por si acaso*) | B2 / C1 | 6 |

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

Apache-2.0
