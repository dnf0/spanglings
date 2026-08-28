# Informative & Non-Spoiling Exercise TODOs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Review and enrich all 339 exercises across all 60 tracks with explicit, informative `### Instructions` blocks (`**TODO**:` and `**Why**:`) and in-editor comments without leaking/spoiling the solution words.

**Architecture:** Standardized exercise markdown format incorporating an actionable task description and grammatical/pragmatic rationale. In-TUI prompt card rendering extracts and highlights the instruction block. An automated test suite enforces 100% presence of non-empty TODOs/Whys and zero solution leaks.

**Tech Stack:** Rust 2021, Ratatui, Markdown, Regex, cargo test.

---

## Tasks

- [ ] **Task 1: Batch 1 Enrichment (Tracks 00–14, 80 exercises)**
  - **Files**: All 80 exercise files across `exercises/00_baseline/` to `exercises/14_connectors/`
  - **Actions**:
    - Add `### Instructions` with `**TODO**:` (actionable instruction) and `**Why**:` (linguistic rationale).
    - Add inline `<!-- TODO: ... -->` above the exercise prompt line.
    - Guarantee zero solution word leaks in the instructions.
  - **Verification**: Verify parsing with `cargo test --test exercise_validity_tests`.

- [ ] **Task 2: Batch 2 Enrichment (Tracks 15–26, 61 exercises)**
  - **Files**: All 61 exercise files across `exercises/15_advanced_imperative/` to `exercises/26_regional_contrasts/`
  - **Actions**:
    - Add `### Instructions` with `**TODO**:` and `**Why**:`.
    - Add inline `<!-- TODO: ... -->` above prompt.
    - Guarantee zero solution word leaks.
  - **Verification**: `cargo test --test exercise_validity_tests`.

- [ ] **Task 3: Batch 3 Enrichment (Tracks 27–41, 90 exercises)**
  - **Files**: All 90 exercise files across `exercises/27_system_design/` to `exercises/41_adverbial_clauses_and_conjunctions/`
  - **Actions**:
    - Add `### Instructions` with `**TODO**:` and `**Why**:`.
    - Add inline `<!-- TODO: ... -->` above prompt.
    - Guarantee zero solution word leaks.
  - **Verification**: `cargo test --test exercise_validity_tests`.

- [ ] **Task 4: Batch 4 Enrichment (Tracks 42–59, 108 exercises)**
  - **Files**: All 108 exercise files across `exercises/42_travel_logistics_and_borders/` to `exercises/59_scalar_concession_and_intensive_connectors/`
  - **Actions**:
    - Add `### Instructions` with `**TODO**:` and `**Why**:`.
    - Add inline `<!-- TODO: ... -->` above prompt.
    - Guarantee zero solution word leaks.
  - **Verification**: `cargo test --test exercise_validity_tests`.

- [ ] **Task 5: TUI Prompt Card Enhancement & Automated Leakage Verification Test**
  - **Files**: `src/tui/ui.rs`, `tests/exercise_todo_tests.rs`
  - **Actions**:
    - In `src/tui/ui.rs`, parse `### Instructions` and render `Task (TODO):` and `Why:` in the prompt card with dedicated color styling.
    - Create `tests/exercise_todo_tests.rs` asserting:
      1. Every one of the 339 exercises contains `### Instructions`, `**TODO**:`, and `**Why**:`.
      2. No instruction text is empty or shorter than 20 characters.
      3. No instruction text or inline `<!-- TODO: -->` comment contains the exact solution string from `<!-- SOLUTION -->`.
  - **Verification**: Run `cargo test --test exercise_todo_tests`.

- [ ] **Task 6: Full Verification, Documentation & Knowledge Graph Update**
  - **Actions**:
    - Run `cargo fmt --check`.
    - Run `cargo clippy --all-targets -- -D warnings`.
    - Run `cargo test`.
    - Update knowledge graph: `uvx --from graphifyy graphify update .`.
    - Run `roborev status` and `roborev show HEAD`.
