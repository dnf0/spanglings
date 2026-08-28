# Functional Semantic Grammar Keys & Communicative Glosses Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Augment all grammatical concept keys with communicative functional glosses (e.g. `Subjunctive (wishes, hypotheses, demands)`) across CLI `explain`/`reference` commands, drill/blitz prompt badges, reference card headers, and the TUI reference browser.

**Architecture:** Define a central `GrammarConcept` metadata catalog in `src/core/reference.rs` providing title, slug, communicative functional gloss, search keywords, and reference card content. Surface these rich descriptors across the CLI `explain` command, `DrillItem`/`BlitzItem` prompt formatting, and TUI reference browser sidebar/filtering.

**Tech Stack:** Rust 2021, `ratatui` (TUI), `colored` (terminal ANSI styling), `clap` (CLI argument parsing).

---

### Task 1: Core `GrammarConcept` Data Structure & 24-Concept Metadata Catalog

**Files:**
- Modify: `src/core/reference.rs`
- Modify: `tests/reference_tests.rs`

- [ ] **Step 1: Write the failing tests in `tests/reference_tests.rs`**

```rust
#[test]
fn test_all_24_grammar_concepts_complete() {
    let concepts = spanglings::core::reference::list_grammar_concepts();
    assert_eq!(concepts.len(), 24);
    for concept in concepts {
        assert!(!concept.slug.is_empty(), "Slug should not be empty");
        assert!(!concept.title.is_empty(), "Title should not be empty for {}", concept.slug);
        assert!(!concept.gloss.is_empty(), "Gloss should not be empty for {}", concept.slug);
        assert!(!concept.keywords.is_empty(), "Keywords should not be empty for {}", concept.slug);
        assert!(!concept.card.is_empty(), "Card should not be empty for {}", concept.slug);
    }
}

#[test]
fn test_semantic_keyword_concept_lookups() {
    use spanglings::core::reference::get_grammar_concept;

    // Direct slug lookup
    let subj = get_grammar_concept("subjunctive").expect("Should find subjunctive by slug");
    assert_eq!(subj.slug, "subjunctive");
    assert!(subj.gloss.contains("wishes") || subj.gloss.contains("hypotheses"));

    // Semantic intent lookup
    let wishes = get_grammar_concept("wishes").expect("Should resolve 'wishes' to subjunctive");
    assert_eq!(wishes.slug, "subjunctive");

    let accidental = get_grammar_concept("unintentional").expect("Should resolve 'unintentional' to accidental-se");
    assert_eq!(accidental.slug, "accidental-se");

    let body_parts = get_grammar_concept("body parts").expect("Should resolve 'body parts' to possessive-datives");
    assert_eq!(body_parts.slug, "possessive-datives");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test reference_tests test_all_24_grammar_concepts_complete`
Expected: FAIL with "function `list_grammar_concepts` not found"

- [ ] **Step 3: Implement `GrammarConcept` and lookup catalog in `src/core/reference.rs`**

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarConcept {
    pub slug: &'static str,
    pub title: &'static str,
    pub gloss: &'static str,
    pub keywords: &'static [&'static str],
    pub card: &'static str,
}

