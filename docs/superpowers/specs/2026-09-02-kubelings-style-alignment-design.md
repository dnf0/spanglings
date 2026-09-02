# Design Specification: Kubelings Style & Standalone Layout Alignment

**Date:** 2026-09-02  
**Status:** Approved  
**Target Repository:** `dnf0/spanglings`  

---

## 1. Overview & Objective

Align the Spanglings WebAssembly browser learning platform (`https://dnf0.github.io/spanglings/playground/`) with the architecture, visual polish, and layout ergonomics of **Kubelings** (`https://dnf0.github.io/kubelings/playground/`).

### Goals:
1. **Standalone Dedicated App (`docs/playground/index.html`)**: Provide a full-window (`100vw × 100vh`) standalone web application with a 48px top navigation bar (`#standalone-header`), brand badge (`🇪🇸 Spanglings ⚡ Interactive Playground`), top navigation links (`📖 Documentation`, `📚 Syllabus`, `🐙 GitHub`), and `🌓 Theme` toggle.
2. **Theme & CSS System Alignment (`playground.css`)**: Implement Kubelings CSS variable architecture supporting both Dark Slate (`[data-md-color-scheme="slate"]`, `html[data-theme="dark"]`) and Light (`html[data-theme="light"]`) modes with Catppuccin-inspired slate hues.
3. **Component Polish**:
   - Status pill with pulsing dot indicator (`.status-loading`, `.status-ready`, `.status-running`).
   - Accordion topic explorer with complete count badges and gradient progress bar.
   - Rounded action toolbar with keyboard shortcut badges (`[⌘↵]`, `[?]`, `[⛶]`).
   - Spanish accent helper toolbar (`á`, `é`, `í`, `ó`, `ú`, `ñ`, `ü`, `¿`, `¡`).
4. **Preserve Spanglings Strengths**:
   - Retain dual-mode switcher: Mode A (Curriculum Workspace) and Mode B (Rapid Arcade Arena).
   - Retain dual-layer pedagogical feedback (`💡 Meaning / Communicative Context` + `📐 Grammar Rule / Structural Law`).
   - Retain zero audio distractions and zero screen shaking.
   - Retain SM-2 spaced repetition decay tracking and JSON state export/import.

---

## 2. Architecture & File Structure

```
docs/
├── playground/
│   └── index.html               <-- Dedicated standalone edge-to-edge HTML application
├── assets/
│   └── playground/
│       ├── playground.css       <-- CSS variable theme system matching Kubelings
│       ├── playground.js        <-- UI controller & Rapid Arcade engine
│       ├── storage.js           <-- SM-2 & concept mastery persistence engine
│       └── playground-bundle.json <-- 24 topics, 136 frames, 262 arcade items
mkdocs.yml                       <-- Navigation updated to playground/index.html
tests/
└── test_docs_playground.py      <-- Strict tests validating standalone page, nav, and build
```

---

## 3. Detailed Component Specifications

### 3.1 Standalone HTML Page (`docs/playground/index.html`)
- **Container**: Full-bleed `width: 100vw; height: 100vh; overflow: hidden;`
- **Header (`#standalone-header`)**:
  - Left: Brand icon `🇪🇸`, Title `Spanglings`, `<span class="brand-badge">⚡ Interactive Playground</span>`.
  - Right: `📖 Documentation` (link to `../`), `📚 Syllabus` (link to `../syllabus/` or reference), `GitHub` (link to `https://github.com/dnf0/spanglings`), and `🌓 Theme` button.
- **Root Element**: `<main id="standalone-playground-root"><div id="spanglings-app" class="spanglings-playground"></div></main>`
- **Theme Script**: Reads `localStorage.getItem("spanglings-theme")`, toggles `data-theme="dark|light"`, and syncs Monaco editor theme (`vs-dark` vs `vs`).

### 3.2 CSS Theme Palette (`playground.css`)
- Variable structure:
  - `--pg-bg`: `#ffffff` (light) / `#1e1e2e` (dark slate)
  - `--pg-card-bg`: `#ffffff` (light) / `#181825` (dark slate)
  - `--pg-sidebar-bg`: `#f8fafc` (light) / `#11111b` (dark slate)
  - `--pg-header-bg`: `#f8fafc` (light) / `#11111b` (dark slate)
  - `--pg-border`: `#e2e8f0` (light) / `#313244` (dark slate)
  - `--pg-border-focus`: `#0284c7` (light) / `#89b4fa` (dark slate)
  - `--pg-accent`: `#0284c7` (light) / `#89b4fa` (dark slate)
  - `--pg-term-bg`: `#0b0f19` (light) / `#0f0f17` (dark slate)
  - `--pg-term-header-bg`: `#111827` (light) / `#181825` (dark slate)
  - `--pg-term-text`: `#f1f5f9` (light) / `#cdd6f4` (dark slate)
- Split layout: `320px 1fr` grid with responsive breakpoints for `< 1024px` (280px sidebar) and `< 768px` (stacked).

### 3.3 Mode Switcher & Arcade Arena Integration
- Top toolbar mode switcher allowing immediate toggle between **Curriculum Workspace** and **Rapid Arcade Arena**.
- Seamless integration with existing `SpanglingsStorage` and `SpanglingsArcadeEngine`.

---

## 4. Verification & Quality Gates

- `uv run pytest tests/test_docs_playground.py` and `uv run pytest` pass 100%.
- `uv run mkdocs build --strict` runs with 0 warnings/errors.
- `cargo test --all-targets` passes 100%.
- `uv run ruff check scripts/ tests/` and `uv run pyright tests/ scripts/` pass with 0 errors.
