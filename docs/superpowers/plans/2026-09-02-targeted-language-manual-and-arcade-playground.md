# Targeted Spanish Language Manual, Bidirectional Deep-Linking & Rapid Arcade Fix Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Choose an execution mode:
> 1. `superpowers:subagent-driven-development` (recommended for multi-agent reviews, backed by `SKILL.state` / `.agent-state/state.json`)
> 2. `agent-rules:stateful-execution` (SKILL.state) (recommended for deterministic single-agent linear execution)
> 3. `superpowers:executing-plans` (batch execution with manual checkpoints)
> Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform Spanglings documentation from a software manual into a developer-grade Spanish Language Learning Manual across all 24 curriculum topics with contrast tables and decision matrices, fix the Rapid Arcade Arena browser execution, and enable seamless bidirectional deep-linking between documentation and the interactive playground.

**Architecture:** 
1. Fix CSS selector scoping (`.spanglings-playground.arcade-mode`) so `#arcade-arena-container` renders cleanly and hides editor panes upon mode switch.
2. Add URL query parameter deep-linking (`?mode=arcade`, `?topic=<slug>`, `?exercise=<id>`) and manual cross-links in `playground.js` and `playground/index.html`.
3. Author the complete, 24-topic `docs/manual.md` featuring communicative mental models, grammar rule decision tables, common traps, and playground action CTAs.
4. Restructure `docs/index.md`, `docs/syllabus.md`, and `mkdocs.yml` to streamline the user experience around the Language Manual and Interactive Playground.

**Tech Stack:** Python 3.12, pytest, MkDocs Material, Vanilla JS ES Modules, Rust WebAssembly (`wasm-pack`), CSS Custom Properties.

## Global Constraints
- Every topic section in the manual must provide both an intuitive communicative mental model (`💡 Communicative Mental Model`) and a structural rule matrix (`📐 Grammar Rules & Decision Matrix`).
- All manual topics must include direct deep-links to both the Curriculum Syntax Studio (`../playground/?topic=<slug>`) and Rapid Showdown Duel (`../playground/?mode=arcade&topic=<slug>`).
- Playground must provide bidirectional navigation to `../manual/#<topic-slug>`.
- `uv run mkdocs build --strict` must succeed with zero warnings and zero broken links.
- All tests must pass (`uv run pytest`, `cargo test --all-targets`, `uv run ruff check`, `uv run pyright`).

---

### Task 1: Fix Rapid Arcade Arena CSS Selector & View Activation

**Files:**
- Modify: `docs/assets/playground/playground.css:1220-1240`
- Test: `tests/test_arcade_ui.py`

**Interfaces:**
- Consumes: `#spanglings-app.spanglings-playground` DOM element.
- Produces: CSS rules for `.spanglings-playground.arcade-mode` hiding `.syllabus-pane`, `.editor-pane`, and `.diagnostics-pane`, and flexing `.arcade-arena-container`.

- [ ] **Step 1: Write the failing unit test for arcade CSS selector scoping**

In `tests/test_arcade_ui.py`:
```python
def test_arcade_css_selector_targets_spanglings_playground(repo_root: Path) -> None:
    """Verify playground.css targets .spanglings-playground.arcade-mode or #spanglings-app."""
    css_path = repo_root / "docs" / "assets" / "playground" / "playground.css"
    assert css_path.exists()
    content = css_path.read_text(encoding="utf-8")

    assert ".spanglings-playground.arcade-mode" in content or "#spanglings-app.arcade-mode" in content, (
        "playground.css must include .spanglings-playground.arcade-mode selector"
    )
```

- [ ] **Step 2: Run test to verify it fails**

Run: `uv run pytest tests/test_arcade_ui.py::test_arcade_css_selector_targets_spanglings_playground -v`
Expected: FAIL (assertion error: `.spanglings-playground.arcade-mode` not found).

- [ ] **Step 3: Update CSS rules in `docs/assets/playground/playground.css`**

In `docs/assets/playground/playground.css`:
```css
.spanglings-playground.arcade-mode .syllabus-pane,
.spanglings-playground.arcade-mode .editor-pane,
.spanglings-playground.arcade-mode .diagnostics-pane,
#spanglings-app.arcade-mode .syllabus-pane,
#spanglings-app.arcade-mode .editor-pane,
#spanglings-app.arcade-mode .diagnostics-pane {
  display: none !important;
}

.spanglings-playground.arcade-mode .arcade-arena-container,
#spanglings-app.arcade-mode .arcade-arena-container {
  display: flex !important;
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `uv run pytest tests/test_arcade_ui.py -v`
Expected: PASS (all tests in test_arcade_ui.py pass).

- [ ] **Step 5: Commit changes**

```bash
git add docs/assets/playground/playground.css tests/test_arcade_ui.py
git commit --no-gpg-sign -m "fix(playground): fix active arcade mode css selector scoping"
```

---

### Task 2: Playground URL Query Parameter Deep-Linking & Manual Cross-Linking

**Files:**
- Modify: `docs/assets/playground/playground.js`
- Modify: `docs/playground/index.html`
- Test: `tests/test_playground_ui.py`

**Interfaces:**
- Consumes: `window.location.search` (`?mode=arcade`, `?topic=<slug>`, `?exercise=<id>`).
- Produces: `_applyUrlQueryParams()` method in `SpanglingsPlaygroundApp` and `📖 View in Manual` action links pointing to `../manual/#<topic-slug>`.

