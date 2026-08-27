# Linguistic Knowledge Graph & Practical Everyday Curriculum Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a Directed Acyclic Graph (DAG) Linguistic Knowledge Graph, concept-linked compiler diagnostics, concept mastery state tracking, and expand the curriculum from 231 to 267 handcrafted exercises (Tracks 42–47) with retroactive conceptual tagging for all existing exercises (Tracks 00–41).

**Architecture:** A static zero-overhead in-memory linguistic ontology DAG in `src/core/graph.rs` models relationships between grammar primitives and situational synthesis tracks. `Exercise` structs parse concept tags and prerequisite links. Compiler diagnostics and the SRS weakness profiler traverse the graph backwards to identify root grammatical causes.

**Tech Stack:** Rust (edition 2021), `serde`, `serde_json`, `ratatui`, `chrono`, `clap`.

---

### Task 1: Core Linguistic Graph Engine & Ontology

**Files:**
- Create: `src/core/graph.rs`
- Modify: `src/core/mod.rs`
- Test: `tests/graph_tests.rs`

- [ ] **Step 1: Write failing graph tests in `tests/graph_tests.rs`**

```rust
use spanglings::core::curriculum::Level;
use spanglings::core::graph::{get_default_linguistic_graph, ConceptCategory, ConceptId, LinguisticGraph};

#[test]
fn test_default_graph_ontology_is_valid_dag() {
    let graph = get_default_linguistic_graph();
    assert!(graph.nodes.len() >= 40, "Expected at least 40 concepts in ontology, found {}", graph.nodes.len());
    
    // Validate that graph has no cycles (DAG property)
    assert!(graph.validate_no_cycles().is_ok(), "Graph contains cycles!");
}

#[test]
fn test_graph_prerequisite_traversal() {
    let graph = get_default_linguistic_graph();
    let target = ConceptId("subjunctive_temporal_future".to_string());
    let prereqs = graph.get_all_ancestor_prerequisites(&target);
    assert!(!prereqs.is_empty());
    assert!(prereqs.iter().any(|id| id.0 == "subjunctive_volition_influence" || id.0 == "irregular_subjunctive_stems"));
}

#[test]
fn test_learning_frontier_calculation() {
    let graph = get_default_linguistic_graph();
    let mut mastered = std::collections::HashSet::new();
    mastered.insert(ConceptId("irregular_present_stems".to_string()));
    mastered.insert(ConceptId("irregular_preterite_stems".to_string()));
    mastered.insert(ConceptId("irregular_subjunctive_stems".to_string()));
    
    let frontier = graph.get_learning_frontier(&mastered);
    assert!(!frontier.is_empty());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test graph_tests`  
Expected: FAIL with missing module `graph`.

- [ ] **Step 3: Implement `src/core/graph.rs` and export in `src/core/mod.rs`**

