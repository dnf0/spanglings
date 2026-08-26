# Smart Diagnostics & Pedagogical Enhancements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Weakness Profiler with category error analysis in `spanglings progress`, Accentuation cheat sheet in `spanglings explain accents`, and 60-second Rapid-Fire Blitz Mode (`spanglings blitz`).

**Architecture:**
1. **Accentuation Engine**: Add `ACCENTS_CARD` to `src/core/reference.rs` with aliases (`accents`, `accentuation`, `tildes`, `acentuacion`).
2. **Weakness Profiler**: Extend `ProgressSummary` and `src/cli/commands/progress.rs` to compute error rates per topic/category and recommend targeted exercises based on SRS reviews and failures.
3. **Blitz Mode (`spanglings blitz`)**: Implement `src/cli/commands/blitz.rs` with a countdown timer / time-trial (default 60s or custom `--duration <secs>`), quick prompts from baseline irregulars and clitic exercises, tracking score, streaks, and words/minute.

---

### Task 1: Accentuation & Orthographic Stress Reference Card

**Files:**
- Modify: `src/core/reference.rs`
- Modify: `tests/reference_tests.rs`

- [ ] **Step 1: Implement `ACCENTS_CARD` in `src/core/reference.rs`**
Cover:
- Agudas (stressed on last syllable, accent if ending in n, s, vocal)
- Llanas / Graves (stressed on penultimate syllable, accent if NOT ending in n, s, vocal)
- Esdrújulas / Sobreesdrújulas (always accented)
- Diptongos vs Hiatos (vocal abierta + vocal cerrada tónica: *país*, *día*, *continúo*)
- Diacritical accents (*tú* vs *tu*, *él* vs *el*, *mí* vs *mi*, *dé* vs *de*, *sé* vs *se*, *más* vs *mas*, *aún* vs *aun*, *por qué* vs *porque* vs *porqué*)

- [ ] **Step 2: Add integration test in `tests/reference_tests.rs`**
- [ ] **Step 3: Run `cargo test --test reference_tests` and commit**
```bash
git add src/core/reference.rs tests/reference_tests.rs
git commit -m "feat(engine): add Spanish accentuation and orthographic stress reference card"
```

---

### Task 2: Weakness Profiler & Topic Error Diagnostics

**Files:**
- Modify: `src/core/state.rs`
- Modify: `src/cli/commands/progress.rs`
- Create: `tests/weakness_profiler_tests.rs`

- [ ] **Step 1: Enhance `AppState` and `ProgressSummary`**
- Track failure counts per exercise/topic in `AppState` or compute from `SrsItem` repetitions and intervals (low repetitions or interval = 1 with reviews).
- In `src/cli/commands/progress.rs`, compute:
  - Accuracy / retention rate per topic
  - Identify top 3 weak areas (topics with lowest ease factor or highest pending reviews)
  - Recommend specific targeted exercises / drill commands
  - Add to JSON output in `ProgressSummary`.

- [ ] **Step 2: Add integration tests in `tests/weakness_profiler_tests.rs`**
- [ ] **Step 3: Run `cargo test` and commit**
```bash
git add src/core/state.rs src/cli/commands/progress.rs tests/weakness_profiler_tests.rs
git commit -m "feat(diagnostics): add weakness profiler and targeted topic recommendations to progress"
```

---

### Task 3: 60-Second Rapid-Fire Blitz Mode (`spanglings blitz`)

**Files:**
- Create: `src/cli/commands/blitz.rs`
- Modify: `src/cli/commands/mod.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`
- Create: `tests/blitz_tests.rs`

- [ ] **Step 1: Implement `src/cli/commands/blitz.rs`**
- CLI arguments: `spanglings blitz [--seconds <u64>] [--topic <topic>]`
- Loads rapid-fire prompts from baseline irregulars, clitics, and verb stem tables.
- Interactive terminal input with deadline / elapsed timer.
- Terminal output shows instant correct/incorrect feedback, final score, streak, accuracy %, and speed (items per minute).
- Non-interactive testable helper `run_blitz_item` and test simulation.

- [ ] **Step 2: Wire `Commands::Blitz` into CLI & dispatcher**
- [ ] **Step 3: Add unit/integration tests in `tests/blitz_tests.rs`**
- [ ] **Step 4: Verify with `cargo test`, `cargo clippy`, `cargo fmt`**
- [ ] **Step 5: Commit**
```bash
git add src/cli/commands/blitz.rs src/cli/commands/mod.rs src/cli/mod.rs src/main.rs tests/blitz_tests.rs
git commit -m "feat(cli): add 60-second rapid-fire blitz drill mode"
```

---

### Task 4: Documentation Update & Verification

**Files:**
- Modify: `README.md`
- Modify: `docs/BACKLOG.md`

- [ ] **Step 1: Update README.md with `spanglings blitz` and `spanglings explain accents`**
- [ ] **Step 2: Mark Focus Area 3 completed in `docs/BACKLOG.md`**
- [ ] **Step 3: Rebuild knowledge graph with `graphify`**
- [ ] **Step 4: Commit and push**
```bash
git add README.md docs/BACKLOG.md
git commit -m "docs: document weakness profiler, accent rules, and blitz mode"
git push origin feat/spanglings-design
```
