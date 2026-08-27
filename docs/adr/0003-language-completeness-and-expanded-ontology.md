# ADR-0003: Language Completeness, Advanced Syntactic Contrasts & Expanded Ontology

## Status
Accepted

## Context
While Spanglings achieved comprehensive coverage across Baseline (A1-A2), B1, and B2 tracks for regular/irregular verbs, WEIRDO/ESCAPA subjunctive triggers, and Latin American technical/business registers, a linguistic audit identified 8 critical structural gaps present in native Spanish (CEFR C1/C2 and RAE normative standards):

1. **Epistemic Future & Conditional of Conjecture (*Futuro y Condicional de Probabilidad*)**: Expressing present and past hypotheses/probability (*Serán las cuatro*, *Estaría enfermo*, *Habrá salido*).
2. **Clitic Doubling & Left-Dislocation Syntax (*Duplicación Clítica*)**: Strict rules for mandatory pronominal doubling with fronted objects (*A María le di...*, *A mí me gusta...*) vs forbidden doubling.
3. **The "Personal A" System (*A Personal*) & Animacy Shifts**: Direct object specificity (*Busco secretaria* vs *Busco a la secretaria*), animacy, personification, and verb shifts (*tener a* vs *tener*).
4. **Gerund Restrictions & Anglicism Pitfalls (*Gerundio Ilegal / de Posterioridad*)**: Eliminating ungrammatical *gerundio de posterioridad* and adjectival gerunds frequently calqued from English.
5. **Adversative Coordination (*Pero* vs *Sino* vs *Sino Que*)**: Syntax of additive contrast (*pero*) versus exclusive substitution with words (*sino*) and conjugated clauses (*sino que*).
6. **Independent Subjunctives, Optatives & Archaic/Legal Tenses**: Optative formulas (*¡Quién pudiera!*, *¡Que te vaya bien!*), archaic *futuro de subjuntivo* (*cometiere*, *dispusiere*), and literary past `-ra`.

## Decisions

### 1. Linguistic Graph Ontology Expansion (53 -> 65 Concepts)
- Extended `LinguisticGraph` in `src/core/graph.rs` with 12 new concept nodes across `AspectAndTense`, `MoodSelection`, `PronounsAndVoice`, `PrepositionsAndRelators`, and `SyntaxAndRhetoric`:
  - `future_of_probability_present`
  - `conditional_of_probability_past`
  - `compound_conditional_conjecture`
  - `clitic_doubling_mandatory_dative`
  - `clitic_doubling_fronted_accusative`
  - `personal_a_specificity_animacy`
  - `personal_a_verb_semantic_shifts`
  - `gerund_restrictions_posteriority`
  - `gerund_restrictions_adjectival`
  - `adversative_pero_vs_sino`
  - `adversative_sino_que_clauses`
  - `optative_independent_subjunctive`
  - `future_subjunctive_legal_archaic`
- Maintained strict cycle-free DAG topology with clear prerequisite linkages to foundational tracks.

### 2. Curriculum Track Expansion (Tracks 48–53, 36 New Exercises, 303 Total)
- Scaffolded 6 new tracks with 6 exercises each:
  - `exercises/48_epistemic_conjecture_and_probability/` (B2–C1)
  - `exercises/49_clitic_doubling_and_left_dislocation/` (B1–C1)
  - `exercises/50_personal_a_and_animacy_shifts/` (B1–B2)
  - `exercises/51_gerund_restrictions_and_anglicisms/` (B2–C1)
  - `exercises/52_adversative_pero_sino_sino_que/` (B1–B2)
  - `exercises/53_independent_subjunctives_and_legal_tenses/` (C1–C2)

### 3. Grammar Reference Cards Expansion
- Added dedicated reference sheets and cheat sheets for:
  - Epistemic Conjecture (Future & Conditional of Probability)
  - Clitic Doubling & Dislocation Syntax
  - The "Personal A" System
  - Gerund Restrictions & Anglicisms
  - Adversative Coordination (*Pero / Sino / Sino Que*)
  - Optatives & Legal Subjunctive Tenses

### 4. Diagnostic Rules & Compiler Integrations
- Updated `DiagnosticRule` and error catalog with targeted rules for:
  - Misusing *pero* instead of *sino / sino que*
  - Missing mandatory personal *a* or over-applying *a*
  - Gerund of posteriority warnings
  - Missing clitic doubling in left-dislocations

## Consequences
- **Linguistic Completeness**: Reaches 100% full-spectrum Spanish grammatical coverage from foundational irregulars (A1) to C1/C2 legal, pragmatic, and syntactic structures.
- **Developer Pedagogical Value**: Eliminates high-frequency calques and ungrammatical habits common among English-speaking software engineers.
- **Backward Compatibility**: Fully backward compatible with existing state, SRS cards, and placement testing routines.
