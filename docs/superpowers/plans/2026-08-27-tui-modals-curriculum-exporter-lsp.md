# Spanglings Next-Gen Capabilities Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Focus Areas 6–9: In-TUI Modal Power Tools (Conjugator, Reference Browser, Help Overlay), C1 Technical & RFC Curriculum Tracks (27–30), Portable Study Pack & Anki/Markdown Exporters with State Sync, and a Native Language Server Protocol (LSP) Engine for IDEs.

**Architecture:** 
- **TUI Modals (`src/tui/`)**: Extend `App` and `AppMode` with modal overlays for `Conjugating`, `BrowsingReference`, and `Help`, rendered via Ratatui centered floating layout chunks.
- **Curriculum (`exercises/`)**: Author 24 advanced B2-C1 markdown exercises across 4 new tracks (27: System Design & Post-Mortems, 28: Concessive Subjunctive Clauses, 29: Verbal Periphrases, 30: Executive & Leadership Collocations).
- **Export & Sync (`src/cli/commands/export.rs`, `src/cli/commands/sync.rs`)**: Anki TSV/APKG deck generator, Markdown study notes builder, and portable JSON state backup/import engine.
- **LSP Engine (`src/lsp/`)**: Lightweight JSON-RPC 2.0 stdio server providing live editor diagnostics, hover grammar sheets, autocompletions, and code actions.

**Tech Stack:** Rust 2021, Ratatui, Crossterm, Clap, Serde, Serde JSON, Sha2, Include Dir.

---

### Task 1: In-TUI Verb Conjugator Modal (`SPANG-050`)

**Files:**
- Modify: `src/tui/app.rs`
- Modify: `src/tui/ui.rs`
- Modify: `src/tui/events.rs`
- Test: `tests/tui_tests.rs`

- [ ] **Step 1: Write the failing unit and integration tests**

```rust
// tests/tui_tests.rs
#[test]
fn test_tui_conjugator_modal_navigation_and_lookup() {
    let exercises = sample_exercises();
    let mut app = App::new(exercises, false);
    assert_eq!(app.mode, AppMode::Editing);

    // Open conjugator modal
    app.enter_conjugator();
    assert_eq!(app.mode, AppMode::Conjugating);
    assert_eq!(app.conjugator_query, "");

    // Type "tener"
    for c in "tener".chars() {
        app.insert_conjugator_char(c);
    }
    assert_eq!(app.conjugator_query, "tener");
    app.submit_conjugation();
    assert!(app.conjugator_table.is_some());

    // Exit conjugator modal
    app.exit_conjugator();
    assert_eq!(app.mode, AppMode::Editing);
}
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test --test tui_tests test_tui_conjugator_modal_navigation_and_lookup`
Expected: FAIL with missing fields/methods on `App` and `AppMode::Conjugating`.

- [ ] **Step 3: Implement Conjugator Modal state and UI rendering**

Extend `AppMode` with `Conjugating` in `src/tui/app.rs`, implement `enter_conjugator`, `exit_conjugator`, `insert_conjugator_char`, `delete_conjugator_char`, `submit_conjugation`. In `src/tui/ui.rs`, add `draw_conjugator_modal` rendering a centered 75% popup with verb search box and colorized conjugation grids. Wire `'c'` key in `src/tui/events.rs`.

- [ ] **Step 4: Run tests to verify pass**

Run: `cargo test --test tui_tests test_tui_conjugator_modal_navigation_and_lookup`
Expected: PASS

- [ ] **Step 5: Verify formatting and linter**

Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`

- [ ] **Step 6: Commit**

```bash
git add src/tui/app.rs src/tui/ui.rs src/tui/events.rs tests/tui_tests.rs
git commit --no-gpg-sign -m "feat(tui): add interactive in-tui verb conjugator popup modal"
```

---

### Task 2: In-TUI Grammar Reference Browser & Help Modals (`SPANG-051`, `SPANG-053`)

**Files:**
- Modify: `src/tui/app.rs`
- Modify: `src/tui/ui.rs`
- Modify: `src/tui/events.rs`
- Test: `tests/tui_tests.rs`

- [ ] **Step 1: Write failing tests for reference browser and help modals**

```rust
// tests/tui_tests.rs
#[test]
fn test_tui_reference_browser_modal() {
    let exercises = sample_exercises();
    let mut app = App::new(exercises, false);
    
    app.enter_reference_browser();
    assert_eq!(app.mode, AppMode::BrowsingReference);
    assert!(!app.ref_topics.is_empty());

    app.next_ref_topic();
    assert_eq!(app.ref_selected_idx, 1);

    app.exit_reference_browser();
    assert_eq!(app.mode, AppMode::Editing);
}

