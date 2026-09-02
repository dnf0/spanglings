# Spanglings WebAssembly Browser Platform Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and deploy the client-side WebAssembly browser platform for Spanglings at `https://dnf0.github.io/spanglings/playground/`, featuring full 24-topic curriculum workspace, Monaco editor with accent assist, clean single-key rapid showdown arcade, and `localStorage` state persistence with CLI JSON portability.

**Architecture:** A lightweight Rust WebAssembly bridge (`src/wasm.rs`) compiled via `wasm-bindgen` coupled with a zero-backend static client (`docs/assets/playground/`) integrated into Material for MkDocs. State persistence matches the CLI's `~/.local/share/spanglings/state.json` schema.

**Tech Stack:** Rust (`wasm-bindgen`, `serde-wasm-bindgen`), WebAssembly, JavaScript (ES6 Modules), Monaco Editor, Material for MkDocs, CSS3 Split-Pane layout.

## Global Constraints
- Target URL: `https://dnf0.github.io/spanglings/playground/`
- Zero external server dependencies (100% static on GitHub Pages).
- Exact CLI state schema compatibility for JSON export/import (`~/.local/share/spanglings/state.json`).
- High-contrast, clean developer-grade terminal UI (zero sound effects or visual screen shakes).
- Strict `mkdocs build --strict` zero-warning compliance.
- Conventional commits with `--no-gpg-sign`.

---

### Task 1: Rust WebAssembly Interface & Evaluator Bridge

**Files:**
- Modify: `Cargo.toml`
- Create: `src/wasm.rs`
- Modify: `src/lib.rs`
- Create: `tests/wasm_tests.rs`

**Interfaces:**
- Consumes: `src/core/generator.rs`, `src/core/arcade.rs`, `src/core/evaluator.rs`, `src/core/srs.rs`
- Produces:
  - `get_curriculum_catalog_json() -> String`
  - `evaluate_exercise_wasm(frame_id: &str, user_input: &str) -> String`
  - `get_arcade_catalog_json(mode: &str) -> String`
  - `evaluate_arcade_choice_wasm(item_id: &str, user_choice: &str, elapsed_ms: u64) -> String`
  - `calculate_sm2_review_wasm(ease_factor: f32, interval: u32, repetitions: u32, grade: u8) -> String`

- [ ] **Step 1: Update `Cargo.toml` and `src/lib.rs` for wasm-bindgen**

Add optional wasm dependencies and `cdylib` crate-type:
```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = { version = "0.2", optional = true }
serde-wasm-bindgen = { version = "0.6", optional = true }

[features]
default = []
wasm = ["wasm-bindgen", "serde-wasm-bindgen"]
```

In `src/lib.rs`:
```rust
#[cfg(feature = "wasm")]
pub mod wasm;
```

- [ ] **Step 2: Write tests in `tests/wasm_tests.rs`**

```rust
use spanglings::core::arcade::get_showdown_pool;
use spanglings::core::generator::get_all_frames;
use spanglings::wasm::*;

#[test]
fn test_wasm_curriculum_catalog_returns_all_frames() {
    let catalog_json = get_curriculum_catalog_json();
    assert!(!catalog_json.is_empty());
    assert!(catalog_json.contains("ser-vs-estar"));
    assert!(catalog_json.contains("subjunctive-present"));
}

#[test]
fn test_wasm_evaluate_exercise_valid_and_invalid() {
    let valid_result = evaluate_exercise_wasm("ser-vs-estar-01", "es");
    assert!(valid_result.contains("\"is_valid\":true") || valid_result.contains("\"is_valid\": true"));
    
    let invalid_result = evaluate_exercise_wasm("ser-vs-estar-01", "esta");
    assert!(invalid_result.contains("\"is_valid\":false") || invalid_result.contains("\"is_valid\": false"));
}

#[test]
fn test_wasm_arcade_catalog_and_evaluation() {
    let catalog = get_arcade_catalog_json("all");
    assert!(catalog.contains("items"));
    
    let eval_res = evaluate_arcade_choice_wasm("showdown-ser-estar-0", "es", 250);
    assert!(eval_res.contains("is_correct"));
    assert!(eval_res.contains("meaning") || eval_res.contains("plain_english"));
    assert!(eval_res.contains("explanation"));
}

#[test]
fn test_wasm_sm2_calculation() {
    let sm2_res = calculate_sm2_review_wasm(2.5, 1, 0, 5);
    assert!(sm2_res.contains("ease_factor"));
    assert!(sm2_res.contains("interval"));
}
```

- [ ] **Step 3: Implement `src/wasm.rs`**

