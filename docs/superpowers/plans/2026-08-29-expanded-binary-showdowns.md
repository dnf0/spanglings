# Implementation Plan: Expanded 16-Pair Spanish Binary Contrast Showdowns

**Branch:** `feat/expanded-16-binary-showdowns`  
**Spec Reference:** [`docs/superpowers/specs/2026-08-29-expanded-binary-showdowns-design.md`](file:///Users/danielfisher/repos/spanglings/docs/superpowers/specs/2026-08-29-expanded-binary-showdowns-design.md)

---

## Tasks Overview

- [ ] **Task 1: Core Engine Showdown Expansion (`src/core/arcade.rs`)**
  - Add 8 new enum variants to `ShowdownPair` (`TenerHaber`, `SaberConocer`, `MuyMucho`, `PedirPreguntar`, `LlevarTraer`, `HaberEstar`, `IrIrse`, `BienBueno`).
  - Implement `from_str`, `slug`, `title`, and `list_showdown_pairs` for all 16 pairs, including intuitive aliases (`have`, `know`, `ask`, `very-much`, etc.).
  - Implement rich curated sentence banks for all 8 new pairs with nuanced linguistic explanations.
  - Update `tests/arcade_tests.rs` to verify generation and correctness across all 16 pairs.

- [ ] **Task 2: CLI Command & Topic Parser Integration (`src/cli/commands/arcade.rs`, `src/cli/mod.rs`, `src/main.rs`)**
  - Update `select_arcade_items` and `spanglings arcade <pair>` topic parsing for all 16 showdown pairs.
  - Update `tests/cli_arcade_tests.rs` to verify CLI invocation and stats for new pairs.

- [ ] **Task 3: TUI Showdown Selector & Modal Polish (`src/tui/app.rs`, `src/tui/ui.rs`, `src/tui/events.rs`)**
  - Enable direct showdown selection and testing within the TUI.
  - Update `tests/tui_arcade_tests.rs`.

- [ ] **Task 4: Verification, Linting, & Knowledge Graph Update**
  - Run `cargo test` across all suites.
  - Run `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`.
  - Run `uvx --from graphifyy graphify update .`.
  - Conduct final code review and prepare for PR merge.

