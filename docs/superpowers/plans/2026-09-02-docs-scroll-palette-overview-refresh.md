# Docs Scrolling, Modern Palette, and Overview Refresh Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Choose an execution mode:
> 1. `superpowers:subagent-driven-development` (recommended for multi-agent reviews, backed by `SKILL.state` / `.agent-state/state.json`)
> 2. `agent-rules:stateful-execution` (SKILL.state) (recommended for deterministic single-agent linear execution)
> 3. `superpowers:executing-plans` (batch execution with manual checkpoints)
> Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restore full vertical scrolling across all MkDocs pages, modernize the documentation color palette from harsh red/amber to clean indigo/slate, and rewrite `docs/index.md` to cleanly showcase the Language Manual, Dual WebAssembly Playground, and Syllabus without legacy cruft.

**Architecture:**
1. Isolate playground layout CSS by removing `docs/assets/playground/playground.css` from `mkdocs.yml` `extra_css` and removing global `html, body { overflow: hidden; }` from `playground.css`. Add `docs/assets/extra.css` for clean MkDocs styling.
2. Update `mkdocs.yml` theme palette to use `primary: indigo` and `accent: indigo` (and `scheme: slate` for dark mode), matching modern developer doc standards.
3. Overhaul `docs/index.md` to highlight the three core pillars: Spanish Language Manual, Interactive Web Playground (Curriculum + Arcade), and Syllabus.

**Tech Stack:** Python 3.12, MkDocs Material, CSS3, Pytest.

## Global Constraints
- All 65+ pytest tests and 101 cargo tests must pass.
- `mkdocs build --strict` must pass without warnings or broken links.
- No global `overflow: hidden` on documentation pages.
- Palette must use professional, high-contrast Material colors (indigo/slate).

---

### Task 1: Fix Documentation Scrolling & CSS Scoping

**Files:**
- Modify: `docs/assets/playground/playground.css:150-170`
- Modify: `mkdocs.yml:55-60`
- Create: `docs/assets/extra.css`
- Modify: `tests/test_docs_playground.py`

**Interfaces:**
- Consumes: MkDocs Material layout structure
- Produces: Unblocked vertical scrolling on all docs pages

- [ ] **Step 1: Write failing test in `tests/test_docs_playground.py`**
- [ ] **Step 2: Run test to verify it fails**
- [ ] **Step 3: Update `docs/assets/playground/playground.css`, `mkdocs.yml`, and create `docs/assets/extra.css`**
- [ ] **Step 4: Run tests to verify they pass**
- [ ] **Step 5: Commit changes**

---

### Task 2: Modernize Palette & Color Scheme

**Files:**
- Modify: `mkdocs.yml:8-25`
- Modify: `docs/assets/extra.css`
- Modify: `tests/test_docs_playground.py`

**Interfaces:**
- Consumes: MkDocs Material palette system
- Produces: Polished indigo/slate dark and clean light themes

- [ ] **Step 1: Write test for modern palette in `tests/test_docs_playground.py`**
- [ ] **Step 2: Run test to verify it fails**
- [ ] **Step 3: Update `mkdocs.yml` palette configuration**
- [ ] **Step 4: Run test to verify it passes**
- [ ] **Step 5: Commit changes**

---

### Task 3: Overhaul & Modernize Docs Overview (`docs/index.md`)

**Files:**
- Modify: `docs/index.md`
- Modify: `tests/test_docs_playground.py`

**Interfaces:**
- Consumes: Language Manual, Playground, and Syllabus URLs
- Produces: High-impact, streamlined Spanglings landing page

- [ ] **Step 1: Write test for index overview in `tests/test_docs_playground.py`**
- [ ] **Step 2: Run test to verify it fails**
- [ ] **Step 3: Update `docs/index.md`**
- [ ] **Step 4: Run tests to verify they pass**
- [ ] **Step 5: Commit changes**
