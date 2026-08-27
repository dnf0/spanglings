# VS Code Extension & Latin American Spanish Curriculum Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a packaged VS Code / Cursor extension (`spanglings-vscode`) featuring native LSP client lifecycle management, sidebar tree explorer, streak/SRS status bar widgets, and command palette actions, followed by 5 high-impact Latin American curriculum tracks (30 new exercises across Mexican startups, Colombian professional nuances, Rioplatense executive voseo, LatAm tech Anglicism elimination, and enterprise SLA risk negotiation).

**Architecture:** The VS Code extension resides in `editors/vscode/`, packaging a lightweight TypeScript extension using `vscode-languageclient` to connect to `spanglings lsp` via stdio, parsing `spanglings list --json` and `spanglings progress --json` to drive the sidebar tree view and status bar items. The Latin American curriculum is structured in `exercises/31_*` to `exercises/35_*`, adhering strictly to Spanglings' golden exercise specification, diagnostic rules, and 3-tier hints.

**Tech Stack:** TypeScript, `vscode-languageclient`, VS Code Extension API, Rust, Ratatui, `spanglings lsp`.

---

### Task 1: VS Code Extension Manifest, Build Pipeline & Scaffolding

**Files:**
- Create: `editors/vscode/package.json`
- Create: `editors/vscode/tsconfig.json`
- Create: `editors/vscode/esbuild.config.js`
- Create: `editors/vscode/README.md`
- Create: `editors/vscode/.vscodeignore`

- [ ] **Step 1: Create package.json and extension manifest**
- [ ] **Step 2: Create tsconfig.json and esbuild.config.js**
- [ ] **Step 3: Run npm install and build check**
- [ ] **Step 4: Commit**

---

### Task 2: LSP Client & Status Bar Widget Integration

**Files:**
- Create: `editors/vscode/src/lspClient.ts`
- Create: `editors/vscode/src/statusBar.ts`
- Modify: `editors/vscode/src/extension.ts`

- [ ] **Step 1: Implement LSP Client Manager (`src/lspClient.ts`)**
- [ ] **Step 2: Implement Status Bar Item (`src/statusBar.ts`)**
- [ ] **Step 3: Compile extension and verify bundling**
- [ ] **Step 4: Commit**

---

### Task 3: Exercise Explorer Tree View & Command Handlers

**Files:**
- Create: `editors/vscode/src/exerciseTree.ts`
- Create: `editors/vscode/src/commands.ts`
- Modify: `editors/vscode/src/extension.ts`

- [ ] **Step 1: Implement Exercise Tree Data Provider (`src/exerciseTree.ts`)**
- [ ] **Step 2: Implement Extension Commands (`src/commands.ts`)**
- [ ] **Step 3: Wire into `src/extension.ts` and test build**
- [ ] **Step 4: Build extension**
- [ ] **Step 5: Commit**

---

### Task 4: Track 31 – Mexican Tech, Startup & Venture Capital Spanish (`exercises/31_mexican_tech_and_startups/`)

**Files:**
- Create: `exercises/31_mexican_tech_and_startups/01_levantamiento_capital.md`
- Create: `exercises/31_mexican_tech_and_startups/02_ronda_semilla_valuacion.md`
- Create: `exercises/31_mexican_tech_and_startups/03_tasa_de_abandono.md`
- Create: `exercises/31_mexican_tech_and_startups/04_ajuste_producto_mercado.md`
- Create: `exercises/31_mexican_tech_and_startups/05_tasa_de_quema.md`
- Create: `exercises/31_mexican_tech_and_startups/06_ronda_de_inversion_serie_a.md`

- [ ] **Step 1: Write all 6 exercises with golden curriculum metadata, diagnostic rules, alternatives, and 3-tier hints**
- [ ] **Step 2: Run curriculum validity test**
- [ ] **Step 3: Commit**

---

### Task 5: Track 32 – Colombian & Andean Professional Nuances (`exercises/32_colombian_professional_nuances/`)

