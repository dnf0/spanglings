# Spanglings WebAssembly Playground & Arcade Arena

> **Zero-backend client-side Spanish learning environment powered by WebAssembly and client storage.**  
> Master B1–C1 Spanish grammar, verb mechanics, aspectual distinctions (*pretérito vs imperfecto*), subjunctive triggers, and idiomatic collocations directly in your browser with instant compiler feedback.

---

<!-- Interactive WebAssembly & JavaScript Playground Application Mount -->
<div id="spanglings-app"></div>

<!-- Monaco AMD Script Loader -->
<script src="https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/loader.js"></script>
<!-- Spanglings WebAssembly & Storage App Controller -->
<script type="module" src="../assets/playground/playground.js"></script>

---

## 🎮 Playground Architecture & Overview

The **Spanglings Playground** brings the complete developer-grade terminal learning experience into a client-side, zero-latency WebAssembly sandbox. All verification, sentence generation, spaced repetition tracking, and diagnostic evaluation run 100% locally in your browser.

### Key Capabilities

| Feature | Description |
| :--- | :--- |
| **⚡ 0ms Latency Compilation** | Validates Spanish conjugations, clitic placements, and mood selection in < 15ms. |
| **🔒 100% Offline & Private** | Progress and custom settings persist exclusively in browser `localStorage`. No accounts, no tracking. |
| **💡 Dual-Layer Explanations** | Every exercise and duel pairs an intuitive communicative mental model with a structural grammar rule. |
| **⛶ Zen & Fullscreen Mode** | Toggle distraction-free full-screen mode to turn your browser into a dedicated language terminal. |
| **⌨️ Keyboard-First Workflow** | Native shortcuts for accent insertion, code submission (`Ctrl+Enter`), and arcade rapid answers (`1`-`4` or `j`/`k`/`l`/`;`). |

---

## 🕹️ Two Operational Modes

### Mode A: Curriculum Workspace (Dual-Pane Editor)

The **Curriculum Workspace** mirrors the Spanglings CLI watcher and TUI experience across all **24 core grammar tracks** and **136 structural sentence frames**:

1. **Track Navigation**: Select any topic from the sidebar (e.g. *Subjunctive Triggers*, *Por vs. Para*, *Indirect Object Clitics*, *Conditional Sentences*).
2. **Monaco Code Editor**: Edit sentence code with syntax highlighting, line numbers, and error markers.
3. **Accent Toolbar**: Insert Spanish diacritics (`á`, `é`, `í`, `ó`, `ú`, `ñ`, `ü`, `¿`, `¡`) with one click or using keyboard shortcuts.
4. **Progressive 3-Tier Hint System**:
   - `Hint 1 (?)`: Subtle clue highlighting the critical structural dependency.
   - `Hint 2 (??)`: Explicit grammar rule and morphological pattern.
   - `Hint 3 (???)`: Complete verified target solution with dual-layer breakdown.
5. **Dual-Layer Feedback Cards**:
   - `💡 Meaning / Context`: Real-world communicative intent and mental model.
   - `📐 Grammar Rule`: Formal syntactic transformation and morphological rationale.

```
+-----------------------------------------------------------------------------+
| 🇪🇸 Spanglings WebAssembly Workspace                    [Curriculum] [Arcade] |
+----------------------+-----------------------------+------------------------+
| 📚 Track Navigation  | 📝 Monaco Code Editor       | 💡 Pedagogical Feedback|
|  • Subjunctive (A1)  |                             |                        |
|  • Por vs Para (B1)  |  Dudo que ellos [vengan]    |  ✓ CORRECT!            |
|  • Clitics (B2)      |  a la fiesta esta noche.    |  💡 Meaning: Doubt      |
|  • Regimen (C1)      |                             |  📐 Rule: Subjunctive  |
|                      |  [á] [é] [í] [ó] [ú] [ñ]    |     required after     |
|                      |  [✓ Submit (Ctrl+Enter)]    |     verbs of denial    |
+----------------------+-----------------------------+------------------------+
```

