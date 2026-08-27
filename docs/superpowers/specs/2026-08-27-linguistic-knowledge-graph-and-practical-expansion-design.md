# Design Specification: Linguistic Knowledge Graph & Practical Everyday Curriculum Expansion

**Date**: 2026-08-27  
**Status**: Approved / In Progress  
**Authors**: Antigravity & User  
**Target Architecture**: `spanglings` Core Engine, CLI, TUI, and Curriculum Catalog  

---

## 1. Overview & Pedagogical Objective

Spanglings transitions from a set of linear exercise tracks to a **Directed Acyclic Graph (DAG) Linguistic Knowledge Graph**. In this architecture:
1. Every exercise links to one or more foundational **Grammar Primitives / Concepts** (e.g. `subjunctive_temporal_future`, `accidental_involuntary_se`, `por_vs_para_purpose`).
2. Practical, situational, and professional domains (Travel, Banking, Consumer Disputes, Repairs, News, and Conversational Fluency) are designed as **applied syntheses** of these underlying concepts, preventing rote phrasebook memorization.
3. The compiler diagnostic engine and smart SRS weakness profiler traverse prerequisite edges in the graph to trace errors back to root grammatical concepts, directing learners to targeted foundational remediation.
4. The curriculum expands from 231 to **267 handcrafted exercises across 48 tracks** (adding Tracks 42–47), with retroactive conceptual metadata applied to all existing 231 exercises (Tracks 00–41).

---

## 2. Core Architecture & Components

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Grammar Knowledge Graph                         │
│                          (src/core/graph.rs)                           │
│                                                                        │
│   [00: Baseline Stems] ───► [03: Subjunctive Volition]                 │
│            │                              │                            │
│            ▼                              ▼                            │
│   [05: Subjunctive Temp] ──────► [42: Travel Logistics & Borders]     │
│            │                              │                            │
│            ▼                              ▼                            │
│   [10: Accidental Se] ─────────► [45: Home Repairs & Utilities]        │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                   Concept-Aware Diagnostic Engine                      │
│                       (src/core/diagnostic.rs)                         │
│                                                                        │
│   error[E0502]: Subjunctive required for prospective temporal clause  │
│      = help: 'en cuanto' triggers the subjunctive for future events    │
│      = note: Linked Foundation: Track 05 (Subjunctive Conjunctions)    │
│      = note: Recommended Drill: 'spanglings drill --concept ...'       │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                      Persistent State & Profiler                       │
│                     (src/core/state.rs, progress.rs)                   │
│                                                                        │
│   - ConceptMastery scores (EWMA / derived from linked SRS cards)       │
│   - Targeted Conceptual Weakness Profiler                              │
│   - Learning Frontier Calculation                                      │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Data Models & Schemas

### 3.1 Grammar Concept Ontology (`src/core/graph.rs`)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConceptId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNode {
    pub id: ConceptId,
    pub title: String,
    pub category: ConceptCategory,
    pub level: Level,
    pub description: String,
    pub reference_topic: Option<String>,
    pub prerequisite_concepts: Vec<ConceptId>,
    pub foundational_track: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConceptCategory {
    AspectAndTense,
    MoodSelection,
    PronounsAndVoice,
    PrepositionsAndRelators,
    SyntaxAndRhetoric,
    SociolinguisticRegisters,
    PracticalPragmatics,
}

#[derive(Debug, Clone, Default)]
pub struct LinguisticGraph {
    pub nodes: HashMap<ConceptId, ConceptNode>,
    pub outgoing_edges: HashMap<ConceptId, Vec<ConceptId>>, // prerequisites required
    pub incoming_edges: HashMap<ConceptId, Vec<ConceptId>>, // unlocked downstream concepts
}
```

### 3.2 Enhanced Exercise Metadata (`src/core/exercise.rs`)

```rust
pub struct Exercise {
    pub path: PathBuf,
    pub id: String,
    pub level: Level,
    pub topic: String,
    pub exercise_type: ExerciseType,
    pub is_done: bool,
    pub title: String,
    pub solution: String,
    pub alternatives: Vec<String>,
    pub diagnostic_rules: Vec<DiagnosticRule>,
    pub hints: Vec<String>,
    pub raw_content: String,
    