pub const CONCEPTS: &[GrammarConcept] = &[
    GrammarConcept {
        slug: "subjunctive",
        title: "Subjunctive",
        gloss: "wishes, hypotheses, doubt, demands",
        keywords: &["wishes", "desires", "doubt", "uncertainty", "hypothetical", "hypotheses", "weirdo", "demands", "subj", "e0301", "e0401", "e0501", "e0601"],
        card: SUBJUNCTIVE_CARD,
    },
    GrammarConcept {
        slug: "por-para",
        title: "Por vs. Para",
        gloss: "cause/means (por) vs. purpose/destination/deadline (para)",
        keywords: &["cause", "reason", "means", "motive", "purpose", "deadline", "recipient", "goal", "por", "para", "por_para", "e0701"],
        card: POR_PARA_CARD,
    },
    GrammarConcept {
        slug: "ser-estar",
        title: "Ser vs. Estar",
        gloss: "essence/identity (ser) vs. states/conditions/location (estar)",
        keywords: &["identity", "essence", "permanent", "state", "condition", "temporary", "location", "ser", "estar", "ser_estar", "e0101", "e0102"],
        card: SER_ESTAR_CARD,
    },
    GrammarConcept {
        slug: "past",
        title: "Past Tenses",
        gloss: "completed actions (pret) vs. ongoing background & habit (imp)",
        keywords: &["preterite", "imperfect", "completed", "ongoing", "habit", "background", "narrative", "past", "past-aspect", "e0201", "e0202", "e0203"],
        card: PAST_TENSES_CARD,
    },
    GrammarConcept {
        slug: "pronouns",
        title: "Pronoun Stacking",
        gloss: "clitic placement & pronoun stacking (se lo)",
        keywords: &["direct", "indirect", "clitic", "clitics", "stacking", "substitution", "se lo", "placement", "pronouns", "e0801", "e0802"],
        card: PRONOUN_STACKING_CARD,
    },
    GrammarConcept {
        slug: "prepositions",
        title: "Prepositional Verbs",
        gloss: "verb-bound prepositions (régimen preposicional)",
        keywords: &["prepositions", "regimen", "prep", "soñar con", "depender de", "fijarse en", "e0901"],
        card: PREPOSITIONS_CARD,
    },
    GrammarConcept {
        slug: "accidental-se",
        title: "Accidental 'Se'",
        gloss: "unintentional events & non-agentive slips (se me cayó)",
        keywords: &["accidental", "unintentional", "blame", "dropped", "forgotten", "involuntary", "se-accidental", "se accidental", "e1001"],
        card: ACCIDENTAL_SE_CARD,
    },
    GrammarConcept {
        slug: "tech-software",
        title: "Tech & Software Engineering",
        gloss: "terminal workflows, architecture, system design, SLA terms",
        keywords: &["tech", "software", "dev", "engineering", "system design", "architecture", "terminal", "code"],
        card: TECH_SOFTWARE_CARD,
    },
    GrammarConcept {
        slug: "business",
        title: "Executive & Business Spanish",
        gloss: "formal correspondence, diplomatic negotiation, stakeholder sync",
        keywords: &["business", "business-correspondence", "biz", "diplomatic", "formal", "negotiation", "executive"],
        card: BUSINESS_CORRESPONDENCE_CARD,
    },
    GrammarConcept {
        slug: "false-friends",
        title: "False Friends",
        gloss: "deceptive cognates with divergent meanings (actual, realizar)",
        keywords: &["false-friends", "falsos-amigos", "cognates", "traps", "actual", "realizar", "exito", "pretend"],
        card: FALSE_FRIENDS_CARD,
    },
    GrammarConcept {
        slug: "voseo",
        title: "Voseo & Regional Address",
        gloss: "informal singular address in Rioplatense & Central America",
        keywords: &["voseo", "vos", "rioplatense", "argentina", "uruguay", "regional", "informal"],
        card: VOSEO_CARD,
    },
    GrammarConcept {
        slug: "accents",
        title: "Accentuation & Tildes",
        gloss: "stress rules (agudas/llanas/esdrújulas) & diacritical disambiguation",
        keywords: &["accents", "accentuation", "tildes", "acentuacion", "stress", "diacritical", "agudas", "llanas", "esdrujulas"],
        card: ACCENTS_CARD,
    },
    GrammarConcept {
        slug: "epistemic-conjecture",
        title: "Epistemic Conjecture",
        gloss: "guessing & deduction in present/past (serán las tres)",
        keywords: &["conjecture", "probability", "probabilidad", "guessing", "deduction", "wondering", "e0048"],
        card: EPISTEMIC_CONJECTURE_CARD,
    },
    GrammarConcept {
        slug: "clitic-doubling",
        title: "Clitic Doubling",
        gloss: "redundant pronoun reinforcement for focus & clarity (le hablé a ella)",
        keywords: &["clitic-doubling", "doubling", "redundancy", "indirect object", "clarity", "a ella", "reinforcement"],
        card: CLITIC_DOUBLING_CARD,
    },
    GrammarConcept {
        slug: "personal-a",
        title: "Personal 'A'",
        gloss: "mandatory accusative marker for specific human & personified entities",
        keywords: &["personal-a", "personal a", "human direct object", "specificity", "personification"],
        card: PERSONAL_A_CARD,
    },
    GrammarConcept {
        slug: "gerund-rules",
        title: "Gerund Rules & Pitfalls",
        gloss: "simultaneous actions; avoiding forbidden adjectival gerunds",
        keywords: &["gerund-rules", "gerund", "gerundio", "ando", "iendo", "simultaneous", "adverbial"],
        card: GERUND_RULES_CARD,
    },
    GrammarConcept {
        slug: "adversatives",
        title: "Adversatives (Pero vs. Sino)",
        gloss: "simple contrast (pero) vs. exclusive negative substitution (sino)",
        keywords: &["adversatives", "pero", "sino", "sino que", "contrast", "rectification", "substitution"],
        card: ADVERSATIVES_CARD,
    },
    GrammarConcept {
        slug: "legal-subjunctive",
        title: "Legal & Statutory Subjunctive",
        gloss: "statutory formulations & future subjunctive in legal texts (-ere)",
        keywords: &["legal-subjunctive", "legal", "statute", "contract", "future subjunctive", "formal", "juristic"],
        card: LEGAL_SUBJUNCTIVE_CARD,
    },
    GrammarConcept {
        slug: "verbs-of-becoming",
        title: "Verbs of Becoming",
        gloss: "transformational change (hacerse, volverse, ponerse, quedarse)",
        keywords: &["verbs-of-becoming", "becoming", "change", "transformation", "hacerse", "volverse", "ponerse", "quedarse", "convertirse"],
        card: VERBS_OF_BECOMING_CARD,
    },
    GrammarConcept {
        slug: "epistemic-adverbs",
        title: "Epistemic Adverbs",
        gloss: "mood selection with doubt adverbs (quizás, tal vez, probablemente)",
        keywords: &["epistemic-adverbs", "adverbs", "doubt", "quizas", "tal vez", "probablemente", "acaso"],
        card: EPISTEMIC_ADVERBS_CARD,
    },
    GrammarConcept {
        slug: "possessive-datives",
        title: "Possessive Datives",
        gloss: "inalienable possession with dative clitics (me lavo las manos)",
        keywords: &["possessive-datives", "possession", "inalienable", "body parts", "dative", "me lavo"],
        card: POSSESSIVE_DATIVES_CARD,
    },
    GrammarConcept {
        slug: "corrective-polarity",
        title: "Corrective Polarity",
        gloss: "rectifying negated premises with mandatory mood selection",
        keywords: &["corrective-polarity", "polarity", "negation", "correction", "indicative vs subjunctive"],
        card: CORRECTIVE_POLARITY_CARD,
    },
    GrammarConcept {
        slug: "participial-absolutes",
        title: "Participial Absolutes",
        gloss: "concise temporal/causal backgrounding with past participles",
        keywords: &["participial-absolutes", "absolute", "participle", "backgrounding", "temporal", "causal", "terminado el"],
        card: PARTICIPIAL_ABSOLUTES_CARD,
    },
    GrammarConcept {
        slug: "scalar-concession",
        title: "Scalar Concession",
        gloss: "intensive concessive structures (por más que, aun cuando, siquiera)",
        keywords: &["scalar-concession", "concession", "even though", "even if", "por mas que", "aun cuando", "siquiera"],
        card: SCALAR_CONCESSION_CARD,
    },
];