**Files:**
- Create: `exercises/32_colombian_professional_nuances/01_hacer_una_vuelta.md`
- Create: `exercises/32_colombian_professional_nuances/02_estar_pendiente.md`
- Create: `exercises/32_colombian_professional_nuances/03_caer_en_cuenta.md`
- Create: `exercises/32_colombian_professional_nuances/04_echar_reversa.md`
- Create: `exercises/32_colombian_professional_nuances/05_poner_la_lupa.md`
- Create: `exercises/32_colombian_professional_nuances/06_dar_papaya_security.md`

- [ ] **Step 1: Write all 6 exercises with golden metadata, diagnostics, and hints**
- [ ] **Step 2: Run curriculum validity test**
- [ ] **Step 3: Commit**

---

### Task 6: Track 33 – Rioplatense Production Voseo & Engineering (`exercises/33_rioplatense_production_voseo/`)

**Files:**
- Create: `exercises/33_rioplatense_production_voseo/01_sacar_a_produccion.md`
- Create: `exercises/33_rioplatense_production_voseo/02_bancarse_la_carga.md`
- Create: `exercises/33_rioplatense_production_voseo/03_ponerse_las_pilas_oncall.md`
- Create: `exercises/33_rioplatense_production_voseo/04_dar_de_baja_servidor.md`
- Create: `exercises/33_rioplatense_production_voseo/05_hacer_un_laburo_fino.md`
- Create: `exercises/33_rioplatense_production_voseo/06_no_te_quedes_atras.md`

- [ ] **Step 1: Write all 6 exercises with golden metadata, diagnostics, and hints**
- [ ] **Step 2: Run curriculum validity test**
- [ ] **Step 3: Commit**

---

### Task 7: Track 34 – LatAm Tech Anglicism Elimination & Precision Spanish (`exercises/34_latam_anglicism_elimination/`)

**Files:**
- Create: `exercises/34_latam_anglicism_elimination/01_rastrear_vs_trackear.md`
- Create: `exercises/34_latam_anglicism_elimination/02_personalizar_vs_customizar.md`
- Create: `exercises/34_latam_anglicism_elimination/03_rendir_vs_performar.md`
- Create: `exercises/34_latam_anglicism_elimination/04_descartar_vs_deprecar.md`
- Create: `exercises/34_latam_anglicism_elimination/05_redireccionar_vs_forwardear.md`
- Create: `exercises/34_latam_anglicism_elimination/06_restablecer_vs_resetear.md`

- [ ] **Step 1: Write all 6 exercises with golden metadata, diagnostics, and hints**
- [ ] **Step 2: Run curriculum validity test**
- [ ] **Step 3: Commit**

---

### Task 8: Track 35 – Latin American Enterprise SLA & Risk Governance (`exercises/35_latam_enterprise_risk_and_sla/`)

**Files:**
- Create: `exercises/35_latam_enterprise_risk_and_sla/01_acuerdo_confidencialidad.md`
- Create: `exercises/35_latam_enterprise_risk_and_sla/02_lucro_cesante.md`
- Create: `exercises/35_latam_enterprise_risk_and_sla/03_clausula_rescision.md`
- Create: `exercises/35_latam_enterprise_risk_and_sla/04_mitigacion_riesgos.md`
- Create: `exercises/35_latam_enterprise_risk_and_sla/05_indemnidad_legal.md`
- Create: `exercises/35_latam_enterprise_risk_and_sla/06_penalizaciones_sla.md`

- [ ] **Step 1: Write all 6 exercises with golden metadata, diagnostics, and hints**
- [ ] **Step 2: Run curriculum validity test**
- [ ] **Step 3: Commit**

---

### Task 9: Full Verification, Documentation & Knowledge Graph Update

**Files:**
- Modify: `README.md`
- Modify: `docs/BACKLOG.md`
- Update: `graphify-out/`

- [ ] **Step 1: Update README.md and docs/BACKLOG.md with Tracks 31–35 (195 exercises across 36 tracks) and VS Code extension instructions**
- [ ] **Step 2: Run full verification suite**
- [ ] **Step 3: Update knowledge graph**
- [ ] **Step 4: Commit**