    // New Concept-Linking Fields
    pub concept_tags: Vec<String>,
    pub prerequisites: Vec<String>,
    pub grammar_focus: Option<String>,
    pub contrast_note: Option<String>,
}
```

### 3.3 State Model Extension (`src/core/state.rs`)

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConceptMastery {
    pub concept_id: String,
    pub mastery_score: f32, // 0.0 to 1.0
    pub total_reviews: u32,
    pub lapses: u32,
    pub last_practiced: Option<DateTime<Utc>>,
}

// In AppState:
#[serde(default)]
pub concept_mastery: HashMap<String, ConceptMastery>,
```

---

## 4. Curriculum Expansion: Tracks 42 to 47 (36 New Exercises)

### Track 42: Travel Logistics, Flight Disruptions & Border Control (`42_travel_logistics_and_borders`)
- **Level**: B1–B2
- **Concepts**: `subjunctive_temporal_future`, `impersonal_se`, `reported_speech_backshifting`, `conditional_courtesy`.
- **Exercises**:
  1. `01_flight_delays_and_gates`: Prospective temporal clause (*en cuanto aterricemos*).
  2. `02_missed_connection_rebooking`: Reported speech obligation (*el agente me aseguró que nos reubicaría*).
  3. `03_baggage_loss_claim`: Passive *se* and formal claim (*se extravió el equipaje facturado*).
  4. `04_customs_declaration`: Conditional exception (*a menos que exceda el límite libre de impuestos*).
  5. `05_car_rental_excess`: Prepositional insurance clause (*cobertura a todo riesgo con franquicia*).
  6. `06_boarding_announcement`: Impersonal directive (*se ruega a los pasajeros que permanezcan sentados*).

### Track 43: Banking Operations, Taxes & Practical Finances (`43_banking_taxes_and_finances`)
- **Level**: B2–C1
- **Concepts**: `prepositional_regimes_financial`, `causal_consecutive_connectors`, `nominalization_aspectual`.
- **Exercises**:
  1. `01_tax_declaration_withholding`: Prepositional regime (*la retención asciende al quince por ciento*).
  2. `02_vat_invoicing_and_deductions`: Causal connector (*dado que el gasto es deducible*).
  3. `03_opening_bank_account_proof_of_funds`: Subjunctive request (*requieren que presente un justificante de ingresos*).
  4. `04_mortgage_fixed_vs_variable`: Contrastive locution (*optar por tipo fijo en lugar de variable*).
  5. `05_international_wire_fees`: Consecutive connector (*por ende, se aplicará una comisión por cambio de divisa*).
  6. `06_tax_audit_compliance`: Participial absolute (*habiendo aportado toda la documentación fiscal*).

### Track 44: Consumer Complaints & Dispute Rights (`44_consumer_complaints_and_rights`)
- **Level**: B2–C1
- **Concepts**: `subjunctive_demanding_volition`, `hypothetical_legal_conditionals`, `formal_assertive_register`.
- **Exercises**:
  1. `01_defective_product_warranty`: Demanding subjunctive (*exijo que sustituyan el dispositivo defectuoso*).
  2. `02_formal_refund_claim`: Volition formula (*solicito que procedan al reembolso íntegro*).
  3. `03_unauthorized_billing_charge`: Accidental/involuntary chargeback (*se me ha cobrado una tarifa indebida*).
  4. `04_misleading_advertising_dispute`: Concessive contrast (*por más que aleguen un error tipográfico*).
  5. `05_legal_arbitration_ultimatum`: Second conditional (*de no recibir una solución satisfactoria, acudiré a arbitraje de consumo*).
  6. `06_official_complaint_form_submission`: Passive/impersonal formalization (*queda formalmente interpuesta la hoja de reclamaciones*).

### Track 45: Home Maintenance, Utilities & Repairs (`45_home_maintenance_and_repairs`)
- **Level**: B1–B2
- **Concepts**: `accidental_involuntary_se`, `temporal_mood_contrast`, `middle_voice_aspect`.
- **Exercises**:
  1. `01_plumbing_leak_emergency`: Accidental event (*se nos rompió una tubería empotrada*).
  2. `02_tripped_circuit_breaker`: Involuntary outage (*se fue la luz al saltar el disyuntor general*).
  3. `03_boiler_breakdown`: Temporal clause with subjunctive (*estaremos sin agua caliente hasta que venga el técnico*).
  4. `04_coordinating_technician_window`: Future prospective (*en cuanto llegue el fontanero, revisará la caldera*).
  5. `05_strata_hoa_maintenance_notice`: Formal notification (*se notifica a la comunidad de propietarios que*).
  6. `06_appliance_replacement_warranty`: Involuntary failure vs repair (*se averió el compresor de la nevera*).