#[test]
fn test_tui_help_modal() {
    let exercises = sample_exercises();
    let mut app = App::new(exercises, false);
    
    app.enter_help();
    assert_eq!(app.mode, AppMode::Help);
    app.exit_help();
    assert_eq!(app.mode, AppMode::Editing);
}
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test --test tui_tests test_tui_reference_browser_modal`
Expected: FAIL

- [ ] **Step 3: Implement Reference Browser and Help modal handlers and UI**

Add `AppMode::BrowsingReference` and `AppMode::Help` to `src/tui/app.rs`. In `src/tui/ui.rs`, draw full-text searchable reference browser with topic list on the left and scrollable card on the right; draw help overlay displaying all hotkeys. In `src/tui/events.rs`, wire `'r'` for reference browser, `'?'` / `'h'` / `F1` for help overlay.

- [ ] **Step 4: Run tests to verify pass**

Run: `cargo test --test tui_tests`
Expected: PASS

- [ ] **Step 5: Verify formatting and linter**

Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`

- [ ] **Step 6: Commit**

```bash
git add src/tui/app.rs src/tui/ui.rs src/tui/events.rs tests/tui_tests.rs
git commit --no-gpg-sign -m "feat(tui): add reference card browser modal and keybinding help overlay"
```

---

### Task 3: Advanced C1 Technical, RFC & Professional Track Expansion (`SPANG-060` - `SPANG-063`)

**Files:**
- Create: `exercises/27_system_design/01_post_mortem_outage.md` ... `06_microservices_resilience.md`
- Create: `exercises/28_advanced_subjunctive/01_por_mucho_que.md` ... `06_a_condicion_de_que.md`
- Create: `exercises/29_verbal_periphrases/01_llevar_gerundio.md` ... `06_acabar_por.md`
- Create: `exercises/30_executive_leadership/01_hacer_hincapie.md` ... `06_zanjar_disputa.md`
- Test: `tests/curriculum_tests.rs`

- [ ] **Step 1: Write integration tests verifying all 24 new exercises load and validate**

```rust
// tests/curriculum_tests.rs
#[test]
fn test_new_advanced_tracks_loaded_and_valid() {
    let curriculum = Curriculum::load_embedded().expect("embedded curriculum should load");
    
    // Check Track 27 (System Design)
    let track_27 = curriculum.find_by_topic("system_design");
    assert_eq!(track_27.len(), 6);

    // Check Track 28 (Concessive Subjunctive)
    let track_28 = curriculum.find_by_topic("advanced_subjunctive");
    assert_eq!(track_28.len(), 6);

    // Check Track 29 (Verbal Periphrases)
    let track_29 = curriculum.find_by_topic("verbal_periphrases");
    assert_eq!(track_29.len(), 6);

    // Check Track 30 (Executive Leadership)
    let track_30 = curriculum.find_by_topic("executive_leadership");
    assert_eq!(track_30.len(), 6);
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `cargo test --test curriculum_tests test_new_advanced_tracks_loaded_and_valid`
Expected: FAIL (exercises not yet authored)

- [ ] **Step 3: Author all 24 high-register exercise markdown files**

Craft high-quality pedagogical exercises with context explanations, sentence prompts, solutions, and 3-tier hints following Spanglings Markdown specification.

- [ ] **Step 4: Run test to verify pass**

Run: `cargo test --test curriculum_tests`
Expected: PASS with 140 total exercises verified.

- [ ] **Step 5: Verify formatting and linter**

Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`

- [ ] **Step 6: Commit**

```bash
git add exercises/ tests/curriculum_tests.rs
git commit --no-gpg-sign -m "feat(curriculum): add tracks 27-30 covering system design RFCs, advanced subjunctive, periphrases, and leadership collocations"
```

---

### Task 4: Anki Deck, Markdown Study Notes Exporter & State Sync (`SPANG-070` - `SPANG-072`)

**Files:**
- Create: `src/cli/commands/export.rs`
- Create: `src/cli/commands/sync.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/cli/commands/mod.rs`
- Modify: `src/main.rs`
- Test: `tests/export_sync_tests.rs`

- [ ] **Step 1: Write failing tests for Anki export, Markdown export, and State sync**

```rust
// tests/export_sync_tests.rs
#[test]
fn test_anki_tsv_export_generation() {
    let exercises = sample_exercises();
    let state = AppState::default();
    let tsv = generate_anki_tsv(&exercises, &state, false);
    assert!(tsv.contains("#separator:tab"));
    assert!(tsv.contains("¿Cómo se dice?"));
}

#[test]
fn test_markdown_study_guide_export() {
    let exercises = sample_exercises();
    let state = AppState::default();
    let md = generate_markdown_notes(&exercises, &state);
    assert!(md.contains("# Spanglings Study Notes"));
    assert!(md.contains("## Topic:"));
}

#[test]
fn test_state_sync_export_and_import() {
    let mut state = AppState::default();
    state.mark_completed("test_01");
    
    let exported = export_state_json(&state).unwrap();
    let imported = import_state_json(&exported).unwrap();
    assert!(imported.is_completed("test_01"));
}
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test --test export_sync_tests`
Expected: FAIL

