# Interactive Onboarding Guided Tour (`spanglings tour`) Design Specification

- **Feature**: First-Run Interactive Guided Onboarding Tour & TUI Integration
- **Target Subcommand**: `spanglings tour`
- **Specification Date**: 2026-08-27
- **Status**: Draft / Under Review

---

## 1. Objectives & User Journey

### 1.1 Goal
Provide an interactive, hands-on onboarding tour for new Spanglings learners that demystifies tool mechanics, Spanish special characters, concept-aware compiler diagnostics, hint systems, integrated developer tools, and workflow choices (Watch Mode vs TUI).

### 1.2 User Flow
1. **First-Time User Experience**:
   - When a user launches `spanglings` for the first time (state initialized with `tour_completed: false`), the application presents an interactive prompt:
     `Welcome to Spanglings! Would you like to take the 2-minute interactive onboarding tour? [Y/n]`
   - If accepted, the tour starts immediately.
   - If declined, `tour_completed` is set to `false` (prompt won't repeat if explicitly dismissed, or user can run `spanglings tour` anytime).
2. **On-Demand Invocation**:
   - Any learner can run `spanglings tour` (or `spanglings tour --skip-challenges`) directly from the CLI at any time.
   - In the TUI help menu, pressing `[T]` launches the tour.

---

## 2. Architecture & Components

### 2.1 Component Structure
```
src/
├── cli/
│   ├── mod.rs                   # Exposes `Commands::Tour { skip_challenges: bool }`
│   └── commands/
│       ├── mod.rs
│       └── tour.rs              # Dedicated interactive tour engine & station runners
├── core/
│   └── state.rs                 # `AppState.tour_completed: bool` and update helpers
├── tui/
│   ├── app.rs                   # First-run tour dialog modal & [T] shortcut handling
│   └── ui.rs                    # Modal rendering for first-run prompt
```

---

## 3. The 6 Tour Stations & Micro-Challenges

### Station 1: Welcome & Philosophy
- **Concept**: Developer-first Spanish learning through active recall, grammar in production context, and zero busywork (no deleting `<!-- I AM NOT DONE -->` comments).
- **Format**: Narrative walkthrough with colored CLI cards.
- **Controls**: `[Enter]` to proceed, `[q]` to quit.

### Station 2: Anatomy of an Exercise & UTF-8 Accents
- **Concept**: How cloze exercises work, blank placeholders `___`, and typing Spanish special characters (`á, é, í, ó, ú, ñ, ü, ¿, ¡`).
- **Hands-on Micro-Challenge**:
  - Sample prompt: *Quiero que tú (venir) ___ a la reunión.*
  - Learner types the answer `vengas` in the terminal input buffer.
  - Validates UTF-8 character support and accent handling.

### Station 3: Concept-Aware Compiler Diagnostics
- **Concept**: Explaining how Spanglings validates grammar errors and provides compiler-grade diagnostic feedback linked to the Linguistic Knowledge Graph ontology.
- **Hands-on Micro-Challenge**:
  - Deliberately tests an incorrect answer (e.g. *viene* instead of *vengas*).
  - Displays the live formatted diagnostic card (`error[E0301]`, linked concept `subjunctive_volition_influence`, prerequisite `irregular_subjunctive_stems`, and remediation suggestions).

### Station 4: Progressive 3-Tier Hints & Grammar Reference Cards
- **Concept**: How to unstick yourself without spoiling answers using tiered hints (Tier 1 Rule -> Tier 2 Stem -> Tier 3 Solution) and instant reference cards (`spanglings explain <topic>`).
- **Hands-on Micro-Challenge**:
  - Learner presses `[h]` to reveal hint tiers one by one.
  - Learner views a sample grammar reference snippet for Subjunctive Mood.

### Station 5: Integrated Tools: Verb Conjugator & Placement Test
- **Concept**: Built-in developer utilities:
  - High-speed irregular verb conjugation lookup (`spanglings conjugate <verb>`).
  - Diagnostic Placement Test (`spanglings test` / `[t]`) with automated CEFR level fast-tracking.
- **Hands-on Micro-Challenge**:
  - Demonstrates looking up irregular roots (e.g. *proponer* in Preterite or Subjunctive).

### Station 6: Workflow Choices: Watch Mode vs TUI
- **Concept**: Choosing your preferred developer flow:
  1. **Watch Mode (`spanglings watch`)**: Work in VS Code, Neovim, Zed, or Helix with instant live evaluation and raw-mode navigation keystrokes (`[n]`, `[p]`, `[r]`, `[q]`).
  2. **Interactive TUI (`spanglings`)**: Full terminal ratatui application with split panes, fuzzy search (`/`), and modal cheat sheets.
  3. **Targeted Practice**: `spanglings review` (SM-2 spaced repetition), `spanglings drill` (active recall), and `spanglings progress` (concept mastery radar).
- **Completion**: Marks `state.tour_completed = true`, congratulates the learner, and offers instant jump to `spanglings watch`, `spanglings`, or `spanglings test`.

---

## 4. State & Persistence

### 4.1 Schema Update
In `src/core/state.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    #[serde(default)]
    pub completed: HashMap<String, ExerciseProgress>,
    #[serde(default)]
    pub srs_items: HashMap<String, SrsItem>,
    #[serde(default)]
    pub concept_mastery: HashMap<String, ConceptMastery>,
    #[serde(default)]
    pub tour_completed: bool,
    #[serde(default)]
    pub version: u32,
}
```

### 4.2 Backward Compatibility
- Existing `state.json` files without `tour_completed` deserialize automatically to `false` via `#[serde(default)]`.
- Saving preserves all existing user progress, SRS items, and concept mastery scores.

---

## 5. Non-Interactive / CI Mode Safety
- In non-interactive environments (`!io::stdin().is_terminal()`), `spanglings tour` runs in non-interactive batch mode printing all station overviews sequentially without hanging on user input.
- `spanglings tour --skip-challenges` prints formatted overview summaries of all stations in sequence.

---

## 6. Testing Strategy

1. **Unit Tests**:
   - `test_tour_state_persistence`: Verifies `tour_completed` state flag saving and loading.
   - `test_tour_station_catalog`: Verifies all 6 stations have valid titles, descriptions, and challenge metadata.
2. **CLI Integration Tests**:
   - `test_tour_cli_skip_challenges`: Runs `spanglings tour --skip-challenges` in non-interactive mode, asserting exit code 0 and output containing all station headers.
   - `test_tour_command_parsing`: Asserts Clap parses `spanglings tour` and `spanglings tour --skip-challenges`.
3. **TUI Smoke Tests**:
   - Verifies first-run modal renders cleanly when `tour_completed == false`.
