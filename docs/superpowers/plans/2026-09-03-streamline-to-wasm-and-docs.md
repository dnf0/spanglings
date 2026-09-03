# Streamline to Pure WebAssembly & Spanish Language Manual Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Choose an execution mode:
> 1. `superpowers:subagent-driven-development` (recommended for multi-agent reviews, backed by `SKILL.state` / `.agent-state/state.json`)
> 2. `agent-rules:stateful-execution` (SKILL.state) (recommended for deterministic single-agent linear execution)
> 3. `superpowers:executing-plans` (batch execution with manual checkpoints)
> Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove legacy terminal TUI, watcher, and dependencies, establishing Spanglings purely as a Rust WebAssembly Engine powering the interactive browser playground and a comprehensive Spanish Language Reference Manual.

**Architecture:** Rust core (`src/core/`, `src/evaluator/`, `src/reference/`, `src/wasm.rs`) is compiled directly to WebAssembly (`wasm32-unknown-unknown`) via `wasm-pack` and bundled with the interactive frontend in `docs/playground/index.html`. Documentation site is generated with MkDocs Material (`docs/manual.md`, `docs/syllabus.md`, `docs/index.md`).

**Tech Stack:** Rust 2021, WebAssembly (`wasm-bindgen`), Python 3.12 (`pytest`, `mkdocs-material`), JavaScript / Monaco Editor / HTML5 / CSS3.

## Global Constraints
- Zero terminal TUI dependencies (`ratatui`, `crossterm`, `notify` removed).
- Retain 100% of the 24 pedagogical topics, 136 curriculum frames, and 262 arcade showdown items in `src/core/` and `src/reference/`.
- All automated tests (`cargo test`, `uv run pytest`, `uv run mkdocs build --strict`) must pass with exit code 0.
- Conventional commits with `--no-gpg-sign`.

---

### Task 1: Remove TUI and Watcher Modules, Update Cargo Dependencies

**Files:**
- Delete: `src/tui/` (entire directory)
- Delete: `src/watcher/` (entire directory)
- Delete: `tests/tui_tests.rs`, `tests/tui_arcade_tests.rs`, `tests/watcher_tests.rs`, `tests/tour_tests.rs`
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: `src/main.rs`

**Interfaces:**
- Consumes: None
- Produces: Clean `src/lib.rs` exposing `core`, `evaluator`, `reference`, and `wasm` modules without `tui` or `watcher`.

- [ ] **Step 1: Delete TUI & Watcher source and test files**
  Delete `src/tui/`, `src/watcher/`, and TUI/watcher test files.

- [ ] **Step 2: Update Cargo.toml to remove unused terminal dependencies**
  Remove `ratatui`, `crossterm`, `notify`, `notify-debouncer-mini`, `dirs`, `clap_complete`.

- [ ] **Step 3: Update `src/lib.rs` and `src/main.rs`**
  Remove `pub mod tui;` and `pub mod watcher;` from `src/lib.rs`. Simplify `src/main.rs` to print a concise welcome and reference to the documentation / playground.

- [ ] **Step 4: Verify compilation and tests**
  Run: `cargo test --all-targets` and `cargo check --target wasm32-unknown-unknown --features wasm`
  Expected: PASS with 0 errors.

- [ ] **Step 5: Commit changes**
  Run: `git commit --no-gpg-sign -m "refactor(core): remove legacy tui, watcher, and terminal dependencies"`

---

### Task 2: Build WebAssembly Engine and Verify Pytest Suite

**Files:**
- Modify: `scripts/build_wasm.py`
- Test: `tests/test_wasm_package.py`
- Test: `tests/test_arcade_ui.py`
- Test: `tests/test_docs_playground.py`

**Interfaces:**
- Consumes: `src/wasm.rs`
- Produces: `docs/assets/playground/pkg/spanglings.js` and `spanglings_bg.wasm`

- [ ] **Step 1: Execute WebAssembly build pipeline**
  Run: `python scripts/build_wasm.py`
  Expected: Generates fresh `.wasm` and `.js` artifacts in `docs/assets/playground/pkg/`.

- [ ] **Step 2: Run pytest suite across Wasm and documentation tests**
  Run: `uv run pytest -v`
  Expected: All tests in `test_wasm_package.py`, `test_arcade_ui.py`, and `test_docs_playground.py` PASS.

- [ ] **Step 3: Commit changes**
  Run: `git commit --no-gpg-sign -m "feat(wasm): verify clean wasm build and test suite"`

---

### Task 3: Modernize Overview and Documentation (Zero TUI Cruft)

**Files:**
- Modify: `docs/index.md`
- Modify: `README.md`
- Modify: `tests/test_docs_playground.py`

**Interfaces:**
- Consumes: Documentation and assets
- Produces: Clean landing pages and README centered on Language Manual and Interactive Playground.

- [ ] **Step 1: Update `docs/index.md`**
  Remove terminal CLI references (`spanglings init`, `spanglings watch`), terminal demo SVG, and focus purely on the Spanish Language Manual and WebAssembly Playground.

- [ ] **Step 2: Update `README.md`**
  Synchronize README with `docs/index.md` to present Spanglings as a modern web-first Spanish language platform.

- [ ] **Step 3: Run strict MkDocs build and pytest**
  Run: `uv run mkdocs build --strict && uv run pytest tests/test_docs_playground.py -v`
  Expected: PASS with 0 warnings.

- [ ] **Step 4: Commit changes**
  Run: `git commit --no-gpg-sign -m "docs: refresh overview and readme to focus purely on manual and web playground"`

---

### Task 4: Update CI Workflows & Final End-to-End Verification

**Files:**
- Modify: `.github/workflows/ci.yml`

**Interfaces:**
- Consumes: Clean crate and docs
- Produces: Fast, reliable GitHub Actions CI pipeline.

- [ ] **Step 1: Update `.github/workflows/ci.yml`**
  Remove redundant terminal smoke tests, keeping Rust cargo tests, Wasm check, pytest suite, and formatting checks.

- [ ] **Step 2: Run full verification checklist locally**
  Run:
  - `cargo test --all-targets`
  - `cargo check --target wasm32-unknown-unknown --features wasm`
  - `uv run pytest -v`
  - `uv run mkdocs build --strict`
  - `uv run ruff check scripts tests`

- [ ] **Step 3: Commit changes**
  Run: `git commit --no-gpg-sign -m "ci: streamline CI workflows for pure wasm and docs platform"`
