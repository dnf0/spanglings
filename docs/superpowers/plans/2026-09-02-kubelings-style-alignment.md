# Kubelings Style & Standalone Layout Alignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Choose an execution mode:
> 1. `superpowers:subagent-driven-development` (recommended for multi-agent reviews, backed by `SKILL.state` / `.agent-state/state.json`)
> 2. `agent-rules:stateful-execution` (SKILL.state) (recommended for deterministic single-agent linear execution)
> 3. `superpowers:executing-plans` (batch execution with manual checkpoints)
> Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform the Spanglings WebAssembly learning platform into a dedicated edge-to-edge standalone web application matching the exact visual style, header navigation, Catppuccin / Slate theme palette, and layout ergonomics of Kubelings (`https://dnf0.github.io/kubelings/playground/`).

**Architecture:** Create `docs/playground/index.html` with a 48px `#standalone-header` containing branding, documentation/syllabus links, GitHub link, and theme toggling. Overhaul `docs/assets/playground/playground.css` with Kubelings CSS custom properties for dark slate and light modes, pulsing status pill indicators, gradient progress bars, and rounded toolbar controls. Update `mkdocs.yml` navigation and automated verification tests.

**Tech Stack:** HTML5, CSS3 Custom Properties (Catppuccin Slate & Light), Vanilla ES6 JavaScript, WebAssembly (`wasm-bindgen`), Monaco Editor, MkDocs Material.

## Global Constraints
- Preserve full dual-mode operation: Mode A (Curriculum Workspace) and Mode B (Rapid Arcade Arena).
- Preserve dual-layer explanations (`💡 Meaning / Communicative Context` + `📐 Grammar Rule / Structural Law`).
- Maintain zero sound synthesis and zero screen-shake/flashing animations.
- Maintain SM-2 spaced repetition decay curve and JSON state portability (`~/.local/share/spanglings/state.json`).
- Ensure `mkdocs build --strict` completes with 0 warnings/errors.
- All tests in `tests/test_docs_playground.py` and `uv run pytest` must pass 100%.

---

### Task 1: Standalone HTML Page & MkDocs Navigation Alignment

**Files:**
- Create: `docs/playground/index.html`
- Modify: `mkdocs.yml`
- Modify: `tests/test_docs_playground.py`

**Interfaces:**
- Produces: `docs/playground/index.html` standalone application entrypoint mounted at `#spanglings-app`.
- Navigation: `mkdocs.yml` `nav` entry `- Interactive Playground: playground/index.html`.

- [ ] **Step 1: Write failing test in `tests/test_docs_playground.py` for `docs/playground/index.html`**
- [ ] **Step 2: Run test to verify it fails**
- [ ] **Step 3: Create `docs/playground/index.html` with `#standalone-header`, theme toggle, and `#spanglings-app` container**
- [ ] **Step 4: Update `mkdocs.yml` navigation and `not_in_nav`**
- [ ] **Step 5: Run tests and verify they pass**
- [ ] **Step 6: Commit changes**

---

### Task 2: CSS Theme Engine & Slate/Light Palette Overhaul

**Files:**
- Modify: `docs/assets/playground/playground.css`
- Modify: `tests/test_playground_ui.py`
- Modify: `tests/test_arcade_ui.py`

**Interfaces:**
- Produces: CSS variable architecture (`--pg-bg`, `--pg-card-bg`, `--pg-sidebar-bg`, `--pg-header-bg`, `--pg-border`, `--pg-accent`, `--pg-term-*`) supporting `[data-md-color-scheme="slate"]`, `html[data-theme="dark"]`, and `html[data-theme="light"]`.
- Styling: Standalone edge-to-edge layout, pulsing status dot (`.status-loading`, `.status-ready`, `.status-running`), gradient progress bar, complete badge counters, rounded action toolbar buttons.

- [ ] **Step 1: Write tests for Kubelings CSS variable tokens and layout classes**
- [ ] **Step 2: Run tests to verify failure**
- [ ] **Step 3: Update `docs/assets/playground/playground.css` with Kubelings theme variables, standalone layout rules, and component styles**
- [ ] **Step 4: Run tests to verify pass**
- [ ] **Step 5: Commit changes**

---

### Task 3: UI Controller Theme Switching & Fullscreen Layout

**Files:**
- Modify: `docs/assets/playground/playground.js`
- Modify: `tests/test_playground_ui.py`

**Interfaces:**
- Produces: Seamless Monaco editor theme synchronization (`vs-dark` vs `vs`), standalone container sizing, status dot lifecycle updates.

- [ ] **Step 1: Write test for theme switching Monaco sync and status dot management**
- [ ] **Step 2: Run tests to verify failure**
- [ ] **Step 3: Update `docs/assets/playground/playground.js` with theme sync helper and status pill states**
- [ ] **Step 4: Run tests to verify pass**
- [ ] **Step 5: Commit changes**

---

### Task 4: Automated Verification & Strict Documentation Build Check

**Files:**
- Modify: `tests/test_docs_playground.py`
- Modify: `.github/workflows/docs.yml`

**Interfaces:**
- Verifies: Full test suite (`cargo test --all-targets`, `uv run pytest`, `uv run mkdocs build --strict`, `uv run ruff check`).

- [ ] **Step 1: Run comprehensive verification suites**
- [ ] **Step 2: Verify `mkdocs build --strict` output**
- [ ] **Step 3: Commit and finalize task**
