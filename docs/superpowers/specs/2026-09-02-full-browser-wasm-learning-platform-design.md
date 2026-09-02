# Spanglings WebAssembly Browser Platform Design

**Date**: 2026-09-02  
**Status**: Approved  
**Target URL**: `https://dnf0.github.io/spanglings/playground/`  

---

## 1. Executive Summary

The Spanglings WebAssembly Browser Platform brings the entire developer-grade Spanish learning curriculum, interactive exercise compiler, and rapid single-key showdown arcade into a zero-install, zero-server client-side web application hosted on GitHub Pages (`dnf0.github.io/spanglings/playground/`).

By compiling Spanglings' deterministic Rust core (`src/core`) to WebAssembly (`wasm-bindgen`), the browser platform delivers sub-millisecond AST and grammatical diagnostics, full offline SM-2 spaced repetition, and 100% parity with the terminal CLI.

---

## 2. Architectural Blueprint

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Browser Window (https://dnf0.github.io/spanglings/playground/)                         │
│                                                                                        │
│  ┌───────────────────────────┬──────────────────────────────────────────────────────┐  │
│  │ 📚 Curriculum Sidebar     │ 🇪🇸 Workspace & Diagnostic Arena                      │  │
│  │  • 24 Grammar Topics      │  • Mode Switcher: [📝 Workspace] / [⚡ Arcade Arena]  │  │
│  │  • 136 Exercises / Frames │  • Monaco Editor / Fill-in Blanks with Accent Bar    │  │
│  │  • Progress & Mastery %   │  • Colorized Rust Terminal Diagnostics & Diff View   │  │
│  │  • Real-time Search       │  • Dual-Layer Explanations (💡 Meaning + 📐 Rule)    │  │
│  └─────────────┬─────────────┴──────────────────────────┬───────────────────────────┘  │
│                │                                        │                              │
│                ▼                                        ▼                              │
│  ┌───────────────────────────┐            ┌─────────────────────────────────────────┐  │
│  │ LocalStorage State Engine │            │ Rust WebAssembly Engine (<1ms)          │  │
│  │  • CLI Parity Schema      │            │  • wasm-bindgen compiled core           │  │
│  │  • SM-2 Mastery Curves    │◄───────────┤  • 136 Sentence Frames + Evaluator      │  │
│  │  • Export / Import JSON   │            │  • 262 Arcade Items + 5 Custom Engines  │  │
│  └───────────────────────────┘            └─────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Rust WebAssembly Crate (`crates/spanglings-wasm` or `src/wasm.rs`)
* Compiles the deterministic Rust core using `wasm-bindgen` to a lightweight (~500KB) Wasm binary and JS binding module.
* **Exposes**:
  * `get_curriculum_catalog()`: Returns JSON containing all 24 topics, 136 sentence frames, instructions, hints, and cheat sheets.
  * `evaluate_exercise(exercise_id, user_input)`: Validates Spanish input with smart accent handling, normalization, token interpolation, and line-specific error diagnostics.
  * `get_arcade_catalog(mode)`: Returns all 262 showdown pairs and 5 specialized engine datasets.
  * `evaluate_arcade_answer(item_id, user_choice)`: Calculates points, combo multipliers, speed bonuses, and dual-layer (`💡 Meaning:` + `📐 Rule:`) explanation cards.
  * `calculate_sm2_review(ease_factor, interval, repetitions, grade)`: Calculates spaced repetition scheduling updates.

### 2.2 Static Web App Architecture (`docs/playground/` & `docs/assets/playground/`)
* **Entry Point**: `docs/playground.md` rendered via Material for MkDocs.
* **Styles**: `docs/assets/playground/playground.css` offering high-contrast dark/light theme alignment, custom 320px split-pane sidebar, and full-bleed 100vw × 100vh fullscreen layout (`⛶ Fullscreen` / `F11`).
* **Client Logic**: `docs/assets/playground/playground.js` orchestrating Monaco Editor / input components, state synchronization, hotkey listeners, and Wasm module communication.

---

## 3. Client-Side State Persistence (`SpanglingsStorage`)

### 3.1 State Schema (CLI Parity)
```typescript
interface SpanglingsWorkspaceState {
  version: 1;
  lastActiveExerciseId: string;
  activeMode: "curriculum" | "arcade";
  exercises: {
    [exerciseId: string]: {
      status: "not_started" | "in_progress" | "completed";
      userCode: string;
      hintsRevealed: number;
      srs: {
        ease_factor: number;
        interval: number;
        repetitions: number;
        due_timestamp: string;
      };
      lastEvaluatedAt?: string;
      passedAt?: string;
    };
  };
  conceptMastery: {
    [concept: string]: {
      successful_attempts: number;
      failed_attempts: number;
      mastery_score: number; // 0.0 .. 1.0
      last_practiced: string;
    };
  };
  arcadeStats: {
    highScore: number;
    bestStreak: number;
    totalDuelsAnswered: number;
    mistakes: Array<{
      topic: string;
      sentence: string;
      userAnswer: string;
      correctAnswer: string;
      meaning: string;
      rule: string;
    }>;
  };
}
```

### 3.2 State Operations
* **Debounced Auto-Save (300ms)**: Preserves active editor code and exercise state on every keystroke.
* **Export / Import JSON**: One-click download/upload of `spanglings-progress-YYYY-MM-DD.json` enabling bi-directional portability with `~/.local/share/spanglings/state.json`.
* **Reset**: Granular exercise reset to starter template or full progress wipe with confirmation.

---

## 4. UI Layout & User Interactions

### 4.1 Split-Pane Syllabus Explorer (Left 320px)
* **Overall Progress Bar**: Shows completed count and percentage (e.g. `48 / 136 Completed • 35%`).
* **Live Search**: Instant keyword search matching grammar topics, concepts, or trigger words.
* **24 Topic Accordions**: Displays CEFR level badges (`B1`, `B2`, `C1`), completion counters (`4/6 ✓`), and individual exercise items. Clicking any exercise immediately mounts it into the editor.

### 4.2 Mode A: Exercise Workspace
* **Accent Bar**: Quick-click / hotkey insert buttons for `á`, `é`, `í`, `ó`, `ú`, `ñ`, `¿`, `¡`.
* **Monaco Editor / Interactive Blanks**: Syntax highlighting with `___` placeholders, automatic cursor positioning, and instant hotkey evaluation (`Ctrl+Enter`).
* **Progressive Hints (`H` / `?`)**: 3-tier progressive hint drawer revealing lexical clues without spoiling the solution.
* **Diff Reference (`D`)**: Side-by-side comparison against the grammatical reference solution.
* **Terminal Diagnostics**: ANSI colorized compiler diagnostics with line-specific errors and dual-layer explanations.

### 4.3 Mode B: Rapid Arcade Arena (Focused & Clean)
* **Showdown Duel Selector**: Choose specific pairs (`ser vs estar`, `por vs para`, `subjunctive vs indicative`, `se-matrix`, or all mixed).
* **Single-Key Gameplay**: Hotkeys `[1]` / `[2]` or `[J]` / `[K]` with instant sub-millisecond evaluation.
* **Clean Text Feedback**: Displays `✓ CORRECT! (+100 PTS)` or `✗ INCORRECT! Correct answer: X` followed immediately by the dual-layer cards (`💡 Meaning:` + `📐 Rule:`) with clean text formatting (no sound effects or visual shakes).
* **End-of-Round Summary**: Detailed recap table of missed duels with dual-layer review cards.

---

## 5. Verification & Testing Strategy

1. **Rust Wasm Crate Unit Tests**:
   * Verify all 136 sentence frames compile and validate through the Wasm bridge with 100% pass rate.
   * Verify all 262 showdown duel items and 5 specialized engines evaluate properly.
2. **State & Portability Tests**:
   * Test JSON serialization and deserialization compatibility with the native CLI state format.
3. **Docs Build Verification**:
   * Verify clean `mkdocs build --strict` output with zero broken links or missing static assets.