- [ ] **Step 3: Implement export and sync handlers**

Implement `spanglings export anki` (generating TSV with header metadata compatible with Anki import), `spanglings export markdown` (generating clean, structured Markdown study guides), and `spanglings sync export` / `import` (with optional checksum validation and merge options).

- [ ] **Step 4: Run tests to verify pass**

Run: `cargo test --test export_sync_tests`
Expected: PASS

- [ ] **Step 5: Verify formatting and linter**

Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`

- [ ] **Step 6: Commit**

```bash
git add src/cli/commands/export.rs src/cli/commands/sync.rs src/cli/mod.rs src/cli/commands/mod.rs src/main.rs tests/export_sync_tests.rs
git commit --no-gpg-sign -m "feat(export): implement anki flashcards exporter, markdown study guide generator, and portable state sync"
```

---

### Task 5: Native Language Server Protocol (LSP) Engine (`SPANG-080` - `SPANG-082`)

**Files:**
- Create: `src/lsp/mod.rs`
- Create: `src/lsp/server.rs`
- Create: `src/lsp/protocol.rs`
- Create: `src/lsp/diagnostics.rs`
- Create: `src/lsp/hover.rs`
- Modify: `src/lib.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`
- Test: `tests/lsp_tests.rs`

- [ ] **Step 1: Write integration tests for LSP JSON-RPC lifecycle**

```rust
// tests/lsp_tests.rs
#[test]
fn test_lsp_initialize_and_hover_response() {
    let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}"#;
    let mut server = LspServer::new();
    let resp = server.handle_raw_message(init_req).unwrap();
    assert!(resp.contains(r#""capabilities""#));

    // Test hover on Spanish trigger
    let hover_req = r#"{"jsonrpc":"2.0","id":2,"method":"textDocument/hover","params":{"textDocument":{"uri":"file:///exercises/01_subjunctive/01_duda.md"},"position":{"line":5,"character":10}}}"#;
    let hover_resp = server.handle_raw_message(hover_req).unwrap();
    assert!(hover_resp.contains("Grammar Reference"));
}

#[test]
fn test_lsp_live_diagnostics_stream() {
    let mut server = LspServer::new();
    let sample_doc = "---\nid: test\ntitle: Test\ntopic: subjunctive\nlevel: B1\n---\n<!-- I AM NOT DONE -->\nSentence: Ojalá que ___\nAnswer: incorrect";
    let diags = server.validate_document("file:///test.md", sample_doc);
    assert!(!diags.is_empty());
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `cargo test --test lsp_tests`
Expected: FAIL

- [ ] **Step 3: Implement LSP Server and protocol dispatch**

Implement stdio JSON-RPC 2.0 loop in `src/lsp/server.rs`, message serialization/deserialization in `src/lsp/protocol.rs`, diagnostic computation in `src/lsp/diagnostics.rs`, and hover documentation in `src/lsp/hover.rs`. Wire `spanglings lsp` subcommand in `src/cli/mod.rs` and `src/main.rs`.

- [ ] **Step 4: Run test to verify pass**

Run: `cargo test --test lsp_tests`
Expected: PASS

- [ ] **Step 5: Verify formatting and linter**

Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`

- [ ] **Step 6: Commit**

```bash
git add src/lsp/ src/lib.rs src/cli/mod.rs src/main.rs tests/lsp_tests.rs
git commit --no-gpg-sign -m "feat(lsp): implement native Language Server Protocol (LSP) engine for IDE integrations"
```

---

### Task 6: Documentation, ADR & Full System Verification

**Files:**
- Modify: `README.md`
- Modify: `docs/BACKLOG.md`
- Create: `docs/adr/0002-lsp-and-in-tui-modal-architecture.md`

- [ ] **Step 1: Write ADR-0002 documenting LSP & Modal architecture**
- [ ] **Step 2: Update README.md documenting new commands and editor integration instructions**
- [ ] **Step 3: Run entire test suite, clippy, formatting, and update knowledge graph**
- [ ] **Step 4: Commit**

```bash
git add README.md docs/BACKLOG.md docs/adr/0002-lsp-and-in-tui-modal-architecture.md
git commit --no-gpg-sign -m "docs: document in-tui modals, exporter, and lsp server configuration"
```
