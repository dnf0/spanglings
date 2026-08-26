# Graph Report - spanglings  (2026-08-26)

## Corpus Check
- 13 files · ~9,038 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 86 nodes · 95 edges · 12 communities (11 shown, 1 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 2 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d0fd541f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Spanglings Implementation Plan
- Spanglings Design Specification
- check_accent_match
- Level
- Exercise
- spanglings
- 3. Linguistic & Diagnostic Engine

## God Nodes (most connected - your core abstractions)
1. `Spanglings Implementation Plan` - 12 edges
2. `Level` - 8 edges
3. `Exercise` - 8 edges
4. `Spanglings Design Specification` - 8 edges
5. `ExerciseType` - 6 edges
6. `check_accent_match()` - 5 edges
7. `5. Curriculum Map & Tracks (100+ Total Exercises)` - 5 edges
8. `3. Linguistic & Diagnostic Engine` - 4 edges
9. `ParseLevelError` - 3 edges
10. `ParseExerciseTypeError` - 3 edges

## Surprising Connections (you probably didn't know these)
- `test_smart_accent_matching()` --calls--> `check_accent_match()`  [INFERRED]
  tests/normalizer_tests.rs → src/engine/accents.rs
- `Exercise` --references--> `Level`  [EXTRACTED]
  src/core/exercise.rs → src/core/curriculum.rs
- `check_accent_match()` --calls--> `normalize()`  [INFERRED]
  src/engine/accents.rs → src/engine/normalizer.rs

## Import Cycles
- None detected.

## Communities (12 total, 1 thin omitted)

### Community 0 - "Spanglings Implementation Plan"
Cohesion: 0.15
Nodes (12): Spanglings Implementation Plan, Task 10: Complete Curriculum Authoring (Tracks 00-21, 100+ Exercises), Task 11: Automated Golden Test Suite & Verification, Task 1: Project Scaffolding & Cargo Setup, Task 2: Core Domain Models & Markdown Exercise Parser, Task 3: Normalization & Smart Accent Warning Engine, Task 4: Grammar Diagnostics & Compiler-Style Error Formatter, Task 5: SM-2 Spaced Repetition System & State Persistence (+4 more)

### Community 1 - "Spanglings Design Specification"
Cohesion: 0.12
Nodes (15): 1.1 Problem Statement, 1.2 The Solution: Spanglings, 1. Executive Summary & Philosophy, 2.1 CLI Interface & Commands, 2.2 Configuration & SRS State Persistence, 2. Architecture & System Design, 4. Exercise Modalities & Authoring Format, 5. Curriculum Map & Tracks (100+ Total Exercises) (+7 more)

### Community 2 - "check_accent_match"
Cohesion: 0.23
Nodes (7): AccentResult, check_accent_match(), String, strip_accents(), normalize(), String, test_smart_accent_matching()

### Community 3 - "Level"
Cohesion: 0.21
Nodes (9): Level, ParseLevelError, Display, Err, Formatter, FromStr, Result, Self (+1 more)

### Community 4 - "Exercise"
Cohesion: 0.17
Nodes (14): P, PathBuf, DiagnosticRule, Exercise, ExerciseType, ParseExerciseTypeError, Display, Err (+6 more)

### Community 10 - "3. Linguistic & Diagnostic Engine"
Cohesion: 0.50
Nodes (4): 3.1 Accent & Punctuation Handling, 3.2 Compiler-Style Diagnostic Output, 3.3 Diagnostic Error Taxonomy, 3. Linguistic & Diagnostic Engine

## Knowledge Gaps
- **26 isolated node(s):** `spanglings`, `Task 1: Project Scaffolding & Cargo Setup`, `Task 2: Core Domain Models & Markdown Exercise Parser`, `Task 3: Normalization & Smart Accent Warning Engine`, `Task 4: Grammar Diagnostics & Compiler-Style Error Formatter` (+21 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Level` connect `Level` to `Exercise`, `exercise_parser_tests.rs`?**
  _High betweenness centrality (0.083) - this node is a cross-community bridge._
- **Why does `Exercise` connect `Exercise` to `Level`?**
  _High betweenness centrality (0.057) - this node is a cross-community bridge._
- **Why does `Spanglings Design Specification` connect `Spanglings Design Specification` to `3. Linguistic & Diagnostic Engine`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **What connects `spanglings`, `Task 1: Project Scaffolding & Cargo Setup`, `Task 2: Core Domain Models & Markdown Exercise Parser` to the rest of the system?**
  _26 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Spanglings Design Specification` be split into smaller, more focused modules?**
  _Cohesion score 0.125 - nodes in this community are weakly interconnected._