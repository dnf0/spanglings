# Combinatorial Grammar Question Engine, Adaptive Weakness Drills & TUI Mastery Dashboard Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Provide 4,000+ unique practice questions across all 24 `GrammarConcept`s via a declarative combinatorial sentence generator, curriculum exercise extraction, an adaptive weakness-driven drill engine, and an interactive TUI Concept Mastery & Weakness Dashboard (`[m]`).

**Architecture:**
1. `src/core/generator.rs`: Declarative `SentenceFrame` and `DrillGenerator` producing authentic, zero-error practice questions for all 24 grammar topics.
2. `src/core/exercise.rs`: `Exercise::to_drill_items` extracting rapid-fire drill cards from curriculum test cases.
3. `src/cli/commands/drill.rs` & `src/cli/commands/blitz.rs`: Adaptive sampling engine (`--weak`, `--topic`, `--level`, `--track`, `--count`), real-time live `concept_mastery` updates, and session mastery delta reporting.
4. `src/tui/app.rs` & `src/tui/ui.rs`: Concept Mastery Dashboard (`[m]`) with progress bars, lapse indicators, and instant micro-drill triggers (`[d]`, `[w]`, `[r]`).

**Tech Stack:** Rust 2021, `rand`, `ratatui`, `colored`, `clap`.

---

### Task 1: Combinatorial Grammar Question Generator (`src/core/generator.rs`)

**Files:**
- Create: `src/core/generator.rs`
- Modify: `src/core/mod.rs`
- Modify: `src/lib.rs`
- Create: `tests/generator_tests.rs`

- [ ] **Step 1: Write the failing tests in `tests/generator_tests.rs`**

```rust
use spanglings::core::generator::{generate_drill_items_for_topic, generate_random_drill_items};
use spanglings::core::reference::list_grammar_concepts;

#[test]
fn test_all_24_concepts_have_combinatorial_generator_support() {
    let concepts = list_grammar_concepts();
    for concept in concepts {
        let items = generate_drill_items_for_topic(concept.slug, 10);
        assert!(
            !items.is_empty(),
            "Generator should produce items for concept '{}'",
            concept.slug
        );
        for item in &items {
            assert!(!item.trigger_sentence.is_empty());
            assert!(!item.target.is_empty());
            assert!(!item.explanation.is_empty());
            assert!(!item.trigger_sentence.contains('{'), "Unrendered template token in: {}", item.trigger_sentence);
            assert!(!item.trigger_sentence.contains('}'), "Unrendered template token in: {}", item.trigger_sentence);
        }
    }
}

#[test]
fn test_generate_large_question_batch_without_panics() {
    let items = generate_random_drill_items(500);
    assert_eq!(items.len(), 500);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test generator_tests`
Expected: FAIL (module `generator` not found)

- [ ] **Step 3: Implement `src/core/generator.rs`**