### Track 46: News Media, Macroeconomics & Civic Debate (`46_news_media_and_civic_debate`)
- **Level**: B2–C1
- **Concepts**: `concessive_subjunctive_formal`, `cleft_sentences_focus`, `formal_participle_inversion`.
- **Exercises**:
  1. `01_inflation_and_purchasing_power`: Cleft focus (*es la inflación subyacente lo que merma el poder adquisitivo*).
  2. `02_parliamentary_bill_passage`: Formal inversion (*aprobada la ley por mayoría parlamentaria, entrará en vigor*).
  3. `03_judicial_ruling_and_appeals`: Concessive clause (*aun cuando el tribunal falle a favor del demandante*).
  4. `04_employment_market_trends`: Causal connection (*a raíz del incremento en la contratación indefinida*).
  5. `05_central_bank_interest_rates`: Counterfactual conditional (*de haber subido los tipos de interés, el consumo se habría contraído*).
  6. `06_editorial_opinion_articulation`: Balanced discourse marker (*cabe señalar que las previsiones son optimistas*).

### Track 47: Conversational Markers & Pragmatic Nuance (`47_conversational_markers_and_nuance`)
- **Level**: B1–C1
- **Concepts**: `pragmatic_discourse_glue`, `epistemic_mood_selection`, `conversational_softeners`.
- **Exercises**:
  1. `01_dicho_esto_transition`: Nuanced concession (*dicho esto, no es que el proyecto carezca de riesgos* [subj]).
  2. `02_a_fin_de_cuentas_conclusion`: Epistemic assertion (*a fin de cuentas, la decisión final corresponde al equipo* [ind]).
  3. `03_por_si_fuera_poco_accumulation`: Intensified subjunctive trigger (*por si fuera poco, nos exigieron que rehiciéramos el informe*).
  4. `04_en_resumidas_cuentas_synthesis`: Concise recap (*en resumidas cuentas, lo acordado es vinculante*).
  5. `05_menos_mal_que_relief`: Indicative relief trigger (*menos mal que llegaste a tiempo*).
  6. `06_ni_mucho_menos_categorical_refusal`: Emphatic negation (*no pretendemos eludir responsabilidades, ni mucho menos*).

---

## 5. Retroactive Concept Tagging for Tracks 00 to 41 (231 Exercises)

