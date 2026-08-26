pub const TOPICS: &[&str] = &[
    "subjunctive",
    "por-para",
    "ser-estar",
    "past",
    "pronouns",
    "prepositions",
    "accidental-se",
];

pub fn list_reference_topics() -> &'static [&'static str] {
    TOPICS
}

pub fn get_reference_card(topic: &str) -> Option<&'static str> {
    match topic.to_lowercase().replace('_', "-").as_str() {
        "subjunctive" | "subj" | "weirdo" => Some(SUBJUNCTIVE_CARD),
        "por-para" | "por_para" | "por" | "para" => Some(POR_PARA_CARD),
        "ser-estar" | "ser_estar" | "ser" | "estar" => Some(SER_ESTAR_CARD),
        "past" | "preterite" | "imperfect" | "past-aspect" => Some(PAST_TENSES_CARD),
        "pronouns" | "clitics" | "stacking" | "direct-indirect" => Some(PRONOUN_STACKING_CARD),
        "prepositions" | "regimen" | "prep" => Some(PREPOSITIONS_CARD),
        "accidental-se" | "se-accidental" | "accidental" => Some(ACCIDENTAL_SE_CARD),
        _ => None,
    }
}

pub const SUBJUNCTIVE_CARD: &str = r#"
================================================================================
                     SPANISH SUBJUNCTIVE CHEAT SHEET (WEIRDO)
================================================================================
Triggers require: [Main Clause Verb] + "QUE" + [Subject Change]

  W - Wishes / Desires:       querer, desear, preferir, esperar, exigir
  E - Emotions:               alegrarse de, sentir, temer, molestar, encantar
  I - Impersonal Expressions: es necesario que, es importante que, es bueno que
  R - Recommendations:        recomendar, aconsejar, sugerir, pedir
  D - Doubt / Denial:         dudar, no creer, no pensar, negar, no estar seguro
  O - Ojalá:                  ojalá (que)... (Always triggers subjunctive)

Present Subjunctive Endings & Formation:
  1. Take the "YO" form of the present indicative (e.g. tengo, hablo, como)
  2. Drop the "-o"
  3. Add OPPOSITE endings:
     -AR verbs:  -e,  -es,  -e,  -emos,  -éis,  -en
     -ER/-IR:    -a,  -as,  -a,  -amos,  -áis,  -an

DOUBT VS CERTAINTY:
  Indicative:  Creo que viene. / Es cierto que viene. / No dudo que viene.
  Subjunctive: No creo que venga. / No es cierto que venga. / Dudo que venga.
================================================================================
"#;

pub const POR_PARA_CARD: &str = r#"
================================================================================
                          POR VS PARA REFERENCE CARD
================================================================================
POR (Cause, Motive, Passage, Exchange):
  - Cause / Reason:           Lo hizo por amor. (Out of love)
  - Means / Communication:    Hablamos por teléfono. / Enviado por correo.
  - Duration / Approximate:   Estudié por tres horas. / Por la mañana.
  - Movement through/along:   Caminamos por el parque.
  - Exchange / Price:         Te doy 10 euros por el libro.
  - In search of:             Fue a la tienda por leche.

PARA (Goal, Recipient, Deadline, Destination):
  - Purpose / "In order to":  Estudio para aprender. (para + infinitive)
  - Recipient:                Este regalo es para ti.
  - Deadline / Specific time: La tarea es para el lunes.
  - Destination:              Salgo para Madrid mañana.
  - Opinion:                  Para mí, es la mejor opción.
  - Standard of comparison:   Para un niño, habla muy bien.
================================================================================
"#;

pub const SER_ESTAR_CARD: &str = r#"
================================================================================
                         SER VS ESTAR REFERENCE CARD
================================================================================
SER (Identity, Essence, Characteristics, Origin, Time):
  - Identity & Profession:    Soy ingeniero. / Es Daniel.
  - Origin & Material:        Soy de España. / La mesa es de madera.
  - Inherent characteristics: Es alto, inteligente y generoso.
  - Time, Date, Events:       Son las tres. / La fiesta es en mi casa.

