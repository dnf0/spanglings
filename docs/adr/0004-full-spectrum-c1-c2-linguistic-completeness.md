# ADR-0004: Full-Spectrum C1/C2 Linguistic Completeness & Advanced Syntactic Constructs

## Status
Accepted

## Context
Following the implementation of Tracks 48–53 (ADR-0003), a deep CEFR C1/C2 and RAE normative grammar review identified the final 6 structural frontiers necessary for absolute linguistic completeness in Spanish:

1. **Verbs of Becoming & Transformation (*Verbos de Cambio / Devenir*)**: The six distinct nuances of *ponerse* (involuntary/temporary emotional/physical), *quedarse* (loss/aftermath/shock), *hacerse* (intentional effort/profession/ideology), *volverse* (permanent character/psychological transformation), *convertirse en* (radical metamorphosis), and *llegar a ser* (culmination of prolonged effort).
2. **Epistemic Adverbs & Mandatory Mood Selection**: Non-negotiable indicative governance (*a lo mejor*, colloquial *igual*), postposed adverb mood neutrality (postposed *quizás/tal vez* requiring indicative), preposed high-doubt subjunctive, and rhetorical *acaso*.
3. **Datives of Inalienable Possession & Affective Ethic Datives**: Native pronominal possession (*lavarse las manos*, *se le cayó la pantalla*), expressive ethic datives (*¡no me llores!*, *no me toma descansos*), sympathetic group misfortune (*se nos cayó el clúster*), and telic deliberative consumption (*beberse*).
4. **Corrective Negation & Rejected Causes**: Strict mood alternations in *no es que [subj]... sino que [ind]*, *no porque [subj]... sino porque [ind]*, and the consecutive connector *de ahí que [subj]*.
5. **Participial Absolute Constructions (*Construcciones Absolutas de Participio*)**: Preposed narrative absolute clauses (*Concluida la sesión...*, *Aprobadas las políticas...*, *Resueltos los incidentes...*, *Dicho esto...*, *Vista la situación...*) with obligatory gender/number agreement.
6. **Scalar Concession & Intensive Connectors**: Extreme scalar concession (*por mucho que [subj]*, *por más que [subj]*, *por muy + adj + que [subj]*), high-risk concession (*aun a riesgo de que [subj]*), and concessive factuality contrasts (*aun a sabiendas de que [ind]*).

## Decisions

### 1. Linguistic Graph Ontology Expansion (66 -> 81 Concepts)
- Extended `LinguisticGraph` in `src/core/graph.rs` with 15 new concept nodes:
  - `becoming_temporary_state_ponerse`
  - `becoming_resulting_state_quedarse`
  - `becoming_voluntary_evolution_hacerse`
  - `becoming_involuntary_shift_volverse`
  - `becoming_achievement_culmination_llegar_a_ser`
  - `becoming_radical_transformation_convertirse_en`
  - `epistemic_adverbs_indicative_a_lo_mejor_igual`
  - `epistemic_adverbs_positional_mood_quizas_tal_vez`
  - `dative_of_inalienable_possession`
  - `ethic_affective_sympathetic_dative`
  - `corrective_negation_no_es_que_sino_que`
  - `rejected_cause_consecutive_de_ahi_que`
  - `participial_absolute_clauses`
  - `scalar_concession_por_mucho_que`
  - `intensive_concession_riesgo_vs_sabiendas`
- Preserved strict cycle-free DAG topology with pedagogical prerequisite chains.

### 2. Curriculum Track Expansion (Tracks 54–59, 36 New Exercises, 339 Total across 60 Tracks)
- Created 6 new tracks with 6 exercises each:
  - `exercises/54_verbs_of_becoming_and_transformation/` (B2–C1)
  - `exercises/55_epistemic_adverbs_and_mood_selection/` (B1–B2)
  - `exercises/56_datives_of_possession_and_ethic_datives/` (B1–B2)
  - `exercises/57_corrective_and_concessive_polarities/` (B2–C1)
  - `exercises/58_participial_absolute_constructions/` (C1)
  - `exercises/59_scalar_concession_and_intensive_connectors/` (C1)

### 3. Grammar Reference Cards & Cheat Sheets Expansion (18 -> 24 Cards)
- Added dedicated reference cards in `src/core/reference.rs`:
  - `verbs-of-becoming`
  - `epistemic-adverbs`
  - `possessive-datives`
  - `corrective-polarity`
  - `participial-absolutes`
  - `scalar-concession`

### 4. Diagnostic Rules & Compiler Additions
- Registered diagnostic rules `E0054` through `E0059` in `src/engine/rules.rs` and integrated them into `validate_submission`.

## Consequences
- The Spanglings curriculum reaches 339 handcrafted exercises across 60 tracks.
- Learner coverage spans the complete spectrum of Spanish grammar from A1 foundations to nuanced C2 syntax.
- All 339 exercises are embedded, searchable, testable, and monitored by the weakness profiler and SRS engine.
