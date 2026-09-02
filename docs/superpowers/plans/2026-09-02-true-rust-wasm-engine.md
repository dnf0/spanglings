# True Rust WebAssembly Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Choose an execution mode:
> 1. `superpowers:subagent-driven-development` (recommended for multi-agent reviews, backed by `SKILL.state` / `.agent-state/state.json`)
> 2. `agent-rules:stateful-execution` (SKILL.state) (recommended for deterministic single-agent linear execution)
> 3. `superpowers:executing-plans` (batch execution with manual checkpoints)
> Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Compile Spanglings into a true Rust WebAssembly binary (`.wasm`) using `wasm-pack`, integrate the wasm-bindgen runtime into the browser playground and arcade UI, and automate CI/CD publishing to GitHub Pages.

**Architecture:** Partition `Cargo.toml` dependencies and `src/lib.rs` modules to enable zero-error compilation for `wasm32-unknown-unknown`. Compile with `npx wasm-pack build --target web --out-dir docs/assets/playground/pkg --features wasm`. In `playground.js`, dynamically bootstrap the Rust Wasm module on load and call Rust evaluator functions with automatic graceful fallback.

**Tech Stack:** Rust 2021, `wasm-bindgen`, `serde-wasm-bindgen`, `wasm-pack`, Python 3.12 (pytest), JavaScript ES modules, Monaco Editor, MkDocs Material.

## Global Constraints
- Target architecture: `wasm32-unknown-unknown`.
- Wasm package directory: `docs/assets/playground/pkg/`.
- Dual-layer pedagogical feedback (`💡 Meaning / Context:` + `📐 Grammar Rule:`) mandatory in all evaluation responses.
- Zero audio / zero screen-shake animations.
- All native CLI and TUI tests must continue passing 100%.

---

### Task 1: Cargo Dependency Partitioning & Wasm32 Target Isolation

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: `src/wasm.rs`
- Test: `tests/wasm_tests.rs`

**Interfaces:**
- Produces: Clean compilation for both native target (`cargo test --all-targets`) and `wasm32-unknown-unknown` (`cargo check --target wasm32-unknown-unknown --features wasm`).

- [ ] **Step 1: Write/update the wasm test suite for Rust exports**

```rust
// tests/wasm_tests.rs
use spanglings::wasm::{
    calculate_sm2_review_wasm, evaluate_arcade_choice_wasm, evaluate_exercise_wasm,
    get_arcade_catalog_json, get_curriculum_catalog_json,
};

#[test]
fn test_wasm_exports_return_valid_json() {
    let curriculum = get_curriculum_catalog_json();
    assert!(curriculum.contains("count"));
    assert!(curriculum.contains("exercises"));

    let arcade = get_arcade_catalog_json("all");
    assert!(arcade.contains("items"));

    let eval = evaluate_exercise_wasm("00_01_ser_vs_estar", "Madrid es la capital de España.");
    assert!(eval.contains("\"is_correct\":true"));

    let arcade_eval = evaluate_arcade_choice_wasm("ser_estar_0", "es", 200);
    assert!(arcade_eval.contains("\"is_correct\":true"));

    let sm2 = calculate_sm2_review_wasm(2.5, 1, 1, 5);
    assert!(sm2.contains("ease_factor"));
}
```

- [ ] **Step 2: Update Cargo.toml dependency partitioning**

Move native-only crates (`ratatui`, `crossterm`, `notify`, `notify-debouncer-mini`, `dirs`, `clap`, `clap_complete`) under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`. Ensure `getrandom = { version = "0.2", features = ["js"] }` is set for wasm32.

- [ ] **Step 3: Gate modules in src/lib.rs**

Gate `cli`, `tui`, `watcher`, and `lsp` behind `#[cfg(not(target_arch = "wasm32"))]`.

- [ ] **Step 4: Verify native and wasm compilation**