```rust
use crate::core::arcade::{get_arcade_items_for_mode, get_showdown_pool, ArcadeItem};
use crate::core::evaluator::evaluate_submission;
use crate::core::generator::{get_all_frames, SentenceFrame};
use crate::core::reference::{get_all_concepts, get_reference_card};
use crate::core::srs::{calculate_sm2, SrsGrade};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WasmExerciseEvaluation {
    pub is_valid: bool,
    pub score: u32,
    pub feedback: String,
    pub error_line: Option<usize>,
    pub meaning: String,
    pub rule: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmArcadeEvaluation {
    pub is_correct: bool,
    pub points_earned: u32,
    pub speed_bonus: u32,
    pub correct_answer: String,
    pub meaning: String,
    pub rule: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmSm2Result {
    pub ease_factor: f32,
    pub interval: u32,
    pub repetitions: u32,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn get_curriculum_catalog_json() -> String {
    let frames = get_all_frames();
    let concepts = get_all_concepts();
    serde_json::json!({
        "topics": concepts,
        "frames": frames,
    })
    .to_string()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn evaluate_exercise_wasm(frame_id: &str, user_input: &str) -> String {
    let frames = get_all_frames();
    if let Some(frame) = frames.iter().find(|f| f.id == frame_id) {
        let (is_valid, feedback) = evaluate_submission(frame, user_input);
        let eval = WasmExerciseEvaluation {
            is_valid,
            score: if is_valid { 100 } else { 0 },
            feedback,
            error_line: None,
            meaning: frame.plain_english.to_string(),
            rule: frame.explanation.to_string(),
        };
        serde_json::to_string(&eval).unwrap_or_default()
    } else {
        serde_json::to_string(&WasmExerciseEvaluation {
            is_valid: false,
            score: 0,
            feedback: format!("Exercise '{}' not found", frame_id),
            error_line: None,
            meaning: String::new(),
            rule: String::new(),
        })
        .unwrap_or_default()
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn get_arcade_catalog_json(mode: &str) -> String {
    let items = get_arcade_items_for_mode(mode);
    serde_json::json!({ "items": items }).to_string()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn evaluate_arcade_choice_wasm(item_id: &str, user_choice: &str, elapsed_ms: u64) -> String {
    let items = get_arcade_items_for_mode("all");
    if let Some(item) = items.iter().find(|i| i.id == item_id) {
        let is_correct = item.is_correct_choice(user_choice);
        let speed_bonus = if is_correct && elapsed_ms < 1500 {
            ((1500 - elapsed_ms) / 15) as u32
        } else {
            0
        };
        let points_earned = if is_correct { 100 + speed_bonus } else { 0 };
        let eval = WasmArcadeEvaluation {
            is_correct,
            points_earned,
            speed_bonus,
            correct_answer: item.correct_option().to_string(),
            meaning: item.plain_english.clone(),
            rule: item.explanation.clone(),
        };
        serde_json::to_string(&eval).unwrap_or_default()
    } else {
        serde_json::to_string(&WasmArcadeEvaluation {
            is_correct: false,
            points_earned: 0,
            speed_bonus: 0,
            correct_answer: String::new(),
            meaning: String::new(),
            rule: format!("Item '{}' not found", item_id),
        })
        .unwrap_or_default()
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn calculate_sm2_review_wasm(ease_factor: f32, interval: u32, repetitions: u32, grade: u8) -> String {
    let srs_grade = match grade {
        0 => SrsGrade::Blackout,
        1 => SrsGrade::Incorrect,
        2 => SrsGrade::HardIncorrect,
        3 => SrsGrade::HardCorrect,
        4 => SrsGrade::Good,
        _ => SrsGrade::Easy,
    };
    let update = calculate_sm2(ease_factor, interval, repetitions, srs_grade);
    let res = WasmSm2Result {
        ease_factor: update.ease_factor,
        interval: update.interval,
        repetitions: update.repetitions,
    };
    serde_json::to_string(&res).unwrap_or_default()
}
```

- [ ] **Step 4: Run tests and verify**

```bash
cargo test --features wasm --test wasm_tests
```
Expected: PASS with 4 tests passed.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/lib.rs src/wasm.rs tests/wasm_tests.rs
git commit --no-gpg-sign -m "feat(wasm): add WebAssembly interface and evaluator bridge"
```

---

### Task 2: Curriculum & Arcade Web Bundle Generator

**Files:**
- Create: `scripts/build_playground_bundle.py`
- Create: `tests/test_playground_bundle.py`
- Create: `docs/assets/playground/playground-bundle.json`

**Interfaces:**
- Consumes: `src/core/generator.rs`, `src/core/arcade.rs`, `src/core/reference.rs`
- Produces: `docs/assets/playground/playground-bundle.json`

- [ ] **Step 1: Write test for bundle generator in `tests/test_playground_bundle.py`**

```python
import json
from pathlib import Path

def test_playground_bundle_structure_and_completeness():
    bundle_path = Path("docs/assets/playground/playground-bundle.json")
    assert bundle_path.exists(), "Bundle file must exist"
    
    with open(bundle_path, "r", encoding="utf-8") as f:
        data = json.load(f)
    
    assert "version" in data
    assert "topics" in data
    assert "frames" in data
    assert "arcade_items" in data
    assert len(data["topics"]) == 24, "Must contain all 24 topics"
    assert len(data["frames"]) >= 136, "Must contain at least 136 sentence frames"
    assert len(data["arcade_items"]) >= 260, "Must contain all showdown items"
    
    # Verify every frame has plain_english and explanation
    for frame in data["frames"]:
        assert frame.get("plain_english"), f"Frame {frame['id']} missing plain_english"
        assert frame.get("explanation"), f"Frame {frame['id']} missing explanation"
