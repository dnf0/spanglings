# Spanglings Language Completeness Expansion (Tracks 48–53) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expand Spanglings from 48 tracks (267 exercises) to **54 tracks (303 exercises)** and expand the linguistic knowledge graph from 53 to **65 concepts**, closing all 8 identified linguistic gaps (Epistemic Future/Conditional, Clitic Doubling, Personal A, Gerund Restrictions, Adversative Pero/Sino/Sino Que, and Independent/Legal Subjunctives).

**Architecture:** Extend `LinguisticGraph` in `src/core/graph.rs` with 12 new nodes and prerequisite edges. Add 6 new reference topics to `src/core/reference.rs`. Author 36 new exercises under `exercises/48_...` through `exercises/53_...` with 3-tier hints, ontology tags, and context notes. Add targeted diagnostic rules in `src/engine/validator.rs`. Update MkDocs documentation and syllabus.

**Tech Stack:** Rust (1.75+), Ratatui, MkDocs Material, PyMdown Extensions.

---

### Task 1: Linguistic Knowledge Graph (53 -> 65 Concepts) & Reference Cards
**Files:**
- Modify: `src/core/graph.rs`
- Modify: `src/core/reference.rs`
- Modify: `tests/graph_tests.rs`
- Modify: `tests/reference_tests.rs`

- [ ] **Step 1: Add 12 new ConceptNodes to `src/core/graph.rs` with prerequisites and category mappings**
- [ ] **Step 2: Add 6 new Reference Cards in `src/core/reference.rs` (`epistemic-conjecture`, `clitic-doubling`, `personal-a`, `gerund-rules`, `adversatives`, `legal-subjunctive`)**
- [ ] **Step 3: Update unit tests in `tests/graph_tests.rs` and `tests/reference_tests.rs` to verify DAG validity and reference lookup**
- [ ] **Step 4: Commit Task 1 atomically**

---

### Task 2: Author Curriculum Tracks 48, 49, and 50 (18 Exercises)
**Files:**
- Create: `exercises/48_epistemic_conjecture_and_probability/` (01–06)
- Create: `exercises/49_clitic_doubling_and_left_dislocation/` (01–06)
- Create: `exercises/50_personal_a_and_animacy_shifts/` (01–06)

- [ ] **Step 1: Author Track 48 (6 exercises on Present/Past Conjecture, Compound Conditionals, Epistemic Modality)**
- [ ] **Step 2: Author Track 49 (6 exercises on Mandatory Left-Dislocation, Dative Doubling, Tonic Pronoun Reduplication)**
- [ ] **Step 3: Author Track 50 (6 exercises on Specific vs Non-specific Humans, Personification, Verb Shifts with Personal A)**
- [ ] **Step 4: Verify parsing and syntax in test suite**
- [ ] **Step 5: Commit Task 2 atomically**

---

### Task 3: Author Curriculum Tracks 51, 52, and 53 (18 Exercises)
**Files:**
- Create: `exercises/51_gerund_restrictions_and_anglicisms/` (01–06)
- Create: `exercises/52_adversative_pero_sino_sino_que/` (01–06)
- Create: `exercises/53_independent_subjunctives_and_legal_tenses/` (01–06)

- [ ] **Step 1: Author Track 51 (6 exercises on Gerund of Posteriority Elimination, Adjectival Gerunds, Simultaneous Aspect)**
- [ ] **Step 2: Author Track 52 (6 exercises on Pero vs Sino vs Sino Que, Adversative Subordination, Exclusive Contrast)**
- [ ] **Step 3: Author Track 53 (6 exercises on Optative Formulas, ¡Quién pudiera!, Legal Future Subjunctive, Literary Past -ra)**
- [ ] **Step 4: Verify parsing and validation of all 303 exercises in test suite**
- [ ] **Step 5: Commit Task 3 atomically**

---

### Task 4: Diagnostic Rules & Targeted Feedback for New Traps
**Files:**
- Modify: `src/engine/validator.rs`
- Modify: `src/engine/diagnostics.rs`
- Modify: `tests/diagnostic_rule_tests.rs`

- [ ] **Step 1: Add diagnostic rules for *pero* vs *sino/sino que*, missing personal *a*, ungrammatical gerund of posteriority, and missing clitic doubling**
- [ ] **Step 2: Add test cases in `tests/diagnostic_rule_tests.rs`**
- [ ] **Step 3: Commit Task 4 atomically**

---

### Task 5: Documentation, Syllabus, Backlog & MkDocs Build
**Files:**
- Modify: `docs/syllabus.md`
- Modify: `docs/grammar-reference.md`
- Modify: `docs/BACKLOG.md`
- Modify: `README.md`
- Verify: `mkdocs build --strict`

- [ ] **Step 1: Update `docs/syllabus.md` with Tracks 48–53 and 65-concept ontology**
- [ ] **Step 2: Update `docs/grammar-reference.md` with new cheat sheets**
- [ ] **Step 3: Update `docs/BACKLOG.md` adding Focus Area 16 (Language Completeness Expansion)**
- [ ] **Step 4: Update `README.md` metrics (54 tracks, 303 exercises, 65 concepts)**
- [ ] **Step 5: Run `uvx --with mkdocs-material mkdocs build --strict` to verify docs build**
- [ ] **Step 6: Commit Task 5 atomically**

---

### Task 6: Final Verification, Roborev & Knowledge Graph Update
**Files:**
- Verify: `cargo test`
- Verify: `cargo clippy --all-targets -- -D warnings`
- Verify: `cargo fmt --check`
- Update: `graphify update .`

- [ ] **Step 1: Run complete test suite across all 27+ test files**
- [ ] **Step 2: Run linter and formatter checks**
- [ ] **Step 3: Run `uvx --from graphifyy graphify update .` to update knowledge graph**
- [ ] **Step 4: Push branch, update PR, and present final report**