Implement `SentenceFrame`, parameter substitution, and registered frames for all 24 `GrammarConcept`s:
- `subjunctive`: Triggers (WEIRDO, doubt, conjunctions) $\times$ irregular/stem-changing verbs (*tener, poner, salir, venir, decir, hacer, ir, ser, saber, ver, dar, haber, querer, poder, dormir, pedir, sentir, traducir, conducir*) $\times$ subjects (*yo, tú, él, nosotros, ellos*).
- `por-para`: Cause, motive, means, exchange vs destination, recipient, deadline, purpose, opinion.
- `ser-estar`: Identity, origin, characteristics vs states, conditions, locations, adjectives that shift meaning (*listo, aburrido, rico, atento, verde*).
- `past`: Preterite triggers (*ayer, anoche, de repente*) vs Imperfect background (*siempre, todos los días, mientras*).
- `pronouns`: Clitic stacking (*me lo, te lo, se lo, nos lo*) with direct and indirect object substitution.
- `prepositions`: Prepositional régime verbs (*soñar con, depender de, fijarse en, insistir en, contar con, acordarse de, tardar en, renunciar a*).
- `accidental-se`: Non-agentive slips (*caer, olvidar, romper, perder, acabar, quedar*) with dative clitics (*me, te, le, nos, les*).
- `tech-software`: Software workflows (*desplegar, compilar, alojar, refactorizar, depurar, implementar, migrar, autenticar*).
- `business`: Professional correspondence (*remitir, aplazar, acordar, encabezar, convocar, liderar, negociar*).
- `false-friends`: Deceptive cognates (*actualmente, realizar, atender, pretender, constipado, decepción, carpeta, suceso*).
- `voseo`: Rioplatense address (*tenés, sos, podés, querés, sabés, decís, vení, hacé*).
- `accents`: Diacritical disambiguation (*él/el, tú/tu, mí/mi, sí/si, té/te, dé/de, sé/se, más/mas*).
- `epistemic-conjecture`: Future of probability (*serán, tendrá, estará, habrá*).
- `clitic-doubling`: Redundant pronoun reinforcement (*a María le di, a ellos les pareció*).
- `personal-a`: Human direct object marker (*visité a mi abuela* vs *visité el museo*).
- `gerund-rules`: Simultaneous adverbial gerunds (*caminando, leyendo*) vs adjectival restrictions.
- `adversatives`: *pero* vs *sino* vs *sino que*.
- `legal-subjunctive`: Juristic conditions (*hubiere, fuere, resultare*).
- `verbs-of-becoming`: *hacerse, volverse, ponerse, quedarse, convertirse en*.
- `epistemic-adverbs`: *quizás, tal vez, probablemente, acaso* + mood selection.
- `possessive-datives`: Inalienable possession (*me lavo las manos, se cortó el pelo*).
- `corrective-polarity`: Negated premises with corrective subjunctive.
- `participial-absolutes`: Participial clauses (*terminada la reunión, concluidos los análisis*).
- `scalar-concession`: Intensive concessions (*por más que, aun cuando, siquiera*).

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test generator_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/core/generator.rs src/core/mod.rs src/lib.rs tests/generator_tests.rs
git commit --no-gpg-sign -m "feat(core): implement combinatorial grammar question engine across all 24 concepts"
```

---

### Task 2: Curriculum Exercise-to-Drill Extractor

**Files:**
- Modify: `src/core/exercise.rs`
- Modify: `src/core/curriculum.rs`
- Create: `tests/extractor_tests.rs`

- [ ] **Step 1: Write failing tests in `tests/extractor_tests.rs`**

```rust
use spanglings::core::curriculum::load_curriculum;

