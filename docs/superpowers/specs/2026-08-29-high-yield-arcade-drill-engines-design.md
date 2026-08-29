# Design Specification: 5 High-Yield Advanced Drill Engines

**Date:** 2026-08-29  
**Author:** Google DeepMind / Antigravity Pair Programming  
**Status:** DRAFT (Under Subagent Review)

---

## 1. Motivation & Overview

To accelerate Spanish mastery for neurodivergent and rapid-fire learners, we introduce 5 specialized High-Yield Drill Engines into `spanglings arcade` (and TUI Arena `[x]`):

1. 🎯 **Prepositional Regimen Engine (`regimen` / `prepositions`)**: Master non-intuitive verb-preposition bonds (*soñar con, pensar en, acordarse de, fijarse en, depender de*).
2. ⚡ **Irregular Verb Speed Gun (`irregulars` / `verbs`)**: Instantaneous single-key recognition of stem-changing irregular preterite, present subjunctive, and future forms.
3. ⚠️ **False Friends Trap Detector (`false-friends` / `cognates`)**: Rapid override training for deceptive English-Spanish cognates (*actual, embarazada, éxito, atender, sensible, largo, carpeta, soportar, pretender, realizar*).
4. 🧩 **The "Se" Matrix (`se-matrix` / `se`)**: Master the 5 faces of Spanish *Se* (Accidental Involuntary Dative, Impersonal, Passive Reflexive, Reciprocal, Aspectual).
5. 💬 **Discourse Connectors & Flow (`connectors` / `discourse`)**: B2/C1 sentence transition connectors (*sin embargo, no obstante, por lo tanto, dado que, a pesar de que, por ende*).

---

## 2. Engine Architecture & Topic Definitions

Each engine is accessible via dedicated topic slugs and aliases in both CLI (`spanglings arcade <topic>`) and TUI Arena (`[x]` -> `[s]` or `[Tab]`), and automatically integrates into the default mixed arcade pool.

### 2.1 Engine Details

| Engine Slug | Aliases | Question Count & Variety | Mechanics & Format |
| :--- | :--- | :--- | :--- |
| **`regimen`** | `prepositions`, `prep` | 16 curated sentences across 6 preposition classes (`de`, `en`, `con`, `a`, `por`, `para`) | Cloze with 4-choice preposition distractors + concise explanation of the verb regimen rule. |
| **`irregulars`** | `verbs`, `irregular` | 16 curated sentences targeting irregular stems (*quise, cupo, traje, conduje, anduve, sepa, quepo, valgo, haya, pondré*) | Rapid tense/mood cloze with authentic morphological distractors. |
| **`false-friends`** | `cognates`, `falsos-amigos` | 16 curated sentences targeting classic traps (*actual vs real, éxito vs salida, embarazada vs avergonzado, atender vs asistir, sensible vs sensato, largo vs grande, carpeta vs alfombra, soportar vs apoyar*) | 4-choice or 2-choice duel disambiguating the intended meaning from the deceptive English false cognate. |
| **`se-matrix`** | `se`, `se-types` | 16 curated sentences contrasting the 5 types of *Se* (accidental *se me cayó*, impersonal *se vive bien*, passive *se venden*, reciprocal *se ayudan*, aspectual *se comió*) | Cloze or functional classification identifying the correct pronoun combination or usage context. |
| **`connectors`** | `discourse`, `transitions` | 16 curated sentences covering causal, adversative, concessive, and consecutive connectors (*sin embargo, no obstante, por lo tanto, en cambio, dado que, a pesar de que, por ende*) | Cloze testing discourse flow and register. |

---

## 3. Integration Points

1. **`src/core/arcade.rs`**:
   - Define dedicated sentence pools: `ENGINE_REGIMEN_POOL`, `ENGINE_IRREGULARS_POOL`, `ENGINE_FALSE_FRIENDS_POOL`, `ENGINE_SE_MATRIX_POOL`, `ENGINE_CONNECTORS_POOL`.
   - Update `generate_specialized_engine_items(slug: &str, count: usize) -> Vec<ArcadeItem>`.
   - Update `select_arcade_items` to resolve engine slugs in topic matching and include them in the default mixed mode pool.
2. **`src/cli/commands/arcade.rs` & `src/main.rs`**:
   - Support CLI arguments: `spanglings arcade regimen`, `spanglings arcade irregulars`, `spanglings arcade false-friends`, `spanglings arcade se`, `spanglings arcade connectors`.
3. **`src/tui/app.rs` & `src/tui/ui.rs`**:
   - TUI arena header and cycling displays engine titles cleanly.
4. **Testing**:
   - `tests/arcade_tests.rs`: Validate all 5 engines generate valid non-empty items with 4 distinct choices, valid correct index, and rich explanations.
   - `tests/cli_arcade_tests.rs`: Validate CLI argument dispatch and selection for all 5 engine slugs and aliases.
   - `tests/tui_arcade_tests.rs`: Validate TUI arcade arena rendering and single-key navigation across all 5 engines.