Run: `cargo test --all-targets`
Run: `cargo check --target wasm32-unknown-unknown --features wasm`
Expected: 100% PASS with 0 errors.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/lib.rs src/wasm.rs tests/wasm_tests.rs
git commit --no-gpg-sign -m "feat(wasm): partition dependencies and gate native modules for wasm32"
```

---

### Task 2: Wasm Compilation Pipeline (`wasm-pack`) & Package Artifacts

**Files:**
- Create: `scripts/build_wasm.py`
- Modify: `pyproject.toml`
- Test: `tests/test_wasm_package.py`

**Interfaces:**
- Produces: `docs/assets/playground/pkg/spanglings.js` and `docs/assets/playground/pkg/spanglings_bg.wasm`.

- [ ] **Step 1: Write test for generated Wasm package artifacts**

Create `tests/test_wasm_package.py` to verify that `docs/assets/playground/pkg/spanglings.js` and `docs/assets/playground/pkg/spanglings_bg.wasm` exist and export `init`, `get_curriculum_catalog_json`, `evaluate_exercise_wasm`, `get_arcade_catalog_json`, `evaluate_arcade_choice_wasm`, and `calculate_sm2_review_wasm`.

- [ ] **Step 2: Create scripts/build_wasm.py build runner**

Implement `scripts/build_wasm.py` to invoke `wasm-pack build --target web --out-dir docs/assets/playground/pkg --release --features wasm` and verify outputs.

- [ ] **Step 3: Execute build_wasm.py to generate artifacts**

Run: `python scripts/build_wasm.py`
Expected: Generates `pkg/spanglings.js` and `pkg/spanglings_bg.wasm`.

- [ ] **Step 4: Run package test to verify artifacts**

Run: `uv run pytest tests/test_wasm_package.py`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add scripts/build_wasm.py tests/test_wasm_package.py docs/assets/playground/pkg/
git commit --no-gpg-sign -m "feat(wasm): add wasm-pack build pipeline and generated pkg artifacts"
```

---

### Task 3: Browser UI Controller Rust Wasm Integration & Fallback

**Files:**
- Modify: `docs/assets/playground/playground.js`
- Modify: `docs/assets/playground/playground.css`
- Modify: `docs/playground/index.html`
- Test: `tests/test_wasm_ui_integration.py`

**Interfaces:**
- Consumes: `docs/assets/playground/pkg/spanglings.js` and `spanglings_bg.wasm`.
- Produces: Full dual-mode UI execution powered directly by the compiled Rust WebAssembly module, with graceful fallback.

- [ ] **Step 1: Write test for browser Wasm loader & UI lifecycle**

Create `tests/test_wasm_ui_integration.py` using Node.js subprocess testing to verify Wasm initialization, evaluator function invocation, status pill states, and fallback handling.

- [ ] **Step 2: Update playground.js to bootstrap Rust WebAssembly**

Update `docs/assets/playground/playground.js`:
- Dynamic import of `./pkg/spanglings.js` and `initWasm()`.
- Route `evaluateExercise` through `evaluate_exercise_wasm`.
- Route `evaluateArcadeChoice` through `evaluate_arcade_choice_wasm`.
- Route `calculate_sm2_review` through `calculate_sm2_review_wasm`.
- Set status pill to `● Rust Wasm Engine Active` (green dot) on success, or `○ Wasm Fallback Mode` (muted) on error.

- [ ] **Step 3: Verify all browser and python tests pass**

Run: `uv run pytest tests/test_wasm_ui_integration.py tests/test_playground_ui.py tests/test_arcade_ui.py`
Expected: 100% PASS.

- [ ] **Step 4: Commit**

```bash
git add docs/assets/playground/playground.js docs/assets/playground/playground.css docs/playground/index.html tests/test_wasm_ui_integration.py
git commit --no-gpg-sign -m "feat(playground): integrate rust wasm module with dual-mode workspace and fallback"
```

---

### Task 4: CI/CD Pipeline & Full Verification

**Files:**
- Modify: `.github/workflows/docs.yml`
- Test: Full test suite

**Interfaces:**
- Produces: Automated build and deployment of WebAssembly binary to GitHub Pages on every `main` push.

- [ ] **Step 1: Update .github/workflows/docs.yml**

Add wasm build steps before `mkdocs build`:
```yaml
      - name: Install wasm32 target & wasm-pack
        run: |
          rustup target add wasm32-unknown-unknown
          npx wasm-pack build --target web --out-dir docs/assets/playground/pkg --release --features wasm
```

- [ ] **Step 2: Run complete local verification suite**

Run: `cargo test --all-targets`
Run: `uv run pytest`
Run: `uv run mkdocs build --strict`
Expected: 0 errors, 0 warnings.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/docs.yml
git commit --no-gpg-sign -m "ci(docs): automate wasm-pack compilation in documentation workflow"
```
