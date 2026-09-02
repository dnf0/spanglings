# Spanglings Targeted Language Manual, Bidirectional Deep-Linking & Rapid Arcade Fix

## 1. Problem Statement
1. **Software Manual Bias in Documentation**: The current Spanglings documentation (`docs/index.md`, `docs/cli-reference.md`, `docs/onboarding-guide.md`) reads heavily like a developer tooling/software architecture guide instead of a targeted, pedagogical Spanish Language Manual. Users seeking to master challenging Spanish grammatical areas (*por vs para*, *ser vs estar*, subjunctive triggers, clitic doubling, accidental *se*, false friends) need rich linguistic explanations, contrast tables, mental models, and decision matrices.
2. **Rapid Arcade Arena Mode Inactive in Browser**: In `docs/assets/playground/playground.css`, the active arcade mode CSS selector targets `.playground-container.arcade-mode` rather than `.spanglings-playground.arcade-mode` / `#spanglings-app.arcade-mode`. As a result, switching tabs to the Rapid Arcade Arena does not hide curriculum editor panes or display `#arcade-arena-container`.
3. **Lack of Bidirectional Playground <-> Documentation Linking**: Learners in the documentation cannot jump directly to practice a specific topic in the interactive playground or arcade duel, and learners in the playground cannot jump directly to read the comprehensive grammatical reference in the manual.
4. **Scope Consolidation**: The project needs a streamlined focus centering strictly around the rich Spanish Language Manual and the Dual Interactive Playground (Monaco Syntax Studio + Rapid Arcade Arena).

---

## 2. Architecture & Design

### 2.1 Documentation Structure (`docs/`)
We will transform the documentation into a clean, developer-tailored Spanish Language Learning platform:

1. **`docs/index.md` (Overview & Pedagogical Architecture)**:
   - High-level introduction to the **Syntax Compiler Model** of Spanish language acquisition.
   - CEFR Level Progression overview (A1 Foundation -> C1 Mastery).
   - Prominent launch banner to the **Standalone Interactive Web Playground**.
   - Navigation links to the Comprehensive Language Manual and Curriculum Syllabus.
2. **`docs/manual.md` (The Comprehensive Spanish Language Manual)**:
   - Covers all 24 core curriculum topics organized into three clear CEFR tiers:
     - **Tier 1: Foundations & Aspectual Geometry (A1–A2)**:
       1. *Ser vs Estar* (Essence vs State & Adjective Semantic Shifts)
       2. *Por vs Para* (Origin/Cause Vector vs Destination/Goal Vector)
       3. *Pretérito vs Imperfecto* (Aspectual Boundaries: Completed Action vs Continuous Background)
       4. Direct & Indirect Clitics (*Lo/La/Le* Distinction & Clitic Doubling Matrix)
       5. *Gustar* & Inverted Psychological Predicates
       6. Reflexive & Pronominal Verbs (True Reflexive vs Inherent Pronominal)
       7. Stem-Changing Verbs (E->IE, O->UE, E->I, U->UE Mechanics)
       8. Prepositional Geometry & Mandatory Contractions (*a, de, en, con, sin*)
     - **Tier 2: Mood, Triggers & Pragmatic Voice (B1–B2)**:
       9. Present Subjunctive & Matrix Triggers (WEIRDOS Mnemonic & Decision Matrix)
       10. Imperfect Subjunctive & Hypothetical *Si* Clauses (Conditional Pairings)
       11. Imperative Mood & Clitic Attachment Rules (Affirmative Enclitic vs Negative Proclitic)
       12. Accidental & Unintentional *Se* Construction (Blameless Verb Architecture)
       13. Passive *Se* vs Impersonal *Se* (Agreement with Patient vs Fixed Singular)
       14. Possessive Datives (Inalienable Body Parts & Articles)
       15. Relative Pronouns & Clauses (*que, quien, el cual, cuyo*)
       16. Gerund Mechanics & Prohibited Adjectival Gerunds (Adverbial Circumstance)
     - **Tier 3: Advanced Nuance, Registers & Edge Mechanics (B2–C1)**:
       17. Verbs of Becoming (*hacerse, ponerse, volverse, quedarse, convertirse en*)
       18. Scalar Concession & Connectors (*aunque, a pesar de que, por mucho que*)
       19. Epistemic Conjecture & Probability Futures (*será, estaría, habrá sido*)
       20. Corrective Polarity (*pero vs sino vs sino que*)
       21. False Friends & Cognitive Traps (*actualmente, realizar, soportar, constipado*)
       22. Voseo Mechanics (Rioplatense & Central American Morphological Paradigms)
       23. Technical, Software & Data Collocations (*desplegar, fallar, compilar, enrutar*)
       24. Professional & Legal Subjunctive Forms (*-ere* Archaisms & Epistolary Formulas)
