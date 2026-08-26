# Graph Report - spanglings  (2026-08-26)

## Corpus Check
- 18 files · ~12,204 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 116 nodes · 141 edges · 14 communities (13 shown, 1 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 10 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e5de2207`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Spanglings Implementation Plan
- Spanglings Design Specification
- check_accent_match
- Level
- Exercise
- spanglings
- validate_submission
- Grammar Diagnostics & Compiler-Style Error Formatter Implementation Plan
- Diagnostic

## God Nodes (most connected - your core abstractions)
1. `Exercise` - 13 edges
2. `Spanglings Implementation Plan` - 12 edges
3. `validate_submission()` - 11 edges
4. `Level` - 8 edges
5. `Spanglings Design Specification` - 8 edges
6. `Grammar Diagnostics & Compiler-Style Error Formatter Implementation Plan` - 7 edges
7. `ExerciseType` - 6 edges
8. `check_accent_match()` - 6 edges
9. `Diagnostic` - 6 edges
10. `ValidationResult` - 6 edges

## Surprising Connections (you probably didn't know these)
- `test_smart_accent_matching()` --calls--> `check_accent_match()`  [INFERRED]
  tests/normalizer_tests.rs → src/engine/accents.rs
- `test_validation_alternative_success()` --calls--> `validate_submission()`  [INFERRED]
  tests/diagnostic_rule_tests.rs → src/engine/validator.rs
- `test_validation_general_fallback_diagnostic()` --calls--> `validate_submission()`  [INFERRED]
  tests/diagnostic_rule_tests.rs → src/engine/validator.rs
- `test_validation_success()` --calls--> `validate_submission()`  [INFERRED]
  tests/diagnostic_rule_tests.rs → src/engine/validator.rs
- `test_validation_targeted_diagnostic_error()` --calls--> `validate_submission()`  [INFERRED]
  tests/diagnostic_rule_tests.rs → src/engine/validator.rs

## Import Cycles
- None detected.

## Communities (14 total, 1 thin omitted)

### Community 0 - "Spanglings Implementation Plan"
Cohesion: 0.15
Nodes (12): Spanglings Implementation Plan, Task 10: Complete Curriculum Authoring (Tracks 00-21, 100+ Exercises), Task 11: Automated Golden Test Suite & Verification, Task 1: Project Scaffolding & Cargo Setup, Task 2: Core Domain Models & Markdown Exercise Parser, Task 3: Normalization & Smart Accent Warning Engine, Task 4: Grammar Diagnostics & Compiler-Style Error Formatter, Task 5: SM-2 Spaced Repetition System & State Persistence (+4 more)

### Community 1 - "Spanglings Design Specification"
Cohesion: 0.10
Nodes (19): 1.1 Problem Statement, 1.2 The Solution: Spanglings, 1. Executive Summary & Philosophy, 2.1 CLI Interface & Commands, 2.2 Configuration & SRS State Persistence, 2. Architecture & System Design, 3.1 Accent & Punctuation Handling, 3.2 Compiler-Style Diagnostic Output (+11 more)

### Community 2 - "check_accent_match"
Cohesion: 0.23
Nodes (7): AccentResult, check_accent_match(), String, strip_accents(), normalize(), String, test_smart_accent_matching()

### Community 3 - "Level"
Cohesion: 0.21
Nodes (9): Level, ParseLevelError, Display, Err, Formatter, FromStr, Result, Self (+1 more)

### Community 4 - "Exercise"
Cohesion: 0.17
Nodes (14): P, PathBuf, DiagnosticRule, Exercise, ExerciseType, ParseExerciseTypeError, Display, Err (+6 more)

### Community 10 - "validate_submission"
Cohesion: 0.18
Nodes (13): AccentMode, get_rule_title(), String, extract_user_answer(), Option, String, validate_submission(), ValidationResult (+5 more)

### Community 12 - "Grammar Diagnostics & Compiler-Style Error Formatter Implementation Plan"
Cohesion: 0.25
Nodes (7): Grammar Diagnostics & Compiler-Style Error Formatter Implementation Plan, Task 1: Scaffolding and Failing Tests, Task 2: Standard Error Codes, Task 3: Compiler-Style Diagnostics, Task 4: Accent Mode Integration and Module Exporting, Task 5: Core Validation Logic & Submission Evaluator, Task 6: Verification and Automated Quality Control

### Community 13 - "Diagnostic"
Cohesion: 0.50
Nodes (3): Diagnostic, Option, String

## Knowledge Gaps
- **32 isolated node(s):** `spanglings`, `Task 1: Scaffolding and Failing Tests`, `Task 2: Standard Error Codes`, `Task 3: Compiler-Style Diagnostics`, `Task 4: Accent Mode Integration and Module Exporting` (+27 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Exercise` connect `Exercise` to `validate_submission`, `Level`, `exercise_parser_tests.rs`?**
  _High betweenness centrality (0.217) - this node is a cross-community bridge._
- **Why does `validate_submission()` connect `validate_submission` to `check_accent_match`, `Exercise`?**
  _High betweenness centrality (0.109) - this node is a cross-community bridge._
- **Why does `Level` connect `Level` to `Exercise`, `exercise_parser_tests.rs`?**
  _High betweenness centrality (0.100) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `validate_submission()` (e.g. with `check_accent_match()` and `normalize()`) actually correct?**
  _`validate_submission()` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `spanglings`, `Task 1: Scaffolding and Failing Tests`, `Task 2: Standard Error Codes` to the rest of the system?**
  _32 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Spanglings Design Specification` be split into smaller, more focused modules?**
  _Cohesion score 0.1 - nodes in this community are weakly interconnected._