#[test]
fn test_curriculum_exercises_convert_to_drill_items() {
    let curriculum = load_curriculum().expect("Failed to load curriculum");
    let mut total_drills = 0;
    for exercise in &curriculum.exercises {
        let drills = exercise.to_drill_items();
        total_drills += drills.len();
    }
    assert!(total_drills >= 300, "Should extract at least 300 drill items from curriculum, found {}", total_drills);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test extractor_tests`
Expected: FAIL

- [ ] **Step 3: Implement `Exercise::to_drill_items` in `src/core/exercise.rs`**

Extract drillable items by pairing test case expectations, context sentences, and exercise solutions into `DrillItem`s with accurate topic, prompt, target, and explanations.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test extractor_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/core/exercise.rs src/core/curriculum.rs tests/extractor_tests.rs
git commit --no-gpg-sign -m "feat(core): implement curriculum exercise-to-drill extraction engine"
```

---

### Task 3: Adaptive Weakness-Driven Drill Engine & CLI Flags

**Files:**
- Modify: `src/cli/mod.rs`
- Modify: `src/cli/commands/drill.rs`
- Modify: `src/cli/commands/blitz.rs`
- Modify: `tests/drill_tests.rs`
- Modify: `tests/blitz_tests.rs`

- [ ] **Step 1: Write failing tests in `tests/drill_tests.rs`**

```rust
#[test]
fn test_adaptive_weakness_drill_selection() {
    let mut state = AppState::default();
    state.update_concept_mastery("subjunctive", 1, chrono::Utc::now()); // weak
    state.update_concept_mastery("por-para", 5, chrono::Utc::now());     // strong

    let items = select_drill_items(&state, DrillFilter {
        weak_only: true,
        topic: None,
        level: None,
        track: None,
        count: 5,
    });

    assert_eq!(items.len(), 5);
    // Subjunctive should appear frequently due to low mastery
    assert!(items.iter().any(|i| i.topic == "subjunctive"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test drill_tests test_adaptive_weakness_drill_selection`
Expected: FAIL

- [ ] **Step 3: Update `src/cli/mod.rs`, `drill.rs`, and `blitz.rs`**

Add `--weak`, `--topic`, `--level`, `--track`, and `--count` flags to `Commands::Drill` and `Commands::Blitz`.
Integrate with `generator.rs` and `exercise.rs` for infinite sampling.
During drill runs, update `state.concept_mastery` in real-time and save `state.json`.
Print post-session mastery delta summary.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test drill_tests && cargo test --test blitz_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/cli/mod.rs src/cli/commands/drill.rs src/cli/commands/blitz.rs tests/drill_tests.rs tests/blitz_tests.rs
git commit --no-gpg-sign -m "feat(drill): add adaptive weakness sampling, CLI filters, and live mastery delta reporting"
```

---

### Task 4: TUI Concept Mastery & Weakness Dashboard Modal (`[m]`)

**Files:**
- Modify: `src/tui/app.rs`
- Modify: `src/tui/ui.rs`
- Modify: `tests/tui_tests.rs`

- [ ] **Step 1: Write failing tests in `tests/tui_tests.rs`**

```rust
#[test]
fn test_tui_concept_mastery_dashboard_modal_navigation_and_rendering() {
    let exercises = get_test_exercises();
    let mut app = App::new(exercises, false);

    assert!(!app.show_mastery_dashboard);
    app.handle_key(crossterm::event::KeyEvent::from(crossterm::event::KeyCode::Char('m')));
    assert!(app.show_mastery_dashboard);

    // Verify rendering does not panic
    let backend = ratatui::backend::TestBackend::new(120, 40);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();
    terminal.draw(|f| app.draw(f)).unwrap();
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test tui_tests test_tui_concept_mastery_dashboard_modal_navigation_and_rendering`
Expected: FAIL

- [ ] **Step 3: Update `src/tui/app.rs` and `src/tui/ui.rs`**

In `src/tui/app.rs`:
- Add `show_mastery_dashboard: bool`, `mastery_selected_idx: usize`.
- Handle `[m]` key toggle, `[j]`/`[k]` / `[↑]`/`[↓]` navigation.
- Handle `[d]` to launch focused drill, `[w]` for weakness drill, `[r]` / `[Enter]` to open reference card.

In `src/tui/ui.rs`:
- Implement `draw_mastery_dashboard_modal(f, app, area)`.
- Render overall progress gauge and 24-concept table with styled progress bars (`[████████░░░░] 65%`), review counters, and lapse badges.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test tui_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/tui/app.rs src/tui/ui.rs tests/tui_tests.rs
git commit --no-gpg-sign -m "feat(tui): implement Concept Mastery & Weakness Dashboard modal with hotkey drill triggers"
```

---

### Task 5: End-to-End Workspace Verification & Knowledge Graph Update

- [ ] **Step 1: Run complete workspace test suite**
Run: `cargo test`
Expected: 100% passing (0 failures).

- [ ] **Step 2: Run compiler linter & formatter**
Run: `cargo clippy --all-targets -- -D warnings && cargo fmt --check`
Expected: 0 warnings, clean formatting.

- [ ] **Step 3: Update knowledge graph**
Run: `uvx --from graphifyy graphify update .`

- [ ] **Step 4: Commit knowledge graph**
```bash
git add graphify-out/
git commit --no-gpg-sign -m "docs: update knowledge graph" || true
```
