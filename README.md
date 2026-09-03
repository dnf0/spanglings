# Spanglings 🇪🇸 🦀

[![CI](https://github.com/dnf0/spanglings/actions/workflows/ci.yml/badge.svg)](https://github.com/dnf0/spanglings/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust: 1.75+](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Docs: Live](https://img.shields.io/badge/docs-gh--pages-blue)](https://dnf0.github.io/spanglings/)
[![WebAssembly: Native](https://img.shields.io/badge/WebAssembly-Compiled%20Wasm-purple.svg)](https://dnf0.github.io/spanglings/playground/)
[![Curriculum: 24 Topics • 398 Items](https://img.shields.io/badge/Curriculum-24%20Topics%20%E2%80%A2%20398%20Items-emerald.svg)](https://dnf0.github.io/spanglings/syllabus/)
[![CEFR: A1 to C1](https://img.shields.io/badge/CEFR-A1%20%E2%86%92%20C1%20Mastery-gold.svg)](https://dnf0.github.io/spanglings/syllabus/)

> **Spanglings builds the syntax compiler; real-world usage supplies the data.**  
> A developer-grade Spanish language learning system featuring a comprehensive reference manual, a zero-install WebAssembly interactive playground, and a structured 24-topic CEFR curriculum. Inspired by [Rustlings](https://github.com/rust-lang/rustlings) and [Raylings](https://github.com/dnf0/raylings).

---

## 🚀 Core Learning Pillars

Spanglings provides a zero-install, developer-grade learning environment across three unified pillars:

1. 📘 **[Spanish Language Manual](https://dnf0.github.io/spanglings/manual/)**: Master all 24 pedagogical topics with dual-layer explanations — cognitive communicative mental models and compiler-grade grammar decision matrices.
2. ⚡ **[Interactive Web Playground](https://dnf0.github.io/spanglings/playground/)**: Zero-installation browser learning platform powered by compiled WebAssembly. Features the **Curriculum Syntax Studio** (with Monaco editor & live diagnostic compiler) and the **Rapid Arcade Arena** (real-time showdown duels with speed multiplier scoring).
3. 🗺️ **[Curriculum Syllabus](https://dnf0.github.io/spanglings/syllabus/)**: Complete curriculum roadmap spanning 24 topics, 136 sentence frames, and 262 arcade showdown duels across 3 CEFR tiers.

---

## 💡 Pedagogical Philosophy: The Dual-Layer Approach

Most language learning platforms rely on gamified flashcards and repetitive multiple-choice matching. They fail to teach the underlying **generative mental models** that developers use every day. 

Natural languages are structural systems with strict morphological transformations, scope rules, and pragmatic contracts. **Spanglings** approaches Spanish acquisition through a **compiler-driven pedagogical architecture**:

| Layer | Component | Focus |
| :--- | :--- | :--- |
| **💡 Layer 1** | **Communicative Mental Model** | Intuitive cognitive metaphors that explain *why* native speakers choose a specific construction in real conversation. |
| **📐 Layer 2** | **Structural Decision Matrix** | Strict grammatical rules, scope triggers, exception matrices, and morphological transformations. |

---

## 🏛️ Language Architecture: 24 Topics across 3 CEFR Tiers

Spanglings categorizes the entire Spanish grammar continuum into 24 core pedagogical domains across 3 CEFR competency tiers:

### 🟢 Tier 1: Foundations & Aspectual Geometry (A1–A2)
Fundamental syntactic distinctions and morphosyntactic mechanics:
- **Ser vs. Estar** (`#ser-estar`): Essence & identity vs. transient state, posture, and location.
- **Por vs. Para** (`#por-para`): Backward motive & trajectory vs. forward goal, recipient & deadline.
- **Past Tenses & Aspect** (`#past-tenses`): Completed bounded events (preterite) vs. narrative background (imperfect).
- **Pronoun Stacking** (`#pronouns`): Indirect and direct object clitic ordering and the *se lo* rule.
- **Verbs like Gustar** (`#gustar`): Inverted experiential argument structures.
- **Reflexive Verbs** (`#reflexive`): Inherent, reciprocal, and middle-voice pronominal markers.
- **Stem-Changing Verbs** (`#stem-changing`): Radical vowel alternations in stressed present stems.
- **Prepositional Regimen** (`#prepositions`): Bound prepositions (*soñar con*, *insistir en*, *acordarse de*).

### 🟡 Tier 2: Mood, Triggers & Pragmatic Voice (B1–B2)
Modality, hypothetical conditions, and non-agentive structures:
- **Present Subjunctive** (`#subjunctive`): Volition, emotion, doubt, and non-existent antecedents.
- **Imperfect Subjunctive** (`#imperfect-subjunctive`): Counterfactual *si*-clauses and hypothetical requests.
- **Imperative Mood** (`#imperative`): Affirmative vs. negative commands and clitic binding.
- **Accidental "Se"** (`#accidental-se`): De-agentified involuntary events (*se me cayó*).
- **Passive vs. Impersonal "Se"** (`#passive-impersonal-se`): Agentless passives and general human agents.
- **Possessive Datives** (`#possessive-datives`): Inalienable possession with definite articles.
- **Relative Pronouns** (`#relative-pronouns`): Restrictive vs. non-restrictive clauses (*que*, *quien*, *el que*, *cuyo*).
- **Gerund Syntax Rules** (`#gerund-rules`): Simultaneous adverbial modification vs. forbidden adjective gerunds.

### 🟣 Tier 3: Advanced Nuance, Registers & Edge Mechanics (B2–C1)
Subtle semantic shifts, pragmatic discourse markers, and specialized domains:
- **Verbs of Becoming** (`#verbs-of-becoming`): Nuanced transformations (*ponerse*, *quedarse*, *hacerse*, *volverse*, *convertirse en*).
- **Scalar Concession** (`#scalar-concession`): Intensive concessive polarity (*por mucho que*, *aun a riesgo de que*).
- **Epistemic Conjecture** (`#epistemic-conjecture`): Future and conditional of probability (*serán las 4*, *estaría cansado*).
- **Adversatives & Rectification** (`#adversatives`): Restrictive *pero* vs. exclusive corrective *sino* / *sino que*.
- **False Friends** (`#false-friends`): Deceptive cognates (*actualmente*, *embarazada*, *constipado*).
- **Voseo Conjugation** (`#voseo`): Rioplatense second-person singular morphology (*vos tenés*, *vos podés*).
- **Software & Tech Spanish** (`#tech`): Authentic engineering, cloud, and systems vocabulary.
- **Legal & Statutory Spanish** (`#legal`): Formal administrative and statutory registers.

---

## ⚡ WebAssembly Interactive Modes

Spanglings compiles its full Rust diagnostic and evaluation engine directly to WebAssembly (`wasm32-unknown-unknown` + `wasm-bindgen`), delivering instant evaluation in the browser with zero server latency or installation overhead.

### 1. 🖥️ Curriculum Syntax Studio
- **Monaco Code Editor**: Full syntax editing with virtual Spanish accent keyboard bar (`á`, `é`, `í`, `ó`, `ú`, `ñ`, `ü`, `¡`, `¿`).
- **Rustc-Style Diagnostics**: Real-time compiler feedback with error codes, token markers, and direct hints.
- **Progressive 3-Tier Hints**: Tier 1 Clue $\rightarrow$ Tier 2 Structural Rule $\rightarrow$ Tier 3 Concrete Solution.
- **Dual-Layer Mental Models**: Collapsible cognitive mental model and grammar rule tabs for every sentence frame.

### 2. ⚡ Rapid Arcade Arena
- **Showdown Duels**: Rapid-fire multiple choice duels across 262 specialized grammar challenges.
- **Speed Multiplier Scoring**: Bonus points awarded for responses under 3.0s and 1.5s thresholds.
- **Streaks & Combos**: Streak tracking, combo multipliers, and session recaps with replay of missed items.
- **Keyboard Shortcuts**: Number keys `[1]`–`[4]` for instant option selection, `[Space]` to advance, `[Escape]` to exit.

### 3. 💾 Local State & Spaced Repetition (SM-2)
- **SuperMemo-2 Recall Scheduling**: Tracks interval, repetition count, and ease factors locally in `localStorage`.
- **Zero-Cloud Privacy**: Learning state stays entirely in your browser.
- **JSON Export & Import**: Backup, restore, or synchronize your progress across devices via standard JSON format.

---

## 🌐 The *lings Ecosystem

If you enjoy hands-on technical mastery, explore our companion platforms:

- ☸️ [**Kubelings**](https://github.com/dnf0/kubelings) — Hands-on interactive CLI learning environment for Kubernetes.
- 🏗️ [**Terralings**](https://github.com/dnf0/terralings) — Master Terraform and OpenTofu through interactive infrastructure-as-code exercises.
- ⚡ [**Raylings**](https://github.com/dnf0/raylings) — Learn distributed AI, Ray Core actors, and scalable clusters through hands-on Python exercises.
- 🦀 [**Rustlings**](https://github.com/rust-lang/rustlings) — Small exercises to get you used to reading and writing Rust code.

---

## 📖 Documentation & Links

Full documentation and interactive tools are available online:

- 🚀 **[Documentation Home](https://dnf0.github.io/spanglings/)**
- 📘 **[Spanish Language Manual](https://dnf0.github.io/spanglings/manual/)**
- ⚡ **[Interactive Web Playground](https://dnf0.github.io/spanglings/playground/)**
- 🗺️ **[Curriculum Syllabus](https://dnf0.github.io/spanglings/syllabus/)**
- 🤝 **[Contributing Guide](https://dnf0.github.io/spanglings/contributing/)**

---

## License

This project is licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