Implement `ConceptId`, `ConceptNode`, `ConceptCategory`, `LinguisticGraph`, cycle detection via Kahn's algorithm / DFS, and complete default ontology mapping.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test graph_tests`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/core/graph.rs src/core/mod.rs tests/graph_tests.rs
git commit --no-gpg-sign -m "feat(core): implement linguistic knowledge graph engine and concept ontology"
```

---

### Task 2: Enhanced Exercise Metadata & Concept Parser

**Files:**
- Modify: `src/core/exercise.rs`
- Test: `tests/exercise_parser_tests.rs`

- [ ] **Step 1: Write test for concept metadata parsing in `tests/exercise_parser_tests.rs`**

```rust
#[test]
fn test_parse_exercise_with_concepts_and_prerequisites() {
    let content = r#"<!--
id: test_concepts_01
level: B1
type: cloze
title: Test Concept Linking
topic: travel_logistics_and_borders
concepts: ["subjunctive_temporal_future", "impersonal_se"]
prerequisites: ["05_subjunctive_conjunctions"]
grammar_focus: "Subjunctive required for prospective time clauses."
-->

### Context
Test context

### Exercise
En cuanto <!-- ANSWER -->, saldremos.
"#;
    let ex = Exercise::from_markdown("exercises/test.md", content).expect("Failed to parse");
    assert_eq!(ex.concept_tags, vec!["subjunctive_temporal_future", "impersonal_se"]);
    assert_eq!(ex.prerequisites, vec!["05_subjunctive_conjunctions"]);
    assert_eq!(ex.grammar_focus.as_deref(), Some("Subjunctive required for prospective time clauses."));
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `cargo test --test exercise_parser_tests`  
Expected: FAIL with missing struct fields.

- [ ] **Step 3: Update `Exercise` and `Exercise::from_markdown` in `src/core/exercise.rs`**

Add fields `concept_tags: Vec<String>`, `prerequisites: Vec<String>`, `grammar_focus: Option<String>`, `contrast_note: Option<String>` and parser extraction logic with backwards-compatible defaults (`vec![]` / `None`).

- [ ] **Step 4: Run test to verify passes**

Run: `cargo test --test exercise_parser_tests`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/core/exercise.rs tests/exercise_parser_tests.rs
git commit --no-gpg-sign -m "feat(core): extend exercise model and parser with concept linking metadata"
```

---

### Task 3: State Concept Mastery & Targeted Weakness Profiler

**Files:**
- Modify: `src/core/state.rs`
- Modify: `src/cli/commands/progress.rs`
- Test: `tests/weakness_profiler_tests.rs`

- [ ] **Step 1: Write test for concept mastery aggregation in `tests/weakness_profiler_tests.rs`**

Test that SRS lapses across exercises with shared concept tags produce aggregated `ConceptMastery` and root-cause diagnostic advice in `spanglings progress`.

- [ ] **Step 2: Run test to verify failure**

Run: `cargo test --test weakness_profiler_tests`

- [ ] **Step 3: Implement `ConceptMastery` in `src/core/state.rs` and update `progress.rs`**

- [ ] **Step 4: Run test to verify passes**

Run: `cargo test --test weakness_profiler_tests`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/core/state.rs src/cli/commands/progress.rs tests/weakness_profiler_tests.rs
git commit --no-gpg-sign -m "feat(state,progress): add concept mastery tracking and graph-aware weakness profiler"
```

---

### Task 4: Concept-Aware Diagnostic Compiler & Cross-Links

**Files:**
- Modify: `src/core/diagnostic.rs`
- Modify: `src/engine/evaluator.rs`
- Test: `tests/diagnostic_tests.rs`

- [ ] **Step 1: Write test in `tests/diagnostic_tests.rs` verifying diagnostic cross-reference output**

Verify compiler error output contains `= note: Linked Foundation: Track XX (<Topic>)`.

- [ ] **Step 2: Run test to verify failure**

- [ ] **Step 3: Implement diagnostic note enrichment in `src/core/diagnostic.rs`**

- [ ] **Step 4: Run test to verify passes**

Run: `cargo test --test diagnostic_tests`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/core/diagnostic.rs src/engine/evaluator.rs tests/diagnostic_tests.rs
git commit --no-gpg-sign -m "feat(diagnostic): enrich compiler errors with linked foundation concepts and drills"
```

---

### Task 5: Retroactive Conceptual Tagging of Tracks 00–41 (231 Exercises)

**Files:**
- Modify: All markdown files under `exercises/00_baseline` through `exercises/41_adverbial_clauses_and_conjunctions`

- [ ] **Step 1: Write script to annotate all 231 existing exercises with concept tags, prerequisites, and grammar focus**
- [ ] **Step 2: Run annotation script and verify all files parse cleanly**
- [ ] **Step 3: Run `cargo test` to ensure zero parsing regressions**
- [ ] **Step 4: Commit**

```bash
git add exercises/
git commit --no-gpg-sign -m "feat(curriculum): tag all 231 existing exercises with concepts and prerequisites"
```

---

### Task 6: Curriculum Expansion: Tracks 42 to 47 (36 New Exercises)

**Files:**
- Create: `exercises/42_travel_logistics_and_borders/` (6 exercises)
- Create: `exercises/43_banking_taxes_and_finances/` (6 exercises)
- Create: `exercises/44_consumer_complaints_and_rights/` (6 exercises)
- Create: `exercises/45_home_maintenance_and_repairs/` (6 exercises)
- Create: `exercises/46_news_media_and_civic_debate/` (6 exercises)
- Create: `exercises/47_conversational_markers_and_nuance/` (6 exercises)

- [ ] **Step 1: Create all 36 exercises with context, cloze solutions, alternatives, diagnostic rules, concept tags, and hints**
- [ ] **Step 2: Verify all 36 exercises are solvable and valid**
- [ ] **Step 3: Commit**

```bash
git add exercises/
git commit --no-gpg-sign -m "feat(curriculum): add tracks 42-47 practical everyday and professional Spanish expansion"
```

---

### Task 7: CLI Filtering, Reference Cards & Golden Validation

**Files:**
- Modify: `src/cli/commands/list.rs`
- Modify: `src/cli/commands/drill.rs`
- Modify: `src/cli/mod.rs`
- Modify: `tests/exercise_validity_tests.rs`
- Modify: `README.md` & `docs/BACKLOG.md`

- [ ] **Step 1: Update `tests/exercise_validity_tests.rs` to validate all 267 exercises against the graph ontology**
- [ ] **Step 2: Add `--concept` filter flag to `spanglings list` and `spanglings drill`**
- [ ] **Step 3: Run full verification suite (`cargo test --all-targets`, `cargo clippy`, `cargo fmt`)**
- [ ] **Step 4: Commit and update documentation**

```bash
git add src/cli/ tests/ README.md docs/BACKLOG.md
git commit --no-gpg-sign -m "feat(cli,docs): add concept filtering to list and drill, validate full 267 exercise catalog"
```