- [ ] **Step 1: Write the failing unit tests for query param parsing and manual links**

In `tests/test_playground_ui.py`:
```python
def test_playground_js_handles_url_query_params(repo_root: Path) -> None:
    """Verify playground.js parses ?mode=arcade, ?topic=, and ?exercise= query params."""
    js_path = repo_root / "docs" / "assets" / "playground" / "playground.js"
    content = js_path.read_text(encoding="utf-8")

    assert "URLSearchParams" in content
    assert "applyUrlParams" in content or "parseUrlParams" in content
    assert "manual/#" in content or "manual" in content


def test_standalone_header_links_to_manual(repo_root: Path) -> None:
    """Verify docs/playground/index.html header links to ../manual/."""
    html_path = repo_root / "docs" / "playground" / "index.html"
    content = html_path.read_text(encoding="utf-8")

    assert 'href="../manual/"' in content
```

- [ ] **Step 2: Run tests to verify failure**

Run: `uv run pytest tests/test_playground_ui.py -k "url_query_params or links_to_manual" -v`
Expected: FAIL.

- [ ] **Step 3: Implement query parameter parsing and manual linking in `playground.js` and `index.html`**

In `docs/assets/playground/playground.js`:
- Add `applyUrlParams()` method to check `new URLSearchParams(window.location.search)`.
- If `mode === 'arcade'`, activate arcade mode and set `arcadeEngine.mode = topicParam || 'all'`.
- If `topicParam` present in curriculum mode, find and select matching exercise.
- In topic drawer & diagnostics card, render `[📖 View Topic in Manual](../manual/#${topicSlug})`.

In `docs/playground/index.html`:
- Update header link from `📖 Documentation` to `📘 Language Manual` pointing to `../manual/`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `uv run pytest tests/test_playground_ui.py -v`
Expected: PASS.

- [ ] **Step 5: Commit changes**

```bash
git add docs/assets/playground/playground.js docs/playground/index.html tests/test_playground_ui.py
git commit --no-gpg-sign -m "feat(playground): add query param deep-linking and bidirectional manual links"
```

---

### Task 3: Comprehensive Spanish Language Manual (`docs/manual.md`)

**Files:**
- Create: `docs/manual.md`
- Test: `tests/test_docs_playground.py`

**Interfaces:**
- Consumes: 24 linguistic topics, mental models, decision matrices, contrast rules, and playground practice URLs.
- Produces: High-quality, exhaustive Spanish grammar manual with 24 topic chapters across CEFR Tiers 1, 2, and 3.

- [ ] **Step 1: Write failing test verifying `docs/manual.md` completeness**

In `tests/test_docs_playground.py`:
```python
def test_comprehensive_language_manual_structure(repo_root: Path) -> None:
    """Verify docs/manual.md exists and contains all 24 topics with mental models, rules, and deep links."""
    manual_path = repo_root / "docs" / "manual.md"
    assert manual_path.exists(), f"docs/manual.md must exist at {manual_path}"

    content = manual_path.read_text(encoding="utf-8")
    assert "# Spanglings Spanish Language Manual" in content

    # Check 24 topics
    expected_topics = [
        "ser-estar", "por-para", "past-tenses", "pronouns", "gustar", "reflexive",
        "stem-changing", "prepositions", "subjunctive", "imperfect-subjunctive",
        "imperative", "accidental-se", "passive-impersonal-se", "possessive-datives",
        "relative-pronouns", "gerund-rules", "verbs-of-becoming", "scalar-concession",
        "epistemic-conjecture", "adversatives", "false-friends", "voseo", "tech", "legal"
    ]
    for topic in expected_topics:
        assert f"id=\"{topic}\"" in content or f"#{topic}" in content or topic in content

    assert "💡 Communicative Mental Model" in content
    assert "📐 Grammar Rules & Decision Matrix" in content
    assert "playground/?topic=" in content
    assert "playground/?mode=arcade" in content
```

- [ ] **Step 2: Run test to verify failure**

