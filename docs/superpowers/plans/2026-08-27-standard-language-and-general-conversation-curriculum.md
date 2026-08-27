# Standard Spanish & General Conversational Curriculum Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expand Spanglings into a 100% comprehensive general standard and conversational Spanish fluency engine by adding 6 new tracks (36 new exercises), bringing the total curriculum from 195 to **231 exercises across 42 tracks**.

**Architecture:** Exercises will be created in `exercises/36_*` through `exercises/41_*`, strictly following Spanglings' golden exercise specification, 3-tier progressive hints, diagnostic rules, and full accent forgiveness/strict modes. All references, CLI commands, and test suites will be updated and validated.

---

### Task 1: Track 36 – Everyday Life, Housing & Practical Bureaucracy (`exercises/36_everyday_life_and_housing/`)
- [ ] Create `01_contrato_arrendamiento.md` (Lease agreements & security deposits: *contrato de arrendamiento / fianza*)
- [ ] Create `02_alta_servicios_hogar.md` (Setting up utilities: *dar de alta el suministro de agua / electricidad*)
- [ ] Create `03_transferencia_bancaria.md` (Banking & fees: *transferencia interbancaria / comisión por transferencia*)
- [ ] Create `04_solicitar_reembolso.md` (Customer service & returns: *solicitar un reembolso / comprobante de compra*)
- [ ] Create `05_recoger_paquete.md` (Post office & parcels: *recoger un paquete en la oficina postal*)
- [ ] Create `06_hacer_transbordo.md` (Transit & transportation: *hacer transbordo en la estación central*)
- [ ] Verify with `cargo test --test exercise_validity_tests`
- [ ] Commit with `--no-gpg-sign`

---

### Task 2: Track 37 – Healthcare, Medical Encounters & Symptoms (`exercises/37_healthcare_and_symptoms/`)
- [ ] Create `01_describir_sintomas.md` (Describing sharp pain & dizziness: *tener punzadas agudas / sentir mareos*)
- [ ] Create `02_consulta_receta_medica.md` (Doctor prescriptions: *recetar medicamentos / tomar una dosis cada ocho horas*)
- [ ] Create `03_farmacia_efectos_secundarios.md` (Pharmacy & side effects: *medicamento de venta libre / prospecto informativo*)
- [ ] Create `04_acudir_urgencias.md` (Emergency room & medical discharge: *acudir al servicio de urgencias / dar el alta médica*)
- [ ] Create `05_cita_especialista.md` (Specialist appointments: *pedir cita con el especialista / revisión anual*)
- [ ] Create `06_alergias_alimentarias.md` (Allergies & dietary needs: *ser alérgico a / tener intolerancia severa a*)
- [ ] Verify with `cargo test --test exercise_validity_tests`
- [ ] Commit with `--no-gpg-sign`

---

### Task 3: Track 38 – Dining, Socializing, Small Talk & Nightlife (`exercises/38_dining_and_social_conversation/`)
- [ ] Create `01_pedir_cuenta_separado.md` (Dining out: *pedir la cuenta por separado / la salsa aparte*)
- [ ] Create `02_hacer_planes_quedar.md` (Making plans: *quedar a las ocho / cancelar a última hora*)
- [ ] Create `03_cumplidos_cortesia.md` (Polite social turns: *¡qué bien te queda! / no te preocupes por nada*)
- [ ] Create `04_muletillas_conversacionales.md` (Discourse softeners & fillers: *la verdad es que / por cierto / a ver*)
- [ ] Create `05_anecdotas_pasarla_bien.md` (Recounting stories & banter: *pasarla muy bien / armar un buen plan*)
- [ ] Create `06_despedidas_seguimiento.md` (Goodbyes: *estamos en contacto / avísame en cuanto llegues*)
- [ ] Verify with `cargo test --test exercise_validity_tests`
- [ ] Commit with `--no-gpg-sign`

---

### Task 4: Track 39 – Nuanced Prepositions & Spatial/Temporal Locutions (`exercises/39_nuanced_prepositions_and_locutions/`)
- [ ] Create `01_hacia_vs_hasta.md` (*Hacia* vs *Hasta* direction vs destination endpoint)
- [ ] Create `02_tras_vs_segun_vs_bajo.md` (*Tras*, *Según*, and *Bajo la condición de*)
- [ ] Create `03_a_base_de_a_expensas.md` (Compound prepositions: *a base de*, *a expensas de*)
- [ ] Create `04_al_cabo_de_dentro_de.md` (Temporal intervals: *al cabo de unos meses / dentro de poco*)
- [ ] Create `05_a_lo_largo_de_alrededor.md` (Spatial extension: *a lo largo de la avenida / alrededor del edificio*)
- [ ] Create `06_a_raiz_de_por_medio_de.md` (Origin & causal triggers: *a raíz de lo ocurrido / por medio de*)
- [ ] Verify with `cargo test --test exercise_validity_tests`
- [ ] Commit with `--no-gpg-sign`

---

### Task 5: Track 40 – Middle-Voice Shifts & Reflexive Nuances (`exercises/40_middle_voice_and_reflexive_shifts/`)
- [ ] Create `01_ir_vs_irse.md` (*Ir* to a place vs *Irse* departure/leaving)
- [ ] Create `02_dormir_vs_dormirse.md` (*Dormir* sleep vs *Dormirse* falling asleep)
- [ ] Create `03_comer_vs_comerse.md` (*Comer* vs *Comerse* telic / eating it all up)
- [ ] Create `04_llevar_vs_llevarse.md` (*Llevar* transport vs *Llevarse* take away / get along)
- [ ] Create `05_quedar_vs_quedarse.md` (*Quedar* meet/suit vs *Quedarse* stay/remain)
- [ ] Create `06_volver_vs_volverse.md` (*Volver* return vs *Volverse* transform permanently)
- [ ] Verify with `cargo test --test exercise_validity_tests`
- [ ] Commit with `--no-gpg-sign`

---

### Task 6: Track 41 – Advanced Temporal, Manner & Concessive Adverbial Clauses (`exercises/41_adverbial_clauses_and_conjunctions/`)
- [ ] Create `01_a_medida_que_conforme.md` (*A medida que* / *Conforme* with indicative vs subjunctive)
- [ ] Create `02_segun_de_modo_que.md` (*Según* / *De modo que* manner & consequence)
- [ ] Create `03_en_tanto_que_mientras.md` (*En tanto que* / *Mientras tanto*)
- [ ] Create `04_tan_pronto_como_apenas.md` (*Tan pronto como* / *Apenas* with future subjunctive anticipation)
- [ ] Create `05_salvo_que_a_menos_que.md` (*Salvo que* / *A menos que* / *A no ser que* mandatory subjunctive)
- [ ] Create `06_siempre_y_cuando.md` (*Siempre y cuando* / *A condición de que* strict condition)
- [ ] Verify with `cargo test --test exercise_validity_tests`
- [ ] Commit with `--no-gpg-sign`

---

### Task 7: Full Verification, Documentation & Knowledge Graph Update
- [ ] Update `README.md` curriculum table and `docs/BACKLOG.md` (231 exercises across 42 tracks)
- [ ] Run full test & lint suite: `cargo clippy --all-targets -- -D warnings && cargo fmt --check && cargo test`
- [ ] Rebuild knowledge graph with `uvx --from graphifyy graphify update .`
- [ ] Commit with `--no-gpg-sign`
