# Functional Semantic Grammar Keys & Communicative Glosses Design Specification

## 1. Objective & Motivation

Grammatical terminology alone (e.g. `subjunctive`, `accidental_se`, `por_para`, `epistemic_conjecture`) can feel abstract and opaque to developers learning Spanish. Learners often know *what* communicative intent they want to express (wishes, hypothetical doubts, unintentional slips, deadlines vs causes) before they know the formal grammatical category name.

This feature establishes a unified `GrammarConcept` catalog across Spanglings that augments every grammar topic with a **communicative functional gloss** (e.g., `Subjunctive (wishes, hypotheses, demands)`), surfacing these descriptors across:
1. **Drill & Blitz Prompts**: In-question badge headers showing the topic and communicative purpose.
2. **`spanglings explain` / `spanglings reference`**: Interactive discovery listing showing formal topic + functional description, with semantic keyword lookups (e.g., `spanglings explain wishes`).
3. **TUI Reference Browser & Modal**: Sidebar topic lists and card headers showing both formal title and communicative gloss.
4. **Search Engine**: Matching on both grammatical terms and communicative goals.

---

## 2. Architecture & Data Model

### Central Metadata Definition (`src/core/reference.rs`)

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarConcept {
    /// Canonical CLI slug (e.g., "subjunctive", "por-para", "accidental-se")
    pub slug: &'static str,
    /// Formal human-readable title (e.g., "Subjunctive", "Por vs. Para", "Accidental 'Se'")
    pub title: &'static str,
    /// Functional communicative purpose (e.g., "wishes, hypotheses, doubt, demands")
    pub gloss: &'static str,
    /// Aliases and semantic search keywords (e.g., &["wishes", "hypotheses", "doubt", "desires", "weirdo"])
    pub keywords: &'static [&'static str],
    /// Markdown reference card content
    pub card: &'static str,
}
```

### Full 24 Grammar Concept Catalog

| Slug | Title | Communicative Functional Gloss | Primary Keywords |
| :--- | :--- | :--- | :--- |
| `subjunctive` | Subjunctive | wishes, hypotheses, doubt, demands | wishes, desires, doubt, uncertainty, hypothetical, weirdo, demands |
| `por-para` | Por vs. Para | cause/means (por) vs. purpose/destination/deadline (para) | cause, reason, means, motive, purpose, deadline, recipient, goal |
| `ser-estar` | Ser vs. Estar | essence/identity (ser) vs. states/conditions/location (estar) | identity, essence, permanent, state, condition, temporary, location |
| `past` | Past Tenses | completed actions (pret) vs. ongoing background & habit (imp) | preterite, imperfect, completed, ongoing, habit, background, narrative |
| `pronouns` | Pronoun Stacking | clitic placement & pronoun stacking (se lo) | direct, indirect, clitic, stacking, substitution, se lo, placement |
| `prepositions` | Prepositional Verbs | verb-bound prepositions (*régimen preposicional*) | prepositions, regimen, soñar con, depender de, fijarse en |
| `accidental-se` | Accidental *Se* | unintentional events & non-agentive slips (se me cayó) | accidental, unintentional, blame, dropped, forgotten, involuntary |
| `imperative` | Imperative Mood | direct commands, urgent requests, instructions | commands, orders, instructions, imperative, direct request |
| `future` | Future & Conditional | future certainty, hypothetical conjecture, polite inquiries | future, conditional, probability, speculation, polite request |
| `false-friends` | False Friends | deceptive cognates with divergent meanings | cognates, false friends, traps, actual, realizar, éxito |
| `voseo` | Voseo & Regional Address | informal singular address in Rioplatense & Central America | voseo, vos, rioplatense, argentina, uruguay, regional, informal |
| `accents` | Accentuation & Tildes | stress rules (agudas/llanas/esdrújulas) & diacritical marks | accents, tildes, stress, diacritical, agudas, llanas, esdrújulas |
| `epistemic-conjecture` | Epistemic Conjecture | guessing & deduction in present/past (serán las tres) | guessing, deduction, probability, conjecture, wondering |
| `clitic-doubling` | Clitic Doubling | redundant pronoun reinforcement for focus & clarity | doubling, redundancy, indirect object, clarity, a ella, reinforcement |
| `personal-a` | Personal *A* | accusative marker for specific human & personified entities | personal a, human direct object, specificity, personification |
| `gerund-rules` | Gerund Rules | simultaneous actions; avoiding forbidden adjectival gerunds | gerund, ando, iendo, simultaneous, progressive, adverbial |
| `adversatives` | Adversatives (*Pero* vs. *Sino*) | simple contrast (pero) vs. exclusive negative substitution (sino) | pero, sino, sino que, contrast, rectification, substitution |
| `legal-subjunctive` | Legal & Statutory Subjunctive | statutory formulations & future subjunctive in legal texts | legal, statute, contract, future subjunctive, formal, juristic |
| `verbs-of-becoming` | Verbs of Becoming | transformational change (hacerse, volverse, ponerse, quedarse) | change, transformation, become, hacerse, volverse, ponerse, quedarse |
| `epistemic-adverbs` | Epistemic Adverbs | mood selection with doubt adverbs (quizás, tal vez) | adverbs, doubt, quizas, tal vez, probablemente, acaso |
| `possessive-datives` | Possessive Datives | inalienable possession with dative clitics (me lavo las manos) | possession, inalienable, body parts, dative, me lavo |
| `corrective-polarity` | Corrective Polarity | rectifying negated premises with mandatory mood selection | polarity, negation, correction, indicative vs subjunctive |
| `participial-absolutes` | Participial Absolutes | concise temporal/causal backgrounding with past participles | absolute, participle, backgrounding, temporal, causal, terminado el |
| `scalar-concession` | Scalar Concession | intensive concessive structures (por más que, aun cuando) | concession, even though, even if, por mas que, aun cuando, siquiera |

---

## 3. Surface Integrations

### 1. Drill & Blitz Prompt Layout
- Header format:
  ```text
  Q[current]/[total] [{Title} ({gloss}) | {formula_cue}]
  Sentence: "{trigger_sentence}" (verb: {target_verb} | subject: {target_subject})
  Answer >
  ```
- If formula cue is empty, cleanly renders `[{Title} ({gloss})]`.
- Example:
  ```text
  Q1/5 [Subjunctive (wishes, hypotheses, doubt) | drop -o -> opposite vowel -a]
  Sentence: "Dudo que yo ____ los libros en la mesa." (verb: poner | subject: yo)
  ```

### 2. CLI `spanglings explain` / `spanglings reference`
- Running `spanglings reference` or `spanglings explain` (no args):
  - Formats aligned columns showing:
    - Bullet + CLI slug
    - Title + Functional Communicative Gloss
  - Displays usage helper: `Usage: spanglings explain <topic-or-intent> (e.g. spanglings explain wishes)`
- Running `spanglings explain <query>`:
  - Searches slug, aliases, title, and keywords.
  - Matches queries like `spanglings explain wishes` -> renders `SUBJUNCTIVE_CARD`.
  - Matches queries like `spanglings explain "unintentional events"` -> renders `ACCIDENTAL_SE_CARD`.

### 3. TUI Reference Browser Modal
- Sidebar list items formatted as:
  `{Title} ({gloss})`
- The TUI search bar filters by slug, title, and gloss text in real time.

---

## 4. Verification & Testing

1. **Catalog Completeness**:
   - Test in `tests/reference_tests.rs` asserting all 24 `GrammarConcept` instances have non-empty slugs, titles, functional glosses, keywords, and cards.
2. **Semantic Keyword Lookup**:
   - Test queries `"wishes"`, `"hypotheses"`, `"unintentional"`, `"deadline"`, `"body parts"` resolve accurately.
3. **Drill & Blitz Formatting**:
   - Unit tests validating prompt rendering includes the functional gloss badge.
4. **TUI Render Safety**:
   - Ensure long functional glosses are gracefully handled in terminal viewports without layout clipping or panics.