3. **`docs/syllabus.md` (Curriculum Syllabus & Practice Directory)**:
   - Complete index of all 24 topics, 136 curriculum frames, 16 showdown pairs, and 5 specialized drill engines.
   - Deep links to the corresponding manual section and direct practice links in the playground.
4. **`mkdocs.yml` (Streamlined Navigation)**:
   - Clean navigation bar:
     - Overview (`index.md`)
     - Spanish Language Manual (`manual.md`)
     - Curriculum Syllabus (`syllabus.md`)
     - Interactive Playground (`playground/index.html`)
     - Contributing (`contributing.md`)

---

### 2.2 Content Template for Each Manual Topic Section
Every topic section in `docs/manual.md` will strictly follow the Spanglings dual-layer standard:

```markdown
### <Topic Title> (e.g. 1. Ser vs Estar)
**CEFR Level**: A1 | **Prerequisites**: None | **Track**: `ser-estar`

#### 💡 Communicative Mental Model
[Intuitive spatial/psychological context explaining the native speaker perspective]

#### 📐 Strict Grammar Rules & Decision Matrix
| Trigger / Context | Verb / Form | Minimal Example | English Translation |
| :--- | :--- | :--- | :--- |
| Characteristic / Identity | *Ser* | *Es ingeniero* | He is an engineer |
| Temporary State / Condition | *Estar* | *Está cansado* | He is tired |
| Semantic Adjective Shift | *Ser listo* vs *Estar listo* | *Es listo* / *Está listo* | He is smart / He is ready |

#### ⚠️ High-Frequency Pitfalls & Traps
[Common English speaker mistakes and how to avoid them]

#### ⚡ Interactive Practice
- [▶ Practice in Curriculum Syntax Studio](../playground/?topic=ser-estar)
- [⚔ Battle in Rapid Showdown Duel](../playground/?mode=arcade&topic=ser-vs-estar)
```

---

### 2.3 Rapid Arcade Arena Fix & Bidirectional Deep-Linking

1. **CSS Selector Alignment**:
   In `docs/assets/playground/playground.css`:
   - Change `.playground-container.arcade-mode ...` to `.spanglings-playground.arcade-mode .syllabus-pane, .spanglings-playground.arcade-mode .editor-pane, .spanglings-playground.arcade-mode .diagnostics-pane { display: none; }` and `.spanglings-playground.arcade-mode .arcade-arena-container { display: flex; }`.
   - Ensure `#arcade-arena-container` occupies the full container height with proper vertical scrolling and centering.

2. **Playground Deep-Linking & URL Parameter Handling**:
   In `docs/assets/playground/playground.js`:
   - Parse `window.location.search` on startup:
     - `?mode=arcade`: Switch directly to Arcade tab and trigger `renderArcadeView()`.
     - `?topic=<slug>`: If in curriculum mode, select the topic and first frame matching the slug. If in arcade mode, set `arcadeEngine.mode = <slug>` and start the round or select the option in the dropdown.
     - `?exercise=<id>`: Select specific exercise ID.
   - Add a `📖 View in Manual` link in the topic cheat sheet drawer and diagnostics pane pointing to `../manual/#<topic-slug>`.

3. **Header Link in Standalone Shell**:
   In `docs/playground/index.html`:
   - Update the top navigation button to point directly to `../manual/` (`📘 Language Manual`).

---

## 3. Verification & Testing Strategy

1. **Python Unit Tests (`tests/test_docs_playground.py` & `tests/test_playground_ui.py`)**:
   - Verify `docs/manual.md` exists and contains all 24 topic headings, mental models, grammar rules, tables, and playground deep-links.
   - Verify `mkdocs.yml` navigation includes `manual.md` and `playground/index.html`.
   - Verify CSS selectors for `.spanglings-playground.arcade-mode` in `playground.css`.
   - Verify URL parameter handling for `?mode=arcade` and `?topic=...` in `playground.js`.
2. **Automated Documentation Build**:
   - `uv run mkdocs build --strict` with zero warnings or broken links.
3. **Full Rust & Python Test Suite**:
   - `cargo test --all-targets` (all unit & integration tests pass).
   - `uv run pytest` (all test suites pass).
   - `uv run ruff check` and `uv run pyright` pass cleanly.