ESTAR (States, Conditions, Locations, Progressive):
  - Physical Location:        El libro está en la mesa. / Estoy en Londres.
  - Temporary condition/mood: Está cansado. / Está rota la ventana.
  - Present Continuous:       Estoy estudiando español.

ADJECTIVE MEANING SHIFTS:
  - ser listo (smart)         vs estar listo (ready)
  - ser rico (wealthy)        vs estar rico (delicious - food)
  - ser atento (courteous)    vs estar atento (paying attention)
  - ser verde (green color)   vs estar verde (unripe / inexperienced)
================================================================================
"#;

pub const PAST_TENSES_CARD: &str = r#"
================================================================================
                    PRETERITE VS IMPERFECT REFERENCE CARD
================================================================================
PRETERITE (Completed, Bounded Actions):
  - Specific completed event: Ayer compré un coche.
  - Action with time limit:   Vivió en Madrid durante cinco años.
  - Chain of events:          Llegó, abrió la puerta y salió.
  - Interrupting action:      ...cuando sonó el teléfono.

IMPERFECT (Ongoing, Habitual, Background Setting):
  - Habitual past actions:    De niño, jugaba en la calle todos los días.
  - Ongoing background:       Llovía y hacía frío.
  - Age, Time, Weather:       Tenía 20 años. / Eran las seis.
  - Mental/Emotional state:   Quería salir, pero no sabía adónde ir.

MEANING CHANGERS:
  - conocer: conocí (met for 1st time)  vs conocía (knew / was familiar)
  - saber:   supe (found out / learned) vs sabía (knew information)
  - querer:  quise (attempted/tried)    vs quería (wanted/desired)
  - poder:   pude (managed/succeeded)   vs podía (had the capability)
================================================================================
"#;

pub const PRONOUN_STACKING_CARD: &str = r#"
================================================================================
                    DOUBLE OBJECT PRONOUNS & ACCENTS
================================================================================
ORDER RULE: [REFLEXIVE] -> [INDIRECT] -> [DIRECT]
  Indirect: me, te, le, nos, os, les
  Direct:   me, te, lo/la, nos, os, los/las

THE "LE / LES -> SE" RULE (Avoiding Cacophony):
  When Indirect (le/les) is followed by Direct (lo/la/los/las), 'le' becomes 'SE'
  to avoid the awkward consecutive 'l' sounds ("le lo"):
  * Le lo doy -> SE LO DOY.

PLACEMENT RULES:
  1. BEFORE conjugated verb:  "Se lo dije ayer."
  2. ATTACHED to infinitive:  "Voy a decírselo." (Needs written accent!)
  3. ATTACHED to gerund:      "Estoy explicándotelo." (Needs written accent!)
  4. ATTACHED to command:     "¡Dímelo ahora!" (Needs written accent!)
================================================================================
"#;

pub const PREPOSITIONS_CARD: &str = r#"
================================================================================
                     VERBAL REGIMEN (VERBS + PREPOSITIONS)
================================================================================
Common verb + preposition combinations (régimen preposicional):
  - a:       acostumbrarse a, atreverse a, negarse a, aspirar a, comprometerse a
  - de:      acordarse de, alegrarse de, enterarse de, quejarse de, depender de
  - en:      pensar en, confiar en, insistir en, tardar en, fijarse en
  - con:     contar con, soñar con, conformarse con, tropezar con, casarse con
  - por:     preocuparse por, interesarse por, luchar por, votar por
================================================================================
"#;

pub const ACCIDENTAL_SE_CARD: &str = r#"
================================================================================
                       ACCIDENTAL / UNINTENDED "SE"
================================================================================
STRUCTURE:
  "Se" + [Indirect Object Pronoun] + [Verb] + [Subject (the dropped item)]

PRONOUNS: me, te, le, nos, os, les (indicates who was affected/blamed)

EXAMPLES:
  - Se me cayeron las llaves.     (I dropped the keys / The keys fell from me.)
  - Se le olvidó la cartera.      (He/she forgot the wallet.)
  - Se nos rompió el vaso.        (We broke the glass / The glass broke on us.)
  - Se te quemó la comida.        (You burned the food.)

COMMON VERBS: caer, olvidar, romper, perder, acabar, quemar, descomponer
================================================================================
"#;
