# Design Specification: Informative & Non-Spoiling Exercise TODOs

## 1. Context & Motivation
Spanglings provides a full-spectrum curriculum spanning 60 tracks and 339 exercises (from A1 Baseline through C1–C2 advanced pragmatics and technical domains).

Currently, exercise markdown files contain a high-level `Grammar Rule` banner, an English translation in `Context`, and a prompt sentence with a blank `___` placeholder. While functional, exercises lack explicit, per-exercise **TODO task descriptions** detailing:
1. **WHAT**: Precisely what grammatical operation, conjugation, inflection, or lexical choice is being requested.
2. **WHY**: The linguistic constraint, trigger, agreement rule, or pragmatic convention mandating that specific form.

Crucially, as per project requirements, the TODO instructions must **NEVER spoil the exact answer token/phrase**. The solution belongs strictly inside the internal `<!-- SOLUTION -->` block and tiered progressive hints.

---

## 2. Standardized Exercise File Template

Every exercise markdown file across all 60 tracks will adhere to the following schema:

```markdown
# Track Title: Exercise Subtitle
<!-- id: [exercise_id] | level: [Level] | topic: [topic_id] | type: cloze | concepts: [...] | prerequisites: [...] | grammar_focus: "..." -->

> **Grammar Rule**: [High-level grammatical principle or track focus]

### Context
English: "[Full English translation giving situational context]"

### Instructions
**TODO**: [Actionable instruction without spoiling the answer]
**Why**: [Linguistic explanation of trigger, aspect, mood, agreement, or pragmatic constraint]

### Exercise
<!-- TODO: [One-line summary for in-editor watch mode without spoiling solution] -->
[Spanish sentence with ___ placeholder]

<!-- SOLUTION
[exact_solution]
-->

<!-- ALTERNATIVES
[valid_synonyms_or_orthographic_variants]
-->

<!-- DIAGNOSTIC_RULES
pattern: "[common_mistake]" | code: "E00XX" | message: "[Compiler diagnostic feedback]"
-->

<!-- HINTS
Tier 1: [Conceptual orientation hint]
Tier 2: [Structural / morphological hint]
Tier 3: [Direct solution hint]
-->
```

---

## 3. Strict Non-Spoiling Rules & Validation Policy

1. **Zero Direct Lexical Spoilers**:
   - The exact solution word/phrase (e.g. `quepo`, `estamos`, `fuera`, `desplegar`, `conmutación por error`) must **never** be written in the `TODO`, `Why`, or inline `<!-- TODO: ... -->` text.
   - Reference verbs using infinitives in parentheses, such as *(caber)*, *(tener)*, *(ser vs estar)*, or describe the required grammatical category (e.g. "1st-person plural imperfect subjunctive of the main verb").

2. **Pedagogical Actionability**:
   - The **TODO** must specify person, number, tense, mood, polarity, or prepositional regime when relevant.
   - The **Why** must articulate the syntactic trigger (e.g., matrix verb of influence with change of subject, impersonal expression of doubt, telic aspectual boundary, concessive conjunction with unverified event, formal register collocation).

3. **Automated Leakage & Quality Verification**:
   - A dedicated verification script will check all 339 files:
     - Verifying that `### Instructions`, `**TODO**:`, and `**Why**:` exist in every single exercise.
     - Ensuring `### Instructions` content is non-empty and contains at least 30 characters.
     - Ensuring zero occurrence of the `<!-- SOLUTION -->` string inside the `### Instructions` block or inline `<!-- TODO: -->` comment.

---

## 4. TUI, LSP & Tooling Enhancements

1. **TUI Prompt Card (`src/tui/ui.rs`)**:
   - Update `src/tui/ui.rs` to extract and display the `Instructions / TODO` block clearly under a dedicated header in the interactive terminal UI.
2. **Validator Compatibility (`src/engine/validator.rs`)**:
   - Ensure the validator continues to accurately identify the sentence line and ignores `### Instructions`, `**TODO**:`, `**Why**:`, and `<!-- TODO: ... -->`.
3. **LSP Diagnostics & Hover Card (`src/lsp/`)**:
   - Ensure hover cards and diagnostic line pointers seamlessly accommodate the added instruction lines.

---

## 5. Phased Execution Plan

The update covers 339 exercises across 60 tracks:
* **Batch 1 (Tracks 00–14, 80 exercises)**: Baseline, Ser vs Estar, Past Aspects, Subjunctive (WEIRDO, Relative, Conjunctions), Conditionals, Por vs Para, Clitics, Prepositions, Accidental Se, Pluperfect Subjunctive, Periphrases, Concessives, Connectors.
* **Batch 2 (Tracks 15–26, 61 exercises)**: Advanced Imperative, Relative Pronouns, Passive Voice, Cleft Sentences, Participles, Pasiva Refleja, Collocations, Tech/Software, Business/Diplomatic, False Friends, Register Elevation, Regional Contrasts.
* **Batch 3 (Tracks 27–41, 90 exercises)**: System Design, Advanced Subjunctive Clauses, Advanced Verbal Periphrases, Executive Leadership, Mexican Tech/Startups, Colombian Nuances, Rioplatense Voseo, LatAm Anglicism Elimination, SLA/Risk Governance, Everyday Life/Housing, Healthcare/Medical, Dining/Socializing, Prepositions/Locutions, Middle Voice/Reflexive Shifts, Adverbial Clauses.
* **Batch 4 (Tracks 42–59, 108 exercises)**: Practical living (Travel, Banking, Complaints, Maintenance, Media, Conversational Nuance), Epistemic Conjecture, Clitic Doubling, Personal A, Gerunds, Adversatives, Legal/Optative, Verbs of Becoming, Epistemic Adverbs, Datives of Possession/Ethic, Corrective Polarity, Participial Absolutes, Scalar Concession.
* **Batch 5**: TUI rendering update, validator check, automated leakage test, and test suite verification.
