# Design Specification: Spanglings Rapid Single-Key ADHD Arcade & Showdown Engine

**Date:** 2026-08-28  
**Author:** Google DeepMind / Antigravity Pair Programming  
**Status:** DRAFT (Under User Review)

---

## 1. Problem Statement & Motivation

Traditional language drills require typing full conjugated Spanish words (e.g., *escribiéramos*, *condujeron*, *desarrollándolo*) followed by pressing `Enter` and reading multiline feedback. 

For learners with **ADHD** or those seeking **rapid-fire flow states**, this typing friction causes:
1. **Cognitive Fatigue & Speed Lag**: Typing 10+ characters per question slows practice down to 5–10 seconds per item.
2. **Loss of Dopamine Loop**: Delays between stimulus and feedback break focus and cause habituation/boredom within seconds.
3. **Friction in Binary Grammatical Dilemmas**: Core Spanish dilemmas (*Por vs Para*, *Ser vs Estar*, *Subjunctive vs Indicative*, *Pretérito vs Imperfecto*) are fundamentally binary choices that are best learned through high-volume, rapid-fire pattern recognition at 1–2 items per second.

---

## 2. Core Architecture & Game Loops

### 2.1 Two Zero-Friction Rapid Modes

```
+-----------------------------------------------------------------------+
|  MODE 1: Binary Showdown (j / k or ← / →)                            |
|  High-frequency paired grammatical contrasts                           |
|  e.g. Por vs Para, Ser vs Estar, Subjuntivo vs Indicativo             |
+-----------------------------------------------------------------------+
                                  │
                                  ▼
+-----------------------------------------------------------------------+
|  MODE 2: 4-Choice Rapid Cloze (1 / 2 / 3 / 4)                        |
|  1 correct target + 3 algorithmic smart distractors across 24 topics   |
|  e.g. Tense foils, subjunctive foils, person foils, false friends    |
+-----------------------------------------------------------------------+
```

### 2.2 Hyper-Speed Auto-Advance Mechanics
- **Zero Enter Keys**: Answering requires exactly **one physical keypress** (`j`, `k`, `1`, `2`, `3`, `4`, `←`, `→`).
- **Instant 150–200ms Result Flash**:
  - Correct: Instant Green glow (`✓`), `+100 XP`, combo streak increments (`🔥 5x Streak!`), subtle audio tick/pop.
  - Incorrect: Instant Red flash (`✗`), displays correct answer + formula cue for 400ms, resets combo streak, registers SM-2 lapse.
- **Immediate Auto-Transition**: Advances to the next question automatically without requiring any keypress to proceed.

---

## 3. Data Structures & Engine (`src/core/arcade.rs`)

### 3.1 Showdown Pairs
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShowdownPair {
    PorPara,         // Por [j] vs Para [k]
    SerEstar,        // Ser [j] vs Estar [k]
    SubjInd,         // Subjuntivo [j] vs Indicativo [k]
    PretImp,         // Pretérito [j] vs Imperfecto [k]
    TuUsted,         // Tú / Informal [j] vs Usted / Formal [k]
    LoLe,            // Direct (lo/la) [j] vs Indirect (le/les) [k]
    SinoPero,        // Sino [j] vs Pero [k]
    ParaQuePorque,   // Para que (+subj) [j] vs Porque (+ind) [k]
}
```

### 3.2 Arcade Item & Smart Distractor Generation
```rust
pub struct ArcadeItem {
    pub topic: String,
    pub trigger_sentence: String,
    pub prompt_cue: String,
    pub options: Vec<String>,      // Exactly 2 (for Showdown) or 4 (for Cloze)
    pub correct_index: usize,       // 0..options.len()
    pub explanation: String,
}
```
- **Distractor Synthesis**:
  - For verb conjugation clozes: generates (1) opposite mood foil (e.g., *tenga* vs *tiene*), (2) wrong person/number foil (e.g., *tengan*, *tengamos*), and (3) tense confusion foil (e.g., *tuviera*).
  - For prepositions/pronouns: samples authentic lexical confusions from `src/core/reference.rs`.

---

## 4. Visual Stimulation & Gamification

### 4.1 Combo Multipliers & Ranks
- **1x–2x**: Normal (`✨ Streak: 2`)
- **3x–4x**: `⚡ Quick! (3x Multiplier)`
- **5x–9x**: `🔥 ON FIRE! (5x Multiplier)`
- **10x–14x**: `🚀 UNSTOPPABLE! (10x Multiplier)`
- **15x–19x**: `👑 GODLIKE! (15x Multiplier)`
- **20x+**: `⚡⚡ ULTRA INSTINCT (20x Multiplier) ⚡⚡`

### 4.2 Score Calculation & Speed Bonus
$$\text{Score} = (\text{Base } 100 \times \text{Combo Multiplier}) + \text{Speed Bonus}$$
- Response $< 800\text{ms}$: $+100\text{ XP}$ (*Lightning*)
- Response $< 1500\text{ms}$: $+50\text{ XP}$ (*Swift*)
- Response $\ge 1500\text{ms}$: $+0\text{ XP}$

### 4.3 Native Audio Cues (Zero External Dependencies)
- Uses non-blocking background `afplay` on macOS (`/System/Library/Sounds/Tink.aiff` on correct, `Sosumi.aiff` or `Basso.aiff` on lapse) or ASCII terminal bell (`\x07`) on Linux.
- Disabled via `--no-sound` or config toggle.

---

## 5. UI & CLI Integration

### 5.1 CLI Command (`spanglings arcade`)
```bash
spanglings arcade                         # Mixed rapid-choice session (20 questions)
spanglings arcade --showdown por-para     # Pure Por vs Para showdown (j vs k)
spanglings arcade --showdown ser-estar    # Pure Ser vs Estar showdown
spanglings arcade --showdown subj-ind     # Pure Subjunctive vs Indicative
spanglings arcade --weak                  # Rapid 4-choice items targeting lowest-mastery concepts
spanglings arcade -n 50 --sound           # 50-item sprint with audio effects
```

### 5.2 TUI Arcade Arena Modal (`[x]` / `[d]`)
- Accessible anywhere in the TUI via keybinding **`[x]`** (Arcade) or **`[d]`** (Topic Drill).
- Displays a dedicated full-screen arcade overlay:
  - Header: Animated combo flame badge, timer bar, score & multiplier.
  - Question Card: Large highlighted sentence with cloze gap `[ ____ ]`.
  - Choices:
    - In **Showdown mode**: 2 large side-by-side cards: `[ J ] Por` vs `[ K ] Para`.
    - In **4-Choice mode**: 4 large numbered blocks: `[ 1 ] tenga`, `[ 2 ] tiene`, `[ 3 ] tuviera`, `[ 4 ] tendré`.
  - Immediate responsive key handling (`1`-`4`, `j`/`k`, `←`/`→`) with zero cursor navigation needed.

---

## 6. Testing & Quality Gates
1. Unit tests in `tests/arcade_tests.rs`:
   - All 8 `ShowdownPair` generators produce valid 2-option items with correct answers.
   - 4-choice generator produces exactly 4 unique options with 1 correct target across all 24 concepts.
   - Distractor generator produces grammatical foils without duplicate options.
2. CLI and TUI navigation tests in `tests/tui_tests.rs`:
   - Headless single-key arcade navigation, streak accumulation, and live mastery updates.
3. Strict verification: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`.