All 231 existing exercises across Tracks 00 through 41 will be mapped with canonical `concept_tags`, `prerequisites`, and `grammar_focus` metadata:
- `00_baseline` → `["irregular_preterite_stems", "irregular_present_stems", "baseline_false_friends"]`
- `01_ser_vs_estar` → `["ser_vs_estar_essence_state", "adjective_meaning_shifts_ser_estar"]`
- `02_past_aspects` → `["preterite_vs_imperfect_aspect", "meaning_shifts_past_verbs"]`
- `03_subjunctive_weirdo` → `["subjunctive_volition_influence", "subjunctive_emotion_reaction", "subjunctive_doubt_denial"]`
- `04_subjunctive_relative` → `["subjunctive_unreal_relative_clauses"]`
- `05_subjunctive_conjunctions` → `["subjunctive_temporal_future", "subjunctive_purpose_para_que", "subjunctive_conditional_conjunctions"]`
- `06_imperfect_subjunctive_conditionals` → `["imperfect_subjunctive_morphology", "second_conditional_si_tuviera"]`
- `07_por_vs_para` → `["por_cause_agent_means", "para_purpose_recipient_deadline"]`
- `08_pronoun_stacking` → `["direct_indirect_clitic_stacking", "spurious_se_rule", "clitic_written_accentuation"]`
- `09_prepositional_regimes` → `["fixed_prepositional_regimes"]`
- `10_accidental_se` → `["accidental_involuntary_se", "plural_agreement_involuntary_se"]`
- `11_pluperfect_subjunctive` → `["pluperfect_subjunctive_morphology", "third_conditional_si_hubiera_sabido"]`
- `12_verbal_periphrases` → `["aspectual_verbal_periphrases", "modal_verbal_periphrases"]`
- `13_advanced_concessives` → `["concessive_subjunctive_formal", "reduplicative_subjunctive"]`
- `14_connectors` → `["formal_discourse_connectors", "de_ahi_que_subjunctive"]`
- `15_indirect_speech` → `["reported_speech_tense_backshifting", "deictic_shift_indirect_speech"]`
- `16_idioms` → `["fixed_conversational_idioms"]`
- `17_negated_perception` → `["negated_cognition_perception_subjunctive"]`
- `18_cleft_sentences` → `["cleft_sentences_focus_emphasis"]`
- `19_formal_inversion` → `["participial_absolute_inversion", "gerund_adverbial_clauses"]`
- `20_passive_refleja` → `["passive_refleja_agentless", "impersonal_se_vs_passive_se"]`
- `21_nuanced_collocations` → `["high_register_noun_verb_collocations"]`
- `22_tech_software` → `["software_engineering_tech", "git_ci_cd_workflows"]`
- `23_business_diplomatic` → `["formal_bureaucratic_legal", "diplomatic_negotiation_formulas"]`
- `24_false_friends` → `["high_frequency_cognate_traps"]`
- `25_register_elevation` → `["formal_register_elevation", "literary_verb_equivalents"]`
- `26_regional_contrasts` → `["rioplatense_voseo", "pan_american_latam"]`
- `27_system_design` → `["software_engineering_tech", "distributed_systems_architecture"]`
- `28_advanced_subjunctive_clauses` → `["reduplicative_subjunctive", "exceptive_subjunctive_salvo_que"]`
- `29_advanced_verbal_periphrases` → `["high_register_periphrases", "idiomatic_aspectual_periphrases"]`
- `30_executive_leadership` → `["formal_bureaucratic_legal", "strategic_leadership_diplomacy"]`
- `31_mexican_tech_and_startups` → `["pan_american_latam", "startup_venture_capital_mexico"]`
- `32_colombian_professional_nuances` → `["pan_american_latam", "colombian_professional_idioms"]`
- `33_rioplatense_production_voseo` → `["rioplatense_voseo", "production_engineering_voseo"]`
- `34_latam_anglicism_elimination` → `["pan_american_latam", "anglicism_elimination_precision"]`
- `35_latam_enterprise_risk_and_sla` → `["pan_american_latam", "formal_bureaucratic_legal", "sla_indemnification"]`
- `36_everyday_life_and_housing` → `["practical_pragmatics", "housing_leases_bureaucracy"]`
- `37_healthcare_and_symptoms` → `["practical_pragmatics", "medical_symptoms_prescriptions"]`
- `38_dining_and_social_conversation` → `["practical_pragmatics", "dining_socializing_banter"]`
- `39_nuanced_prepositions_and_locutions` → `["compound_prepositional_locutions", "spatial_temporal_locutions"]`
- `40_middle_voice_and_reflexive_shifts` → `["middle_voice_aspectual_shifts", "telic_consumption_shifts"]`
- `41_adverbial_clauses_and_conjunctions` → `["subjunctive_concessive", "adverbial_manner_temporal_conjunctions"]`

---

## 6. CLI Command Enhancements

1. `spanglings list [--concept <concept_id>] [--category <category>]`
   - Filter exercise catalog by underlying linguistic concept.
2. `spanglings drill [--concept <concept_id>]`
   - Target active recall drill sessions directly at a specific grammar concept.
3. `spanglings progress [--json]`
   - Displays new **Targeted Conceptual Weakness Profiler** aggregating lapses across the knowledge graph.

---

## 7. Verification & Quality Gates

- **Unit Tests**: Full coverage for graph traversal, prerequisite cycles check (DAG property verified), and concept aggregation in `tests/graph_tests.rs`.
- **Golden Validity**: `tests/exercise_validity_tests.rs` validates that all 267 exercises across all 48 tracks contain valid solutions, well-formed markdown frontmatter, and valid concept IDs present in the ontology.
- **CI Hygiene**: 100% clean check on `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`.
