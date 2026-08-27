# Spanglings MkDocs Material Documentation Site Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a complete, production-grade MkDocs Material documentation website for Spanglings matching the developer experience and visual standard of Kubelings and Raylings, complete with automated GitHub Pages CI/CD.

**Architecture:** Configure `mkdocs.yml` with `material` theme, slate/light palette toggle, search, code copy, and rich markdown extensions. Author modular docs pages for overview, quickstart, onboarding guide, 48-track syllabus & 53-concept ontology, CLI/TUI reference, in-terminal grammar cheat sheets, and contribution guide. Add GitHub Actions `docs.yml` workflow.

**Tech Stack:** MkDocs, MkDocs Material (`mkdocs-material`), PyMdown Extensions, GitHub Actions (`actions/setup-python`, `astral-sh/setup-uv`).

---

### Task 1: MkDocs Configuration & GitHub Pages Workflow
**Files:**
- Create: `mkdocs.yml`
- Create: `.github/workflows/docs.yml`

- [ ] **Step 1: Create `mkdocs.yml` with Material theme, palette toggle, markdown extensions, navigation, and `not_in_nav` exclusion for internal ADRs/backlog**
- [ ] **Step 2: Create `.github/workflows/docs.yml` for automated GitHub Pages deployment on push to `main`**
- [ ] **Step 3: Commit initial configuration**

---

### Task 2: Core Documentation Pages (Overview, Getting Started, Onboarding Guide)
**Files:**
- Create: `docs/index.md`
- Create: `docs/getting-started.md`
- Create: `docs/onboarding-guide.md`

- [ ] **Step 1: Author `docs/index.md` (Overview, features, quick example, LSP editor integration, *lings ecosystem)**
- [ ] **Step 2: Author `docs/getting-started.md` (Zero-install quickstart, cargo install, build from source, basic workflow, keybindings)**
- [ ] **Step 3: Author `docs/onboarding-guide.md` (Comprehensive illustrated learner's guide: 6 tour stations, cloze blanks, accent handling, rustc-style diagnostics, progressive hints, power modals)**
- [ ] **Step 4: Commit core documentation pages**

---

### Task 3: Curriculum Syllabus, 53-Concept Ontology & Grammar Reference
**Files:**
- Create: `docs/syllabus.md`
- Create: `docs/grammar-reference.md`

- [ ] **Step 1: Author `docs/syllabus.md` (48 tracks, 267 exercises, CEFR breakdown, and full 53-concept linguistic knowledge graph ontology)**
- [ ] **Step 2: Author `docs/grammar-reference.md` (All 12 grammar cheat sheets: ser/estar, past aspects, subjunctive triggers, por/para, clitic pronouns, accidental se, accents, tech, business, false friends, voseo, periphrases)**
- [ ] **Step 3: Commit syllabus and grammar reference**

---

### Task 4: CLI Reference, Contributing Guides & Root Contributing File
**Files:**
- Create: `docs/cli-reference.md`
- Create: `docs/contributing.md`
- Create: `CONTRIBUTING.md`

- [ ] **Step 1: Author `docs/cli-reference.md` (Every CLI subcommand, options, JSON formats, exit codes, and full TUI keybinding cheat sheet)**
- [ ] **Step 2: Author `docs/contributing.md` & `CONTRIBUTING.md` (Development workflow, testing, authoring tracks/exercises, linting, architecture)**
- [ ] **Step 3: Commit CLI reference and contributing guidelines**

---

### Task 5: Build Verification, Link Integrity & Graphify Update
**Files:**
- Verify: `mkdocs build --strict`
- Verify: `cargo test`

- [ ] **Step 1: Run `uvx --with mkdocs-material mkdocs build --strict` to verify zero broken links, invalid paths, or syntax errors**
- [ ] **Step 2: Run all Rust verification checks (`cargo test`, `cargo clippy`, `cargo fmt`)**
- [ ] **Step 3: Run `uvx --from graphifyy graphify update .` to update knowledge graph**
- [ ] **Step 4: Commit and present final completion summary**
