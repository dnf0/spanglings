# Implementation Plan: 5 High-Yield Advanced Drill Engines

**Branch:** `feat/high-yield-arcade-engines`  
**Spec Reference:** [`docs/superpowers/specs/2026-08-29-high-yield-arcade-drill-engines-design.md`](file:///Users/danielfisher/repos/spanglings/docs/superpowers/specs/2026-08-29-high-yield-arcade-drill-engines-design.md)

---

## Tasks Overview

- [ ] **Task 1: Core Engine Implementation (`src/core/arcade.rs`)**
  - Implement curated sentence pools for:
    - Prepositional Regimen (`ENGINE_REGIMEN_POOL`)
    - Irregular Verb Speed Gun (`ENGINE_IRREGULARS_POOL`)
    - False Friends Trap Detector (`ENGINE_FALSE_FRIENDS_POOL`)
    - The "Se" Matrix (`ENGINE_SE_MATRIX_POOL`)
    - Discourse Connectors & Flow (`ENGINE_CONNECTORS_POOL`)
  - Implement `generate_specialized_engine_items(slug, count)` and integrate into `generate_4choice_items` & `select_arcade_items`.
  - Add tests in `tests/arcade_tests.rs` verifying item generation, valid distractor counts, and explanation completeness for all 5 engines.

- [ ] **Task 2: CLI Command & Argument Dispatch (`src/cli/commands/arcade.rs`, `src/cli/mod.rs`, `src/main.rs`)**
  - Update CLI argument parser and topic matcher to handle all 5 engine slugs and their aliases (`regimen`, `prepositions`, `irregulars`, `verbs`, `false-friends`, `cognates`, `se`, `se-matrix`, `connectors`, `discourse`).
  - Update `select_arcade_items` to include specialized engine items in the default mixed mode pool.
  - Update `tests/cli_arcade_tests.rs`.

- [ ] **Task 3: TUI Integration & Showdown/Engine Navigation Polish (`src/tui/app.rs`, `src/tui/ui.rs`, `src/tui/events.rs`)**
  - Ensure TUI arcade arena displays clear engine names and titles when launched with engine topics.
  - Update `tests/tui_arcade_tests.rs`.

- [ ] **Task 4: Complete Verification, Formatting, & Knowledge Graph Update**
  - Run `cargo test` across all suites.
  - Run `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`.
  - Run `uvx --from graphifyy graphify update .`.
  - Conduct final code review and merge via PR.

