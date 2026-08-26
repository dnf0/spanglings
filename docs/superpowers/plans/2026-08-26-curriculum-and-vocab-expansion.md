# Curriculum & Advanced Vocabulary Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expand Spanglings curriculum with 5 new tracks (25 new exercises) covering Tech & Software Engineering Spanish, Formal Business & Diplomatic Correspondence, False Friends & Cognate Traps, Register Elevation (C1), and Regional Variations (*Voseo*), plus new in-terminal reference explanation cards.

**Architecture:** Create new Markdown exercise files in `exercises/22_tech_software/` through `exercises/26_regional_contrasts/`. Add reference cards in `src/engine/reference.rs`. Verify all new exercises validate with the golden validator (`test_all_curriculum_exercises_are_valid_and_solvable`).

**Tech Stack:** Rust 2021, Markdown parser, Golden curriculum test validator.

---

### Task 1: Track 22 - Tech & Software Engineering Spanish (B2-C1)

**Files:**
- Create: `exercises/22_tech_software/01_deployment_git.md`
- Create: `exercises/22_tech_software/02_debugging_fixes.md`
- Create: `exercises/22_tech_software/03_concurrency_deadlocks.md`
- Create: `exercises/22_tech_software/04_performance_latency.md`
- Create: `exercises/22_tech_software/05_api_routing.md`
- Test: `tests/exercise_validity_tests.rs`

- [ ] **Step 1: Create 5 Tech & Software Engineering Exercises**
Cover:
1. `01_deployment_git.md`: *desplegar a producción* / *solicitud de extracción*
2. `02_debugging_fixes.md`: *depurar el código* / *subsanar la vulnerabilidad*
3. `03_concurrency_deadlocks.md`: *bloqueo mutuo (deadlock)* / *concurrencia*
4. `04_performance_latency.md`: *rendimiento* / *latencia de red*
5. `05_api_routing.md`: *enrutamiento de peticiones* / *procesamiento por lotes*

- [ ] **Step 2: Run golden validity test**
Run: `cargo test --test exercise_validity_tests`
Expected: PASS.

- [ ] **Step 3: Commit**
```bash
git add exercises/22_tech_software/
git commit -m "feat(curriculum): add Track 22 - Tech and Software Engineering Spanish"
```

---

### Task 2: Track 23 - Formal Business & Diplomatic Correspondence (B2-C1)

**Files:**
- Create: `exercises/23_business_diplomatic/01_email_opening_closing.md`
- Create: `exercises/23_business_diplomatic/02_negotiation_terms.md`
- Create: `exercises/23_business_diplomatic/03_debt_settlement.md`
- Create: `exercises/23_business_diplomatic/04_dismissing_proposals.md`
- Create: `exercises/23_business_diplomatic/05_contract_stipulations.md`
- Test: `tests/exercise_validity_tests.rs`

- [ ] **Step 1: Create 5 Formal Business & Diplomatic Exercises**
Cover:
1. `01_email_opening_closing.md`: *Quedo a su entera disposición*
2. `02_negotiation_terms.md`: *En lo que atañe a lo convenido* / *acordar los términos*
3. `03_debt_settlement.md`: *saldar la deuda pendiente*
4. `04_dismissing_proposals.md`: *desestimar la propuesta* / *desestimar el recurso*
5. `05_contract_stipulations.md`: *estipular en las cláusulas* / *llevar a cabo*

- [ ] **Step 2: Run golden validity test**
Run: `cargo test --test exercise_validity_tests`
Expected: PASS.

- [ ] **Step 3: Commit**
```bash
git add exercises/23_business_diplomatic/
git commit -m "feat(curriculum): add Track 23 - Formal Business and Diplomatic Correspondence"
```

---

### Task 3: Track 24 - False Friends & High-Frequency Trap Drills

**Files:**
- Create: `exercises/24_false_friends/01_actualmente_actually.md`
- Create: `exercises/24_false_friends/02_eventualmente_eventually.md`
- Create: `exercises/24_false_friends/03_pretender_realizar.md`
- Create: `exercises/24_false_friends/04_soportar_apoyar.md`
- Create: `exercises/24_false_friends/05_sensible_embarazada.md`
- Test: `tests/exercise_validity_tests.rs`

- [ ] **Step 1: Create 5 False Friends & Trap Exercises**
Cover:
1. `01_actualmente_actually.md`: *actualmente* (currently) vs *en realidad / de hecho* (actually)
2. `02_eventualmente_eventually.md`: *eventualmente* (occasionally/by chance) vs *con el tiempo / finalmente* (eventually)
3. `03_pretender_realizar.md`: *pretender* (to attempt/claim) vs *fingir* (to pretend) & *realizar* (to make) vs *darse cuenta de* (to realize)
4. `04_soportar_apoyar.md`: *soportar* (to tolerate) vs *apoyar / respaldar* (to support)
5. `05_sensible_embarazada.md`: *sensible* (sensitive) vs *sensato* (sensible) & *constipado* (having a cold)

- [ ] **Step 2: Run golden validity test**
Run: `cargo test --test exercise_validity_tests`
Expected: PASS.

- [ ] **Step 3: Commit**
```bash
git add exercises/24_false_friends/
git commit -m "feat(curriculum): add Track 24 - High Frequency False Friends and Traps"
```

---

### Task 4: Track 25 (Register Elevation) & Track 26 (Regional Contrasts & Voseo)

**Files:**
- Create: `exercises/25_register_elevation/01_acometer_proyectos.md`
- Create: `exercises/25_register_elevation/02_suscitar_controversia.md`
- Create: `exercises/25_register_elevation/03_surtir_efecto.md`
- Create: `exercises/25_register_elevation/04_albergar_dudas.md`
- Create: `exercises/25_register_elevation/05_arrojar_luz.md`
- Create: `exercises/26_regional_contrasts/01_voseo_present.md`
- Create: `exercises/26_regional_contrasts/02_voseo_imperative.md`
- Create: `exercises/26_regional_contrasts/03_latin_america_ustedes.md`
- Create: `exercises/26_regional_contrasts/04_lexical_variants_driving_talking.md`
- Create: `exercises/26_regional_contrasts/05_clitic_order_leismo.md`
- Test: `tests/exercise_validity_tests.rs`

- [ ] **Step 1: Create Track 25 and Track 26 Exercises**
- [ ] **Step 2: Run golden validity test**
Run: `cargo test --test exercise_validity_tests`
Expected: PASS (all 141 exercises valid and solvable).

- [ ] **Step 3: Commit**
```bash
git add exercises/25_register_elevation/ exercises/26_regional_contrasts/
git commit -m "feat(curriculum): add Track 25 Register Elevation and Track 26 Regional Contrasts"
```

---

### Task 5: New In-Terminal Reference Cheat Sheets & Documentation Update

**Files:**
- Modify: `src/engine/reference.rs`
- Modify: `tests/reference_tests.rs`
- Modify: `README.md`
- Modify: `docs/BACKLOG.md`

- [ ] **Step 1: Add new reference cards in `src/engine/reference.rs`**
Add:
- `tech_software`
- `business_correspondence`
- `false_friends`
- `voseo`

- [ ] **Step 2: Update `tests/reference_tests.rs` and verify**
Run: `cargo test --all-targets`
Expected: PASS.

- [ ] **Step 3: Update `README.md` and `docs/BACKLOG.md`**
- [ ] **Step 4: Commit**
```bash
git add src/engine/reference.rs tests/reference_tests.rs README.md docs/BACKLOG.md
git commit -m "feat(engine): add tech, business, and false-friends reference cards and docs"
```
