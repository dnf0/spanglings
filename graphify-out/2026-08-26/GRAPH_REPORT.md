# Graph Report - spanglings  (2026-08-26)

## Corpus Check
- 33 files · ~16,731 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 221 nodes · 312 edges · 19 communities (17 shown, 2 thin omitted)
- Extraction: 88% EXTRACTED · 12% INFERRED · 0% AMBIGUOUS · INFERRED: 39 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `db5805f8`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Spanglings Implementation Plan
- Spanglings Design Specification
- check_accent_match
- find_all_exercises
- Exercise
- main
- spanglings
- AppState
- validate_submission
- Grammar Diagnostics & Compiler-Style Error Formatter Implementation Plan
- progress.rs
- SrsItem
- get_reference_card
- run_drill
- Commands

## God Nodes (most connected - your core abstractions)
1. `AppState` - 20 edges
2. `Exercise` - 16 edges
3. `validate_submission()` - 15 edges
4. `find_all_exercises()` - 13 edges
5. `Spanglings Implementation Plan` - 12 edges
6. `get_reference_card()` - 10 edges
7. `SrsItem` - 9 edges
8. `Level` - 8 edges
9. `Spanglings Design Specification` - 8 edges
10. `run_exercise()` - 7 edges

## Surprising Connections (you probably didn't know these)
- `test_run_exercise_with_file_path()` --calls--> `run_exercise()`  [INFERRED]
  tests/cli_tests.rs → src/cli/commands/run.rs
- `test_get_accidental_se_reference_card()` --calls--> `get_reference_card()`  [INFERRED]
  tests/reference_tests.rs → src/core/reference.rs
- `test_get_past_tenses_reference_card()` --calls--> `get_reference_card()`  [INFERRED]
  tests/reference_tests.rs → src/core/reference.rs
- `test_get_por_para_reference_card()` --calls--> `get_reference_card()`  [INFERRED]
  tests/reference_tests.rs → src/core/reference.rs
- `test_get_prepositions_reference_card()` --calls--> `get_reference_card()`  [INFERRED]
  tests/reference_tests.rs → src/core/reference.rs

## Import Cycles
- 2-file cycle: `src/core/curriculum.rs -> src/core/exercise.rs -> src/core/curriculum.rs`

## Communities (19 total, 2 thin omitted)

### Community 0 - "Spanglings Implementation Plan"
Cohesion: 0.15
Nodes (12): Spanglings Implementation Plan, Task 10: Complete Curriculum Authoring (Tracks 00-21, 100+ Exercises), Task 11: Automated Golden Test Suite & Verification, Task 1: Project Scaffolding & Cargo Setup, Task 2: Core Domain Models & Markdown Exercise Parser, Task 3: Normalization & Smart Accent Warning Engine, Task 4: Grammar Diagnostics & Compiler-Style Error Formatter, Task 5: SM-2 Spaced Repetition System & State Persistence (+4 more)

### Community 1 - "Spanglings Design Specification"
Cohesion: 0.10
Nodes (19): 1.1 Problem Statement, 1.2 The Solution: Spanglings, 1. Executive Summary & Philosophy, 2.1 CLI Interface & Commands, 2.2 Configuration & SRS State Persistence, 2. Architecture & System Design, 3.1 Accent & Punctuation Handling, 3.2 Compiler-Style Diagnostic Output (+11 more)

### Community 2 - "check_accent_match"
Cohesion: 0.23
Nodes (7): AccentResult, check_accent_match(), String, strip_accents(), normalize(), String, test_smart_accent_matching()

### Community 3 - "find_all_exercises"
Cohesion: 0.10
Nodes (22): Option, Result, show_hint(), list_exercises(), Result, reset_exercise(), Result, run_exercise() (+14 more)

### Community 4 - "Exercise"
Cohesion: 0.10
Nodes (18): Level, Display, Formatter, FromStr, DiagnosticRule, Exercise, ExerciseType, ParseExerciseTypeError (+10 more)

### Community 7 - "AppState"
Cohesion: 0.14
Nodes (13): HashSet, AppState, ExerciseStat, DateTime, Default, Option, Path, PathBuf (+5 more)

### Community 10 - "validate_submission"
Cohesion: 0.11
Nodes (20): Result, run_review_session(), Diagnostic, Option, String, get_rule_title(), String, extract_user_answer() (+12 more)

### Community 12 - "Grammar Diagnostics & Compiler-Style Error Formatter Implementation Plan"
Cohesion: 0.25
Nodes (7): Grammar Diagnostics & Compiler-Style Error Formatter Implementation Plan, Task 1: Scaffolding and Failing Tests, Task 2: Standard Error Codes, Task 3: Compiler-Style Diagnostics, Task 4: Accent Mode Integration and Module Exporting, Task 5: Core Validation Logic & Submission Evaluator, Task 6: Verification and Automated Quality Control

### Community 13 - "progress.rs"
Cohesion: 0.33
Nodes (5): HashMap, render_progress_bar(), Result, String, show_progress()

### Community 14 - "SrsItem"
Cohesion: 0.18
Nodes (8): calculate_sm2_review(), DateTime, Default, Option, Self, Utc, SrsItem, test_sm2_repetition_intervals()

### Community 15 - "get_reference_card"
Cohesion: 0.19
Nodes (13): Result, show_explanation(), get_reference_card(), list_reference_topics(), Option, test_get_accidental_se_reference_card(), test_get_past_tenses_reference_card(), test_get_por_para_reference_card() (+5 more)

### Community 16 - "run_drill"
Cohesion: 0.40
Nodes (4): DrillItem, Option, Result, run_drill()

### Community 17 - "Commands"
Cohesion: 0.60
Nodes (4): Cli, Commands, Option, String

## Knowledge Gaps
- **33 isolated node(s):** `spanglings`, `DrillItem`, `Task 1: Scaffolding and Failing Tests`, `Task 2: Standard Error Codes`, `Task 3: Compiler-Style Diagnostics` (+28 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `validate_submission()` connect `validate_submission` to `check_accent_match`, `find_all_exercises`, `Exercise`, `AppState`?**
  _High betweenness centrality (0.172) - this node is a cross-community bridge._
- **Why does `AppState` connect `AppState` to `progress.rs`, `SrsItem`?**
  _High betweenness centrality (0.155) - this node is a cross-community bridge._
- **Why does `Exercise` connect `Exercise` to `validate_submission`, `find_all_exercises`?**
  _High betweenness centrality (0.152) - this node is a cross-community bridge._
- **Are the 10 inferred relationships involving `validate_submission()` (e.g. with `run_review_session()` and `run_exercise()`) actually correct?**
  _`validate_submission()` has 10 INFERRED edges - model-reasoned connections that need verification._
- **Are the 7 inferred relationships involving `find_all_exercises()` (e.g. with `show_hint()` and `list_exercises()`) actually correct?**
  _`find_all_exercises()` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `spanglings`, `DrillItem`, `Task 1: Scaffolding and Failing Tests` to the rest of the system?**
  _33 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Spanglings Design Specification` be split into smaller, more focused modules?**
  _Cohesion score 0.1 - nodes in this community are weakly interconnected._