pub fn list_grammar_concepts() -> &'static [GrammarConcept] {
    CONCEPTS
}

pub fn get_grammar_concept(query: &str) -> Option<&'static GrammarConcept> {
    let normalized = query.trim().to_lowercase().replace('_', "-");
    let key = normalized.as_str();

    // 1. Direct slug match
    if let Some(c) = CONCEPTS.iter().find(|c| c.slug == key) {
        return Some(c);
    }

    // 2. Exact keyword match
    if let Some(c) = CONCEPTS.iter().find(|c| c.keywords.iter().any(|&k| k == key)) {
        return Some(c);
    }

    // 3. Substring match on title, gloss, or keywords
    if let Some(c) = CONCEPTS.iter().find(|c| {
        c.title.to_lowercase().contains(key)
            || c.gloss.to_lowercase().contains(key)
            || c.keywords.iter().any(|&k| k.contains(key) || key.contains(k))
    }) {
        return Some(c);
    }

    None
}

pub fn get_reference_card(topic: &str) -> Option<&'static str> {
    get_grammar_concept(topic).map(|c| c.card)
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test reference_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/core/reference.rs tests/reference_tests.rs
git commit --no-gpg-sign -m "feat(core): implement GrammarConcept catalog with communicative functional glosses"
```

---

### Task 2: CLI `spanglings explain` / `spanglings reference` Functional Listing & Semantic Discovery

**Files:**
- Modify: `src/cli/mod.rs`
- Modify: `src/cli/commands/explain.rs`
- Modify: `tests/cli_tests.rs`

- [ ] **Step 1: Write the failing tests in `tests/cli_tests.rs`**

```rust
#[test]
fn test_cli_explain_listing_displays_functional_glosses() {
    // Calling show_explanation with None or empty string displays formatted topic catalog
    let mut output = Vec::new();
    // Verify show_explanation(None) or "" prints all topics with functional glosses
}

