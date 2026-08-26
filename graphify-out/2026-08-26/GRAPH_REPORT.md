# Graph Report - spanglings  (2026-08-26)

## Corpus Check
- 9 files · ~8,618 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 69 nodes · 74 edges · 10 communities (9 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3fb285f1`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Spanglings Implementation Plan
- Spanglings Design Specification
- 5. Curriculum Map & Tracks (100+ Total Exercises)
- Level
- Exercise
- spanglings
- .from_markdown

## God Nodes (most connected - your core abstractions)
1. `Spanglings Implementation Plan` - 12 edges
2. `Level` - 8 edges
3. `Exercise` - 8 edges
4. `Spanglings Design Specification` - 8 edges
5. `ExerciseType` - 6 edges
6. `5. Curriculum Map & Tracks (100+ Total Exercises)` - 5 edges
7. `3. Linguistic & Diagnostic Engine` - 4 edges
8. `ParseLevelError` - 3 edges
9. `DiagnosticRule` - 3 edges
10. `1. Executive Summary & Philosophy` - 3 edges

## Surprising Connections (you probably didn't know these)
- `Exercise` --references--> `Level`  [EXTRACTED]
  src/core/exercise.rs → src/core/curriculum.rs

## Import Cycles
- None detected.

## Communities (10 total, 1 thin omitted)

### Community 0 - "Spanglings Implementation Plan"
Cohesion: 0.15
Nodes (12): Spanglings Implementation Plan, Task 10: Complete Curriculum Authoring (Tracks 00-21, 100+ Exercises), Task 11: Automated Golden Test Suite & Verification, Task 1: Project Scaffolding & Cargo Setup, Task 2: Core Domain Models & Markdown Exercise Parser, Task 3: Normalization & Smart Accent Warning Engine, Task 4: Grammar Diagnostics & Compiler-Style Error Formatter, Task 5: SM-2 Spaced Repetition System & State Persistence (+4 more)

### Community 1 - "Spanglings Design Specification"
Cohesion: 0.13
Nodes (14): 1.1 Problem Statement, 1.2 The Solution: Spanglings, 1. Executive Summary & Philosophy, 2.1 CLI Interface & Commands, 2.2 Configuration & SRS State Persistence, 2. Architecture & System Design, 3.1 Accent & Punctuation Handling, 3.2 Compiler-Style Diagnostic Output (+6 more)

### Community 2 - "5. Curriculum Map & Tracks (100+ Total Exercises)"
Cohesion: 0.40
Nodes (5): 5. Curriculum Map & Tracks (100+ Total Exercises), Track 00: Baseline Reflex Drills (A1-Pre-B1 Irregular Stems & High-Yield Vocab), Track 01: B1 (Core Mastery & Subjunctive Launchpad), Track 02: B2 (Advanced Fluency & Complex Structures), Track 03: C1 (Stylistic Nuance, Precision & Native Collocations)

### Community 3 - "Level"
Cohesion: 0.21
Nodes (9): Level, ParseLevelError, Display, Err, Formatter, FromStr, Result, Self (+1 more)

### Community 4 - "Exercise"
Cohesion: 0.25
Nodes (8): PathBuf, DiagnosticRule, Exercise, ExerciseType, Display, FromStr, String, Vec

### Community 7 - ".from_markdown"
Cohesion: 0.29
Nodes (5): P, Err, Formatter, Result, Self

## Knowledge Gaps
- **26 isolated node(s):** `spanglings`, `Task 1: Project Scaffolding & Cargo Setup`, `Task 2: Core Domain Models & Markdown Exercise Parser`, `Task 3: Normalization & Smart Accent Warning Engine`, `Task 4: Grammar Diagnostics & Compiler-Style Error Formatter` (+21 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Level` connect `Level` to `Exercise`?**
  _High betweenness centrality (0.106) - this node is a cross-community bridge._
- **Why does `Exercise` connect `Exercise` to `Level`, `.from_markdown`?**
  _High betweenness centrality (0.085) - this node is a cross-community bridge._
- **Why does `Spanglings Design Specification` connect `Spanglings Design Specification` to `5. Curriculum Map & Tracks (100+ Total Exercises)`?**
  _High betweenness centrality (0.065) - this node is a cross-community bridge._
- **What connects `spanglings`, `Task 1: Project Scaffolding & Cargo Setup`, `Task 2: Core Domain Models & Markdown Exercise Parser` to the rest of the system?**
  _26 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Spanglings Design Specification` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._