---

### Mode B: Rapid Arcade Arena (Showdowns & Specialized Engines)

The **Rapid Arcade Arena** is a high-speed, interactive drill environment designed to build instantaneous syntactic intuition through two game types:

#### 1. 16 Binary Showdown Duels
High-velocity discrimination duels targeting the most notorious Spanish contrast pairs:
- **Ser vs. Estar**: Essence vs. state, condition, and characteristic.
- **Por vs. Para**: Cause/means/duration vs. destination/recipient/deadline.
- **Pretérito vs. Imperfecto**: Completed endpoint vs. ongoing background aspect.
- **Subjunctive vs. Indicative**: Doubt/desire/non-assertion vs. factual declaration.
- **Saber vs. Conocer**: Factual knowledge/skills vs. acquaintance/familiarity.
- **Pedir vs. Preguntar**: Requesting actions/objects vs. inquiring for information.
- **Llevar vs. Traer**: Movement away from vs. movement toward the speaker.
- **Muy vs. Mucho**: Adverbial degree modifier vs. noun quantifier.
- **Bien vs. Bueno**: Adverbial manner vs. adjectival quality.
- **Tú vs. Usted**: Informal solidarity vs. formal social distance.
- **Lo/La vs. Le/Les**: Direct accusative object vs. indirect dative recipient.
- **Haber vs. Estar**: Existence (*hay*) vs. definite spatial location (*está*).
- **Tener vs. Haber**: Possession vs. perfect tense compound auxiliary.
- **Ir vs. Irse**: Directional transit vs. emphatic departure/leaving.
- **Sino vs. Pero**: Direct corrective substitution (*not this, but that*) vs. adversative qualification.
- **Para qué vs. Por qué / Porque**: Purpose/goal inquiry vs. cause/reason explanation.

#### 2. 5 Specialized Drill Engines
Targeted mechanical training modules for advanced syntactic nuances:
- **Prepositional Regimen Engine (*Verbos con Régimen*)**: Master mandatory verb-bound prepositions (*soñar con*, *depender de*, *fijarse en*, *acordarse de*).
- **High-Frequency Irregulars Engine**: Stem-changing radical shifts, irregular preterite roots (*puse*, *tuve*, *quise*), and irregular subjunctive stems.
- **False Friends & Cognates Trap Detector**: Deceptive pairs (*embarazada*, *constipado*, *éxito*, *realizar*, *sensible*).
- **The "Se" Matrix (*Las 5 Caras del Se*)**: Disambiguate reflexive, reciprocal, passive, impersonal, and accidental involuntary *se* constructions.
- **Discourse Connectors & Flow (*Conectores B2/C1*)**: Complex transitional phrases (*sin embargo*, *por lo tanto*, *a pesar de que*, *ya que*, *en cuanto*).

#### Arcade Controls & Rapid Answering
- **Key 1 / J**: Select Option 1
- **Key 2 / K**: Select Option 2
- **Key 3 / L**: Select Option 3 (Engines)
- **Key 4 / ;**: Select Option 4 (Engines)
- **Speed Multipliers**: Answer in under 2,000ms for a score velocity bonus (+10 to +50 pts).

---

## ⛶ Fullscreen & Zen Mode

To eliminate browser chrome and enter an immersive terminal session:
1. Click the **⛶ Fullscreen** icon in the playground header navigation bar.
2. The playground expands to fill the entire viewport (`100vw × 100vh`) with fixed z-index elevation.
3. Click the button again or press `Esc` to restore the standard documentation layout.

---

## 💾 Local Storage & CLI Synchronization

All playground sessions maintain strict state parity with the Spanglings CLI:
- **Completed Exercises**: Synced in `spanglings_completed_exercises`.
- **Arcade High Scores**: Recorded by mode and topic in `spanglings_arcade_scores`.
- **Export / Import**: Back up your progress as JSON to migrate between browser playgrounds and your local desktop terminal.
