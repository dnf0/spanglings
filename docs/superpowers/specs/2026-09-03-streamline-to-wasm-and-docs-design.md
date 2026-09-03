# Design Specification: Streamlining Spanglings to Pure WebAssembly & Spanish Language Manual

## 1. Overview & Objective
Spanglings is transitioning from a hybrid terminal-TUI / web experiment into a focused, developer-grade **Spanish Language Learning Ecosystem** consisting of two core pillars:
1. **📘 Spanish Language Reference Manual**: 24 comprehensive topics across 3 CEFR tiers (Foundations A1-A2, Mood & Triggers B1-B2, Nuance & Edge Mechanics B2-C1) with dual-layer explanations (`💡 Communicative Mental Model` + `📐 Grammar Rule & Decision Matrix`).
2. **⚡ Interactive WebAssembly Playground**: Zero-install browser environment powered by compiled Rust WebAssembly (`wasm32-unknown-unknown`), featuring:
   - **Curriculum Syntax Studio**: Monaco-powered editor with real-time compiler diagnostics and contextual cheat sheets.
   - **Rapid Arcade Arena**: Single-key showdown duels (16 pairs, 262 items) and 5 specialized drill engines.

All legacy terminal UI components (`src/tui/`), file watcher routines (`src/watcher/`), and heavy terminal dependencies (`ratatui`, `crossterm`, `notify`, `notify-debouncer-mini`) are permanently removed.

---

## 2. Architecture & Subsystems

```
                                +-----------------------------------+
                                |     Spanglings Documentation      |
                                |       & Web Platform Portal       |
                                +-----------------+-----------------+
                                                  |
                        +-------------------------+-------------------------+
                        |                                                   |
                        v                                                   v
        +-------------------------------+                   +-------------------------------+
        |    Spanish Language Manual    |                   |   Interactive Wasm Studio     |
        |   (24 Topics / Dual-Layer)    |                   |   (Curriculum + Rapid Arena)  |
        |   docs/manual.md & syllabus   |                   |    docs/playground/index.html |
        +---------------+---------------+                   +---------------+---------------+
                        |                                                   |
                        +-------------------------+-------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |    Rust WebAssembly Core Engine   |
                                |     (src/wasm.rs / src/core/)     |
                                +-----------------+-----------------+
                                                  |
                        +-------------------------+-------------------------+
                        |                         |                         |
                        v                         v                         v
        +-----------------------+ +-----------------------+ +-----------------------+
        |  Curriculum Catalog   | |  Showdown / Drills    | |  SM-2 Recall Engine   |
        | 24 Topics / 136 Frames| | 16 Pairs / 5 Engines  | | Spaced Interval Calc  |
        +-----------------------+ +-----------------------+ +-----------------------+
```

### 2.1 Codebase Cleanup: Dropping Terminal TUI
- **Files to Delete**:
  - `src/tui/`: `app.rs`, `mod.rs`, `ui.rs`, `arcade_ui.rs`, `tour.rs`, `modals.rs`, `theme.rs`, and all submodules.
  - `src/watcher/`: `mod.rs` and file watching logic.
  - `tests/tui_tests.rs`, `tests/tui_arcade_tests.rs`, `tests/watcher_tests.rs`, `tests/tour_tests.rs`.
- **Dependencies Removed from `Cargo.toml`**:
  - `ratatui`, `crossterm`, `notify`, `notify-debouncer-mini`, `dirs`, `clap_complete`.
- **Streamlined `src/lib.rs`**:
  - Exposes `core`, `evaluator`, `reference`, and `wasm` modules cleanly for both native testing (`rlib`) and WebAssembly compilation (`cdylib`).

### 2.2 Preserved & Tested Rust Core
- `src/core/`: Exercises, tracks, curriculum structures, spaced repetition (SM-2), weakness profiling, concept mastery DAG.
- `src/evaluator/`: Dual-layer feedback engine, error codes (`E0101` - `E0502`), syntax diagnostics.
- `src/reference/`: All 24 topic reference cards with communicative mental models and decision trees.
- `src/wasm.rs`: Exported WebAssembly functions for JS binding:
  - `get_curriculum_catalog_json()`
  - `evaluate_exercise_wasm(exercise_id, code)`
  - `get_arcade_catalog_json(mode)`
  - `evaluate_arcade_choice_wasm(item_id, selected_choice)`
  - `calculate_sm2_review_wasm(state_json, rating)`

---

## 3. Documentation & Asset Modernization
- **`docs/index.md`**:
  - Clean hero layout showcasing the Spanish Language Manual, Interactive Web Playground, and Curriculum Syllabus.
  - Remove all terminal CLI / TUI references (`cargo install spanglings`, `spanglings init`, `spanglings watch`).
  - Delete or archive `docs/assets/spanglings-demo.svg` (terminal animation) and update visual branding to highlight the browser playground.
- **`README.md`**:
  - Aligned with `docs/index.md` to introduce Spanglings as a WebAssembly-powered Spanish learning platform and comprehensive reference manual.
- **`mkdocs.yml`**:
  - Configured with `md_in_html`, `pymdownx.emoji`, and clean indigo/slate theme.

---

## 4. Build Pipelines & CI
- **`scripts/build_wasm.py`**:
  - Continues to compile `src/wasm.rs` via `wasm-pack build --target web --out-dir docs/assets/playground/pkg --release --features wasm`.
  - Builds and synchronizes fallback JSON bundle.
- **GitHub Actions (`.github/workflows/ci.yml` & `docs.yml`)**:
  - `ci.yml`: Tests Rust core (`cargo test --all-targets`), verifies Wasm build (`cargo check --target wasm32-unknown-unknown --features wasm`), runs pytest suite on Wasm JS bridge & docs.
  - `docs.yml`: Builds WebAssembly artifacts and deploys MkDocs site to GitHub Pages.

---

## 5. Verification & Acceptance Criteria
1. **Compilation**: `cargo check --target wasm32-unknown-unknown --features wasm` and `cargo test` pass with 0 warnings.
2. **WebAssembly Integration**: `python scripts/build_wasm.py` compiles clean `.wasm` and `.js` artifacts into `docs/assets/playground/pkg/`.
3. **Automated Tests**:
   - `uv run pytest -v` passes 100% (including `test_wasm_package.py`, `test_docs_playground.py`, `test_arcade_ui.py`).
   - `uv run mkdocs build --strict` completes with exit code 0.
4. **Documentation**: `docs/index.md` and `README.md` cleanly present the dual platform without terminal cruft.
