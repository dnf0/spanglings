# Rich Context Drill Prompts & Topic Cheat Sheets Specification

**Date**: 2026-08-28  
**Status**: Approved  
**Topic**: Enhanced Grammar Prompts, Sentence Triggers, Formula Cues, and Pre-Session Cheat Sheets in `spanglings drill` and `spanglings blitz`

---

## 1. Problem Statement & Motivation
In rapid conjugation drills (`spanglings drill` and `spanglings blitz`), prompts previously relied on abstract tense nomenclature (e.g., `Present Subjunctive 'yo' form for 'poner' (que yo...)`).
For language learners, abstract grammatical terms can create ambiguity:
- Learners often confuse the target tense (e.g. entering the present indicative `yo pongo` instead of the subjunctive `ponga`).
- Grammatical tenses in Spanish are almost always learned and triggered through subordinate clauses, trigger verbs (e.g., *querer que*, *dudar que*, *es necesario que*), and systemic rules (e.g., the "opposite vowel" rule: `-ar` $\to$ `-e`, `-er/-ir` $\to$ `-a` from the indicative `yo` stem).
- Without immediate in-drill hints or pre-session concept briefings, learners must guess what the prompt is asking for rather than practicing targeted active recall.

---

## 2. Architecture & Data Model

### 2.1 Enhanced `DrillItem` Struct
Located in [`src/cli/commands/drill.rs`](file:///Users/danielfisher/repos/spanglings/src/cli/commands/drill.rs):

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrillItem {
    pub topic: &'static str,            // e.g. "subjunctive", "preterite", "por_para"
    pub formula_cue: &'static str,      // e.g. "drop -o -> opposite vowel '-a'"
    pub trigger_sentence: &'static str, // e.g. "Dudo que yo ____ los libros en la mesa."
    pub target_verb: &'static str,      // e.g. "poner"
    pub target_subject: &'static str,   // e.g. "yo"
    pub target: &'static str,           // e.g. "ponga"
    pub explanation: &'static str,      // e.g. "yo pongo -> drop -o -> add -a -> ponga"
}
```

### 2.2 Pre-Session Topic Cheat Sheets
A dedicated function `get_topic_cheat_sheet(topic: &str) -> Option<&'static str>` provides concise 3-line formula summaries rendered before drills begin:

1. **Subjunctive (`subjunctive`)**:
   ```text
   💡 Subjunctive Formula:
      1. Form Present 'yo' stem: poner ➔ pongo ➔ pong-
      2. Swap opposite vowel: -AR ➔ -e/-en, -ER/-IR ➔ -a/-an (ponga)
      3. Triggers: Wants ('quiero que'), Doubt ('dudo que'), Necessity ('es necesario que')
   ```
2. **Irregular Preterite Stems (`preterite`)**:
   ```text
   💡 Irregular Preterite Rule:
      Irregular stems (tuv-, pus-, sup-, hic-, dij-, anduv-, traj-) take unaccented endings:
      -e, -iste, -o, -imos, -ieron (e.g., yo puse, él puso, ellos pusieron)
   ```
3. **Por vs. Para (`por_para`)**:
   ```text
   💡 Por vs. Para Rule:
      • Por: Cause/Reason, Movement through, Duration, Exchange, Means
      • Para: Purpose ('in order to' + inf), Recipient, Destination, Deadline
   ```
4. **Ser vs. Estar (`ser_estar`)**:
   ```text
   💡 Ser vs. Estar Rule:
      • Ser: Essential identity, Profession, Origin, Event location ('la fiesta es en...')
      • Estar: Physical location ('el libro está en...'), Temporary states, Ongoing (-ando/-iendo)
   ```
5. **Prepositional Verbs (`prepositions`)**:
   ```text
   💡 Prepositional Verbs (Régimen Preposicional):
      soñar CON, acordarse DE, insistir EN, negarse A, contar CON, tardar EN
   ```
6. **Pronouns & Cacophony (`pronouns`)**:
   ```text
   💡 Pronoun Clitic Stacking Rule:
      Indirect Object precedes Direct Object (IOP + DOP).
      When both start with 'l' (le lo, les las), change IOP to 'se' ('se lo', 'se las').
   ```
7. **Accidental *Se* (`accidental_se`)**:
   ```text
   💡 Accidental / Unintentional 'Se':
      Pattern: [Se] + [IOP person affected: me/te/le/nos/les] + [verb agrees with object]
      e.g. Se me cayeron las llaves (The keys dropped on me).
   ```
8. **General / All Topics Summary**:
   A compact multi-tense summary rendered when drilling all topics.

---

## 3. Interactive Execution Flow

### 3.1 Prompt Layout
Each question displays the topic, the formula cue, and the context sentence:

```text
Q1/5 [Subjunctive | drop -o + opposite vowel '-a']
     Sentence: "Dudo que yo ____ los libros en la mesa." (verb: poner | subject: yo)
     Answer  > 
```

### 3.2 In-Drill Live Hinting (`?` / `hint`)
- When the learner inputs `?` or `hint`:
  - Output: `💡 Hint: yo pongo -> drop -o -> add opposite vowel -a`
  - Does **not** penalize the learner, deduct points, or reset streak.
  - Re-prompts the user with `Answer > `.

### 3.3 Evaluation & Feedback
- Handled via `evaluate_drill_answer`:
  - Exact match: `✓ Correct!`
  - Forgiving accent match: `✓ Correct! (Accent note: expected 'esté' with accent on 'e')`
  - Mismatch: `✗ Incorrect. Expected: '<target>' (<explanation>)`

---

## 4. Verification & Testing

1. **Unit Tests**:
   - `test_get_topic_cheat_sheet_all_topics`: Verify all topics have comprehensive formula briefings.
   - `test_drill_item_context_prompts`: Verify all questions contain non-empty trigger sentences, formula cues, target verbs, and explanations.
   - `test_drill_live_hint_response`: Verify `?` and `hint` return the step-by-step cue.
2. **Integration Tests**:
   - Verify `spanglings drill`, `spanglings drill <topic>`, `spanglings drill -n <count>`, and `spanglings blitz` execute cleanly with the new prompt engine.
3. **Regression Tests**:
   - All 125 existing test binaries must pass.