```

- [ ] **Step 2: Implement `scripts/build_playground_bundle.py`**

Create script that extracts catalog data directly into `docs/assets/playground/playground-bundle.json` using Rust CLI or direct extraction.

- [ ] **Step 3: Run pytest on bundle test**

```bash
uv run pytest tests/test_playground_bundle.py
```
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add scripts/build_playground_bundle.py tests/test_playground_bundle.py docs/assets/playground/playground-bundle.json
git commit --no-gpg-sign -m "feat(playground): generate full curriculum and arcade web bundle"
```

---

### Task 3: Client-Side State Persistence Engine (`SpanglingsStorage`)

**Files:**
- Create: `docs/assets/playground/storage.js`
- Create: `tests/test_storage_parity.py`

**Interfaces:**
- Consumes: Browser `localStorage`
- Produces: `SpanglingsStorage` class with `load()`, `save()`, `exportJson()`, `importJson()`, `resetExercise()`, `resetAll()`.

- [ ] **Step 1: Write storage schema test in `tests/test_storage_parity.py`**

Verify export/import schema strictly matches `~/.local/share/spanglings/state.json`.

- [ ] **Step 2: Implement `docs/assets/playground/storage.js`**

Implement `SpanglingsStorage`:
- Debounced (300ms) save to `localStorage.getItem('spanglings_state_v1')`.
- Full SM-2 decay and ease tracking.
- `exportJson()` generating `spanglings-progress-YYYY-MM-DD.json`.
- `importJson(jsonString)` validating and merging state.

- [ ] **Step 3: Run tests and verify**

```bash
uv run pytest tests/test_storage_parity.py
```

- [ ] **Step 4: Commit**

```bash
git add docs/assets/playground/storage.js tests/test_storage_parity.py
git commit --no-gpg-sign -m "feat(playground): implement localStorage state engine with CLI JSON parity"
```

---

### Task 4: Interactive Split-Pane Workspace & Monaco Editor

**Files:**
- Create: `docs/assets/playground/playground.js`
- Create: `docs/assets/playground/playground.css`

**Interfaces:**
- Consumes: `playground-bundle.json`, `storage.js`, Monaco Editor CDN
- Produces: Split-pane UI (Syllabus Sidebar + Exercise Editor + Diagnostics Pane).

- [ ] **Step 1: Implement Split-Pane Layout & Styles in `playground.css`**
- [ ] **Step 2: Implement Syllabus Tree, Monaco Integration, Accent Toolbar, and Evaluation in `playground.js`**
- [ ] **Step 3: Verify interactions in browser**
- [ ] **Step 4: Commit**

```bash
git add docs/assets/playground/playground.js docs/assets/playground/playground.css
git commit --no-gpg-sign -m "feat(playground): add split-pane syllabus explorer and exercise workspace"
```

---

### Task 5: Single-Key Rapid Arcade Arena Engine

**Files:**
- Modify: `docs/assets/playground/playground.js`
- Modify: `docs/assets/playground/playground.css`

**Interfaces:**
- Consumes: `playground-bundle.json`, `storage.js`
- Produces: Rapid Arcade Mode with `[1]`/`[2]` & `[J]`/`[K]` hotkeys, clean score badges, dual-layer feedback, and mistake recap.

- [ ] **Step 1: Implement Arcade Arena state machine in `playground.js`**
- [ ] **Step 2: Render clean prompt card, hotkey listeners, and instant dual-layer text feedback**
- [ ] **Step 3: Implement end-of-round recap table**
- [ ] **Step 4: Commit**

```bash
git add docs/assets/playground/playground.js docs/assets/playground/playground.css
git commit --no-gpg-sign -m "feat(playground): implement single-key rapid arcade arena with dual-layer recap"
```

---

### Task 6: Documentation Page, Fullscreen Mode, and CI Docs Workflow

**Files:**
- Create: `docs/playground.md`
- Modify: `mkdocs.yml`
- Modify: `.github/workflows/docs.yml`

**Interfaces:**
- Consumes: All playground assets
- Produces: Deployed page at `https://dnf0.github.io/spanglings/playground/`

- [ ] **Step 1: Create `docs/playground.md` with full-bleed container and header controls**
- [ ] **Step 2: Update `mkdocs.yml` navigation**
- [ ] **Step 3: Update `.github/workflows/docs.yml` to build bundle and deploy docs**
- [ ] **Step 4: Run `mkdocs build --strict` and verify zero errors**
- [ ] **Step 5: Commit**

```bash
git add docs/playground.md mkdocs.yml .github/workflows/docs.yml
git commit --no-gpg-sign -m "feat(docs): add playground page, fullscreen mode, and automated CI deploy"
```
