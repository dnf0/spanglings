# True Rust WebAssembly Engine Design Specification

- **Date**: 2026-09-02
- **Author**: Antigravity & Pair Programming Agent
- **Target Repository**: `spanglings` (`dnf0/spanglings`)
- **Status**: Approved Spec

---

## 1. Executive Overview

Spanglings treats Spanish natural language acquisition like compiler construction: natural language rules, verb conjugations, and mood triggers are syntax constraints validated in sub-millisecond memory.

This specification defines the true Rust WebAssembly engine compilation pipeline, wasm-bindgen export interface, browser lifecycle loader, and dual-mode learning environment (Monaco Syntax Studio + Rapid Arcade Arena) running 100% client-side in the browser.

---

## 2. Architecture & Target Isolation

### 2.1 Cargo Dependency Partitioning

To enable zero-error compilation for `wasm32-unknown-unknown` without breaking native terminal TUI, file watcher, or CLI subcommands:

1. **Native-Only Dependencies** (`[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`):
   - `ratatui = { version = "0.29", features = ["all-widgets"] }`
   - `crossterm = { version = "0.28", features = ["event-stream"] }`
   - `notify = "7.0"`
   - `notify-debouncer-mini = "0.5"`
   - `dirs = "5.0"`
   - `clap = { version = "4.5", features = ["derive", "cargo"] }`
   - `clap_complete = "4.5"`

2. **Universal Shared Dependencies** (`[dependencies]`):
   - `unicode-normalization = "0.1"`
   - `serde = { version = "1.0", features = ["derive"] }`
   - `serde_json = "1.0"`
   - `chrono = { version = "0.4", features = ["serde", "wasmbind"] }`
   - `colored = "2.1"`
   - `thiserror = "2.0"`
   - `anyhow = "1.0"`
   - `regex = "1.10"`
   - `include_dir = "0.7"`
   - `rand = "0.8"`
   - `wasm-bindgen = { version = "0.2", optional = true }`
   - `serde-wasm-bindgen = { version = "0.6", optional = true }`

3. **Wasm-Specific Target Dependencies** (`[target.'cfg(target_arch = "wasm32")'.dependencies]`):
   - `getrandom = { version = "0.2", features = ["js"] }`

4. **Features**:
   - `wasm = ["wasm-bindgen", "serde-wasm-bindgen"]`

### 2.2 Module Gating (`src/lib.rs`)

```rust
pub mod core;
pub mod engine;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;

#[cfg(not(target_arch = "wasm32"))]
pub mod lsp;

#[cfg(not(target_arch = "wasm32"))]
pub mod tui;

#[cfg(not(target_arch = "wasm32"))]
pub mod watcher;
```

---

## 3. WebAssembly Export Interface (`src/wasm.rs`)

The compiled WebAssembly binary exports 5 high-performance functions to JavaScript:

```rust
#[wasm_bindgen]
pub fn get_curriculum_catalog_json() -> String;

#[wasm_bindgen]
pub fn evaluate_exercise_wasm(frame_id: &str, user_input: &str, accent_mode: Option<String>) -> String;

#[wasm_bindgen]
pub fn get_arcade_catalog_json(mode: &str) -> String;

#[wasm_bindgen]
pub fn evaluate_arcade_choice_wasm(item_id: &str, user_choice: &str, elapsed_ms: u64) -> String;

#[wasm_bindgen]
pub fn calculate_sm2_review_wasm(ease_factor: f32, interval: u32, repetitions: u32, grade: u8) -> String;
```

### 3.1 Dual-Layer Pedagogical Schema
Every evaluation and catalog payload returned by WebAssembly guarantees dual-layer pedagogical fields:
- `meaning` / `plain_english`: Intuitive communicative mental model (e.g. `💡 Meaning: "Expresses location or temporary emotional state"`).
- `rule` / `explanation`: Structural grammar rule and diagnostic explanation (e.g. `📐 Rule: Use 'estar' for geographical position and transient conditions`).

---

## 4. Browser UI & Dual-Engine Integration

### 4.1 Lifecycle & Wasm Initialization (`docs/assets/playground/playground.js`)

1. **Bootstrapping**:
   - Asynchronously imports `./pkg/spanglings.js` and calls `await initWasm()`.
   - On success: updates top bar pill to `● Rust Wasm Engine Active` (pulsing green dot `#22c55e`).
   - If WebAssembly fails (e.g. restricted local `file://` origins): gracefully falls back to `playground-bundle.json` with status `○ Wasm Fallback Mode`.

2. **Mode 1: Monaco Syntax Studio**:
   - 24-topic syllabus sidebar with search and completion checkmarks.
   - Monaco Editor instance with Spanish syntax highlighting and keyboard shortcuts.
   - Accent bar for rapid character insertion (`á`, `é`, `í`, `ó`, `ú`, `ñ`, `ü`, `¿`, `¡`).
   - Accent validation mode dropdown (`Forgiving`, `Strict`, `Off`).
   - 3-tier progressive hint drawer.
   - Compiler-grade diagnostics card with error code tags, offending code line indicators, and dual-layer explanations.

3. **Mode 2: Rapid Arcade Arena**:
   - Single-key hotkeys: `1`/`2`/`3`/`4` or `J`/`K`/`L`/`;`.
   - 16 showdown duel pools + 5 specialized 4-choice engines (`regimen`, `irregulars`, `false-friends`, `se-matrix`, `connectors`).
   - Sub-millisecond scoring: base +100 points, plus speed bonus (+1 per 15ms under 1500ms).
   - Post-round mistake review card with trigger sentences, wrong answers in red, correct answers in green, and communicative rationale.

4. **Persistence & SM-2 Spaced Repetition**:
   - Tracks exercise completion, mastery levels, and interval decay in `localStorage` under `spanglings_state_v1`.
   - Updates item intervals via `calculate_sm2_review_wasm()`.

---

## 5. Build Automation & CI/CD Pipeline

1. **Compilation Step**:
   - Run `npx wasm-pack build --target web --out-dir docs/assets/playground/pkg --release --features wasm`.
   - Run `python scripts/build_playground_bundle.py` (for the fallback bundle).

2. **Automated Verification**:
   - `cargo test --all-targets` (Native unit tests)
   - `cargo test --test wasm_tests` (Wasm bridge tests)
   - `uv run pytest` (Browser UI controller and storage parity tests)
   - `uv run mkdocs build --strict` (Documentation and playground packaging validation)

3. **Continuous Deployment (`.github/workflows/docs.yml`)**:
   - Installs `wasm32-unknown-unknown` target.
   - Builds `wasm-pack` release output before `mkdocs build`.
   - Deploys full package containing `pkg/spanglings_bg.wasm` and `pkg/spanglings.js` to GitHub Pages.