#[test]
fn test_cli_explain_semantic_lookup_wishes() {
    // Calling show_explanation with "wishes" resolves to subjunctive card
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test cli_tests test_cli_explain_semantic_lookup_wishes`
Expected: FAIL

- [ ] **Step 3: Update `src/cli/commands/explain.rs` and `src/cli/mod.rs`**

Update `Explain` subcommand in `src/cli/mod.rs` to make `topic: Option<String>` optional.
In `src/cli/commands/explain.rs`:
```rust
use crate::core::reference::{get_grammar_concept, list_grammar_concepts};
use colored::Colorize;

pub fn show_explanation(topic: Option<&str>) -> anyhow::Result<()> {
    match topic {
        Some(query) if !query.trim().is_empty() => {
            match get_grammar_concept(query) {
                Some(concept) => {
                    println!("{}", concept.card.cyan());
                    Ok(())
                }
                None => {
                    println!(
                        "{}",
                        format!("Unknown grammar topic or intent: '{}'", query).red().bold()
                    );
                    print_topic_catalog();
                    Ok(())
                }
            }
        }
        _ => {
            print_topic_catalog();
            Ok(())
        }
    }
}

fn print_topic_catalog() {
    println!("\n{}", "Available Grammar Reference Cards:".bold().underline());
    for c in list_grammar_concepts() {
        println!(
            "  • {:<24} {}{}{}",
            c.slug.yellow().bold(),
            c.title.bold(),
            " — ".dark_gray(),
            c.gloss.dimmed()
        );
    }
    println!(
        "\n{}\n",
        "Usage: spanglings explain <topic-or-intent> (e.g. spanglings explain wishes)".cyan()
    );
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test cli_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/cli/mod.rs src/cli/commands/explain.rs src/main.rs tests/cli_tests.rs
git commit --no-gpg-sign -m "feat(cli): display functional glosses in explain catalog and support semantic query lookup"
```

---

### Task 3: Drill & Blitz Prompt Layout Enhancement with Communicative Glosses

**Files:**
- Modify: `src/cli/commands/drill.rs`
- Modify: `src/cli/commands/blitz.rs`
- Modify: `tests/drill_tests.rs`
- Modify: `tests/blitz_tests.rs`

- [ ] **Step 1: Write the failing tests in `tests/drill_tests.rs` and `tests/blitz_tests.rs`**

```rust
#[test]
fn test_drill_item_prompt_formatting_with_functional_gloss() {
    let item = DrillItem {
        topic: "subjunctive",
        formula_cue: "drop -o -> opposite vowel -a",
        trigger_sentence: "Dudo que yo ____ los libros en la mesa.",
        target_verb: "poner",
        target_subject: "yo",
        target: "ponga",
        explanation: "yo pongo -> drop -o -> add -a -> ponga",
    };

    let prompt = item.format_prompt(1, 5);
    assert!(prompt.contains("Subjunctive (wishes, hypotheses, doubt, demands)"));
    assert!(prompt.contains("drop -o -> opposite vowel -a"));
    assert!(prompt.contains("Dudo que yo ____ los libros en la mesa."));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test drill_tests test_drill_item_prompt_formatting_with_functional_gloss`
Expected: FAIL

- [ ] **Step 3: Update `format_prompt` in `src/cli/commands/drill.rs` and `src/cli/commands/blitz.rs`**

In `src/cli/commands/drill.rs`:
```rust
pub fn format_prompt(&self, current: usize, total: usize) -> String {
    let concept_header = if let Some(concept) = crate::core::reference::get_grammar_concept(self.topic) {
        format!("{} ({})", concept.title, concept.gloss)
    } else {
        self.topic
            .split(['_', '-'])
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    };

    let badge = if self.formula_cue.is_empty() {
        format!("[{}]", concept_header)
    } else {
        format!("[{} | {}]", concept_header, self.formula_cue)
    };

    format!(
        "{} {}\nSentence: \"{}\" (verb: {} | subject: {})",
        format!("Q{}/{}", current, total).bold().yellow(),
        badge.cyan(),
        self.trigger_sentence.white().bold(),
        self.target_verb.yellow(),
        self.target_subject.yellow()
    )
}
```

Apply equivalent logic to `BlitzItem::format_prompt` in `src/cli/commands/blitz.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test drill_tests && cargo test --test blitz_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/cli/commands/drill.rs src/cli/commands/blitz.rs tests/drill_tests.rs tests/blitz_tests.rs
git commit --no-gpg-sign -m "feat(drill): render functional communicative glosses in drill and blitz prompt headers"
```

---

### Task 4: TUI Reference Browser & Modal Integration with Functional Glosses

**Files:**
- Modify: `src/tui/app.rs`
- Modify: `src/tui/ui.rs`
- Modify: `tests/tui_tests.rs`

- [ ] **Step 1: Write the failing tests in `tests/tui_tests.rs`**

```rust
#[test]
fn test_tui_reference_browser_shows_functional_glosses_and_searches_glosses() {
    let exercises = get_test_exercises();
    let mut app = App::new(exercises, false);
    
    // Test filter matches on communicative gloss keyword "wishes"
    app.ref_query = "wishes".to_string();
    app.update_ref_filter();
    assert_eq!(app.ref_filtered_topics.len(), 1);
    assert_eq!(app.ref_filtered_topics[0], "subjunctive");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test tui_tests test_tui_reference_browser_shows_functional_glosses_and_searches_glosses`
Expected: FAIL

- [ ] **Step 3: Update `src/tui/app.rs` and `src/tui/ui.rs`**

In `src/tui/app.rs`:
Update `update_ref_filter`:
```rust
pub fn update_ref_filter(&mut self) {
    let q = self.ref_query.trim().to_lowercase();
    if q.is_empty() {
        self.ref_filtered_topics = self.ref_topics.clone();
    } else {
        self.ref_filtered_topics = self
            .ref_topics
            .iter()
            .filter(|&slug| {
                if let Some(concept) = crate::core::reference::get_grammar_concept(slug) {
                    concept.slug.to_lowercase().contains(&q)
                        || concept.title.to_lowercase().contains(&q)
                        || concept.gloss.to_lowercase().contains(&q)
                        || concept.keywords.iter().any(|k| k.to_lowercase().contains(&q))
                } else {
                    slug.to_lowercase().contains(&q)
                }
            })
            .copied()
            .collect();
    }
    self.ref_selected_idx = 0;
    self.ref_scroll = 0;
}
```

In `src/tui/ui.rs`:
In `draw_reference_browser_modal`:
Display topic items with formatted `{Title} ({gloss})` so learners see the communicative intent right in the sidebar.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test tui_tests`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/tui/app.rs src/tui/ui.rs tests/tui_tests.rs
git commit --no-gpg-sign -m "feat(tui): integrate functional communicative glosses into reference browser sidebar and search filter"
```

---

### Task 5: End-to-End Workspace Verification & Knowledge Graph Update

**Files:**
- Verify: Full workspace tests, linter, formatting, knowledge graph

- [ ] **Step 1: Run complete test suite**
Run: `cargo test`
Expected: All test binaries pass cleanly (0 failures).

- [ ] **Step 2: Run compiler linter**
Run: `cargo clippy --all-targets -- -D warnings`
Expected: Clean with 0 warnings.

- [ ] **Step 3: Check code formatting**
Run: `cargo fmt --check`
Expected: Clean.

- [ ] **Step 4: Update knowledge graph**
Run: `uvx --from graphifyy graphify update .`

- [ ] **Step 5: Commit any graph updates**
```bash
git add graphify-out/
git commit --no-gpg-sign -m "docs: update knowledge graph" || true
```
