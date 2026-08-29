# Design Specification: Expanded 16-Pair Spanish Binary Contrast Showdowns

**Date:** 2026-08-29  
**Author:** Google DeepMind / Antigravity Pair Programming  
**Status:** DRAFT (Under User Review)

---

## 1. Motivation & Linguistic Scope

English native speakers encounter intense grammatical friction with binary Spanish distinctions where English uses a single word or construction (e.g. "have", "know", "for", "be", "ask", "take/bring", "very/much").

This expansion doubles the Showdown library from 8 to **16 distinct high-stakes Binary Contrast Showdown pairs**, with dedicated focus on **`Tener vs Haber`** ("to have") alongside other high-frequency English interference traps.

---

## 2. The 16 Binary Showdown Pairs

| Pair Slug | Option J (`[J]`/`[1]`/`[←]`) | Option K (`[K]`/`[2]`/`[→]`) | Linguistic Concept & English Confusion Traps |
| :--- | :--- | :--- | :--- |
| **`tener-haber`** | **Tener** | **Haber** | **"To Have"**: Possession (*tengo un coche*), age (*tengo 30 años*), physical states (*tengo hambre/frío*), personal obligation (*tengo que*) vs. Auxiliary (*he visto, habíamos ido*), existential (*hay/había problemas*), impersonal obligation (*hay que*). |
| **`saber-conocer`** | **Saber** | **Conocer** | **"To Know"**: Facts, data, how to do something (*sé la respuesta, sé programar*) vs. Familiarity with people, places, art (*conozco a María, conozco Madrid*). |
| **`muy-mucho`** | **Muy** | **Mucho** | **"Very / Much"**: Invariable adverb before adjectives/adverbs (*muy rápido, muy bien*) vs. Quantifier before nouns / after verbs (*mucho trabajo, muchas gracias, trabajo mucho*). |
| **`pedir-preguntar`**| **Pedir** | **Preguntar** | **"To Ask"**: Requesting an object, service, or action (*pedir ayuda, pedir la cuenta*) vs. Inquiring for information (*preguntar la hora, preguntar si...*). |
| **`llevar-traer`** | **Llevar** | **Traer** | **"Take / Bring"**: Movement away from speaker (*llévatelo a tu casa*) vs. Movement toward speaker (*tráeme un café aquí*). |
| **`haber-estar`** | **Hay / Haber** | **Está / Estar** | **"There is / Location"**: Indefinite existence (*¿Hay un banco por aquí?*) vs. Specific known location (*El banco está a la derecha*). |
| **`ir-irse`** | **Ir** | **Irse** | **"Go / Leave"**: Movement toward a destination (*voy al parque*) vs. Departure / leaving a location (*me voy de la fiesta*). |
| **`bien-bueno`** | **Bien** | **Bueno / Buen** | **"Well / Good"**: Adverb modifying verbs (*lo hiciste bien*) vs. Adjective describing nouns (*un buen plan, es una buena idea*). |
| **`por-para`** | **Por** | **Para** | **"For / By"**: Cause, means, duration, exchange vs. Destination, recipient, deadline, purpose. |
| **`ser-estar`** | **Ser** | **Estar** | **"To Be"**: Identity, essence, origin, characteristics vs. States, conditions, locations, ongoing actions. |
| **`subj-ind`** | **Subjuntivo** | **Indicativo** | **Moods**: Doubt, wishes, emotions, uncertainty vs. Facts, certainty, objective reality. |
| **`pret-imp`** | **Pretérito** | **Imperfecto** | **Past Tenses**: Completed events, bounded time vs. Habitual past, ongoing background, descriptions. |
| **`tu-usted`** | **Tú** | **Usted** | **Register**: Informal / peer address vs. Formal / professional / polite address. |
| **`lo-le`** | **Lo / La (Direct)** | **Le / Les (Indirect)** | **Pronouns**: Direct object (whom/what was acted upon) vs. Indirect object (to/for whom). |
| **`sino-pero`** | **Sino** | **Pero** | **"But"**: Affirmative replacement after negation (*no es rojo sino azul*) vs. Simple contrast (*es caro pero bueno*). |
| **`para-que-porque`**| **Para que (+subj)**| **Porque (+ind)** | **Purpose vs. Reason**: In order that / purpose (*para que aprendas*) vs. Because / causal fact (*porque tengo tiempo*). |

---

## 3. Core Engine Architecture (`src/core/arcade.rs`)

### 3.1 Enum Definition Update
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShowdownPair {
    // Original 8
    PorPara,
    SerEstar,
    SubjInd,
    PretImp,
    TuUsted,
    LoLe,
    SinoPero,
    ParaQuePorque,
    // Expanded 8
    TenerHaber,
    SaberConocer,
    MuyMucho,
    PedirPreguntar,
    LlevarTraer,
    HaberEstar,
    IrIrse,
    BienBueno,
}
```

### 3.2 High-Yield Sentence Pools
- Each new pair will include 10–12 carefully calibrated, authentic sentences covering all nuanced sub-rules (e.g. *tener que* vs *hay que*, *tener años*, *he hecho*, *hay alguien*, *saber + infinitive*, *conocer a alguien*, *muy tarde* vs *mucho tiempo*, *pedir perdón*, *preguntar por alguien*, *llevar* vs *traer*, *quedarse*, *estar bien* vs *ser bueno*).

---

## 4. CLI & TUI Updates

1. **CLI Commands**:
   - `spanglings arcade tener-haber`
   - `spanglings arcade saber-conocer`
   - `spanglings arcade muy-mucho`
   - `spanglings arcade pedir-preguntar`
   - `spanglings arcade llevar-traer`
   - `spanglings arcade haber-estar`
   - `spanglings arcade ir-irse`
   - `spanglings arcade bien-bueno`
2. **TUI Showdown Selection & Keybindings**:
   - Updated autocomplete and CLI parsing supporting all 16 slugs and aliases (e.g. `tener-haber`, `have`, `saber-conocer`, `know`, `muy-mucho`, `ask`, etc.).

---

## 5. Verification Plan
- Unit tests in `tests/arcade_tests.rs`: Ensure all 16 pairs generate valid items with distinct options and valid correct index.
- Integration tests in `tests/cli_arcade_tests.rs`: Validate CLI argument parsing and execution for all 16 showdowns.
- Linter & Formatting: `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`.