Run: `uv run pytest tests/test_docs_playground.py::test_comprehensive_language_manual_structure -v`
Expected: FAIL (`docs/manual.md` not found).

- [ ] **Step 3: Author `docs/manual.md`**

Author `docs/manual.md` with:
- CEFR Overview & Cognitive Roadmap.
- All 24 comprehensive chapters with dual-layer explanations (Mental Model + Grammar Rule Decision Table), high-frequency pitfall warnings, and bidirectional practice CTAs (`../playground/?topic=<slug>` and `../playground/?mode=arcade&topic=<slug>`).

- [ ] **Step 4: Run test to verify it passes**

Run: `uv run pytest tests/test_docs_playground.py::test_comprehensive_language_manual_structure -v`
Expected: PASS.

- [ ] **Step 5: Commit changes**

```bash
git add docs/manual.md tests/test_docs_playground.py
git commit --no-gpg-sign -m "docs(manual): add comprehensive 24-topic spanish language manual"
```

---

### Task 4: Streamline Documentation Hub, Syllabus & MkDocs Navigation

**Files:**
- Modify: `docs/index.md`
- Modify: `docs/syllabus.md`
- Modify: `mkdocs.yml`
- Test: `tests/test_docs_playground.py`

**Interfaces:**
- Consumes: `docs/manual.md`, `docs/syllabus.md`, `docs/playground/index.html`.
- Produces: Clean, focused documentation hub in `docs/index.md` and streamlined `mkdocs.yml` navigation.

- [ ] **Step 1: Write failing test verifying streamlined navigation and syllabus links**

In `tests/test_docs_playground.py`:
```python
def test_mkdocs_navigation_includes_manual_and_syllabus(mkdocs_yml_path: Path) -> None:
    """Verify mkdocs.yml navigation is streamlined around Manual, Syllabus, and Playground."""
    content = mkdocs_yml_path.read_text(encoding="utf-8")
    config = yaml.safe_load(content)

    nav = config.get("nav", [])
    nav_targets = []
    for item in nav:
        if isinstance(item, dict):
            nav_targets.extend(item.values())
        elif isinstance(item, str):
            nav_targets.append(item)

    assert "index.md" in nav_targets
    assert "manual.md" in nav_targets
    assert "syllabus.md" in nav_targets
    assert "playground/index.html" in nav_targets
```

- [ ] **Step 2: Run test to verify failure**

Run: `uv run pytest tests/test_docs_playground.py::test_mkdocs_navigation_includes_manual_and_syllabus -v`
Expected: FAIL (`manual.md` not in nav).

- [ ] **Step 3: Update `docs/index.md`, `docs/syllabus.md`, and `mkdocs.yml`**

- Overhaul `docs/index.md` to introduce the Syntax Compiler Model, Language Architecture, and prominent links to the Language Manual and Interactive Playground.
- Update `docs/syllabus.md` to link all 24 topics to `manual.md#<topic>` and `playground/?topic=<slug>`.
- Update `mkdocs.yml` nav:
  ```yaml
  nav:
    - Overview: index.md
    - Spanish Language Manual: manual.md
    - Curriculum Syllabus: syllabus.md
    - Interactive Playground: playground/index.html
    - Contributing: contributing.md
  ```
- Add `cli-reference.md`, `onboarding-guide.md`, `grammar-reference.md` to `not_in_nav` or clean them up.

- [ ] **Step 4: Run tests and strict documentation build**

Run: `uv run pytest tests/test_docs_playground.py -v && uv run mkdocs build --strict`
Expected: PASS (strict build succeeds in < 2 seconds).

- [ ] **Step 5: Commit changes**

```bash
git add docs/index.md docs/syllabus.md mkdocs.yml tests/test_docs_playground.py
git commit --no-gpg-sign -m "docs(site): streamline documentation hub, syllabus, and navigation"
```

---

### Task 5: Full Verification Suite, Graphify Update & Production Readiness

**Files:**
- Test: All repository tests and linters

**Interfaces:**
- Consumes: Entire codebase, docs site, and Wasm runtime.
- Produces: Passing verification evidence across Rust, Python, and MkDocs.

- [ ] **Step 1: Run complete test and lint suite**

```bash
cargo test --all-targets
uv run pytest
uv run ruff check scripts/ tests/
uv run pyright scripts/ tests/
uv run mkdocs build --strict
```

- [ ] **Step 2: Rebuild Graphify Knowledge Graph**

```bash
uvx --from graphifyy graphify update .
```

- [ ] **Step 3: Commit final updates**

```bash
git add graphify-out/
git commit --no-gpg-sign -m "chore(graph): update knowledge graph for targeted manual and arcade playground"
```

---
